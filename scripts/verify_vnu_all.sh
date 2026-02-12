#!/usr/bin/env bash
set -euo pipefail

cat <<'EOF'
scripts/verify_vnu_all.sh: removed

The in-repo vnu suite runner (`html_inspector_cli vnu` / `html_inspector_vnu`) has been removed.

Use:
  - `cargo test --workspace` for Rust tests
  - `cargo run -p html_inspector_cli -- serve ...` to compare against a running Java vnu server
EOF
exit 1
