#!/usr/bin/env python3
"""
GitHub REST API v2022-11-28 Endpoint Coverage Calculator & Badge Generator

Calculates coverage of active (non-deprecated) GitHub REST API endpoints in Octocrab
by matching OpenAPI operations against Rust source code using regular expressions.
Generates:
  1. Shields.io custom dynamic JSON badge payload.
  2. GitHub Actions Job Summary ($GITHUB_STEP_SUMMARY).
  3. Detailed JSON reports for CI artifacts.
  4. Categorized missing endpoints reports (Markdown/JSON).
"""

import argparse
import json
import os
import re
import sys
import urllib.request
from collections import defaultdict
from typing import Any, Dict, List, Optional, Set, Tuple

DEFAULT_SPEC_URL = "https://raw.githubusercontent.com/github/rest-api-description/main/descriptions/api.github.com/api.github.com.2022-11-28.json"
DEFAULT_CACHE_PATH = ".cache/api.github.com.2022-11-28.json"

# Regex 1: Matches GitHub REST API doc URLs in Rust doc comments
DOC_LINK_REGEX = re.compile(
    r"https://docs\.github\.com/(?:en/)?rest/([a-zA-Z0-9_\-]+/[a-zA-Z0-9_\-]+)(?:\?apiVersion=2022-11-28)?(?:#([a-zA-Z0-9_\-]+))?"
)

# Regex 2: Matches route templates in format!(), route =, uri =
ROUTE_TEMPLATE_REGEX = re.compile(
    r'(?:format!\(\s*|(?:route|uri|path)\s*=\s*)"(/[^"]+)"'
)

# Regex 3: Matches HTTP method calls on octocrab client
HTTP_METHOD_REGEX = re.compile(
    r'crab\.(?:_|)(get|post|put|patch|delete)\w*\('
)


def fetch_or_load_spec(spec_url: str, cache_path: str) -> Dict[str, Any]:
    """Loads OpenAPI spec from cache if present, otherwise downloads and caches it."""
    if os.path.exists(cache_path):
        print(f"Loading cached OpenAPI spec from: {cache_path}")
        with open(cache_path, "r", encoding="utf-8") as f:
            return json.load(f)

    print(f"Downloading GitHub REST API v2022-11-28 OpenAPI spec from: {spec_url}")
    os.makedirs(os.path.dirname(cache_path), exist_ok=True)
    req = urllib.request.Request(spec_url, headers={"User-Agent": "octocrab-coverage-calculator"})
    with urllib.request.urlopen(req) as resp:
        content = resp.read()

    with open(cache_path, "wb") as f:
        f.write(content)

    print(f"Cached OpenAPI spec to: {cache_path} ({len(content)} bytes)")
    return json.loads(content.decode("utf-8"))


def normalize_route(path: str) -> str:
    """Normalizes parameter placeholders like {owner}, {repo}, {} into '{_}'."""
    # Handle self.repo interpolation: /{}/... -> /repos/{owner}/{repo}/...
    path = re.sub(r"^/\{\}/", "/repos/{owner}/{repo}/", path)
    # Normalize all parameter names to {_}
    return re.sub(r"\{[a-zA-Z0-9_]*\}", "{_}", path)


def get_badge_color(percentage: float) -> str:
    """Determines badge color according to coverage percentage."""
    if percentage >= 80.0:
        return "brightgreen"
    elif percentage >= 60.0:
        return "green"
    elif percentage >= 40.0:
        return "yellow"
    elif percentage >= 20.0:
        return "orange"
    else:
        return "red"


def render_progress_bar(percentage: float, width: int = 20) -> str:
    """Generates an ASCII progress bar string."""
    filled = int(round((percentage / 100.0) * width))
    bar = "█" * filled + "░" * (width - filled)
    return f"[{bar}] {percentage:.1f}%"


