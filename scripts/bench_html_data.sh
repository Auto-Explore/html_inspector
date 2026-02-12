#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Profile the HTML data benchmark (html_inspector_html_data_bench) with Valgrind.

Usage:
  bench_html_data.sh [options] [--] <html_inspector_html_data_bench args...>

Options:
  --tool TOOL       callgrind | massif | memcheck (default: callgrind)
  --profile P       release-with-debug | release | debug (default: release-with-debug)
  --out-dir DIR     Output directory (default: <repo>/valgrind-out)
  --no-build        Skip cargo build step
  --trace-children  Profile child processes too (e.g. the spawned Rust server)

Examples:
  # Profile the benchmark binary itself (callgrind.out.*)
  ./scripts/bench_html_data.sh -- --iterations 3 --warmup 1

  # Also profile spawned child processes (very slow; multiple outputs)
  ./scripts/bench_html_data.sh --trace-children -- --iterations 1 --warmup 0

Viewing results:
  callgrind:  kcachegrind <out-dir>/callgrind.out.*
              cg_annotate <out-dir>/callgrind.out.* | less
  massif:     ms_print <out-dir>/massif.out.* | less
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

tool="callgrind"
profile="release-with-debug"
out_dir=""
no_build="false"
trace_children="false"
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
    --trace-children)
      trace_children="true"
      shift
      ;;
    --)
      shift
      app_args+=("$@")
      break
      ;;
    *)
      # Preserve backwards compatibility: treat unknown flags/args as app args.
      app_args+=("$1")
      shift
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

case "${tool}" in
  callgrind|massif|memcheck) ;;
  *)
    echo "invalid --tool: ${tool} (expected callgrind|massif|memcheck)" >&2
    exit 2
    ;;
esac

if ! command -v valgrind >/dev/null 2>&1; then
  echo "valgrind not found in PATH" >&2
  echo "Install it (e.g. Debian/Ubuntu: sudo apt-get install valgrind)" >&2
  exit 127
fi

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ae_dir="$(cd -- "${script_dir}/.." && pwd)"

if [[ -z "${out_dir}" ]]; then
  out_dir="${ae_dir}/valgrind-out"
fi
mkdir -p "${out_dir}"

target_dir_name="${profile}"
if [[ "${profile}" == "debug" ]]; then
  target_dir_name="debug"
fi
bin_path="${ae_dir}/target/${target_dir_name}/html_inspector_html_data_bench"

if [[ "${no_build}" != "true" ]]; then
  build_args=(build --offline -p html_inspector_html_data_bench --manifest-path "${ae_dir}/Cargo.toml")
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
  if [[ "${profile}" == "debug" ]]; then
    echo "  (cd ${ae_dir} && cargo build -p html_inspector_html_data_bench)" >&2
  elif [[ "${profile}" == "release" ]]; then
    echo "  (cd ${ae_dir} && cargo build -p html_inspector_html_data_bench --release)" >&2
  else
    echo "  (cd ${ae_dir} && cargo build -p html_inspector_html_data_bench --profile release-with-debug)" >&2
  fi
  exit 1
fi

valgrind_common=()
if [[ "${trace_children}" == "true" ]]; then
  valgrind_common+=(--trace-children=yes)
fi

case "${tool}" in
  callgrind)
    exec valgrind \
      "${valgrind_common[@]}" \
      --tool=callgrind \
      --callgrind-out-file="${out_dir}/callgrind.out.%p" \
      --collect-jumps=yes \
      --branch-sim=yes \
      --num-callers=30 \
      "${bin_path}" \
      --rust-only \
      --release \
      "${app_args[@]}"
    ;;
  massif)
    exec valgrind \
      "${valgrind_common[@]}" \
      --tool=massif \
      --massif-out-file="${out_dir}/massif.out.%p" \
      --stacks=yes \
      --time-unit=ms \
      "${bin_path}" \
      --rust-only \
      --release \
      "${app_args[@]}"
    ;;
  memcheck)
    exec valgrind \
      "${valgrind_common[@]}" \
      --tool=memcheck \
      --leak-check=full \
      --show-leak-kinds=all \
      --track-origins=yes \
      --error-exitcode=99 \
      "${bin_path}" \
      --rust-only \
      --release \
      "${app_args[@]}"
    ;;
esac
