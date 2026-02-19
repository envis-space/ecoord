use nalgebra::{Isometry3, Point3, Rotation3, Translation3, UnitQuaternion, Vector3};

/// A time-dependent rigid transformation in 3D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
}

impl Transform {
    pub fn new(translation: Vector3<f64>, rotation: UnitQuaternion<f64>) -> Self {
        //let rotation = UnitQuaternion::from_quaternion(rotation);

        Self {
            translation,
            rotation,
        }
    }

    pub fn from(isometry: Isometry3<f64>) -> Self {
        Self {
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

    pub fn prepend_isometry(&mut self, m: &Isometry3<f64>) {
        let isometry = m * self.isometry();
        self.translation = isometry.translation.vector;
        self.rotation = isometry.rotation;
    }

    pub fn append_isometry(&mut self, m: &Isometry3<f64>) {
        let isometry = self.isometry() * m;
        self.translation = isometry.translation.vector;
        self.rotation = isometry.rotation;
    }

    pub fn transform_point(&self, pt: &Point3<f64>) -> Point3<f64> {
        let rotated_point = self.rotation().transform_point(pt);
        let _translated_point = self.translation().transform_point(pt);
        rotated_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::Unit;

    fn make_iso(translation: Vector3<f64>, rotation: UnitQuaternion<f64>) -> Isometry3<f64> {
        Isometry3::from_parts(Translation3::from(translation), rotation)
    }

    fn assert_isometries_equivalent_by_points(a: &Isometry3<f64>, b: &Isometry3<f64>) {
        // Compare via point images to avoid any quaternion sign ambiguity (q and -q represent the same rotation).
        let points = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.5, -2.0, 0.25),
        ];

        for p in points {
            let pa = a.transform_point(&p);
            let pb = b.transform_point(&p);
            assert_relative_eq!(pa.coords, pb.coords, epsilon = 1e-12);
        }
    }

    #[test]
    fn prepend_isometry_identity_does_not_change_transform() {
        let t0 = Transform::new(
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.7),
        );

        let mut t = t0;
        let m = Isometry3::<f64>::identity();
        t.prepend_isometry(&m);

        assert_relative_eq!(t.translation, t0.translation, epsilon = 1e-12);

        let got = t.isometry();
        let expected = t0.isometry();
        assert_isometries_equivalent_by_points(&got, &expected);
    }

    #[test]
    fn prepend_isometry_pure_translation_matches_left_multiplication() {
        let t0 = Transform::new(
            Vector3::new(0.5, -1.0, 2.0),
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.3),
        );

        let m = make_iso(Vector3::new(10.0, 0.0, -4.0), UnitQuaternion::identity());

        let mut t = t0;
        t.prepend_isometry(&m);

        let expected_iso = m * t0.isometry();

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }

    #[test]
    fn prepend_isometry_pure_rotation_matches_left_multiplication() {
        let t0 = Transform::new(
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -0.4),
        );

        let m = make_iso(
            Vector3::zeros(),
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 1.1),
        );

        let mut t = t0;
        t.prepend_isometry(&m);

        let expected_iso = m * t0.isometry();

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }

    #[test]
    fn prepend_isometry_general_case_matches_left_multiplication() {
        // A non-trivial m and a non-trivial self transform.
        let t0 = Transform::new(
            Vector3::new(-2.0, 0.25, 7.0),
            UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::new(1.0, 2.0, 3.0)), 0.9),
        );

        let axis: Unit<Vector3<f64>> = Unit::new_normalize(Vector3::new(-1.0, 0.5, 0.25));
        let m = make_iso(
            Vector3::new(3.0, -4.0, 1.5),
            UnitQuaternion::from_axis_angle(&axis, -0.75),
        );

        let mut t = t0;
        t.prepend_isometry(&m);

        let expected_iso = m * t0.isometry();

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }

    #[test]
    fn append_isometry_identity_does_not_change_transform() {
        let t0 = Transform::new(
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.7),
        );

        let mut t = t0;
        let m = Isometry3::<f64>::identity();
        t.append_isometry(&m);

        assert_relative_eq!(t.translation, t0.translation, epsilon = 1e-12);

        let got = t.isometry();
        let expected = t0.isometry();
        assert_isometries_equivalent_by_points(&got, &expected);
    }

    #[test]
    fn append_isometry_pure_translation_matches_right_multiplication() {
        let t0 = Transform::new(
            Vector3::new(0.5, -1.0, 2.0),
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.3),
        );

        let m = make_iso(Vector3::new(10.0, 0.0, -4.0), UnitQuaternion::identity());

        let mut t = t0;
        t.append_isometry(&m);

        let expected_iso = t0.isometry() * m;

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }

    #[test]
    fn append_isometry_pure_rotation_matches_right_multiplication() {
        let t0 = Transform::new(
            Vector3::new(1.0, 2.0, 3.0),
            UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -0.4),
        );

        let m = make_iso(
            Vector3::zeros(),
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 1.1),
        );

        let mut t = t0;
        t.append_isometry(&m);

        let expected_iso = t0.isometry() * m;

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }

    #[test]
    fn append_isometry_general_case_matches_right_multiplication() {
        let t0 = Transform::new(
            Vector3::new(-2.0, 0.25, 7.0),
            UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::new(1.0, 2.0, 3.0)), 0.9),
        );

        let axis: Unit<Vector3<f64>> = Unit::new_normalize(Vector3::new(-1.0, 0.5, 0.25));
        let m = make_iso(
            Vector3::new(3.0, -4.0, 1.5),
            UnitQuaternion::from_axis_angle(&axis, -0.75),
        );

        let mut t = t0;
        t.append_isometry(&m);

        let expected_iso = t0.isometry() * m;

        assert_relative_eq!(
            t.translation,
            expected_iso.translation.vector,
            epsilon = 1e-12
        );
        assert_isometries_equivalent_by_points(&t.isometry(), &expected_iso);
    }
}
