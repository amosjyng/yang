fn case_change(input: &str) -> usize {
    if input.len() < 2 {
        return input.len();
    }
    let mut chars = input.chars().enumerate();
    let (_, initial_char) = chars.next().unwrap(); // because we already tested len >= 2
    let initial_case = initial_char.is_lowercase();
    for (i, c) in chars {
        // should be iterating only over remaining chars
        if c.is_lowercase() != initial_case {
            return i;
        }
    }
    input.len()
}

/// Struct that stores a name in a unified representation, allowing for conversions between
/// different casing formats
#[derive(Default)]
pub struct NameTransform<'a> {
    /// The segmented component words of the name
    pub words: Vec<&'a str>,
}

impl<'a> NameTransform<'a> {
    /// Parse a camel-cased name.
    pub fn from_camel_case(name: &str) -> NameTransform {
        let mut words = Vec::new();
        let mut remainder = name;
        let mut starting_lowercase = name
            .chars()
            .next()
            .map(|c| c.is_lowercase())
            .unwrap_or_default();
        while !remainder.is_empty() {
            let mut next_boundary = case_change(remainder);
            if starting_lowercase {
                starting_lowercase = false; // add entire lowercase word as one unit
            } else if next_boundary == remainder.len() {
                // add rest of word as a unit, by not changing anything
            } else if next_boundary > 1 {
                // multi-character acronym
                next_boundary -= 1; // last capital letter is part of next word
            } else if next_boundary == 1 {
                // single capital letter turned into lowercase
                next_boundary += case_change(&remainder[1..]); // find next lowercase
            }
            words.push(&remainder[0..next_boundary]);
            remainder = &remainder[next_boundary..];
        }
        NameTransform { words }
    }

    /// Parse a snake-cased name.
    pub fn from_snake_case(name: &str) -> NameTransform {
        NameTransform {
            words: name.split('_').filter(|s| !s.is_empty()).collect(),
        }
    }

    /// Parse a kebab-cased name.
    pub fn from_kebab_case(name: &str) -> NameTransform {
        NameTransform {
            words: name.split('-').filter(|s| !s.is_empty()).collect(),
        }
    }

    /// Output a camel-case name.
    pub fn to_camel_case(&self) -> String {
        let mut snake_case = String::new();
        for word in &self.words {
            snake_case += &word[..1].to_uppercase();
            snake_case += &word[1..];
        }
        snake_case
    }

    /// Output a snake-case name.
    pub fn to_snake_case(&self) -> String {
        let mut words_iter = self.words.iter();
        let mut snake_case = String::new();
        snake_case += &words_iter.next().unwrap_or(&"").to_lowercase();
        for remaining_word in words_iter {
            snake_case += &format!("_{}", remaining_word.to_lowercase());
        }
        snake_case
    }

    /// Output a kebab-case name.
    pub fn to_kebab_case(&self) -> String {
        let mut words_iter = self.words.iter();
        let mut snake_case = String::new();
        snake_case += &words_iter.next().unwrap_or(&"").to_lowercase();
        for remaining_word in words_iter {
            snake_case += &format!("-{}", remaining_word.to_lowercase());
        }
        snake_case
    }
}

