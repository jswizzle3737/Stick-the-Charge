//! Unofficial, dated reproductions of selected Justice Laws sections.

#[allow(dead_code)]
pub struct Legislation {
    pub act: &'static str,
    pub section: &'static str,
    pub current_to: &'static str,
    pub text: &'static str,
    pub source_sha256: &'static str,
}

include!("legislation_data.rs");

pub fn lookup(act: &str, section: &str) -> Option<&'static Legislation> {
    SELECTED_LEGISLATION.iter().find(|entry| entry.act == act && entry.section == section)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_correct_act_and_section_without_inventing_missing_text() {
        let assault = lookup("C-46", "266").unwrap();
        assert_eq!(assault.section, "266");
        let date = assault.current_to.as_bytes();
        assert_eq!(date.len(), 10);
        assert!(date.iter().enumerate().all(|(i, c)| if i == 4 || i == 7 { *c == b'-' } else { c.is_ascii_digit() }));
        assert!(assault.text.starts_with("266 "));
        assert!(assault.text.contains("\n(a)"));
        assert!(assault.text.contains("\n(b)"));
        assert_eq!(assault.source_sha256.len(), 64);
        let drugs = lookup("C-38.8", "4").unwrap();
        assert_eq!(drugs.act, "C-38.8");
        assert!(lookup("C-46", "999999").is_none());
    }

    #[test]
    fn every_selected_charge_has_its_primary_and_related_source_sections() {
        for charge in crate::charges::CHARGES {
            let section = charge.number.split('(').next().unwrap_or(charge.number);
            assert!(lookup(charge.act_code(), section).is_some(), "missing {} section {}", charge.act_code(), section);
            if let Some((_, related)) = charge.related_section() {
                assert!(lookup(charge.act_code(), related).is_some(), "missing {} section {}", charge.act_code(), related);
            }
        }
    }
}
