use std::{borrow::Cow, path::PathBuf};

use tui::widgets::Cell;

pub trait Column {
    type Item;

    fn name(&self) -> &str;

    fn format<'a>(&self, item: &'a Self::Item) -> Cell<'a>;

    fn sort_text<'a>(&self, item: &'a Self::Item) -> Cow<'a, str> {
        let text: String = self.format(item).content.into();
        text.into()
    }

    fn filter_text<'a>(&self, item: &'a Self::Item) -> Cow<'a, str> {
        let text: String = self.format(item).content.into();
        text.into()
    }
}

pub struct SimpleColumn<'i, I> {
    name: &'static str,
    format_fn: fn(&'i I) -> Cell<'i>,
}

impl<'i, I> Column for SimpleColumn<'i, I> {
    type Item = I;

    fn name(&self) -> &str {
        self.name
    }

    fn format<'a>(&self, item: &'a Self::Item) -> Cell<'a> {
        (self.format_fn)(item)
    }
}

impl<'i, I> SimpleColumn<'i, I> {
    pub fn new(name: &'static str, format_fn: fn(&'i I) -> Cell<'i>) -> Self {
        Self { name, format_fn }
    }
}

pub struct PathColumn {
    /// Root prefix to strip.
    prefix: PathBuf,
}

impl Column for PathColumn {
    type Item = PathBuf;

    fn name(&self) -> &str {
        "path"
    }

    fn format<'a>(&self, item: &'a Self::Item) -> Cell<'a> {
        item.strip_prefix(&self.prefix)
            .unwrap_or(item)
            .to_string_lossy()
            .into()
    }
}
