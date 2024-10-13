//
// lang.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

mod queries;
use queries::ALL_LANGS;

/// the language config
#[derive(Debug)]
pub struct LangConfig {
    /// e.g.: ["Typescript", "TSX"], ["Rust"]
    pub lang: &'static [&'static str],
    /// tree-sitter grammar for this language
    pub grammar: fn() -> tree_sitter::Language,
    /// file_extensions for this language
    pub file_extensions: &'static [&'static str],
    /// the query used to extract the class, function definition
    pub query: &'static str,
}

pub struct Lang;

impl Lang {
    /// Determines the language configuration based on the given filename.
    ///
    /// This method attempts to match the file extension of the provided filename
    /// with the supported language configurations. If a match is found, it returns
    /// the corresponding `LangConfig`.
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice that holds the name of the file
    ///
    /// # Returns
    ///
    /// * `Some(&'static LangConfig)` if a matching language configuration is found
    /// * `None` if no matching language configuration is found
    ///
    /// # Example
    ///
    /// ```no_run
    /// use code_splitter::lang::Lang;
    /// let filename = "example.rs";
    /// if let Some(lang_config) = Lang::from_filename(filename) {
    ///     println!("Language: {:?}", lang_config.lang);
    ///     println!("File extensions: {:?}", lang_config.file_extensions);
    /// } else {
    ///     println!("Unsupported file type");
    /// }
    /// ```
    pub fn from_filename(filename: &str) -> Option<&'static LangConfig> {
        let path = std::path::Path::new(filename);
        let file_ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let lang = ALL_LANGS
            .iter()
            .find(|l| l.file_extensions.iter().any(|&ext| ext == file_ext));
        match lang {
            Some(lang) => Some(lang),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("example.rs", Some("Rust"))]
    #[case("unknown.xyz", None)]
    fn test_from_filename(#[case] filename: &str, #[case] expected_lang: Option<&str>) {
        let result = Lang::from_filename(filename);
        match expected_lang {
            Some(lang) => {
                assert!(result.is_some());
                assert_eq!(result.unwrap().lang[0], lang);
            }
            None => assert!(result.is_none()),
        }
    }
}
