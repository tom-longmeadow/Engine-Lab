use super::{ComponentId, ComponentData};


#[derive(Debug, PartialEq, Eq)]
pub enum ModelError<D: ComponentData, I: ComponentId> {
    NotFound(I, D::Kind),
    AlreadyExists(I, D::Kind),
    ValidationError(I, D::Kind, String),
    InvalidId(I, D::Kind),
}

impl<D, I> std::fmt::Display for ModelError<D, I> 
where 
    D: ComponentData, 
    I: ComponentId,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(id, kind) => {
                write!(f, "{:?} {:?} not found", kind, id)
            }
            Self::AlreadyExists(id, kind) => {
                write!(f, "{:?} {:?} already exists", kind, id)
            }
            Self::ValidationError(id, kind, msg) => {
                write!(f, "Validation error for {:?} {:?}: {}", kind, id, msg)
            }
            Self::InvalidId(id, kind) => {
                write!(f, "Invalid id: {:?} for {:?}", id, kind)
            }
        }
    }
}
