#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v llvm-profdata >/dev/null 2>&1; then
  echo "error: llvm-profdata not found in PATH" >&2
  exit 1
fi
if ! command -v llvm-cov >/dev/null 2>&1; then
  echo "error: llvm-cov not found in PATH" >&2
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "error: python3 not found in PATH (needed to parse cargo JSON output)" >&2
  exit 1
fi

OUT_DIR="${COVERAGE_OUT_DIR:-target/coverage}"
HTML_DIR="$OUT_DIR/html"

# Filters out toolchains, deps, and .cargo artifacts from reports.
IGNORE_FILENAME_REGEX="${COVERAGE_IGNORE_FILENAME_REGEX:-(^|.*/)[.](cargo|rustup)(/|$)|/rustc/|/target/|/registry/|/\\.git/}"

# Filter noisy LLVM profile runtime errors (coverage files are still produced for other processes).
exec 3>&2
exec 2> >(sed '/^LLVM Profile Error: Failed to write file /d' >&3)

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

export CARGO_INCREMENTAL=0
export LLVM_PROFILE_FILE="$ROOT/$OUT_DIR/%p-%m.profraw"
export RUSTFLAGS="${RUSTFLAGS:-} -C instrument-coverage"
export CARGO_TARGET_DIR="$ROOT/$OUT_DIR/cargo-target"
export html_inspector_RUN_FULL_VNU=1

# Extra args passed to cargo, e.g. `COVERAGE_CARGO_ARGS="--all-features"`.
read -r -a CARGO_ARGS <<<"${COVERAGE_CARGO_ARGS:-}"

mapfile -t TEST_BINARIES < <(
  cargo test --workspace --no-run --message-format=json "${CARGO_ARGS[@]}" |
    python3 -c '
import json, sys
exes=set()
for line in sys.stdin:
    try:
        msg=json.loads(line)
    except Exception:
        continue
    if msg.get("reason")!="compiler-artifact":
        continue
    exe=msg.get("executable")
    if not exe:
        continue
    profile=msg.get("profile") or {}
    if not profile.get("test", False):
        continue
    exes.add(exe)
for exe in sorted(exes):
    print(exe)
'
)

if [[ ${#TEST_BINARIES[@]} -eq 0 ]]; then
  echo "error: failed to find any test executables from cargo JSON output" >&2
  exit 1
fi

cargo test --workspace "${CARGO_ARGS[@]}" 1>&2

LLVM_COV_OBJECT_ARGS=("${TEST_BINARIES[0]}")
for ((i=1; i<${#TEST_BINARIES[@]}; i++)); do
  LLVM_COV_OBJECT_ARGS+=(--object "${TEST_BINARIES[i]}")
done

# Include workspace binaries that tests may spawn via CARGO_BIN_EXE_*.
if [[ -x "$CARGO_TARGET_DIR/debug/html_inspector_cli" ]]; then
  LLVM_COV_OBJECT_ARGS+=(--object "$CARGO_TARGET_DIR/debug/html_inspector_cli")
fi

shopt -s nullglob
PROFRAWS=("$OUT_DIR"/*.profraw)
shopt -u nullglob
if [[ ${#PROFRAWS[@]} -eq 0 ]]; then
  echo "error: no profraw files found in $OUT_DIR (did tests run?)" >&2
  exit 1
fi

llvm-profdata merge -sparse "${PROFRAWS[@]}" -o "$OUT_DIR/coverage.profdata"

llvm-cov export \
  --instr-profile="$OUT_DIR/coverage.profdata" \
  --ignore-filename-regex="$IGNORE_FILENAME_REGEX" \
  "${LLVM_COV_OBJECT_ARGS[@]}" >"$OUT_DIR/coverage.json"

printf "%s\n" "${TEST_BINARIES[@]}" >"$OUT_DIR/test_binaries.txt"

llvm-cov report \
  --ignore-filename-regex="$IGNORE_FILENAME_REGEX" \
  --instr-profile="$OUT_DIR/coverage.profdata" \
  "${LLVM_COV_OBJECT_ARGS[@]}" >"$OUT_DIR/report.txt"

llvm-cov report \
  --summary-only \
  --ignore-filename-regex="$IGNORE_FILENAME_REGEX" \
  --instr-profile="$OUT_DIR/coverage.profdata" \
  "${LLVM_COV_OBJECT_ARGS[@]}"

llvm-cov show \
  --format=html \
  --output-dir="$HTML_DIR" \
  --ignore-filename-regex="$IGNORE_FILENAME_REGEX" \
  --instr-profile="$OUT_DIR/coverage.profdata" \
  "${LLVM_COV_OBJECT_ARGS[@]}" >/dev/null

python3 - "$ROOT" "$OUT_DIR/coverage.json" <<'PY'
import json, os, sys
root = sys.argv[1]
path = sys.argv[2]
data = json.load(open(path))
files = data["data"][0].get("files", [])
rows = []
for f in files:
    name = f.get("filename", "")
    # Focus on repo crates; toolchain/deps are filtered by ignore regex.
    if not name.startswith(root + "/crates/"):
        continue
    summ = (f.get("summary") or {}).get("lines") or {}
    pct = summ.get("percent", 0)
    covered = summ.get("covered", 0)
    count = summ.get("count", 0)
    rows.append((pct, covered, count, name))
rows.sort(key=lambda r: (r[0], r[3]))
print("\nLowest file line coverage (within crates/):")
for pct, covered, count, name in rows[:20]:
    print(f"{pct:6.2f}%  {covered:5d}/{count:<5d}  {name}")
PY

echo "coverage: $OUT_DIR/report.txt"
echo "coverage: $OUT_DIR/coverage.json"
echo "coverage: $OUT_DIR/test_binaries.txt"
echo "coverage: $HTML_DIR/index.html"
