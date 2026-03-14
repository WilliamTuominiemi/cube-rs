use crate::position::Position3D;

pub struct Cube {
    pub corners: [Position3D; 8],
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            corners: [
                // Bottom face (y = -1.0)
                Position3D {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                }, // Left, Bottom, Back
                Position3D {
                    x: 1.0,
                    y: -1.0,
                    z: -1.0,
                }, // Right, Bottom, Back
                Position3D {
                    x: -1.0,
                    y: -1.0,
                    z: 1.0,
                }, // Left, Bottom, Front
                Position3D {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0,
                }, // Right, Bottom, Front
                // Top face (y = 1.0)
                Position3D {
                    x: -1.0,
                    y: 1.0,
                    z: -1.0,
                }, // Left, Top, Back
                Position3D {
                    x: 1.0,
                    y: 1.0,
                    z: -1.0,
                }, // Right, Top, Back
                Position3D {
                    x: -1.0,
                    y: 1.0,
                    z: 1.0,
                }, // Left, Top, Front
                Position3D {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }, // Right, Top, Front
            ],
        }
    }

    pub fn apply_xz_rotation(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();

        for position in &mut self.corners {
            let old_x = position.x;
            let old_z = position.z;
            position.x = old_x * c - old_z * s;
            position.z = old_x * s + old_z * c;
        }
    }

    pub fn get_lines(&self) -> Vec<Position3D> {
        let left_bottom_back = &self.corners[0];
        let right_bottom_back = &self.corners[1];
        let left_bottom_front = &self.corners[2];
        let right_bottom_front = &self.corners[3];
        let left_top_back = &self.corners[4];
        let right_top_back = &self.corners[5];
        let left_top_front = &self.corners[6];
        let right_top_front = &self.corners[7];

        let p = 6;

        let lines = left_bottom_back
            .get_line_to(right_bottom_back, p)
            .into_iter()
            .chain(left_bottom_front.get_line_to(right_bottom_front, p))
            .chain(left_top_back.get_line_to(right_top_back, p))
            .chain(left_top_front.get_line_to(right_top_front, p))
            .chain(left_bottom_front.get_line_to(left_bottom_back, p))
            .chain(right_bottom_front.get_line_to(right_bottom_back, p))
            .chain(left_top_front.get_line_to(left_top_back, p))
            .chain(right_top_front.get_line_to(right_top_back, p))
            .chain(left_bottom_back.get_line_to(left_top_back, p))
            .chain(right_bottom_back.get_line_to(right_top_back, p))
            .chain(left_bottom_front.get_line_to(left_top_front, p))
            .chain(right_bottom_front.get_line_to(right_top_front, p))
            .collect();

        lines
    }
}

#[cfg(test)]

mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_applying_xz_rotation() {
        let mut cube = Cube::new();

        let expected_corners = [
            Position3D {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            Position3D {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            Position3D {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            Position3D {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            Position3D {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            Position3D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Position3D {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            Position3D {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
        ];

        cube.apply_xz_rotation(PI / 2.0);

        let epsilon = 1e-7;
        for (actual, expected) in cube.corners.iter().zip(expected_corners.iter()) {
            assert!((actual.x - expected.x).abs() < epsilon,);
            assert!((actual.y - expected.y).abs() < epsilon,);
            assert!((actual.z - expected.z).abs() < epsilon,);
        }
    }
}
