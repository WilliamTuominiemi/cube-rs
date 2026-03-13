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
}
