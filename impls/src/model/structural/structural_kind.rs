

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralKind {
    Joint,
    Member,
    Section,
    Material,
    Panel,
}

impl ComponentKind for StructuralKind {}