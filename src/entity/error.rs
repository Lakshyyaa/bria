// #[derive(Error, Debug)]
// pub enum EntityError {
//     #[error("EntityError - UninitializedFieldError: {0}")]
//     UninitializedFieldError(#[from] derive_builder::UninitializedFieldError),
//     #[error("EntityError - LoadEvent: {0}")]
//     LoadEvent(#[from] serde_json::Error),
// }

// Instead of adding the from_es_entity_error, do we just add the 1/2 error enums from above in each?
