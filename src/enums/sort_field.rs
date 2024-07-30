use crate::enums::DetailField;
use crate::models::{Node, OwnerMan};
use crate::traits::{Detail, Name};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// This enum contains all the different ways two nodes can be compared to
/// determine the sorting order.
///
/// It also contains, for every variant, a corresponding variant with trailing
/// underscore that sorts in the opposite direction.
///
/// The `SortField` variants are closely related to those of [`DetailField`].
///
/// The normal sort order for alphabetical fields is A to Z. The natural sort
/// order for numeric fields is 0 to 9. Sort fields with trailing underscore
/// have the opposite of this behaviour.
#[derive(
	Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
	Dev,   // device ID
	Ino,   // inode number
	Nlink, // number of hard links
	Typ,   // node type
	Cat,   // node category (directory or file)

	User, // user name
	Uid,  // user ID

	Group, // group name
	Gid,   // group ID

	Size,   // storage space
	Blocks, // number of blocks

	/// created at
	///
	/// Uses OS-normalised timestamp field
	/// [`created`](std::fs::Metadata::created).
	Btime,
	/// changed at
	///
	/// Uses Unix-specific extension fields
	/// [`ctime`](MetadataExt::ctime) and
	/// [`ctime_nsec`](MetadataExt::ctime_nsec).
	Ctime,
	/// modified at
	///
	/// Uses OS-normalised timestamp field
	/// [`modified`](std::fs::Metadata::modified).
	Mtime,
	/// accessed at
	///
	/// Uses OS-normalised timestamp field
	/// [`accessed`](std::fs::Metadata::accessed).
	Atime,

	Name,  // node name
	Cname, // canonical name (name in lower case with leading symbols stripped)
	Ext,   // file extension

	// Reversed sort by the field
	#[clap(name = "inode_")]
	Inode_,
	#[clap(name = "nlinks_")]
	Nlinks_,
	#[clap(name = "typ_")]
	Typ_,
	#[clap(name = "cat_")]
	Cat_,
	#[clap(name = "user_")]
	User_,
	#[clap(name = "uid_")]
	Uid_,
	#[clap(name = "group_")]
	Group_,
	#[clap(name = "gid_")]
	Gid_,
	#[clap(name = "size_")]
	Size_,
	#[clap(name = "blocks_")]
	Blocks_,
	#[clap(name = "btime_")]
	Btime_,
	#[clap(name = "ctime_")]
	Ctime_,
	#[clap(name = "mtime_")]
	Mtime_,
	#[clap(name = "atime_")]
	Atime_,
	#[clap(name = "name_")]
	Name_,
	#[clap(name = "cname_")]
	Cname_,
	#[clap(name = "ext_")]
	Ext_,

	None, // shorthand: no sorting
}

impl Display for SortField {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let string = self
			.to_possible_value()
			.map(|pos| String::from(pos.get_name()))
			.unwrap_or_default();
		write!(f, "{}", string)
	}
}

impl From<&str> for SortField {
	fn from(value: &str) -> Self {
		match Self::from_str(value, true) {
			Ok(field) => field,
			Err(_) => Self::None,
		}
	}
}

impl SortField {
	/// Clean the given input.
	///
	/// This performs the following operations on the input:
	///
	/// * Expand all shorthand forms.
	/// * Remove duplicated values.
	pub fn clean(input: &[Self]) -> Vec<Self> {
		let mut cleaned = vec![];
		for field in input {
			match field {
				SortField::None => cleaned.clear(),
				_ => cleaned.push(*field),
			}
		}
		// Remove duplicates while preserving order.
		let mut seen = HashSet::new();
		cleaned.retain(|&x| seen.insert(x));
		cleaned
	}

	/// Compare the two given nodes, using this sort field.
	///
	/// This function handles reverse sort fields, the fields suffixed with '_',
	/// by using the natural sort field's logic and then inverting it.
	pub fn compare(&self, a: &Node, b: &Node, owner_man: &mut OwnerMan) -> Ordering {
		let (basis, is_reverse) = self.simplify();

		let ord = basis
			.compare_no_meta(a, b, owner_man)
			.or_else(|| basis.compare_meta(a, b))
			.or_else(|| basis.compare_time(a, b))
			.unwrap_or(Ordering::Equal);

		if is_reverse {
			ord.reverse()
		} else {
			ord
		}
	}

