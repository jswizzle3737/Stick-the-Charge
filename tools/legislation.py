"""Extract one Justice Laws HTML section without paraphrasing its provisions.

This is a building block, not an offence classifier or a legal review.
"""
from dataclasses import dataclass
from datetime import date
from html.parser import HTMLParser
import re


class ExtractionError(ValueError):
    pass


@dataclass(frozen=True)
class ExtractedSection:
    current_to: str
    text: str


class _SectionParser(HTMLParser):
    _VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}
    _BLOCK = {"p", "li", "br", "tr", "h1", "h2", "h3", "h4", "h5", "h6"}
    _UNSUPPORTED = {"img", "svg", "math", "table", "object", "iframe", "canvas"}
    _EDITORIAL = {"MarginalNote", "HistoricalNote", "PITLink", "wb-invisible"}

    def __init__(self, expected_section):
        super().__init__(convert_charrefs=True)
        self.expected = expected_section
        self.stack = []
        self.wrapper_depth = None
        self.section_depth = None
        self.complete = False
        self.found_label = False
        self.parts = []
        self.date_parts = []
        self.issues = []

    def _skipped(self):
        return any(classes & self._EDITORIAL for _, classes, _ in self.stack)

    def handle_starttag(self, tag, attributes):
        attrs = dict(attributes)
        classes = set((attrs.get("class") or "").split())
        void = tag in self._VOID
        if not void:
            self.stack.append((tag, classes, attrs.get("id")))
        if tag == "div" and "wb-txthl" in classes and self.wrapper_depth is None:
            self.wrapper_depth = len(self.stack)
        if tag == "section" and self.wrapper_depth is not None and self.section_depth is None and not self.complete:
            self.section_depth = len(self.stack)
        if self.section_depth is None:
            return
        if re.search(r"not[-_]?in[-_]?force", " ".join(classes), re.IGNORECASE):
            self.issues.append("section contains not-in-force content")
        if attrs.get("id") == f"s-{self.expected}":
            self.found_label = True
        if not self._skipped() and tag in self._UNSUPPORTED:
            self.issues.append(f"unsupported {tag} content")
        if not self._skipped() and tag in self._BLOCK:
            self.parts.append("\n")

    def handle_startendtag(self, tag, attributes):
        self.handle_starttag(tag, attributes)
        if tag not in self._VOID:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        if self.section_depth is not None and not self._skipped() and tag in self._BLOCK:
            self.parts.append("\n")
        for index in range(len(self.stack) - 1, -1, -1):
            if self.stack[index][0] == tag:
                if self.section_depth is not None and index + 1 <= self.section_depth:
                    self.section_depth = None
                    self.complete = True
                if self.wrapper_depth is not None and index + 1 <= self.wrapper_depth:
                    self.wrapper_depth = None
                del self.stack[index:]
                break

    def handle_data(self, text):
        if any(identifier == "assentedDate" for _, _, identifier in self.stack):
            self.date_parts.append(text)
        if self.section_depth is not None and not self._skipped():
            self.parts.append(text)


def extract_section(html, expected_section, minimum_date):
    if not re.fullmatch(r"\d+(?:\.\d+)*", expected_section):
        raise ExtractionError("invalid section number")
    parser = _SectionParser(expected_section)
    parser.feed(html)
    date_match = re.search(r"current to\s+(\d{4}-\d{2}-\d{2})", "".join(parser.date_parts), re.IGNORECASE)
    if not date_match:
        raise ExtractionError("missing consolidation date")
    try:
        current_to = date.fromisoformat(date_match.group(1))
        minimum = date.fromisoformat(minimum_date)
    except ValueError as error:
        raise ExtractionError("invalid consolidation date") from error
    if current_to < minimum:
        raise ExtractionError("source is older than the minimum consolidation date")
    if not parser.complete or not parser.found_label:
        raise ExtractionError("the requested section was not found")
    if parser.issues:
        raise ExtractionError("; ".join(parser.issues))
    lines = [" ".join(line.split()) for line in "".join(parser.parts).splitlines()]
    text = "\n".join(line for line in lines if line)
    if not text:
        raise ExtractionError("the section contains no statutory text")
    return ExtractedSection(current_to.isoformat(), text)
