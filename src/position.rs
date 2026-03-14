#[derive(PartialEq, Debug)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn position_in_terminal_scale(
        &self,
        terminal_width: u32,
        terminal_height: u32,
    ) -> Position2D {
        let mapped_x = (self.x + 1.0) / 2.0 * terminal_width as f32;
        let mapped_y = (self.y + 1.0) / 2.0 * terminal_height as f32;

        Position2D {
            x: mapped_x,
            y: mapped_y,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3D {
    pub fn transform_position_to_2d(&self) -> Position2D {
        let camera_distance = 2.0;
        let z_projected = self.z + camera_distance;
        Position2D {
            x: self.x / z_projected,
            y: self.y / z_projected,
        }
    }

    pub fn get_line_to(&self, other: &Position3D, points: usize) -> Vec<Position3D> {
        (0..points)
            .map(|i| {
                let t = i as f32 / (points - 1) as f32;
                Position3D {
                    x: self.x + (other.x - self.x) * t,
                    y: self.y + (other.y - self.y) * t,
                    z: self.z + (other.z - self.z) * t,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transforming_position_from_3d_to_2d() {
        let position = Position3D {
            x: 0.7,
            y: 0.4,
            z: 0.5,
        };

        let expected_position = Position2D { x: 0.28, y: 0.16 };

        assert_eq!(position.transform_position_to_2d(), expected_position);

        let negative_position = Position3D {
            x: -0.7,
            y: -0.4,
            z: 0.5,
        };

        let negative_expected_position = Position2D { x: -0.28, y: -0.16 };

        assert_eq!(
            negative_position.transform_position_to_2d(),
            negative_expected_position
        );
    }

    #[test]
    fn test_transforming_position_into_terminal_scale() {
        let (terminal_width, terminal_height) = (20, 10);

        let position = Position2D { x: 0.8, y: 0.6 };

        let expected_terminal_scale_position = Position2D { x: 18.0, y: 8.0 };

        assert_eq!(
            position.position_in_terminal_scale(terminal_width, terminal_height),
            expected_terminal_scale_position
        );

        let negative_position = Position2D { x: -0.5, y: -0.7 };

        let expected_negative_terminal_scale_position = Position2D { x: 5.0, y: 1.5 };

        assert_eq!(
            negative_position.position_in_terminal_scale(terminal_width, terminal_height),
            expected_negative_terminal_scale_position
        );
    }

    #[test]

    fn test_creating_line_between_two_points() {
        let position = Position3D {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        };
        let other_position = Position3D {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        };

        let amount_of_points = 5;

        let expected_line = vec![
            Position3D {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            Position3D {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            Position3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Position3D {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Position3D {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
        ];

        assert_eq!(
            position.get_line_to(&other_position, amount_of_points),
            expected_line
        );
    }
}
