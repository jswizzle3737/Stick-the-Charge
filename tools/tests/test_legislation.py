import unittest
from tools.legislation import ExtractionError, extract_section


PAGE = """<!doctype html><html><body>
<div class='legisHeader'><h1 id='wb-cont'>Criminal Code</h1>
<p id='assentedDate'>Act current to 2026-10-19 and last amended on 2026-07-18.</p></div>
<div class='wb-txthl'><section>
<p class='MarginalNote'><span class='wb-invisible'>Marginal note:</span>Example</p>
<p class='Section'><strong><a class='sectionLabel' id='s-266'><span class='sectionLabel'>266</span></a></strong>&nbsp;A person does <em>the act</em> when</p>
<ul><li><p class='Paragraph'><span class='lawlabel'>(a)</span>&nbsp;the first condition applies; or</p></li>
<li><p class='Paragraph'><span class='lawlabel'>(b)</span>&nbsp;the other condition applies.</p></li></ul>
<div class='HistoricalNote'><p>Not part of the statutory text.</p></div>
<div class='PITLink'><a href='old.html'>Previous Version</a></div>
</section></div></body></html>"""


class ExtractSectionTests(unittest.TestCase):
    def test_preserves_words_punctuation_labels_and_order(self):
        section = extract_section(PAGE, expected_section="266", minimum_date="2026-10-19")
        self.assertEqual(section.current_to, "2026-10-19")
        self.assertEqual(section.text, "266 A person does the act when\n(a) the first condition applies; or\n(b) the other condition applies.")

    def test_rejects_wrong_section_or_stale_source(self):
        with self.assertRaises(ExtractionError):
            extract_section(PAGE, expected_section="267", minimum_date="2026-10-19")
        with self.assertRaises(ExtractionError):
            extract_section(PAGE, expected_section="266", minimum_date="2026-10-20")

    def test_refuses_unsupported_visual_content_and_not_in_force_markup(self):
        with self.assertRaises(ExtractionError):
            extract_section(PAGE.replace("the first condition applies", "<img src='diagram.png'>"), "266", "2026-10-19")
        with self.assertRaises(ExtractionError):
            extract_section(PAGE.replace("class='Section'", "class='Section NotInForce'"), "266", "2026-10-19")


if __name__ == "__main__":
    unittest.main()
