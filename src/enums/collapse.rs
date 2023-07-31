use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Collapse {
	/// Name-based collapsing matches this file with another file having the
	/// exact given name.
	Name(String),
	/// Extension-based collapsing matches this file with another file having
	/// the same name and the given extension.
	Ext(String),
}
