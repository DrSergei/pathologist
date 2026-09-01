#!/usr/bin/env python3
"""Build docs/PARSE_FAILURES.md from parse_failures example TSV output."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "docs" / "PARSE_FAILURES.md"

# Directory holding checkouts of the eval corpora (one subdirectory per
# corpus, named as below — clones of github.com/openharmony/<name>).
CORPUS_BASE = Path(os.environ.get("PARSE_CORPUS_BASE", "/home/sergei"))

CORPORA = [
    {
        "id": "hdf",
        "name": "drivers_hdf_core",
        "root": CORPUS_BASE / "drivers_hdf_core",
        "db": Path("/tmp/hdf_parse_check.db"),
        "tsv": Path("/tmp/parse_failures_hdf.tsv"),
    },
    {
        "id": "hiview",
        "name": "hiviewdfx_hiview",
        "root": CORPUS_BASE / "hiviewdfx_hiview",
        "db": Path("/tmp/hiview_parse_check.db"),
        "tsv": Path("/tmp/parse_failures_hiview.tsv"),
    },
    {
        "id": "camera",
        "name": "multimedia_camera_framework",
        "root": CORPUS_BASE / "multimedia_camera_framework",
        "db": Path("/tmp/camera_parse_check.db"),
        "tsv": Path("/tmp/parse_failures_camera.tsv"),
    },
]


def categorize_reason(reason: str) -> str:
    r = reason.strip()
    if "extern template" in r:
        return "extern template instantiations"
    if "IDL/interface macro" in r or "dotted interface name" in r:
        return "IDL dotted interface names"
    if "optional parameter" in r:
        return "default function parameters"
    if "missing ;" in r:
        return "gtest/HWTEST macros (`missing ;`)"
    if "missing type_identifier" in r:
        return "missing type identifiers (often macro-expanded types)"
    if "generic tree-sitter ERROR" in r:
        return "generic ERROR nodes (mixed C++ constructs)"
    if "operator" in r:
        return "operator overload syntax"
    if "preprocess failed" in r:
        return "preprocess failure"
    return "other / mixed"


def summarize(errs: list[dict], note: str | None) -> str:
    if note and note.startswith("preprocess"):
        return note
    if not errs:
        return note or "tree-sitter parse tree contains errors (site not localized)"
    nodes = [e["node"] for e in errs]
    if any("parenthesized_declarator" in n for n in nodes) and any(
        "." in e["snippet"] for e in errs
    ):
        return (
            "IDL/interface macro expands to dotted interface name "
            "(e.g. `ohos.hiviewdfx.IFaultLoggerService`) that tree-sitter-cpp "
            "cannot parse as a C++ declarator"
        )
    if any(
        "extern template" in e["snippet"] or e["snippet"].startswith("template ")
        for e in errs
    ):
        return (
            "explicit template instantiation declarations "
            "(`extern template …`) not supported by tree-sitter-cpp"
        )
    if any("optional_parameter_declaration" in n for n in nodes):
        return "default function parameters with complex types (optional parameter declaration)"
    if any(n == "ERROR" for n in nodes):
        return "generic tree-sitter ERROR node(s) in preprocessed C++"
    if any("operator" in e["snippet"] for e in errs):
        return "C++ operator overload syntax"
    if any("decltype" in e["snippet"] or "auto" in e["snippet"] for e in errs):
        return "C++11+ type syntax (auto/decltype) in declaration"
    top = max(set(nodes), key=nodes.count)
    return f"tree-sitter-cpp node `{top}` at {len(errs)} site(s)"


def load_tsv(tsv: Path, root: Path) -> dict[str, dict]:
    files: dict[str, dict] = defaultdict(lambda: {"errors": [], "note": None})
    if not tsv.exists():
        return files
    for line in tsv.read_text().splitlines():
        parts = line.split("\t", 4)
        if len(parts) < 3:
            continue
        _, path, kind, *rest = parts
        rel = (
            str(Path(path).relative_to(root))
            if path.startswith(str(root))
            else Path(path).name
        )
        if kind == "ERROR":
            detail = rest[0] if rest else ""
            m = re.match(r"line (\d+) col (\d+) \(([^)]+)\) (.*)", detail)
            if m:
                files[rel]["errors"].append(
                    {
                        "line": int(m.group(1)),
                        "col": int(m.group(2)),
                        "node": m.group(3),
                        "snippet": m.group(4),
                    }
                )
            else:
                files[rel]["errors"].append(
                    {"line": 0, "col": 0, "node": "?", "snippet": detail}
                )
        elif kind == "PARSE":
            files[rel]["note"] = rest[0] if rest else kind
        elif kind == "PREPROCESS":
            files[rel]["note"] = "preprocess failed: " + (rest[0] if rest else "")
    return dict(files)


def category_counts(files: dict[str, dict]) -> Counter[str]:
    counts: Counter[str] = Counter()
    for rel, info in files.items():
        reason = summarize(info["errors"], info.get("note"))
        counts[categorize_reason(reason)] += 1
    return counts


def render_corpus(corpus: dict, files: dict[str, dict]) -> list[str]:
    lines: list[str] = []
    name = corpus["name"]
    root = corpus["root"]
    lines.append(f"## {name}")
    lines.append("")
    lines.append(
        f"Generated from `trace analyze {root}` "
        f"({len(files)} files with parse warnings)."
    )
    lines.append(
        "Each entry is a translation unit or header indexed as its own file; "
        "reasons come from tree-sitter ERROR sites in preprocessed source."
    )
    lines.append("")
    lines.append(f"**Total failing files:** {len(files)}")
    lines.append("")

    cats = category_counts(files)
    lines.append("### Failure categories")
    lines.append("")
    lines.append("| Category | Files |")
    lines.append("|----------|------:|")
    for cat, n in cats.most_common():
        lines.append(f"| {cat} | {n} |")
    lines.append("")

    lines.append("### File list")
    lines.append("")
    lines.append("| # | File | Reason | Error sites |")
    lines.append("|---|------|--------|-------------|")
    for i, rel in enumerate(sorted(files.keys()), 1):
        info = files[rel]
        reason = summarize(info["errors"], info.get("note"))
        sites = len(info["errors"]) if info["errors"] else "—"
        lines.append(f"| {i} | `{rel}` | {reason} | {sites} |")
    lines.append("")

    lines.append("### Per-file details")
    lines.append("")
    for rel in sorted(files.keys()):
        info = files[rel]
        lines.append(f"#### `{rel}`")
        lines.append("")
        lines.append(f"**Summary:** {summarize(info['errors'], info.get('note'))}")
        lines.append("")
        if info["errors"]:
            lines.append("| Line | Col | Node kind | Snippet |")
            lines.append("|-----:|----:|-----------|---------|")
            for e in sorted(info["errors"], key=lambda x: (x["line"], x["col"]))[:20]:
                snip = e["snippet"].replace("|", "\\|")
                lines.append(
                    f"| {e['line']} | {e['col']} | `{e['node']}` | `{snip}` |"
                )
            if len(info["errors"]) > 20:
                lines.append(
                    f"| … | … | … | *({len(info['errors']) - 20} more)* |"
                )
        elif info.get("note"):
            lines.append(f"Note: {info['note']}")
        lines.append("")

    return lines


def main() -> int:
    parsed = []
    for corpus in CORPORA:
        files = load_tsv(corpus["tsv"], corpus["root"])
        parsed.append((corpus, files))

    out: list[str] = []
    out.append("# Parse failures — eval corpora")
    out.append("")
    out.append(
        "Files that fail tree-sitter parsing during `trace analyze`. "
        "Regenerate TSV with:"
    )
    out.append("")
    out.append("```bash")
    out.append("trace analyze <ROOT> -o /tmp/out.db --jobs 8")
    out.append(
        "cargo run -p trace-cli --release --example parse_failures -- "
        "<ROOT> --from-db /tmp/out.db > /tmp/parse_failures.tsv"
    )
    out.append("python3 scripts/gen_parse_failures_report.py")
    out.append("```")
    out.append("")
    out.append("## Overview")
    out.append("")
    out.append("| Corpus | Root | Failing files | Top category |")
    out.append("|--------|------|--------------:|--------------|")
    for corpus, files in parsed:
        cats = category_counts(files)
        top = cats.most_common(1)[0][0] if cats else "—"
        out.append(
            f"| `{corpus['name']}` | `{corpus['root']}` | {len(files)} | {top} |"
        )
    out.append("")
    out.append("## Cross-corpus category totals")
    out.append("")
    merged: Counter[str] = Counter()
    for _, files in parsed:
        merged.update(category_counts(files))
    out.append("| Category | HDF | Hiview | Camera | Total |")
    out.append("|----------|----:|-------:|-------:|------:|")
    all_cats = sorted(
        set(merged),
        key=lambda c: (-merged[c], c),
    )
    per_corpus_cats = [category_counts(files) for _, files in parsed]
    for cat in all_cats:
        hdf, hiview, camera = (c.get(cat, 0) for c in per_corpus_cats)
        out.append(f"| {cat} | {hdf} | {hiview} | {camera} | {merged[cat]} |")
    out.append("")

    for corpus, files in parsed:
        out.extend(render_corpus(corpus, files))
        out.append("---")
        out.append("")

    OUT.write_text("\n".join(out).rstrip() + "\n")
    print(f"Wrote {OUT} ({len(parsed)} corpora, {sum(len(f) for _, f in parsed)} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
