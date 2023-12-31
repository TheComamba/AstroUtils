use crate::units::angle::Angle;

use super::direction::Direction;

pub struct EquatorialCoordinates {
    longitude: Angle,
    latitude: Angle,
    axis: Direction,
}

impl EquatorialCoordinates {
    pub const fn new(longitude: Angle, latitude: Angle, axis: Direction) -> Self {
        Self {
            longitude,
            latitude,
            axis,
        }
    }

    pub(crate) fn add_longitude(&mut self, longitude: Angle) {
        self.longitude = self.longitude + longitude;
        self.longitude.normalize();
    }

    pub(crate) fn to_direction(&self) -> Direction {
        //rotate around z
        //rotate around new x
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::{
            direction::Direction,
            earth_equatorial::{
                self, EarthEquatorialCoordinates, EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES,
            },
        },
        tests::TEST_ACCURACY,
        units::angle::Angle,
        Float, PI,
    };

    #[test]
    fn north_pole_points_along_axis() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates = super::EquatorialCoordinates::new(
                        Angle::from_radians(0.),
                        Angle::from_radians(PI / 2.),
                        axis,
                    );
                    let direction = coordinates.to_direction();
                    assert!(direction.eq_within(&axis, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn south_pole_points_along_negative_axis() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates = super::EquatorialCoordinates::new(
                        Angle::from_radians(0.),
                        Angle::from_radians(-PI / 2.),
                        axis,
                    );
                    let direction = coordinates.to_direction();
                    assert!(direction.eq_within(&-axis, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn zero_lies_in_horizontal_plane() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates = super::EquatorialCoordinates::new(
                        Angle::from_radians(0.),
                        Angle::from_radians(0.),
                        axis,
                    );
                    let direction = coordinates.to_direction();
                    assert!(direction.z().abs() < TEST_ACCURACY);
                }
            }
        }
    }

    #[test]
    fn opposite_of_zero_lies_in_horizontal_plane() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates = super::EquatorialCoordinates::new(
                        Angle::from_radians(PI),
                        Angle::from_radians(0.),
                        axis,
                    );
                    let direction = coordinates.to_direction();
                    assert!(direction.z().abs() < TEST_ACCURACY);
                }
            }
        }
    }

    #[test]
    fn behaves_like_earth_equatorial() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        let earth_north = Direction::from_ecliptic(&EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES);

        for long in ordinates.clone() {
            for lat in ordinates.clone() {
                let long = Angle::from_radians(long);
                let lat = Angle::from_radians(lat);

                let equatorial_coordinates =
                    super::EquatorialCoordinates::new(long, lat, earth_north);
                let earth_equatorial_coordinates = EarthEquatorialCoordinates::new(long, lat);

                let expected = earth_equatorial_coordinates.to_direction();
                let actual = equatorial_coordinates.to_direction();
                println!("expected: {},\n actual: {}", expected, actual);
                assert!(actual.eq_within(&expected, TEST_ACCURACY));
            }
        }
    }
}
