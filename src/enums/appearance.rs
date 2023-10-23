/// This enum contains all the different ways a node can appear.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Appearance {
	/// The node appears as a normal listing.
	///
	/// The name of the node is determined from the last segment of the path.
	/// The display text is based on this name.
	Normal,
	/// The node appears as the target of a symlink.
	///
	/// The display text of the node is set to the symlink destination. It is
	/// not based on the name of the node.
	Symlink,
	/// The node appears as the child of another.
	///
	/// The tree-drawing shapes are shown before the name of the node, which is
	/// the same as [`Appearance::Normal`].
	TreeChild,
	/// The node appears as the parent of another.
	///
	/// This provides the ability to use an alternative "open-folder" icon for
	/// directories.
	TreeParent,
	/// The node appears as an individual file being listed.
	///
	/// The name of the node is shown exactly as it was passed to the CLI. It
	/// could be the name, or a relative/absolute path.
	SoloFile,
}
