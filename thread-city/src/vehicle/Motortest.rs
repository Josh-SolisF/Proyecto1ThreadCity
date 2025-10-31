#[cfg(test)]
mod tests {
    use crate::vehicle::Motor::Motion;
    use super::*;

    #[test]
    fn approaching_clamps_at_entry_positive_dir() {
        let mut m = Motion::new(10.0, 4.0, 1); // 10 m/s, dir +1
        // entry = 5 m. Con dt=1s debería intentar pasar a 10, pero clampa en 5.
        m.step_approach(1.0, 5.0);
        assert!((m.pos - 5.0).abs() < 1e-6, "pos debe clamplear a 5.0, got {}", m.pos);
        assert_eq!(m.direction, 1);
    }

    #[test]
    fn approaching_clamps_at_entry_negative_dir() {
        let mut m = Motion::new(10.0, 4.0, -1); // dir -1
        // Suponiendo que parte en 0.0 y la entrada está en -5.0
        m.step_approach(1.0, -5.0);
        assert!((m.pos - (-5.0)).abs() < 1e-6, "pos debe clamplear a -5.0, got {}", m.pos);
        assert_eq!(m.direction, -1);
    }

    #[test]
    fn crossing_advances_position_positive() {
        let mut m = Motion::new(2.0, 4.0, 1);
        m.step_crossing(3.0); // 2 m/s * 3 s = +6 m
        assert!((m.pos - 6.0).abs() < 1e-6);
    }

    #[test]
    fn crossing_advances_position_negative() {
        let mut m = Motion::new(2.0, 4.0, -1);
        m.step_crossing(2.5); // 2 m/s * 2.5 s = -5 m
        assert!((m.pos - (-5.0)).abs() < 1e-6);
    }

    #[test]
    fn direction_is_normalized() {
        let m0 = Motion::new(5.0, 4.0, 0);
        let m5 = Motion::new(5.0, 4.0, 5);
        let mn = Motion::new(5.0, 4.0, -3);
        assert_eq!(m0.direction, 1);
        assert_eq!(m5.direction, 1);
        assert_eq!(mn.direction, -1);
    }

    #[test]
    fn zero_speed_does_not_move() {
        let mut m = Motion::new(0.0, 4.0, 1);
        m.step_approach(1.0, 10.0);
        assert!((m.pos - 0.0).abs() < 1e-6);
        m.step_crossing(1.0);
        assert!((m.pos - 0.0).abs() < 1e-6);
    }
}