use super::{ColumnIndex, ColumnMap, ColumnNames};
use std::collections::BTreeMap;

impl<'s> ColumnMap<'s> for BTreeMap<&'s str, ColumnIndex> {
	fn from(src: impl Iterator<Item = (ColumnIndex, &'s str)>) -> Self {
		Self::from_iter(src.map(|(idx, val)| (val, idx)))
	}
	fn get_column(&self, name: &str) -> Option<&ColumnIndex> {
		self.get(name)
	}
	fn column_names(&'s self) -> ColumnNames<'s> {
		let mut columns = BTreeMap::new();
		for (name, idx) in self.iter() {
			columns.insert(idx, *name);
		}

		ColumnNames {
			names: columns.into_values().rev().collect(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::ColumnMap;
	use crate::columnmap::DuplicateColumnName;
	use std::collections::BTreeMap;

	#[test]
	fn can_iterate_column_names() {
		let columns = "alpha,beta,gamma,#delta,epsilon";
		let map = <BTreeMap<_, _> as ColumnMap>::parse(columns).unwrap();

		let mut columns = map.column_names();
		assert_eq!(columns.next(), Some("alpha"));
		assert_eq!(columns.next(), Some("beta"));
		assert_eq!(columns.next(), Some("gamma"));
		assert_eq!(columns.next(), Some("#delta"));
		assert_eq!(columns.next(), Some("epsilon"));
		assert_eq!(columns.next(), None);
	}

	#[test]
	fn can_perform_column_lookup() {
		let columns =
			<BTreeMap<_, _> as ColumnMap>::parse("#datatype,string,dateTime:RFC3339").unwrap();

		assert_eq!(columns.get_column("#datatype"), Some(&0));
		assert_eq!(columns.get_column("string"), Some(&1));
		assert_eq!(columns.get_column("dateTime:RFC3339"), Some(&2));
		assert_eq!(columns.get_column("#group"), None);
	}

	#[test]
	fn can_parse_empty_str() {
		let columns = <BTreeMap<_, _> as ColumnMap>::parse("").unwrap();

		// The column set "" contains exactly one column with the name "".
		assert_eq!(columns.get_column(""), Some(&0));
		assert_eq!(columns.get_column("anything"), None);
	}

	#[test]
	fn can_detect_duplicate_column_names() {
		let columns = <BTreeMap<_, _> as ColumnMap>::parse("a,b,a").unwrap_err();
		assert_eq!(columns, DuplicateColumnName("a"));
	}
}
