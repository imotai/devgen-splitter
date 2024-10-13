//
// queries.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

use super::LangConfig;

const RUST_QUERY: &'static str = include_str!("../../queries/rust.scm");
const TYPESCRIPT_QUERY: &'static str = include_str!("../../queries/typescript.scm");

static RUST_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Rust"],
    grammar: tree_sitter_rust::language,
    file_extensions: &["rs"],
    query: RUST_QUERY,
};

static TYPESCRIPT_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["TypeScript"],
    grammar: tree_sitter_typescript::language_tsx,
    file_extensions: &["ts", "tsx"],
    query: TYPESCRIPT_QUERY,
};

pub static ALL_LANGS: &[&LangConfig] = &[&RUST_LANG_CONFIG, &TYPESCRIPT_LANG_CONFIG];
