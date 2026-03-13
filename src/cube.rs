use crate::position::{self, Position3D};

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

        self.get_line_between_points(left_bottom_back, right_bottom_back, 6)
    }

    fn get_line_between_points(
        &self,
        first: &Position3D,
        second: &Position3D,
        points: usize,
    ) -> Vec<Position3D> {
        (0..points)
            .map(|i| {
                let t = i as f32 / (points - 1) as f32;
                Position3D {
                    x: first.x + (second.x - first.x) * t,
                    y: first.y + (second.y - first.y) * t,
                    z: first.z + (second.z - first.z) * t,
                }
            })
            .collect()
    }
}
