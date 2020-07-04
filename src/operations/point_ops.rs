use serde::{Deserialize, Serialize};
use segment::types::{PointIdType};
use crate::operations::types::VectorType;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointOps {
    /// Insert or update points
    UpsertPoints {
        collection: String,
        ids: Vec<PointIdType>,
        vectors: Vec<VectorType>,
    },
    /// Delete point if exists
    DeletePoints {
        collection: String,
        ids: Vec<PointIdType>,
    },
}
