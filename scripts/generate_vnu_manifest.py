#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path


CHECKABLE_SUFFIXES = {".html", ".htm", ".xhtml", ".xht"}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def iter_checkable_files(root: Path) -> list[Path]:
    out: list[Path] = []
    for p in root.rglob("*"):
        if not p.is_file():
            continue
        if p.suffix.lower() in CHECKABLE_SUFFIXES:
            out.append(p)
    out.sort(key=lambda p: p.relative_to(root).as_posix())
    return out


def is_under_dir_named(path: Path, root: Path, dirname: str) -> bool:
    rel = path.relative_to(root)
    return dirname in rel.parts[:-1]


def classify_kind(path: Path, root: Path) -> str:
    rel_posix = path.relative_to(root).as_posix()
    if is_under_dir_named(path, root, "invalid"):
        return "invalid"
    if is_under_dir_named(path, root, "valid"):
        return "valid"
    if "novalid" in rel_posix:
        return "invalid"
    if "haswarn" in rel_posix:
        return "haswarn"
    if "hasinfo" in rel_posix:
        return "hasinfo"
    return "valid"


def format_for_path(path: Path) -> str:
    if path.suffix.lower() in {".xhtml", ".xht"}:
        return "xhtml"
    return "html"


@dataclass(frozen=True)
class Options:
    tests_dir: Path
    out: Path


def build_record(
    *,
    rel_posix: str,
    kind: str,
    fmt: str,
) -> dict[str, Any]:
    expect: dict[str, Any] = {}
    expect["max_errors"] = 0

    return {
        "id": rel_posix,
        "input_path": rel_posix,
        "format": fmt,
        "schema": "html",
        "config": {"ignore_missing_lang": True},
        "kind": kind,
        "expected_result": "valid",
        "expect": expect,
    }


def run(opts: Options) -> int:
    tests_dir = opts.tests_dir.resolve()
    if not tests_dir.is_dir():
        raise SystemExit(f"missing tests dir: {tests_dir}")

    records: list[dict[str, Any]] = []
    for path in iter_checkable_files(tests_dir):
        rel_posix = path.relative_to(tests_dir).as_posix()
        kind = classify_kind(path, tests_dir)
        if kind == "invalid":
            continue
        fmt = format_for_path(path)
        records.append(
            build_record(
                rel_posix=rel_posix,
                kind=kind,
                fmt=fmt,
            )
        )

    out = opts.out.resolve()
    out.parent.mkdir(parents=True, exist_ok=True)
    tmp = out.with_suffix(out.suffix + ".tmp")
    with tmp.open("w", encoding="utf-8") as f:
        for r in records:
            f.write(json.dumps(r, ensure_ascii=False, separators=(",", ":")))
            f.write("\n")
    tmp.replace(out)

    print(f"Wrote {out} ({len(records)} cases)")
    return 0


def parse_args() -> Options:
    root = repo_root()
    ap = argparse.ArgumentParser(description="Generate tests/manifest.jsonl for the vendored VNU fixture suite.")
    ap.add_argument(
        "--tests-dir",
        type=Path,
        default=root / "tests",
        help="Suite tests directory (default: <repo>/tests).",
    )
    ap.add_argument(
        "--out",
        type=Path,
        default=root / "tests" / "manifest.jsonl",
        help="Output manifest path (default: <repo>/tests/manifest.jsonl).",
    )
    args = ap.parse_args()
    return Options(tests_dir=args.tests_dir, out=args.out)


if __name__ == "__main__":
    raise SystemExit(run(parse_args()))
