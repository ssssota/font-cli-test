use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

pub fn search_font(name: &str) {
    let family_name = parse_name(name);
    let properties = Properties::new();
    let result = SystemSource::new().select_best_match(&[family_name], &properties);

    let result_string = match result {
        Ok(handle) => match handle {
            Handle::Path {
                path,
                font_index: _,
            } => String::from(path.to_str().unwrap_or_else(|| "None")),
            Handle::Memory {
                bytes: _,
                font_index: _,
            } => String::from("Memory"),
        },
        Err(e) => e.to_string(),
    };
    println!("{}", result_string);
}

fn parse_name(name: &str) -> FamilyName {
    match name {
        "serif" => FamilyName::Serif,
        "sans-serif" => FamilyName::SansSerif,
        "monospace" => FamilyName::Monospace,
        "cursive" => FamilyName::Cursive,
        "fantasy" => FamilyName::Fantasy,
        n => FamilyName::Title(String::from(n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_name_test() {
        assert_eq!(parse_name("serif"), FamilyName::Serif);
        assert_eq!(parse_name("sans-serif"), FamilyName::SansSerif);
        assert_eq!(parse_name("monospace"), FamilyName::Monospace);
        assert_eq!(parse_name("cursive"), FamilyName::Cursive);
        assert_eq!(parse_name("fantasy"), FamilyName::Fantasy);

        assert_eq!(
            parse_name("others"),
            FamilyName::Title(String::from("others"))
        );
    }
}
