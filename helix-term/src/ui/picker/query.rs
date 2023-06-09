//! A lucene-like syntax for the picker.

use std::collections::HashMap;

use crate::ui::fuzzy_match::FuzzyQuery;

// TODO: unit tests for parsing.

#[derive(Default, PartialEq, Eq)]
pub struct Query {
    pub common: (String, FuzzyQuery),
    pub common_indices: Vec<usize>,
    /// A mapping between field name and a tuple of the original column index
    /// and the fuzzy query for that field.
    pub fields: HashMap<String, (usize, String, FuzzyQuery)>,
}

impl Query {
    pub fn new(field_names: &[String], input: &str) -> Self {
        // TODO: this is as basic as possible for a proof-of-concept.
        // We should also support strings like `path:"a b c.txt"` and
        // maybe some other lucene features.
        let mut common = String::new();
        let mut common_indices: Vec<usize> = (0..field_names.len()).collect();
        let mut fields: HashMap<&str, (usize, String)> = HashMap::new();

        for token in input.trim().split_ascii_whitespace() {
            match token.split_once(':') {
                Some((key, value)) if !key.is_empty() => {
                    if let Some((_index, existing_value)) = fields.get_mut(key) {
                        // Concatenate multiple mentions of the same field.
                        existing_value.push(' ');
                        existing_value.push_str(value);
                    } else if let Some(index) = field_names.iter().position(|name| name == key) {
                        // Only insert valid fields.
                        // TODO: case-insensitive?
                        fields.insert(key, (index, value.to_string()));
                        common_indices.remove(index);
                    } else {
                        // If the field is not valid, treat the text as common.
                        if !common.is_empty() {
                            common.push(' ');
                        }
                        common.push_str(token);
                    }
                }
                _ => {
                    if !common.is_empty() {
                        common.push(' ');
                    }
                    common.push_str(token);
                }
            }
        }

        let fields: HashMap<_, _> = fields
            .iter()
            .map(|(key, (index, value))| {
                (
                    key.to_string(),
                    (*index, value.clone(), FuzzyQuery::new(value)),
                )
            })
            .collect();

        Self {
            common: (common.clone(), FuzzyQuery::new(&common)),
            common_indices,
            fields,
        }
    }
}
