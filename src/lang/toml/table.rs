use std::{collections::HashMap, ops::Range};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TomlTable {
    pub(super) span: Range<usize>,
    pub(super) source: String,
    pub(super) entries: HashMap<TomlString, TomlValue>,
}

impl TomlTable {
    pub(super) fn from_node(node: Node, source: impl AsRef<str>) -> Option<Self> {
        match node.as_table() {
            None => None,
            Some(table) => {
                // NOTE: Node is guaranteed to have at least one text range
                let mut text_range = node.text_ranges().next().unwrap();
                for range in node.text_ranges() {
                    text_range = text_range.cover(range);
                }

                let span = Range {
                    start: u32::from(text_range.start()) as usize,
                    end: u32::from(text_range.end()) as usize,
                };

                let source = source.as_ref();
                let text = source[span.clone()].to_string();

                let entries = table
                    .entries()
                    .read()
                    .iter()
                    .map(|(key, node)| {
                        (
                            TomlString::from_key(key.clone(), source),
                            TomlValue::from_node(node.clone(), source),
                        )
                    })
                    .collect();

                Some(Self {
                    span,
                    source: text,
                    entries,
                })
            }
        }
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn entries(&self) -> &HashMap<TomlString, TomlValue> {
        &self.entries
    }

    pub fn find(&self, key: impl AsRef<str>) -> Option<(&TomlString, &TomlValue)> {
        let key = key.as_ref();
        for (k, v) in &self.entries {
            if k.value.as_str() == key {
                return Some((k, v));
            }
        }
        None
    }
}

impl AsRef<HashMap<TomlString, TomlValue>> for TomlTable {
    fn as_ref(&self) -> &HashMap<TomlString, TomlValue> {
        &self.entries
    }
}
