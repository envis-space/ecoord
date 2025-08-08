use crate::FrameId;
use chrono::{DateTime, Utc};
use nalgebra::{Isometry3, Point3, Rotation3, Translation3, UnitQuaternion, Vector3};
use std::fmt;

/// Dedicated type for an identifier of a transform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TransformId {
    pub frame_id: FrameId,
    pub child_frame_id: FrameId,
}

impl TransformId {
    pub fn new(frame_id: FrameId, child_frame_id: FrameId) -> Self {
        assert_ne!(
            frame_id, child_frame_id,
            "frame_id must be different from child_frame_id"
        );
        Self {
            frame_id,
            child_frame_id,
        }
    }
}

impl fmt::Display for TransformId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "frame_id={} child_frame_id={}",
            self.frame_id, self.child_frame_id
        )
    }
}

/// A time-dependent rigid transformation in 3D.
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub timestamp: DateTime<Utc>,
    pub translation: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
}

impl Transform {
    pub fn new(
        timestamp: DateTime<Utc>,
        translation: Vector3<f64>,
        rotation: UnitQuaternion<f64>,
    ) -> Self {
        //let rotation = UnitQuaternion::from_quaternion(rotation);

        Self {
            timestamp,
            translation,
            rotation,
        }
    }

    pub fn from(timestamp: DateTime<Utc>, isometry: Isometry3<f64>) -> Self {
        Self {
            timestamp,
            translation: isometry.translation.vector,
            rotation: isometry.rotation,
        }
    }

    pub fn translation(&self) -> Translation3<f64> {
        Translation3::from(self.translation)
    }

    pub fn rotation(&self) -> Rotation3<f64> {
        Rotation3::from(self.rotation)
    }

    pub fn isometry(&self) -> Isometry3<f64> {
        let translation = self.translation();
        Isometry3::from_parts(translation, self.rotation)
    }

    pub fn transform_point(&self, pt: &Point3<f64>) -> Point3<f64> {
        let rotated_point = self.rotation().transform_point(pt);
        let _translated_point = self.translation().transform_point(pt);
        rotated_point
    }
}
