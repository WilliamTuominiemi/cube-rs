#[derive(PartialEq, Debug)]
pub struct Position2D {
    x: f32,
    y: f32,
}

impl Position2D {}

#[derive(PartialEq)]
pub struct Position3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Position3D {
    pub fn transform_position_to_2d(&self) -> Position2D {
        return Position2D {
            x: self.x / self.z,
            y: self.y / self.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transforming_position_from_3d_to_2d() {
        let position = Position3D {
            x: 0.7,
            y: 0.3,
            z: 0.8,
        };

        let expected_position = Position2D { x: 0.875, y: 0.375 };

        assert_eq!(position.transform_position_to_2d(), expected_position);

        let negative_position = Position3D {
            x: -0.7,
            y: -0.3,
            z: 0.8,
        };

        let negative_expected_position = Position2D {
            x: -0.875,
            y: -0.375,
        };

        assert_eq!(
            negative_position.transform_position_to_2d(),
            negative_expected_position
        );
    }
}
