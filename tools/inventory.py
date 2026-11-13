"""Build an unreviewed federal legislation review queue, not an offence catalogue.

The signals are deliberately broad and neither prove a provision is a criminal
offence nor find every offence. Source files must be refreshed and legally reviewed
before candidates become user-facing charges.
"""
import argparse
from datetime import date
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET


LIMS = "{http://justice.gc.ca/lims}"
EXCLUDE = {"HistoricalNote", "HistoricalNoteSubItem", "RecentAmendments", "RelatedProvisions", "AmendmentsNotInForce"}
OFFENCE = re.compile(r"\b(?:guilty of|commits? (?:an? |the )?(?:offen[cs]e|crime)|offen[cs]e punishable|indictable offen[cs]e|summary conviction)\b", re.IGNORECASE)
PROHIBITION = re.compile(r"\b(?:no person shall|no one shall|shall not|must not|is prohibited from)\b", re.IGNORECASE)


def _text(node, depth=0):
    if depth > 128:
        raise ValueError("XML nesting exceeds the supported limit")
    if node.tag in EXCLUDE or node.tag == "MarginalNote":
        return ""
    parts = [node.text or ""]
    for child in node:
        parts.append(_text(child, depth + 1))
        parts.append(child.tail or "")
    return " ".join(parts)


def _label(node):
    element = node.find("Label")
    if element is None:
        element = node.find("./ScheduleFormHeading/Label")
    return " ".join("".join(element.itertext()).split()) if element is not None else None


def scan_law(raw, source_id, kind, official_current_to):
    root = ET.fromstring(raw)
    identification = root.find("Identification")
    title_element = identification.find("ShortTitle") if identification is not None else None
    title = " ".join("".join(title_element.itertext()).split()) if title_element is not None else source_id
    source_date = root.get(LIMS + "current-date")
    try:
        requires_fresh = date.fromisoformat(source_date) < date.fromisoformat(official_current_to)
    except (TypeError, ValueError):
        requires_fresh = True
    result = {"source_id": source_id, "kind": kind, "title": title, "source_current_to": source_date,
              "requires_fresh_source": requires_fresh, "sections_scanned": 0, "candidates": []}

    def visit(node, schedule=None, in_force=None, depth=0):
        if depth > 128:
            raise ValueError("XML nesting exceeds the supported limit")
        if node.tag in EXCLUDE:
            return
        in_force = node.get("in-force", in_force)
        if node.tag == "Schedule":
            schedule = _label(node) or "Unlabelled schedule"
        if node.tag == "Section":
            result["sections_scanned"] += 1
            text = _text(node)
            signals = []
            if PROHIBITION.search(text):
                signals.append("prohibition")
            if OFFENCE.search(text):
                signals.append("offence_or_punishment")
            if signals:
                note = node.find("MarginalNote")
                heading = " ".join("".join(note.itertext()).split()) if note is not None else None
                result["candidates"].append({"section": _label(node), "schedule": schedule, "heading": heading,
                    "source_element_id": node.get(LIMS + "id"), "in_force_hint": in_force,
                    "signals": signals, "review_status": "unreviewed"})
            return
        for child in node:
            visit(child, schedule, in_force, depth + 1)

    for child in root:
        if child.tag in {"Body", "Schedule"}:
            visit(child)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True, help="checkout of justicecanada/laws-lois-xml")
    parser.add_argument("--current-to", required=True, help="current-to date shown by Justice Laws, YYYY-MM-DD")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    date.fromisoformat(args.current_to)
    commit = subprocess.run(["git", "-C", str(args.source), "rev-parse", "HEAD"], check=True, capture_output=True, text=True, timeout=15).stdout.strip()
    files = [(path, kind) for directory, kind in (("acts", "act"), ("regulations", "regulation"))
             for path in sorted((args.source / "eng" / directory).glob("*.xml"))]
    if not files:
        raise RuntimeError("No English legislation found")
    laws, errors = [], []
    for index, (path, kind) in enumerate(files, 1):
        try:
            laws.append(scan_law(path.read_bytes(), path.stem, kind, args.current_to))
        except (ET.ParseError, ValueError, OSError) as error:
            errors.append({"source_id": path.stem, "kind": kind, "error": str(error)})
        if index % 500 == 0:
            print(f"Scanned {index}/{len(files)} source files.", flush=True)
    report = {"purpose": "Unreviewed discovery queue; not an exhaustive or verified offence catalogue",
              "repository": "https://github.com/justicecanada/laws-lois-xml", "commit": commit,
              "official_home_current_to": args.current_to, "source_file_count": len(files),
              "source_errors": errors, "laws": laws}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", dir=args.output.parent, delete=False) as handle:
        temp_name = handle.name
        json.dump(report, handle, ensure_ascii=False, separators=(",", ":"))
    os.replace(temp_name, args.output)
    print(f"Scanned {len(laws)} sources, found {sum(len(law['candidates']) for law in laws)} unreviewed candidates, {len(errors)} source errors.")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
