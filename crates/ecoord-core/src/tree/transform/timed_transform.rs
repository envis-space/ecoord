use crate::tree::transform::transform::Transform;
use chrono::{DateTime, Utc};
use nalgebra::Isometry3;

/// A time-dependent rigid transformation in 3D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimedTransform {
    pub timestamp: DateTime<Utc>,
    pub transform: Transform,
}

impl TimedTransform {
    pub fn new(timestamp: DateTime<Utc>, transform: Transform) -> Self {
        Self {
            timestamp,
            transform,
        }
    }

    pub fn from(timestamp: DateTime<Utc>, isometry: Isometry3<f64>) -> Self {
        Self {
            timestamp,
            transform: Transform::from(isometry),
        }
    }
}
