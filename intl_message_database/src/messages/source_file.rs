use std::fmt::Formatter;

use serde::Serialize;

use crate::messages::symbols::KeySymbolSet;

use super::{KeySymbol, SourceFileMeta};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SourceFileKind {
    Definition,
    Translation,
}

impl std::fmt::Display for SourceFileKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFileKind::Definition => f.write_str("Definition"),
            SourceFileKind::Translation => f.write_str("Translation"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename = "definition")]
pub struct DefinitionFile {
    file: String,
    meta: SourceFileMeta,
    message_keys: KeySymbolSet,
}

impl DefinitionFile {
    pub fn new(file: String, meta: SourceFileMeta, message_keys: KeySymbolSet) -> Self {
        Self {
            file,
            meta,
            message_keys,
        }
    }
    pub fn file(&self) -> &String {
        &self.file
    }
    pub fn message_keys(&self) -> &KeySymbolSet {
        &self.message_keys
    }
    pub fn meta(&self) -> &SourceFileMeta {
        &self.meta
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename = "translation")]
pub struct TranslationFile {
    file: String,
    locale: KeySymbol,
    #[serde(rename = "messageKeys")]
    message_keys: KeySymbolSet,
}

impl TranslationFile {
    pub fn new(file: String, locale: KeySymbol, message_keys: KeySymbolSet) -> Self {
        Self {
            file,
            locale,
            message_keys,
        }
    }
    pub fn file(&self) -> &String {
        &self.file
    }
    pub fn locale(&self) -> &KeySymbol {
        &self.locale
    }
    pub fn message_keys(&self) -> &KeySymbolSet {
        &self.message_keys
    }
}

/// Representation of a file that either defines or provides translations for
/// some set of messages. The file name is mapped to all of the definitions it
/// affects, along with extra information useful for processing that file.
///
/// SourceFiles allow interactive editing of files to automatically update all
/// of the affected messages safely and efficiently.
#[derive(Debug, Serialize)]
#[serde(tag = "untagged")]
pub enum SourceFile {
    Definition(DefinitionFile),
    Translation(TranslationFile),
}

impl SourceFile {
    pub fn file(&self) -> &String {
        match self {
            SourceFile::Definition(value) => &value.file,
            SourceFile::Translation(value) => &value.file,
        }
    }

    pub fn message_keys(&self) -> &KeySymbolSet {
        match self {
            SourceFile::Definition(value) => &value.message_keys,
            SourceFile::Translation(value) => &value.message_keys,
        }
    }

    #[inline(always)]
    pub fn message_keys_mut(&mut self) -> &mut KeySymbolSet {
        match self {
            SourceFile::Definition(value) => &mut value.message_keys,
            SourceFile::Translation(value) => &mut value.message_keys,
        }
    }

    pub fn set_message_keys(&mut self, new_keys: KeySymbolSet) {
        *self.message_keys_mut() = new_keys;
    }
}
