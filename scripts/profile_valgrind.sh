#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Profile the Rust validator (html_inspector_cli) with Valgrind.

Usage:
  profile_valgrind.sh [--tool TOOL] [--profile P] [--out-dir DIR] [--no-build] [--] <html_inspector_cli args...>

Options:
  --tool TOOL       callgrind | massif | dhat | memcheck (default: callgrind)
  --profile P       release-with-debug | release | debug (default: release)
  --out-dir DIR     Output directory (default: <repo>/valgrind-out)
  --no-build        Skip cargo build step

Examples:
  # CPU profiling (generates callgrind.out.*)
  ./scripts/profile_valgrind.sh --tool callgrind -- check tests/template.html

  # Heap profiling (generates massif.out.*)
  ./scripts/profile_valgrind.sh --tool massif -- file tests/template.html --also-check-css

  # Allocation profiling (generates dhat.out.*; parse with tools/scripts or open in DHAT viewer)
  ./scripts/profile_valgrind.sh --tool dhat -- file tests/template.html

  # Memory correctness + leaks (slow)
  ./scripts/profile_valgrind.sh --tool memcheck -- check tests/template.html

Viewing results:
  callgrind:  kcachegrind <out-dir>/callgrind.out.*
              cg_annotate <out-dir>/callgrind.out.* | less
  massif:     ms_print <out-dir>/massif.out.* | less
  dhat:       file is JSON; see Valgrind DHAT docs for visualization tools
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

tool="callgrind"
profile="release"
out_dir=""
no_build="false"
app_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --tool)
      tool="${2:-}"
      shift 2
      ;;
    --profile)
      profile="${2:-}"
      shift 2
      ;;
    --out-dir)
      out_dir="${2:-}"
      shift 2
      ;;
    --no-build)
      no_build="true"
      shift
      ;;
    --)
      shift
      app_args+=("$@")
      break
      ;;
    -*)
      echo "unknown option: $1" >&2
      echo "If this is an html_inspector_cli argument, add '--' before it." >&2
      usage >&2
      exit 2
      ;;
    *)
      app_args+=("$1")
      shift
      ;;
  esac
done

if [[ "${#app_args[@]}" -eq 0 ]]; then
  echo "missing html_inspector_cli args" >&2
  usage >&2
  exit 2
fi

case "${profile}" in
  release-with-debug|release|debug) ;;
  *)
    echo "invalid --profile: ${profile} (expected release-with-debug|release|debug)" >&2
    exit 2
    ;;
esac

case "${tool}" in
  callgrind|massif|dhat|memcheck) ;;
  *)
    echo "invalid --tool: ${tool} (expected callgrind|massif|dhat|memcheck)" >&2
    exit 2
    ;;
esac

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ae_dir="$(cd -- "${script_dir}/.." && pwd)"

if [[ -z "${out_dir}" ]]; then
  out_dir="${ae_dir}/valgrind-out"
fi
mkdir -p "${out_dir}"

if ! command -v valgrind >/dev/null 2>&1; then
  echo "valgrind not found in PATH" >&2
  echo "Install it (e.g. Debian/Ubuntu: sudo apt-get install valgrind)" >&2
  exit 127
fi

target_dir_name="${profile}"
if [[ "${profile}" == "debug" ]]; then
  target_dir_name="debug"
fi
bin_path="${ae_dir}/target/${target_dir_name}/html_inspector_cli"

if [[ "${no_build}" != "true" ]]; then
  build_args=(build -p html_inspector_cli --manifest-path "${ae_dir}/Cargo.toml")
  if [[ "${profile}" == "release" ]]; then
    build_args+=(--release)
  elif [[ "${profile}" == "release-with-debug" ]]; then
    build_args+=(--profile release-with-debug)
  fi

  # Keep symbols + stable stack traces for profiling tools.
  export CARGO_INCREMENTAL=0
  export RUSTFLAGS="${RUSTFLAGS:-} -C debuginfo=2 -C force-frame-pointers=yes"

  (cd "${ae_dir}" && cargo "${build_args[@]}")
fi

if [[ ! -x "${bin_path}" ]]; then
  echo "binary not found: ${bin_path}" >&2
  echo "Try running without --no-build, or build it manually:" >&2
  echo "  (cd ${ae_dir} && cargo build -p html_inspector_cli --release)" >&2
  exit 1
fi

case "${tool}" in
  callgrind)
    exec valgrind \
      --tool=callgrind \
      --callgrind-out-file="${out_dir}/callgrind.out.%p" \
      --collect-jumps=yes \
      --branch-sim=yes \
      --num-callers=30 \
      "${bin_path}" "${app_args[@]}"
    ;;
  massif)
    exec valgrind \
      --tool=massif \
      --massif-out-file="${out_dir}/massif.out.%p" \
      --stacks=yes \
      --time-unit=ms \
      "${bin_path}" "${app_args[@]}"
    ;;
  dhat)
    exec valgrind \
      --tool=dhat \
      --dhat-out-file="${out_dir}/dhat.out.%p" \
      --show-top-n=50 \
      "${bin_path}" "${app_args[@]}"
    ;;
  memcheck)
    exec valgrind \
      --tool=memcheck \
      --leak-check=full \
      --show-leak-kinds=all \
      --track-origins=yes \
      --error-exitcode=99 \
      "${bin_path}" "${app_args[@]}"
    ;;
esac
