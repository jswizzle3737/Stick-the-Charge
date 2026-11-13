"""Independently compare generated text with retained Justice Laws HTML pages.

Requires lxml. This checks source identity and non-whitespace text, not whether
the provisions or explanations are legally complete or professionally reviewed.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

from lxml import html

ROOT = Path(__file__).resolve().parents[1]
RUST_STRING = r'"(?:\\.|[^"\\])*"'
RECORD = re.compile(
    r'Legislation \{ act: (' + RUST_STRING + r'), section: (' + RUST_STRING
    + r'), current_to: (' + RUST_STRING + r'), text: (' + RUST_STRING
    + r'), source_sha256: (' + RUST_STRING + r') \},'
)
EDITORIAL = {"MarginalNote", "HistoricalNote", "PITLink", "wb-invisible"}


def audit(audit_dir):
    generated = (ROOT / "src" / "legislation_data.rs").read_text(encoding="utf-8")
    records = [tuple(json.loads(item) for item in match) for match in RECORD.findall(generated)]
    if not records or len(records) != generated.count("    Legislation {"):
        raise ValueError("generated records could not be fully parsed")
    for act, section, current_to, text, digest in records:
        raw = (audit_dir / f"{act}_{section}.html").read_bytes()
        if hashlib.sha256(raw).hexdigest() != digest:
            raise ValueError(f"source hash differs for {act} section {section}")
        document = html.fromstring(raw)
        anchors = document.xpath("//*[@id=$anchor]", anchor=f"s-{section}")
        if len(anchors) != 1:
            raise ValueError(f"source anchor missing or duplicated for {act} section {section}")
        wrappers = anchors[0].xpath("ancestor::section[1]")
        if len(wrappers) != 1:
            raise ValueError(f"section wrapper missing for {act} section {section}")
        wrapper = wrappers[0]
        for node in reversed(wrapper.xpath(".//*[@class]")):
            if set(node.get("class", "").split()) & EDITORIAL:
                node.drop_tree()
        original = "".join(wrapper.itertext())
        if "".join(original.split()) != "".join(text.split()):
            raise ValueError(f"source text differs for {act} section {section}")
        date_text = " ".join(document.xpath("//*[@id='assentedDate']//text()"))
        if current_to not in date_text:
            raise ValueError(f"consolidation date differs for {act} section {section}")
    print(f"Checked {len(records)} source hashes, dates and non-whitespace texts.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("audit_dir", type=Path, help="directory captured by refresh_selected --audit-dir")
    audit(parser.parse_args().audit_dir)
