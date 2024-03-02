use super::{
    direction::Direction, ecliptic::EclipticCoordinates, spherical::SphericalCoordinates,
    transformations::rotations::rotated_tuple,
};
use crate::{astro_display::AstroDisplay, error::AstroUtilError, units::distance::DISTANCE_ZERO};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::Distance,
    geometry::{Angle, Area},
};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianCoordinates {
    x: Distance<f64>,
    y: Distance<f64>,
    z: Distance<f64>,
}

impl CartesianCoordinates {
    pub const ORIGIN: CartesianCoordinates = CartesianCoordinates {
        x: DISTANCE_ZERO,
        y: DISTANCE_ZERO,
        z: DISTANCE_ZERO,
    };

    pub const fn new(x: Distance<f64>, y: Distance<f64>, z: Distance<f64>) -> CartesianCoordinates {
        CartesianCoordinates { x, y, z }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn eq_within(&self, other: &CartesianCoordinates, accuracy: Distance<f64>) -> bool {
        use crate::tests::eq_within;

        eq_within(self.x.m, other.x.m, accuracy.m)
            && eq_within(self.y.m, other.y.m, accuracy.m)
            && eq_within(self.z.m, other.z.m, accuracy.m)
    }

    pub fn length(&self) -> Distance<f64> {
        let x = self.x.m;
        let y = self.y.m;
        let z = self.z.m;
        Distance {
            m: (x * x + y * y + z * z).sqrt(),
        }
    }

    pub fn length_squared(&self) -> Area<f64> {
        let x = self.x.m;
        let y = self.y.m;
        let z = self.z.m;
        Area {
            m2: x * x + y * y + z * z,
        }
    }

    pub fn distance(&self, other: &CartesianCoordinates) -> Distance<f64> {
        let diff = self - other;
        diff.length()
    }

    pub fn x(&self) -> Distance<f64> {
        self.x
    }

    pub fn y(&self) -> Distance<f64> {
        self.y
    }

    pub fn z(&self) -> Distance<f64> {
        self.z
    }

    pub fn rotated(&self, angle: Angle<f64>, axis: &Direction) -> CartesianCoordinates {
        let (x, y, z) = rotated_tuple((self.x, self.y, self.z), angle, axis);
        CartesianCoordinates { x, y, z }
    }

    pub fn angle_to(&self, other: &CartesianCoordinates) -> Result<Angle<f64>, AstroUtilError> {
        Ok(self.to_direction()?.angle_to(&other.to_direction()?))
    }

    pub fn to_direction(&self) -> Result<Direction, AstroUtilError> {
        Direction::new(self.x.m, self.y.m, self.z.m)
    }

    pub fn to_ecliptic(&self) -> EclipticCoordinates {
        EclipticCoordinates {
            spherical: self.to_spherical(),
        }
    }

    pub fn to_spherical(&self) -> SphericalCoordinates {
        SphericalCoordinates::cartesian_to_spherical((self.x.m, self.y.m, self.z.m))
    }
}

impl Add for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn add(self, other: &CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn add(self, other: CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn sub(self, other: &CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn sub(self, other: CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Mul<f64> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Div<f64> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl Div<f64> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl Neg for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn neg(self) -> CartesianCoordinates {
        CartesianCoordinates {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn neg(self) -> CartesianCoordinates {
        CartesianCoordinates {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AstroDisplay for CartesianCoordinates {
    fn astro_display(&self) -> String {
        format!(
            "({}, {}, {})",
            self.x.astro_display(),
            self.y.astro_display(),
            self.z.astro_display()
        )
    }
}

impl Display for CartesianCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.astro_display())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{eq, TEST_ACCURACY};

    use super::*;

    #[test]
    fn test_length() {
        let coordinates = CartesianCoordinates {
            x: Distance::from_meters(3.),
            y: Distance::from_meters(4.),
            z: Distance::from_meters(5.),
        };

        assert!(eq(coordinates.length().to_m(), 7.0710678118654755));
    }

    #[test]
    fn subtraction_tests() {
        let ordinates = vec![0., 1., -1., 12.17];
        for x1 in ordinates.iter() {
            for y1 in ordinates.iter() {
                for z1 in ordinates.iter() {
                    for x2 in ordinates.iter() {
                        for y2 in ordinates.iter() {
                            for z2 in ordinates.iter() {
                                let c1 = CartesianCoordinates::new(
                                    Distance::from_meters(*x1),
                                    Distance::from_meters(*y1),
                                    Distance::from_meters(*z1),
                                );
                                let c2 = CartesianCoordinates::new(
                                    Distance::from_meters(*x2),
                                    Distance::from_meters(*y2),
                                    Distance::from_meters(*z2),
                                );
                                let c3 = &c1 - &c2;
                                let c4 = c1.clone() - c2.clone();
                                let c5 = c1 + -c2;
                                assert!(c3.eq_within(&c4, Distance { m: TEST_ACCURACY }));
                                assert!(c4.eq_within(&c5, Distance { m: TEST_ACCURACY }));
                            }
                        }
                    }
                }
            }
        }
    }
}
