use crate::Error::NoTransforms;
use crate::tree::transform::Transform;
use crate::tree::transform::TransformId;
use crate::{Error, ExtrapolationMethod, FrameId, InterpolationMethod, TimedTransform};
use chrono::{DateTime, Utc};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicTransform {
    pub(crate) parent_frame_id: FrameId,
    pub(crate) child_frame_id: FrameId,
    pub interpolation: Option<InterpolationMethod>,
    pub extrapolation: Option<ExtrapolationMethod>,
    pub samples: Vec<TimedTransform>,
}

impl DynamicTransform {
    pub fn new(
        parent_frame_id: FrameId,
        child_frame_id: FrameId,
        interpolation: Option<InterpolationMethod>,
        extrapolation: Option<ExtrapolationMethod>,
        mut samples: Vec<TimedTransform>,
    ) -> Result<Self, Error> {
        if samples.is_empty() {
            return Err(NoTransforms());
        }
        samples.sort_by_key(|s| s.timestamp);

        for window in samples.windows(2) {
            if window[0].timestamp == window[1].timestamp {
                return Err(Error::DuplicateTimestamp(window[1].timestamp));
            }
        }

        Ok(Self {
            parent_frame_id,
            child_frame_id,
            interpolation,
            extrapolation,
            samples,
        })
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

    pub fn sample_timestamps(&self) -> Vec<DateTime<Utc>> {
        self.samples.iter().map(|x| x.timestamp).collect()
    }

    pub fn first_sample_time(&self) -> DateTime<Utc> {
        self.samples
            .first()
            .expect("must at least have one sample")
            .timestamp
    }

    pub fn last_sample_time(&self) -> DateTime<Utc> {
        self.samples
            .last()
            .expect("must at least have one sample")
            .timestamp
    }

    pub fn prepend_isometry(&mut self, m: &Isometry3<f64>) {
        self.samples
            .iter_mut()
            .for_each(|x| x.transform.prepend_isometry(m));
    }

    pub fn append_isometry(&mut self, m: &Isometry3<f64>) {
        self.samples
            .iter_mut()
            .for_each(|x| x.transform.append_isometry(m));
    }
}

impl DynamicTransform {
    pub fn interpolate(&self, timestamp: DateTime<Utc>) -> Transform {
        debug_assert!(
            self.samples.is_sorted_by_key(|t| t.timestamp),
            "transforms must be sorted by timestamp"
        );
        debug_assert!(
            self.samples
                .windows(2)
                .all(|t| t[0].timestamp != t[1].timestamp),
            "transforms must not contain two samples with same timestamps"
        );

        if timestamp < self.first_sample_time() || self.last_sample_time() <= timestamp {
            return match self.extrapolation.unwrap_or_default() {
                ExtrapolationMethod::Constant => {
                    crate::utils::transforms_interpolation::extrapolate_constant(
                        &self.samples,
                        &timestamp,
                    )
                }

                ExtrapolationMethod::Linear => {
                    crate::utils::transforms_interpolation::extrapolate_linear(
                        &self.samples,
                        &timestamp,
                    )
                }
            };
        }

        match self.interpolation.unwrap_or_default() {
            InterpolationMethod::Step => {
                crate::utils::transforms_interpolation::interpolate_step_function(
                    &self.samples,
                    &timestamp,
                )
            }
            InterpolationMethod::Linear => {
                crate::utils::transforms_interpolation::interpolate_linearly(
                    &self.samples,
                    &timestamp,
                )
            }
        }
    }

    pub fn filter_samples_by_time(
        &mut self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<(), Error> {
        let filtered_samples: Vec<TimedTransform> = self
            .samples
            .iter()
            .filter(|t| start_time.is_none_or(|x| x <= t.timestamp))
            .filter(|t| end_time.is_none_or(|x| t.timestamp < x))
            .copied()
            .collect();
        if filtered_samples.is_empty() {
            return Err(NoTransforms());
        }

        self.samples = filtered_samples;
        Ok(())
    }
}
