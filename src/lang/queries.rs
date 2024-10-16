//
// queries.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
// Distributed under terms of the MIT license.
//

use super::LangConfig;

const RUST_QUERY: &'static str = include_str!("../../queries/rust.scm");
const TYPESCRIPT_QUERY: &'static str = include_str!("../../queries/typescript.scm");
const JAVA_QUERY: &'static str = include_str!("../../queries/java.scm");
// empty query means this language doesn't support context splitting
const EMPTY_QUERY: &'static str = "";

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

static JAVA_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Java"],
    grammar: tree_sitter_java::language,
    file_extensions: &["java"],
    query: JAVA_QUERY,
};

static CPP_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["C++"],
    grammar: tree_sitter_cpp::language,
    file_extensions: &["cpp", "cc", "h"],
    query: EMPTY_QUERY,
};

static PYTHON_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Python"],
    grammar: tree_sitter_python::language,
    file_extensions: &["py"],
    query: EMPTY_QUERY,
};

static C_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["C"],
    grammar: tree_sitter_c::language,
    file_extensions: &["c", "h"],
    query: EMPTY_QUERY,
};

static JAVASCRIPT_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["JavaScript"],
    grammar: tree_sitter_javascript::language,
    file_extensions: &["js", "jsx"],
    query: EMPTY_QUERY,
};

static MARKDOWN_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Markdown"],
    grammar: tree_sitter_md::language,
    file_extensions: &["md"],
    query: EMPTY_QUERY,
};

static GO_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Go"],
    grammar: tree_sitter_go::language,
    file_extensions: &["go"],
    query: EMPTY_QUERY,
};

static SOLIDITY_LANG_CONFIG: LangConfig = LangConfig {
    lang: &["Solidity"],
    grammar: devgen_tree_sitter_solidity::language,
    file_extensions: &["sol"],
    query: EMPTY_QUERY,
};

pub static ALL_LANGS: &[&LangConfig] = &[
    &RUST_LANG_CONFIG,
    &TYPESCRIPT_LANG_CONFIG,
    &JAVA_LANG_CONFIG,
    &PYTHON_LANG_CONFIG,
    &C_LANG_CONFIG,
    &JAVASCRIPT_LANG_CONFIG,
    &MARKDOWN_LANG_CONFIG,
    &CPP_LANG_CONFIG,
    &GO_LANG_CONFIG,
    &SOLIDITY_LANG_CONFIG,
];
