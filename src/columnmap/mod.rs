use std::collections::HashSet;

use crate::DEFAULT_DELIMITER;

pub mod btree;

pub type ColumnIndex = u8;

/// Maps column names to column indicies.
///
pub trait ColumnMap<'s, const DELIM: char = DEFAULT_DELIMITER> {
    /// Construct a new `ColumnMap`.
    ///
    fn from(src: impl Iterator<Item = (ColumnIndex, &'s str)>) -> Self;

    /// Get the index for the named column.
    ///
    fn get_column(&self, name: &str) -> Option<&ColumnIndex>;

    /// Returns an `Iterator` over the column names in the column map.
    ///
    fn column_names(&'s self) -> ColumnNames<'s>;

    fn parse(src: &'s str) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let mut names = HashSet::new();
        let mut pairs = Vec::new();
        for (idx, name) in src.split(DELIM).enumerate() {
            let name = name.trim();
            if names.contains(name) {
                return Err(());
            }
            names.insert(name);
            pairs.push((idx as ColumnIndex, name));
        }
        Ok(Self::from(pairs.into_iter()))
    }
}

#[derive(Debug)]
pub struct ColumnNames<'s> {
    pub(crate) names: Vec<&'s str>,
}

impl<'s> Iterator for ColumnNames<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        self.names.pop()
    }
}
