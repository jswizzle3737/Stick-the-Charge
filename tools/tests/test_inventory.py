import unittest
from tools.inventory import scan_law


LAW = b'''<Statute xmlns:lims="http://justice.gc.ca/lims" lims:current-date="2025-07-24">
<Identification><ShortTitle>Example Act</ShortTitle></Identification>
<Body>
  <Section><MarginalNote>Prohibition</MarginalNote><Label>2</Label><Text>No person shall do the specified act.</Text></Section>
  <Section><MarginalNote>Punishment</MarginalNote><Label>3</Label><Text>A person who contravenes section 2 is guilty of an offence.</Text></Section>
  <Section><MarginalNote>Administration</MarginalNote><Label>4</Label><Text>A minister may publish a report.</Text><HistoricalNote>Summary conviction was formerly mentioned here.</HistoricalNote></Section>
  <Section in-force="no"><MarginalNote>Future provision</MarginalNote><Label>5</Label><Text>No person shall do something else.</Text></Section>
</Body>
<Schedule><Label>Schedule A</Label><Section><MarginalNote>Schedule provision</MarginalNote><Label>1</Label><Text>Every person who acts is guilty of an offence.</Text></Section></Schedule>
<RelatedProvisions><Section><Label>99</Label><Text>Everyone commits an offence.</Text></Section></RelatedProvisions>
</Statute>'''


class InventoryTests(unittest.TestCase):
    def test_discovers_candidates_without_claiming_legal_classification(self):
        law = scan_law(LAW, "E-1", "act", "2026-10-19")
        self.assertEqual(law["title"], "Example Act")
        self.assertTrue(law["requires_fresh_source"])
        self.assertEqual(law["sections_scanned"], 5)
        candidates = law["candidates"]
        self.assertEqual([(c["section"], c["schedule"]) for c in candidates], [("2", None), ("3", None), ("5", None), ("1", "Schedule A")])
        self.assertTrue(all(c["review_status"] == "unreviewed" for c in candidates))
        self.assertIn("prohibition", candidates[0]["signals"])
        self.assertIn("offence_or_punishment", candidates[1]["signals"])
        self.assertEqual(candidates[2]["in_force_hint"], "no")


if __name__ == "__main__":
    unittest.main()
