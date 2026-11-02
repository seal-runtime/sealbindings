/// Whether a class has a member, and who can read it. Returned by `StateExt::member_access`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberAccess {
    Missing,
    Public,
    Private,
}

/// How a class's objects can be written as data. Returned by `StateExt::class_data_shape`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassDataShape {
    /// The constructor runs code of its own, or a primary constructor doesn't set every field.
    None,
    /// A primary constructor whose parameters are all the fields: `Point(1, 2)`.
    Positional,
    /// The default constructor, which takes a table of fields: `Point { x = 1, y = 2 }`.
    Table,
}
