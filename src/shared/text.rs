//! Shared text normalization.

/// Drop control characters and layout-only invisibles, convert NBSP to a
/// regular space, strip soft hyphens. Line breaks become single spaces; a
/// CRLF pair is one break. U+200C (ZWNJ) and U+200D (ZWJ) are **preserved**:
/// they carry meaning in Arabic/Indic shaping and emoji sequences.
/// Typographic ligatures (ff, fi, fl, ffi, ffl, st) are expanded to their
/// constituent letters so words are not silently corrupted.
pub fn clean_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\u{a0}' => out.push(' '),
            '\u{ad}' | '\u{200b}' | '\u{feff}' => {}
            '\t' => out.push('\t'),
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push(' ');
            }
            '\n' => out.push(' '),
            // Expand typographic ligatures to ASCII equivalents.
            '\u{fb00}' => out.push_str("ff"),
            '\u{fb01}' => out.push_str("fi"),
            '\u{fb02}' => out.push_str("fl"),
            '\u{fb03}' => out.push_str("ffi"),
            '\u{fb04}' => out.push_str("ffl"),
            '\u{fb05}' | '\u{fb06}' => out.push_str("st"),
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// Collapse whitespace runs to single spaces.
pub fn collapse_ws(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut prev_space = false;
    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_space {
                out.push(' ');
            }
            prev_space = true;
        } else {
            out.push(c);
            prev_space = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::clean_text;

    #[test]
    fn join_controls_preserved() {
        assert_eq!(clean_text("می\u{200c}خواهم"), "می\u{200c}خواهم");
        assert_eq!(clean_text("👨\u{200d}👩\u{200d}👧"), "👨\u{200d}👩\u{200d}👧");
    }

    #[test]
    fn layout_invisibles_stripped() {
        assert_eq!(clean_text("a\u{ad}b\u{200b}c\u{feff}d\u{a0}e"), "abcd e");
    }

    #[test]
    fn ligatures_expanded() {
        assert_eq!(clean_text("classi\u{fb01}es"), "classifies");
        assert_eq!(clean_text("a\u{fb00}ected"), "affected");
        assert_eq!(clean_text("de\u{fb02}ect"), "deflect");
        assert_eq!(clean_text("di\u{fb03}cult"), "difficult");
        assert_eq!(clean_text("waf\u{fb04}e"), "waffle");
        assert_eq!(clean_text("fa\u{fb05}"), "fast");
        assert_eq!(clean_text("ju\u{fb06}"), "just");
    }
}
