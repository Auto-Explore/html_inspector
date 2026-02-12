#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Profile the validator HTTP server (html_inspector_cli serve) with Callgrind, with instrumentation toggled via callgrind_control.

Usage:
  profile_callgrind_server.sh [options]

Options:
  --bind IP            Bind address (default: 127.0.0.1)
  --port PORT          Port (default: 8888)
  --max-bytes N        Max request body bytes (default: 20971520)
  --profile P          Cargo profile: release-with-debug | release | debug (default: release-with-debug)
  --out-dir DIR        Output directory (default: <repo>/valgrind-out)
  --out-file FILE      Callgrind output file (default: <out-dir>/callgrind.out)
  --data-file PATH     File to POST as request body, or '-' for stdin (default: <repo>/tests/template.html if present)
  --content-type TYPE  Content-Type for POST (default: text/html; charset=utf-8)
  --query Q            Query string (default: out=json&doc=<basename of data file>)
  --no-build           Skip cargo build step
  --keep-running       Don't stop server after request; prints PID and waits

Example:
  ./scripts/profile_callgrind_server.sh --data-file tests/template.html --query 'out=json&also_check_css=1'

Outputs:
  - Callgrind data:  <out-file>
  - Server log:      <out-dir>/server.log
  - Last response:   <out-dir>/response.json

View:
  kcachegrind <out-file>
  cg_annotate <out-file> | less
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

bind="127.0.0.1"
port="8888"
max_bytes="20971520"
profile="release-with-debug"
out_dir=""
out_file=""
data_file=""
content_type="text/html; charset=utf-8"
query=""
no_build="false"
keep_running="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bind) bind="${2:-}"; shift 2 ;;
    --port) port="${2:-}"; shift 2 ;;
    --max-bytes) max_bytes="${2:-}"; shift 2 ;;
    --profile) profile="${2:-}"; shift 2 ;;
    --out-dir) out_dir="${2:-}"; shift 2 ;;
    --out-file) out_file="${2:-}"; shift 2 ;;
    --data-file) data_file="${2:-}"; shift 2 ;;
    --content-type) content_type="${2:-}"; shift 2 ;;
    --query) query="${2:-}"; shift 2 ;;
    --no-build) no_build="true"; shift ;;
    --keep-running) keep_running="true"; shift ;;
    *)
      echo "unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "${profile}" in
  release-with-debug|release|debug) ;;
  *)
    echo "invalid --profile: ${profile} (expected release-with-debug|release|debug)" >&2
    exit 2
    ;;
esac

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ae_dir="$(cd -- "${script_dir}/.." && pwd)"
repo_dir="${ae_dir}"

if [[ -z "${out_dir}" ]]; then
  out_dir="${ae_dir}/valgrind-out"
fi
mkdir -p "${out_dir}"

if [[ -z "${data_file}" ]]; then
  if [[ -f "${repo_dir}/tests/template.html" ]]; then
    data_file="${repo_dir}/tests/template.html"
  else
    echo "missing --data-file (and default ${repo_dir}/tests/template.html not found)" >&2
    exit 2
  fi
fi

if [[ -z "${query}" ]]; then
  if [[ "${data_file}" == "-" ]]; then
    query="out=json&doc=stdin"
  else
    query="out=json&doc=$(basename -- "${data_file}")"
  fi
fi

if [[ -z "${out_file}" ]]; then
  out_file="${out_dir}/callgrind.out"
fi

if ! command -v valgrind >/dev/null 2>&1; then
  echo "valgrind not found in PATH" >&2
  echo "Install it (e.g. Debian/Ubuntu: sudo apt-get install valgrind)" >&2
  exit 127
fi
if ! command -v callgrind_control >/dev/null 2>&1; then
  echo "callgrind_control not found in PATH (usually provided by valgrind)" >&2
  exit 127
fi
if ! command -v curl >/dev/null 2>&1; then
  echo "curl not found in PATH" >&2
  exit 127
