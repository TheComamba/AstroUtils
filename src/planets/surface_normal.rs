use astro_coords::{direction::Direction, equatorial::Equatorial};
use simple_si_units::{base::Time, geometry::Angle};

use crate::units::angle::FULL_CIRC;

pub fn surface_normal_at_time(
    mut observer: Equatorial,
    angle_at_epoch: Angle<f64>,
    time_since_epoch: Time<f64>,
    siderial_day: Time<f64>,
) -> Direction {
    if siderial_day.to_seconds().abs() > 1. {
        let time_of_siderial_day = Time::from_s(time_since_epoch.s % siderial_day.s);
        let rotation = angle_at_epoch + (time_of_siderial_day / siderial_day) * FULL_CIRC;
        observer.spherical.longitude = observer.spherical.longitude + rotation;
    }
    observer.to_direction()
}

#[cfg(test)]
mod tests {
    use astro_coords::spherical::Spherical;

    use crate::{
        tests::TEST_ACCURACY,
        units::{angle::ANGLE_ZERO, time::TIME_ZERO},
    };

    use super::*;

    #[test]
    fn surface_normal_at_time_zero_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::Z_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.25);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.25);

        let expected = -&Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::Z_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.25);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_half_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.5);

        let expected = -&Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_body_with_retrograde_rotation() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(-1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.25);

        let expected = -&Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_after_quarter_turn_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = Time::from_yr(0.25);

        let expected = -&Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_with_angle_at_epoch() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_yr(1.);

        let angle_at_epoch = Angle::from_degrees(90.);
        let time_since_epoch = Time::from_yr(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_non_rotating_body_with_axis_tilt() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = TIME_ZERO;

        let angle_at_epoch = ANGLE_ZERO;
        let time_since_epoch = TIME_ZERO;

        let expected = -&Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }
}