def analyze_coverage(
    spec: Dict[str, Any], src_dir: str
) -> Dict[str, Any]:
    """Analyzes active endpoints vs implemented endpoints in Octocrab."""
    paths = spec.get("paths", {})
    active_operations: Dict[Tuple[str, str], Dict[str, Any]] = {}
    deprecated_count = 0

    # Index lookups
    doc_url_to_op: Dict[str, Tuple[str, str]] = {}
    anchor_to_op: Dict[str, Tuple[str, str]] = {}
    norm_path_to_ops: Dict[Tuple[str, str], List[Tuple[str, str]]] = defaultdict(list)
    categories: Dict[Tuple[str, str], str] = {}

    for path, methods in paths.items():
        path_deprecated = methods.get("deprecated", False) if isinstance(methods, dict) else False
        norm_path = normalize_route(path)
        for method, op in methods.items():
            method_upper = method.upper()
            if method_upper not in ["GET", "POST", "PUT", "DELETE", "PATCH"]:
                continue

            # Fully exclude deprecated endpoints so they never count toward total active or missing
            is_deprecated = (
                path_deprecated
                or op.get("deprecated", False)
                or op.get("x-deprecated", False)
            )
            if is_deprecated:
                deprecated_count += 1
                continue

            op_key = (method_upper, path)
            ext_url = (op.get("externalDocs") or {}).get("url", "")
            summary = op.get("summary", "")
            op_id = op.get("operationId", "")
            tags = op.get("tags", [])
            primary_tag = tags[0] if tags else "other"
            categories[op_key] = primary_tag

            active_operations[op_key] = {
                "method": method_upper,
                "path": path,
                "operationId": op_id,
                "summary": summary,
                "externalDocs": ext_url,
                "category": primary_tag,
            }

            if ext_url:
                # e.g., 'actions/artifacts#get-an-artifact'
                norm_doc = re.sub(r"^https?://docs\.github\.com/(?:en/)?rest/", "", ext_url).split("?")[0]
                doc_url_to_op[norm_doc] = op_key
                if "#" in norm_doc:
                    anchor = norm_doc.split("#", 1)[1]
                    anchor_to_op[anchor] = op_key

            norm_path_to_ops[(method_upper, norm_path)].append(op_key)

    total_active = len(active_operations)
    covered_ops: Set[Tuple[str, str]] = set()
    covered_sources: Dict[Tuple[str, str], Set[str]] = defaultdict(set)

    # Scan Rust source files
    for root, _, files in os.walk(src_dir):
        for f in files:
            if not f.endswith(".rs"):
                continue

            filepath = os.path.join(root, f)
            relpath = os.path.relpath(filepath, src_dir)

            with open(filepath, "r", encoding="utf-8", errors="ignore") as fp:
                content = fp.read()

            # 1. Match Doc comments via DOC_LINK_REGEX
            for match in DOC_LINK_REGEX.finditer(content):
                category, anchor = match.group(1), match.group(2)
                doc_key = f"{category}#{anchor}" if anchor else category
                
                matched_op = doc_url_to_op.get(doc_key)
                if not matched_op and anchor:
                    matched_op = anchor_to_op.get(anchor)

                if matched_op:
                    covered_ops.add(matched_op)
                    covered_sources[matched_op].add(relpath)

            # 2. Match Route strings and HTTP method invocations
            routes = ROUTE_TEMPLATE_REGEX.findall(content)
            methods = [m.upper() for m in HTTP_METHOD_REGEX.findall(content)]
            if routes and methods:
                for r in routes:
                    norm_r = normalize_route(r)
                    for m in set(methods):
                        matched_ops = norm_path_to_ops.get((m, norm_r), [])
                        for op_key in matched_ops:
                            covered_ops.add(op_key)
                            covered_sources[op_key].add(relpath)

    # Group statistics by category
    category_stats = defaultdict(lambda: {"total": 0, "covered": 0, "missing": []})
    for op_key, op_info in active_operations.items():
        cat = op_info["category"]
        category_stats[cat]["total"] += 1
        if op_key in covered_ops:
            category_stats[cat]["covered"] += 1
        else:
            category_stats[cat]["missing"].append(op_info)

    covered_count = len(covered_ops)
    missing_count = total_active - covered_count
    coverage_percentage = (covered_count / total_active * 100.0) if total_active > 0 else 0.0

    return {
        "total_active": total_active,
        "covered_count": covered_count,
        "missing_count": missing_count,
        "deprecated_count": deprecated_count,
        "coverage_percentage": round(coverage_percentage, 1),
        "covered_operations": [
            {
                "method": op_key[0],
                "path": op_key[1],
                "operationId": active_operations[op_key]["operationId"],
                "summary": active_operations[op_key]["summary"],
                "category": active_operations[op_key]["category"],
                "sources": sorted(list(covered_sources[op_key])),
            }
            for op_key in covered_ops
        ],
        "missing_operations": [
            op_info
            for op_key, op_info in active_operations.items()
            if op_key not in covered_ops
        ],
        "category_stats": dict(category_stats),
    }