impl<'a> From<&'a str> for NameTransform<'a> {
    fn from(name: &'a str) -> NameTransform<'a> {
        if name.contains('_') {
            Self::from_snake_case(name)
        } else if name.contains('-') {
            Self::from_kebab_case(name)
        } else {
            Self::from_camel_case(name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_camel_case_empty() {
        assert_eq!(
            NameTransform::from_camel_case("").words,
            Vec::<String>::new()
        );
    }

    #[test]
    fn parse_camel_case_verbatim() {
        assert_eq!(NameTransform::from_camel_case("Name").words, vec!["Name"]);
    }

    #[test]
    fn parse_camel_case_simple() {
        assert_eq!(
            NameTransform::from_camel_case("NameTransform").words,
            vec!["Name", "Transform"]
        );
    }

    #[test]
    fn parse_camel_case_acronym() {
        assert_eq!(
            NameTransform::from_camel_case("DNSResolver").words,
            vec!["DNS", "Resolver"]
        );
    }

    #[test]
    fn parse_camel_case_acronym_at_end() {
        assert_eq!(
            NameTransform::from_camel_case("ResolveDNS").words,
            vec!["Resolve", "DNS"]
        );
    }

    #[test]
    fn parse_camel_case_single_letter_caps() {
        assert_eq!(
            NameTransform::from_camel_case("RStudio").words,
            vec!["R", "Studio"]
        );
    }

    #[test]
    fn parse_camel_case_starting_lower_case() {
        assert_eq!(
            NameTransform::from_camel_case("aRealVariable").words,
            vec!["a", "Real", "Variable"]
        );
    }

    #[test]
    fn parse_camel_case_starting_lower_case_word() {
        assert_eq!(
            NameTransform::from_camel_case("anRc").words,
            vec!["an", "Rc"]
        );
    }

    #[test]
    fn parse_snake_case_empty() {
        assert_eq!(
            NameTransform::from_snake_case("").words,
            Vec::<String>::new()
        );
    }

    #[test]
    fn parse_snake_case_verbatim() {
        assert_eq!(NameTransform::from_snake_case("name").words, vec!["name"]);
    }

    #[test]
    fn parse_snake_case_simple() {
        assert_eq!(
            NameTransform::from_snake_case("name_transform").words,
            vec!["name", "transform"]
        );
    }

    #[test]
    fn parse_snake_case_preserve_caps() {
        assert_eq!(
            NameTransform::from_snake_case("DNS_Resolver").words,
            vec!["DNS", "Resolver"]
        );
    }

    #[test]
    fn parse_kebab_case_empty() {
        assert_eq!(
            NameTransform::from_kebab_case("").words,
            Vec::<String>::new()
        );
    }

    #[test]
    fn parse_kebab_case_verbatim() {
        assert_eq!(NameTransform::from_kebab_case("name").words, vec!["name"]);
    }

    #[test]
    fn parse_kebab_case_simple() {
        assert_eq!(
            NameTransform::from_kebab_case("name-transform").words,
            vec!["name", "transform"]
        );
    }

    #[test]
    fn parse_kebab_case_preserve_caps() {
        assert_eq!(
            NameTransform::from_kebab_case("DNS-Resolver").words,
            vec!["DNS", "Resolver"]
        );
    }

    #[test]
    fn auto_parse_camel_case() {
        assert_eq!(NameTransform::from("anRc").words, vec!["an", "Rc"]);
    }

    #[test]
    fn auto_parse_snake_case() {
        assert_eq!(
            NameTransform::from("name_Transform").words,
            vec!["name", "Transform"]
        );
    }

    #[test]
    fn auto_parse_kebab_case() {
        assert_eq!(
            NameTransform::from("DNS-Resolver").words,
            vec!["DNS", "Resolver"]
        );
    }

    #[test]
    fn to_snake_case_empty() {
        assert_eq!(NameTransform::default().to_snake_case(), "".to_owned());
    }

    #[test]
    fn to_snake_case_simple() {
        assert_eq!(
            NameTransform::from_camel_case("Name").to_snake_case(),
            "name".to_owned()
        );
    }

    #[test]
    fn to_snake_case_multiple_words() {
        assert_eq!(
            NameTransform::from_camel_case("NameTransform").to_snake_case(),
            "name_transform".to_owned()
        );
    }

    #[test]
    fn to_kebab_case_empty() {
        assert_eq!(NameTransform::default().to_kebab_case(), "".to_owned());
    }

    #[test]
    fn to_kebab_case_simple() {
        assert_eq!(
            NameTransform::from_camel_case("Name").to_kebab_case(),
            "name".to_owned()
        );
    }

    #[test]
    fn to_kebab_case_multiple_words() {
        assert_eq!(
            NameTransform::from_camel_case("NameTransform").to_kebab_case(),
            "name-transform".to_owned()
        );
    }

    #[test]
    fn to_camel_case_empty() {
        assert_eq!(NameTransform::default().to_camel_case(), "".to_owned());
    }

    #[test]
    fn to_camel_case_simple() {
        assert_eq!(
            NameTransform::from_camel_case("Name").to_camel_case(),
            "Name".to_owned()
        );
    }

    #[test]
    fn to_camel_case_multiple_words() {
        assert_eq!(
            NameTransform::from_camel_case("NameTransform").to_camel_case(),
            "NameTransform".to_owned()
        );
    }

    #[test]
    fn to_camel_case_keep_acronym() {
        assert_eq!(
            NameTransform::from_camel_case("DNSResolver").to_camel_case(),
            "DNSResolver".to_owned()
        );
    }

    #[test]
    fn to_camel_case_uppercase_first() {
        assert_eq!(
            NameTransform::from_camel_case("anRc").to_camel_case(),
            "AnRc".to_owned()
        );
    }
}
