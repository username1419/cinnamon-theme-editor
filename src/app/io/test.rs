#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::time::Instant;
    use std::{env, fs};

    use crate::app::io::parse::{self, *};
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
}
