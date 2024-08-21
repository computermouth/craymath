
include!("bindings.rs");

impl From<[f32; 3]> for Vector2 {
    fn from(f: [f32; 3]) -> Self {
        Self{x: f[0], y: f[1]}
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(f: [f32; 3]) -> Self {
        Self{x: f[0], y: f[1], z: f[2]}
    }
}

impl From<[f32; 16]> for Matrix {
    fn from(f: [f32; 16]) -> Self {
        Self{
            m0: f[0],
            m1: f[1],
            m2: f[2],
            m3: f[3],
            m4: f[4],
            m5: f[5],
            m6: f[6],
            m7: f[7],
            m8: f[8],
            m9: f[9],
            m10: f[10],
            m11: f[11],
            m12: f[12],
            m13: f[13],
            m14: f[14],
            m15: f[15],
        }
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Vector3 {
    fn from(f: [f32; 5]) -> Self {
        Self{x: f[0], y: f[1], z: f[2]}
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Vector3 {
    fn from(f: [f32; 16]) -> Self {
        Self{x: f[0], y: f[1], z: f[2]}
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Quaternion {
    fn from(f: [f32; 5]) -> Self {
        Self{x: f[0], y: f[1], z: f[2], w: f[3]}
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Quaternion {
    fn from(f: [f32; 16]) -> Self {
        Self{x: f[0], y: f[1], z: f[2], w: f[3]}
    }
}

impl From<[f32; 5]> for Rectangle {
    fn from(f: [f32; 5]) -> Self {
        Self{x: f[0], y: f[1], width: f[2], height: f[3]}
    }
}

impl From<[f32; 5]> for Vector2 {
    fn from(f: [f32; 5]) -> Self {
        Self{x: f[0], y: f[1]}
    }
}