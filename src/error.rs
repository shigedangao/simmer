#[derive(Debug)]
pub enum SimmerError {
    Character
}

impl std::error::Error for SimmerError {}

impl std::fmt::Display for SimmerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimmerError::Character => write!(f, "Unable to get the current character while creating the CVC tree")
        }
    }
}