def generate_badge_payload(stats: Dict[str, Any]) -> Dict[str, Any]:
    """Generates the Shields.io dynamic JSON badge payload."""
    pct = stats["coverage_percentage"]
    color = get_badge_color(pct)
    return {
        "schemaVersion": 1,
        "label": "REST API v2022-11-28",
        "message": f"{pct:.1f}%",
        "color": color,
        "total": stats["total_active"],
        "covered": stats["covered_count"],
        "missing": stats["missing_count"],
        "percentage": pct,
    }


def generate_missing_report(missing_ops: List[Dict[str, Any]]) -> str:
    """Generates a structured Markdown report of all missing active endpoints."""
    by_category = defaultdict(list)
    for op in missing_ops:
        by_category[op["category"]].append(op)

    lines = [
        "# Missing Active GitHub REST API (v2022-11-28) Endpoints",
        "",
        f"Total missing active endpoints: **{len(missing_ops)}**",
        "",
        "> This document is automatically generated by `scripts/calculate_api_coverage.py`.",
        "> Deprecated endpoints from the OpenAPI schema are completely excluded.",
        "",
    ]

    for cat in sorted(by_category.keys()):
        ops = by_category[cat]
        lines.append(f"## {cat.capitalize()} ({len(ops)})")
        lines.append("")
        lines.append("| Method | Endpoint Path | Summary | Docs |")
        lines.append("| :--- | :--- | :--- | :--- |")
        for op in sorted(ops, key=lambda x: (x["path"], x["method"])):
            doc_link = f"[Docs]({op['externalDocs']})" if op.get("externalDocs") else "N/A"
            lines.append(f"| `{op['method']}` | `{op['path']}` | {op.get('summary', 'N/A')} | {doc_link} |")
        lines.append("")

    return "\n".join(lines)


def generate_job_summary(stats: Dict[str, Any]) -> str:
    """Generates Markdown for GitHub Actions Job Summaries."""
    total = stats["total_active"]
    covered = stats["covered_count"]
    missing = stats["missing_count"]
    deprecated = stats["deprecated_count"]
    pct = stats["coverage_percentage"]
    progress_bar = render_progress_bar(pct, width=25)

    summary_lines = [
        "# 🐙 GitHub REST API (v2022-11-28) Endpoint Coverage",
        "",
        "| Metric | Count | Percentage |",
        "| :--- | :--- | :--- |",
        f"| **Active Endpoints** | {total:,} | 100.0% |",
        f"| **Covered in Octocrab** | **{covered:,}** | **{pct:.1f}%** |",
        f"| **Missing Endpoints** | {missing:,} | {100.0 - pct:.1f}% |",
        f"| **Deprecated Endpoints** | {deprecated:,} | *(Excluded)* |",
        "",
        f"> `{progress_bar}`",
        "",
        "### 📊 Coverage by API Domain",
        "",
        "| Domain / Category | Covered | Total | Coverage | Status |",
        "| :--- | :--- | :--- | :--- | :--- |",
    ]

    sorted_categories = sorted(
        stats["category_stats"].items(),
        key=lambda item: (item[1]["covered"] / item[1]["total"] if item[1]["total"] else 0),
        reverse=True,
    )

    for cat, cat_data in sorted_categories:
        c_tot = cat_data["total"]
        c_cov = cat_data["covered"]
        c_pct = (c_cov / c_tot * 100.0) if c_tot > 0 else 0.0
        status_emoji = "🟢" if c_pct >= 80.0 else "🟡" if c_pct >= 40.0 else "🔴"
        summary_lines.append(
            f"| **{cat}** | {c_cov} | {c_tot} | {c_pct:.1f}% | {status_emoji} |"
        )

    summary_lines.extend([
        "",
        f"<details>",
        f"<summary>🔍 Click to view all missing active endpoints ({missing} total by category)</summary>",
        "",
    ])

    by_category = defaultdict(list)
    for op in stats["missing_operations"]:
        by_category[op["category"]].append(op)

    for cat in sorted(by_category.keys()):
        ops = by_category[cat]
        summary_lines.append(f"<details><summary><b>{cat}</b> ({len(ops)} missing)</summary>\n")
        summary_lines.append("| Method | Path | Summary | Docs |")
        summary_lines.append("| :--- | :--- | :--- | :--- |")
        for op in ops:
            doc_link = f"[Docs]({op['externalDocs']})" if op.get("externalDocs") else "N/A"
            summary_lines.append(
                f"| `{op['method']}` | `{op['path']}` | {op.get('summary', 'N/A')} | {doc_link} |"
            )
        summary_lines.append("\n</details>\n")

    summary_lines.append("</details>\n")
    return "\n".join(summary_lines)


