//
// queries.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

use super::LangConfig;

const RUST_QUERY: &'static str = include_str!("../../queries/rust.scm");

static RUST_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Rust"],
    grammar: tree_sitter_rust::language,
    file_extensions: &["rs"],
    query: RUST_QUERY,
};

pub static ALL_LANGS: &[&LangConfig] = &[&RUST_LANG_CONFIG];