fi

if [[ "${data_file}" != "-" && ! -f "${data_file}" ]]; then
  echo "data file not found: ${data_file}" >&2
  exit 2
fi

bin_dir="${ae_dir}/target/${profile}"
if [[ "${profile}" == "debug" ]]; then
  bin_dir="${ae_dir}/target/debug"
fi
bin_path="${bin_dir}/html_inspector_cli"

if [[ "${no_build}" != "true" ]]; then
  build_args=(build -p html_inspector_cli --manifest-path "${ae_dir}/Cargo.toml" --profile "${profile}")
  if [[ "${profile}" == "debug" ]]; then
    build_args=(build -p html_inspector_cli --manifest-path "${ae_dir}/Cargo.toml")
  fi

  export CARGO_INCREMENTAL=0
  export RUSTFLAGS="${RUSTFLAGS:-} -C force-frame-pointers=yes"

  (cd "${ae_dir}" && cargo "${build_args[@]}")
fi

if [[ ! -x "${bin_path}" ]]; then
  echo "binary not found: ${bin_path}" >&2
  echo "Try running without --no-build, or build it manually:" >&2
  echo "  (cd ${ae_dir} && cargo build -p html_inspector_cli --profile ${profile})" >&2
  exit 1
fi

server_log="${out_dir}/server.log"
resp_file="${out_dir}/response.json"
rm -f "${server_log}" "${resp_file}"

valgrind \
  --tool=callgrind \
  --callgrind-out-file="${out_file}" \
  --dump-instr=yes \
  --instr-atstart=no \
  --collect-jumps=yes \
  --sigill-diagnostics=no \
  --error-limit=no \
  "${bin_path}" serve "${port}" --bind "${bind}" --max-bytes "${max_bytes}" \
  >"${server_log}" 2>&1 &
vg_pid="$!"

cleanup() {
  if kill -0 "${vg_pid}" 2>/dev/null; then
    kill -INT "${vg_pid}" 2>/dev/null || true
    wait "${vg_pid}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

base_url="http://${bind}:${port}"

deadline_s=10
start_s="$(date +%s)"
while true; do
  if curl -fsS "${base_url}/health" >/dev/null 2>&1; then
    break
  fi
  now_s="$(date +%s)"
  if (( now_s - start_s > deadline_s )); then
    echo "server did not become healthy within ${deadline_s}s (see ${server_log})" >&2
    exit 1
  fi
  sleep 0.05
done

callgrind_control -i on "${vg_pid}" >/dev/null
curl -fsS \
  -H "Content-Type: ${content_type}" \
  --data-binary @"${data_file}" \
  "${base_url}/?${query}" \
  >"${resp_file}"
callgrind_control -i off "${vg_pid}" >/dev/null
callgrind_control --dump "${vg_pid}" >/dev/null

if [[ "${keep_running}" == "true" ]]; then
  echo "server still running under valgrind (pid=${vg_pid})" >&2
  echo "toggle: callgrind_control -i on|off ${vg_pid}" >&2
  echo "dump:   callgrind_control --dump ${vg_pid}" >&2
  echo "stop:   kill -INT ${vg_pid}" >&2
  wait "${vg_pid}" || true
  trap - EXIT
  exit 0
fi

cleanup
trap - EXIT

shopt -s nullglob
callgrind_outputs=()
if [[ -f "${out_file}" ]]; then
  callgrind_outputs+=("${out_file}")
fi
callgrind_outputs+=("${out_file}".*)
shopt -u nullglob

if [[ "${#callgrind_outputs[@]}" -eq 0 ]]; then
  echo "no callgrind output found at ${out_file} (see ${server_log})" >&2
  exit 1
fi

echo "wrote:" >&2
for f in "${callgrind_outputs[@]}"; do
  echo "  ${f}" >&2
done
echo "log:   ${server_log}" >&2
echo "resp:  ${resp_file}" >&2
