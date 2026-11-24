#![cfg(test)]

use std::fs;

// Adjust the number of `..` to match how deep this crate sits under the repo root.
const GLOSSARY_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/GLOSSARY.md");

/// Terms that must be defined, matched case-insensitively as a prefix of the term cell.
const REQUIRED_TERMS: [&str; 6] = [
    "delegation",
    "keeper pass",
    "pending policy update",
    "reputation score",
    "vote batch",
    "whitelisted asset",
];

fn glossary_entries() -> Vec<(String, String)> {
    let text = fs::read_to_string(GLOSSARY_PATH).expect("docs/GLOSSARY.md must be readable");

    text.lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
            if cells.len() < 2 {
                return None;
            }
            let term = cells[0]
                .trim_matches(|c| c == '*' || c == '`')
                .trim()
                .to_string();
            let definition = cells[1..].join(" | ");
            Some((term, definition))
        })
        // Skip the header row and the `---|---` separator row.
        .filter(|(term, _)| {
            !term.is_empty()
                && !term.eq_ignore_ascii_case("term")
                && !term.chars().all(|c| c == '-' || c == ':' || c == ' ')
        })
        .collect()
}

#[test]
fn glossary_defines_all_required_terms() {
    let entries = glossary_entries();

    for required in REQUIRED_TERMS {
        let entry = entries
            .iter()
            .find(|(term, _)| term.to_ascii_lowercase().starts_with(required));

        match entry {
            Some((_, definition)) => assert!(
                !definition.trim().is_empty(),
                "glossary entry for '{required}' has an empty definition"
            ),
            None => panic!("docs/GLOSSARY.md is missing an entry for '{required}'"),
        }
    }
}

#[test]
fn glossary_has_no_duplicate_terms() {
    let entries = glossary_entries();
    let mut seen = std::collections::HashSet::new();
    for (term, _) in &entries {
        assert!(
            seen.insert(term.to_ascii_lowercase()),
            "duplicate glossary term: {term}"
        );
    }
}

#[test]
fn glossary_is_alphabetical() {
    let terms: Vec<String> = glossary_entries()
        .into_iter()
        .map(|(term, _)| term.to_ascii_lowercase())
        .collect();

    for pair in terms.windows(2) {
        assert!(
            pair[0] <= pair[1],
            "glossary out of order: '{}' should come after '{}'",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn glossary_has_original_plus_new_entries() {
    // 17 original entries + 6 new ones.
    assert!(glossary_entries().len() >= 23);
}
