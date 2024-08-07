use simple_si_units::geometry::Angle;
use std::f64::consts::PI;

use crate::astro_display::AstroDisplay;

pub const ANGLE_ZERO: Angle<f64> = Angle { rad: 0. };
pub(crate) const FULL_CIRC: Angle<f64> = Angle { rad: 2. * PI };
pub(crate) const QUARTER_CIRC: Angle<f64> = Angle { rad: 2. * PI / 4. };
pub(crate) const HALF_CIRC: Angle<f64> = Angle { rad: 2. * PI / 2. };
#[cfg(test)]
pub(crate) const THREE_QUARTER_CIRC: Angle<f64> = Angle {
    rad: 2. * PI * 3. / 4.,
};

pub const DEGREE: Angle<f64> = Angle {
    rad: 2. * PI / 360.,
};
pub const ARCSEC: Angle<f64> = Angle {
    rad: 2. * PI / (360. * 60. * 60.),
};
pub const SECOND_ANGLE: Angle<f64> = Angle {
    rad: 2. * PI / (24. * 60. * 60.),
};

pub fn angle_from_arcsecs(arcsec: f64) -> Angle<f64> {
    arcsec * ARCSEC
}

pub fn angle_to_arcsecs(angle: &Angle<f64>) -> f64 {
    angle / &ARCSEC
}

pub fn angle_from_second_angle(second_angle: f64) -> Angle<f64> {
    second_angle * SECOND_ANGLE
}

pub fn angle_to_second_angle(angle: &Angle<f64>) -> f64 {
    angle / &SECOND_ANGLE
}

/*
* Normalize the angle to a range of −π to +π radians, -180° to 180°.
*/
pub fn normalized_angle(mut angle: Angle<f64>) -> Angle<f64> {
    angle.rad %= FULL_CIRC.rad;
    if angle > HALF_CIRC {
        angle -= FULL_CIRC;
    } else if angle < -HALF_CIRC {
        angle += FULL_CIRC;
    }
    angle
}

#[cfg(test)]
pub(crate) fn angle_eq_within(
    actual: Angle<f64>,
    expected: Angle<f64>,
    accuracy: Angle<f64>,
) -> bool {
    let diff = normalized_angle(actual - expected);
    diff.rad.abs() < accuracy.rad
}

#[cfg(test)]
pub(crate) fn angle_eq(actual: Angle<f64>, expected: Angle<f64>) -> bool {
    use crate::tests::TEST_ACCURACY;

    angle_eq_within(actual, expected, Angle { rad: TEST_ACCURACY })
}

impl AstroDisplay for Angle<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} °", self.to_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::eq;

    #[test]
    fn arcsec_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_arcsecs(input);
            let output = angle_to_arcsecs(&angle);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn second_angle_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_second_angle(input);
            let output = angle_to_second_angle(&angle);
            assert!(eq(input, output));
        }
    }
}
