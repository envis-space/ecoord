use crate::FrameId;
use std::fmt;

/// Dedicated type for an identifier of a transform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TransformId {
    pub parent_frame_id: FrameId,
    pub child_frame_id: FrameId,
}

impl TransformId {
    pub fn new(parent_frame_id: FrameId, child_frame_id: FrameId) -> Self {
        assert_ne!(
            parent_frame_id, child_frame_id,
            "frame_id must be different from child_frame_id"
        );
        Self {
            parent_frame_id,
            child_frame_id,
        }
    }
}

impl fmt::Display for TransformId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "frame_id={} child_frame_id={}",
            self.parent_frame_id, self.child_frame_id
        )
    }
}

impl From<(FrameId, FrameId)> for TransformId {
    fn from((parent_frame_id, child_frame_id): (FrameId, FrameId)) -> Self {
        Self::new(parent_frame_id, child_frame_id)
    }
}

impl From<(&FrameId, &FrameId)> for TransformId {
    fn from((parent_frame_id, child_frame_id): (&FrameId, &FrameId)) -> Self {
        Self::new(parent_frame_id.clone(), child_frame_id.clone())
    }
}

impl From<(&str, &str)> for TransformId {
    fn from((parent, child): (&str, &str)) -> Self {
        Self::new(FrameId::from(parent), FrameId::from(child))
    }
}