	// =======
	// Private
	// =======

	/// Convert a `SortField` instance into a pair of `SortField` and direction.
	///
	/// For natural order fields, i.e. fields without trailing '_', the outcome
	/// is the same. For reverse order fields, i.e. fields with trailing '_',
	/// the outcome is the natural order field and the direction is reversed.
	///
	/// # Returns
	///
	/// * the basis for the field, the natural order field corresponding to this
	/// * whether the field is reversed from the natural order
	fn simplify(&self) -> (Self, bool) {
		let name = self.to_string();
		if name.ends_with('_') {
			(name.trim_end_matches('_').into(), true)
		} else {
			(*self, false)
		}
	}

	/// Compare the two given nodes based on a non-metadata field.
	///
	/// This function can perform comparisons based on fields that do not need
	/// metadata at all, or account for the `meta` field being `Err`.
	fn compare_no_meta(&self, a: &Node, b: &Node, owner_man: &mut OwnerMan) -> Option<Ordering> {
		let ord = match self {
			SortField::Name => a.name.cmp(&b.name),
			SortField::Cname => a.cname().cmp(&b.cname()),
			SortField::Ext => a.ext().cmp(&b.ext()),
			SortField::Typ => a.typ.cmp(&b.typ),
			SortField::Cat => a.typ.cat().cmp(&b.typ.cat()),
			SortField::User => a.user_val(owner_man).cmp(&b.user_val(owner_man)),
			SortField::Group => a.group_val(owner_man).cmp(&b.group_val(owner_man)),
			_ => return None,
		};
		Some(ord)
	}

	/// Compare the two given nodes based on a metadata field.
	///
	/// This function must be careful to ensure that the `meta` field is `Ok`.
	fn compare_meta(&self, a: &Node, b: &Node) -> Option<Ordering> {
		match (&a.meta_ok(), &b.meta_ok()) {
			(Some(a), Some(b)) => {
				let ord = match self {
					SortField::Dev => a.dev().cmp(&b.dev()),
					SortField::Ino => a.ino().cmp(&b.ino()),
					SortField::Nlink => a.nlink().cmp(&b.nlink()),
					SortField::Uid => a.uid().cmp(&b.uid()),
					SortField::Gid => a.gid().cmp(&b.gid()),
					SortField::Size => a.size().cmp(&b.size()),
					SortField::Blocks => a.blocks().cmp(&b.blocks()),
					_ => return None,
				};
				Some(ord)
			}
			_ => None,
		}
	}

	/// Compare two nodes on the basis of a timestamp field.
	///
	/// This is extracted into a separate function to prevent repetition for 4
	/// timestamp fields.
	fn compare_time(&self, a: &Node, b: &Node) -> Option<Ordering> {
		let field = match self {
			SortField::Btime => DetailField::Btime,
			SortField::Ctime => DetailField::Ctime,
			SortField::Mtime => DetailField::Mtime,
			SortField::Atime => DetailField::Atime,
			_ => unreachable!("src/enums/sort_fields.rs / impl SortField / cmp_time"),
		};
		let a = a.time_val(field);
		let b = b.time_val(field);
		match (a, b) {
			(Some(a), Some(b)) => Some(a.cmp(&b)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::SortField;

	macro_rules! make_clean_test {
		( $($name:ident: $input:expr => $expected:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(SortField::clean($input), $expected);
                }
            )*
		};
	}

	make_clean_test!(
		test_none_clears: &[SortField::Mtime, SortField::None, SortField::Gid] => vec![
			SortField::Gid,
		],
		test_removes_duplicates: &[SortField::Gid, SortField::Gid, SortField::User, SortField::Gid] => vec![
			SortField::Gid,
			SortField::User,
		],
	);
}
