use crate::FrameId;
use crate::tree::transform::Transform;
use crate::tree::transform::TransformId;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct StaticTransform {
    pub(crate) parent_frame_id: FrameId,
    pub(crate) child_frame_id: FrameId,
    pub transform: Transform,
}

impl StaticTransform {
    pub fn new(parent_frame_id: FrameId, child_frame_id: FrameId, transform: Transform) -> Self {
        Self {
            parent_frame_id,
            child_frame_id,
            transform,
        }
    }

    pub fn parent_frame_id(&self) -> &FrameId {
        &self.parent_frame_id
    }

    pub fn child_frame_id(&self) -> &FrameId {
        &self.child_frame_id
    }

    pub fn transform_id(&self) -> TransformId {
        TransformId::new(self.parent_frame_id.clone(), self.child_frame_id.clone())
    }

    pub fn prepend_isometry(&mut self, m: &Isometry3<f64>) {
        self.transform.prepend_isometry(m);
    }

    pub fn append_isometry(&mut self, m: &Isometry3<f64>) {
        self.transform.append_isometry(m);
    }
}
