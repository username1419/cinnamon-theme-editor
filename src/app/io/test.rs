#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Instant;
    use std::{env, fs};

    use crate::app::io::parse::{self};
    use crate::app::io::parser::property_value::*;
    use crate::app::io::parser::selector::*;

    #[test]
    fn returns_simple_for_class_selector() {
        assert_eq!(SelectorType::Simple, Selector::get_type(".id"));
    }

    #[test]
    fn returns_simple_for_id_selector() {
        assert_eq!(SelectorType::Simple, Selector::get_type("#main"));
    }

    #[test]
    fn returns_simple_for_tag_selector() {
        assert_eq!(SelectorType::Simple, Selector::get_type("div"));
    }

    #[test]
    fn returns_compound_for_tag_with_class_and_id() {
        assert_eq!(SelectorType::Compound, Selector::get_type("p#id.class"));
    }

    #[test]
    fn returns_compound_for_tag_with_multiple_classes() {
        assert_eq!(
            SelectorType::Compound,
            Selector::get_type("button.primary.large")
        );
    }

    #[test]
    fn returns_complex_for_space_separated_selectors() {
        assert_eq!(SelectorType::Complex, Selector::get_type("p a#id"));
    }

    #[test]
    #[should_panic]
    fn panics_when_selector_has_leading_space() {
        Selector::get_type(" p");
    }

    #[test]
    #[should_panic]
    fn panics_when_selector_has_trailing_space() {
        Selector::get_type("p ");
    }

    #[test]
    fn returns_compound_even_if_dot_or_hash_is_first() {
        // like a class + id chain (no tag)
        assert_eq!(SelectorType::Compound, Selector::get_type(".foo#bar"));
    }

    #[test]
    fn handles_no_match_characters() {
        // just an element name, no . or #
        assert_eq!(SelectorType::Simple, Selector::get_type("span"));
    }

    #[test]
    fn complex_takes_precedence_over_compound() {
        // should still be Complex even if multiple matches appear after a space
        assert_eq!(SelectorType::Complex, Selector::get_type("div .foo#bar"));
    }

    #[test]
    #[ignore = "benchmark"]
    fn benchmark_parse_general() {
        let home =
            Path::new("/usr/share/themes/Mint-Y-Dark-Blue/cinnamon/cinnamon.css").to_path_buf();
        let raw = fs::read_to_string(home.clone()).unwrap();

        let start = Instant::now();
        let parsed = parse::StyleSheet::parse(home, raw);
        let duration = start.elapsed();

        println!("Finished parsing in {}ms.", duration.as_millis());
        let mut out_path = env::home_dir().unwrap();
        out_path.push("out.log");
        fs::write(out_path.clone(), format!("{:#?}", parsed))
            .inspect(|_| {
                println!(
                    "Successfully wrote result to {}",
                    out_path.to_str().unwrap()
                )
            })
            .ok();
    }

    // ── ValueUnit::from_str ──────────────────────────────────────────────────

    #[test]
    fn value_unit_from_str_px() {
        assert_eq!(ValueUnit::from_str("px"), ValueUnit::Px);
    }

    #[test]
    fn value_unit_from_str_em() {
        assert_eq!(ValueUnit::from_str("em"), ValueUnit::Em);
    }

    #[test]
    fn value_unit_from_str_rem() {
        assert_eq!(ValueUnit::from_str("rem"), ValueUnit::Rem);
    }

    #[test]
    fn value_unit_from_str_percent() {
        assert_eq!(ValueUnit::from_str("%"), ValueUnit::Percent);
    }

    #[test]
    fn value_unit_from_str_unknown_returns_none() {
        assert_eq!(ValueUnit::from_str("vw"), ValueUnit::None);
    }

    #[test]
    fn value_unit_from_str_empty_returns_none() {
        assert_eq!(ValueUnit::from_str(""), ValueUnit::None);
    }

    // ── ValueUnit::match_end ─────────────────────────────────────────────────

    #[test]
    fn value_unit_match_end_px() {
        assert_eq!(ValueUnit::match_end("13px"), ValueUnit::Px);
    }

    #[test]
    fn value_unit_match_end_em() {
        assert_eq!(ValueUnit::match_end("2em"), ValueUnit::Em);
    }

    #[test]
    fn value_unit_match_end_rem() {
        // "rem" ends_with both "em" and "rem"; the last winning pattern is rem
        assert_eq!(ValueUnit::match_end("1.5rem"), ValueUnit::Rem);
    }

    #[test]
    fn value_unit_match_end_percent() {
        assert_eq!(ValueUnit::match_end("100%"), ValueUnit::Percent);
    }

    #[test]
    fn value_unit_match_end_no_unit_returns_none() {
        assert_eq!(ValueUnit::match_end("42"), ValueUnit::None);
    }

    // ── ValueUnit Display ────────────────────────────────────────────────────

    #[test]
    fn value_unit_display_px() {
        assert_eq!(ValueUnit::Px.to_string(), "px");
    }

    #[test]
    fn value_unit_display_em() {
        assert_eq!(ValueUnit::Em.to_string(), "em");
    }

    #[test]
    fn value_unit_display_rem() {
        assert_eq!(ValueUnit::Rem.to_string(), "rem");
    }

    #[test]
    fn value_unit_display_percent() {
        assert_eq!(ValueUnit::Percent.to_string(), "%");
    }

    #[test]
    fn value_unit_display_none_is_empty() {
        assert_eq!(ValueUnit::None.to_string(), "");
    }

    #[test]
    fn value_unit_display_other_is_empty() {
        assert_eq!(ValueUnit::Other.to_string(), "");
    }

    // ── Value::from_raw_single ───────────────────────────────────────────────

    #[test]
    fn from_raw_single_px() {
        let v = Value::from_raw_single("13px");
        assert_eq!(v.get_value(), "13");
        assert_eq!(v.get_unit(), &ValueUnit::Px);
    }

    #[test]
    fn from_raw_single_rem() {
        let v = Value::from_raw_single("1.5rem");
        assert_eq!(v.get_value(), "1.5");
        assert_eq!(v.get_unit(), &ValueUnit::Rem);
    }

    #[test]
    fn from_raw_single_percent() {
        let v = Value::from_raw_single("100%");
        assert_eq!(v.get_value(), "100");
        assert_eq!(v.get_unit(), &ValueUnit::Percent);
    }

    #[test]
    fn from_raw_single_unitless_number() {
        let v = Value::from_raw_single("0");
        assert_eq!(v.get_value(), "0");
        assert_eq!(v.get_unit(), &ValueUnit::None);
    }

    #[test]
    fn from_raw_single_non_numeric_keyword() {
        // e.g. a CSS keyword like "auto" — not numeric, so whole string is the value
        let v = Value::from_raw_single("auto");
        assert_eq!(v.get_value(), "auto");
        assert_eq!(v.get_unit(), &ValueUnit::None);
    }

    #[test]
    fn from_raw_single_function_value() {
        let v = Value::from_raw_single("calc(100% - 8px)");
        // starts with 'c', not numeric → whole raw string is the value
        assert_eq!(v.get_value(), "calc(100% - 8px)");
        assert_eq!(v.get_unit(), &ValueUnit::None);
    }

    // ── Value::from_raw ──────────────────────────────────────────────────────

    #[test]
    fn from_raw_four_values() {
        let parsed = Value::from_raw("13px 9px 13px 9px");
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn from_raw_single_token() {
        let parsed = Value::from_raw("42px");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].get_value(), "42");
        assert_eq!(parsed[0].get_unit(), &ValueUnit::Px);
    }

    #[test]
    fn from_raw_mixed_units() {
        let parsed = Value::from_raw("1rem 50% 0 8px");
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0].get_unit(), &ValueUnit::Rem);
        assert_eq!(parsed[1].get_unit(), &ValueUnit::Percent);
        assert_eq!(parsed[2].get_unit(), &ValueUnit::None);
        assert_eq!(parsed[3].get_unit(), &ValueUnit::Px);
    }

    #[test]
    fn from_raw_extra_whitespace() {
        let parsed = Value::from_raw("  10px   20px  ");
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn from_raw_empty_string() {
        assert!(Value::from_raw("").is_empty());
    }

    #[test]
    fn from_raw_function_notation() {
        let parsed = Value::from_raw("calc(100% - 8px) 4px");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].get_value(), "calc(100% - 8px)");
        assert_eq!(parsed[1].get_value(), "4");
    }

    // ── Value::to_string ─────────────────────────────────────────────────────

    #[test]
    fn value_to_string_with_unit() {
        let v = Value::from_raw_single("16px");
        assert_eq!(v.to_string(), "16px");
    }

    #[test]
    fn value_to_string_unitless() {
        let v = Value::from_raw_single("0");
        assert_eq!(v.to_string(), "0");
        assert_eq!(v.get_value(), "0");
        assert_eq!(v.get_unit(), &ValueUnit::None);
    }

    // ── Setters ──────────────────────────────────────────────────────────────

    #[test]
    fn set_value_updates_value() {
        let mut v = Value::from_raw_single("10px");
        v.set_value("20".to_string());
        assert_eq!(v.get_value(), "20");
    }

    #[test]
    fn set_unit_updates_unit() {
        let mut v = Value::from_raw_single("10px");
        v.set_unit(ValueUnit::Rem);
        assert_eq!(v.get_unit(), &ValueUnit::Rem);
        assert_eq!(v.to_string(), "10rem");
    }
}
