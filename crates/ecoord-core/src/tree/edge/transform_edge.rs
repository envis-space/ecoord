use crate::FrameId;
use crate::tree::edge::dynamic_transform::DynamicTransform;
use crate::tree::edge::static_transform::StaticTransform;
use crate::tree::transform::Transform;
use crate::tree::transform::TransformId;
use chrono::{DateTime, Utc};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum TransformEdge {
    Static(StaticTransform),
    Dynamic(DynamicTransform),
}

impl TransformEdge {
    pub fn at_time(&self, timestamp: DateTime<Utc>) -> Transform {
        match self {
            Self::Static(x) => x.transform,
            Self::Dynamic(x) => x.interpolate(timestamp),
        }
    }

    pub fn parent_frame_id(&self) -> &FrameId {
        match self {
            Self::Static(x) => &x.parent_frame_id,
            Self::Dynamic(x) => &x.parent_frame_id,
        }
    }

    pub fn child_frame_id(&self) -> &FrameId {
        match self {
            Self::Static(x) => &x.child_frame_id,
            Self::Dynamic(x) => &x.child_frame_id,
        }
    }

    pub fn transform_id(&self) -> TransformId {
        match self {
            Self::Static(x) => x.transform_id(),
            Self::Dynamic(x) => x.transform_id(),
        }
    }

    pub fn prepend_isometry(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::Static(x) => x.prepend_isometry(m),
            Self::Dynamic(x) => x.prepend_isometry(m),
        }
    }

    pub fn append_isometry(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::Static(x) => x.append_isometry(m),
            Self::Dynamic(x) => x.append_isometry(m),
        }
    }
}