def main():
    parser = argparse.ArgumentParser(
        description="Calculate GitHub REST API v2022-11-28 coverage for Octocrab."
    )
    parser.add_argument(
        "--spec-url",
        default=DEFAULT_SPEC_URL,
        help="URL of the official GitHub REST API OpenAPI spec (2022-11-28)",
    )
    parser.add_argument(
        "--spec-cache",
        default=DEFAULT_CACHE_PATH,
        help="Local file path for caching the OpenAPI specification",
    )
    parser.add_argument(
        "--src-dir",
        default="src",
        help="Path to Octocrab src directory (default: src)",
    )
    parser.add_argument(
        "--badge-out",
        help="Path to output Shields.io endpoint-coverage.json file",
    )
    parser.add_argument(
        "--summary-out",
        help="Path to output GitHub Actions Job Summary Markdown file",
    )
    parser.add_argument(
        "--details-out",
        help="Path to output full coverage details JSON file",
    )
    parser.add_argument(
        "--missing-out",
        help="Path to output missing endpoints file (.md or .json)",
    )

    args = parser.parse_args()

    # Load OpenAPI spec
    spec = fetch_or_load_spec(args.spec_url, args.spec_cache)

    # Analyze coverage
    print(f"Scanning '{args.src_dir}' for covered endpoints...")
    stats = analyze_coverage(spec, args.src_dir)

    print("\n" + "=" * 50)
    print(f"Active Endpoints:  {stats['total_active']}")
    print(f"Covered:           {stats['covered_count']}")
    print(f"Missing:           {stats['missing_count']}")
    print(f"Coverage:          {stats['coverage_percentage']:.1f}%")
    print("=" * 50 + "\n")

    # Output badge JSON
    if args.badge_out:
        badge_payload = generate_badge_payload(stats)
        os.makedirs(os.path.dirname(args.badge_out) or ".", exist_ok=True)
        with open(args.badge_out, "w", encoding="utf-8") as f:
            json.dump(badge_payload, f, indent=2)
        print(f"Saved Shields.io badge JSON to: {args.badge_out}")

    # Output details JSON
    if args.details_out:
        os.makedirs(os.path.dirname(args.details_out) or ".", exist_ok=True)
        with open(args.details_out, "w", encoding="utf-8") as f:
            json.dump(stats, f, indent=2)
        print(f"Saved full coverage details JSON to: {args.details_out}")

    # Output Missing Endpoints report (MD or JSON)
    if args.missing_out:
        os.makedirs(os.path.dirname(args.missing_out) or ".", exist_ok=True)
        if args.missing_out.endswith(".json"):
            with open(args.missing_out, "w", encoding="utf-8") as f:
                json.dump(stats["missing_operations"], f, indent=2)
        else:
            missing_md = generate_missing_report(stats["missing_operations"])
            with open(args.missing_out, "w", encoding="utf-8") as f:
                f.write(missing_md)
        print(f"Saved missing endpoints report to: {args.missing_out}")

    # Output Job Summary
    summary_markdown = generate_job_summary(stats)
    if args.summary_out:
        os.makedirs(os.path.dirname(args.summary_out) or ".", exist_ok=True)
        with open(args.summary_out, "a", encoding="utf-8") as f:
            f.write(summary_markdown + "\n")
        print(f"Appended GitHub Actions Job Summary to: {args.summary_out}")


if __name__ == "__main__":
    main()
