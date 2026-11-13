# Federal offence catalogue status

The app currently includes 146 selected entries, not a complete federal offence
catalogue. Many entries follow the requested explanation, penalty and statutory
text layout; other entries have shorter illustrative summaries. These explanations have not received legal
review. Neither a candidate identified below nor the presence of an existing
entry establishes that all elements, exceptions, or related laws are covered.

The packaged legislation was retrieved from Justice Laws section pages current
to 2026-10-19. It contains 170 primary and related sections. The text is an
unofficial reproduction and normalizes whitespace, so the linked official
pages remain the source of record. The source date is displayed in the app.

`data/federal-review-inventory.json` is a discovery queue from the Justice
Canada XML repository. It scans 971 Acts and 4,897 regulations and contains
11,397 possible passages. These are not 11,397 verified criminal offences.
The keyword-based scan has false positives and false negatives; some regulatory
offences may not be criminal offences. All source files in that snapshot have
an older current-to date than the Justice Laws home page as of this refresh.
Each candidate requires fresh sourcing and legal classification before use.

From the `app` directory, refresh the existing selected sections with:

```powershell
python -m tools.refresh_selected --audit-dir "$env:TEMP\stick-charge-law-audit"
```

Failures are reported and the affected text is left unavailable. The command
does not discover all offences or review the summaries. With `lxml` installed,
independently check retained source hashes, consolidation dates and the
non-whitespace text of every generated section:

```powershell
python -m tools.audit_selected "$env:TEMP\stick-charge-law-audit"
```

The audit verifies faithful extraction only. It does not decide whether all
relevant provisions are included or whether a legal explanation is correct.
After refreshing, rebuild the Rust WebAssembly and package the mobile app before
the refreshed text appears in that build.
