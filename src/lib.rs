#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod middleware;
#[allow(unused_imports)]
use middleware as craymath;
#[allow(unused_imports)]
use raymath;

const ITERATIONS: usize = 10000000;
const ITER_DIV: usize =   1000;
// const ITERATIONS: usize = 10000;
// const ITER_DIV: usize =   100;

impl PartialEq<craymath::Matrix> for raymath::Matrix {
    fn eq(&self, other: &craymath::Matrix) -> bool {
        ((self.m0 .is_nan() && other.m0 .is_nan()) || self.m0  == other.m0 ) &&
        ((self.m1 .is_nan() && other.m1 .is_nan()) || self.m1  == other.m1 ) &&
        ((self.m2 .is_nan() && other.m2 .is_nan()) || self.m2  == other.m2 ) &&
        ((self.m3 .is_nan() && other.m3 .is_nan()) || self.m3  == other.m3 ) &&
        ((self.m4 .is_nan() && other.m4 .is_nan()) || self.m4  == other.m4 ) &&
        ((self.m5 .is_nan() && other.m5 .is_nan()) || self.m5  == other.m5 ) &&
        ((self.m6 .is_nan() && other.m6 .is_nan()) || self.m6  == other.m6 ) &&
        ((self.m7 .is_nan() && other.m7 .is_nan()) || self.m7  == other.m7 ) &&
        ((self.m8 .is_nan() && other.m8 .is_nan()) || self.m8  == other.m8 ) &&
        ((self.m9 .is_nan() && other.m9 .is_nan()) || self.m9  == other.m9 ) &&
        ((self.m10.is_nan() && other.m10.is_nan()) || self.m10 == other.m10) &&
        ((self.m11.is_nan() && other.m11.is_nan()) || self.m11 == other.m11) &&
        ((self.m12.is_nan() && other.m12.is_nan()) || self.m12 == other.m12) &&
        ((self.m13.is_nan() && other.m13.is_nan()) || self.m13 == other.m13) &&
        ((self.m14.is_nan() && other.m14.is_nan()) || self.m14 == other.m14) &&
        ((self.m15.is_nan() && other.m15.is_nan()) || self.m15 == other.m15)
    }
}

impl PartialEq<raymath::Matrix> for craymath::Matrix {
    fn eq(&self, other: &raymath::Matrix) -> bool {
        ((self.m0 .is_nan() && other.m0 .is_nan()) || self.m0  == other.m0 ) &&
        ((self.m1 .is_nan() && other.m1 .is_nan()) || self.m1  == other.m1 ) &&
        ((self.m2 .is_nan() && other.m2 .is_nan()) || self.m2  == other.m2 ) &&
        ((self.m3 .is_nan() && other.m3 .is_nan()) || self.m3  == other.m3 ) &&
        ((self.m4 .is_nan() && other.m4 .is_nan()) || self.m4  == other.m4 ) &&
        ((self.m5 .is_nan() && other.m5 .is_nan()) || self.m5  == other.m5 ) &&
        ((self.m6 .is_nan() && other.m6 .is_nan()) || self.m6  == other.m6 ) &&
        ((self.m7 .is_nan() && other.m7 .is_nan()) || self.m7  == other.m7 ) &&
        ((self.m8 .is_nan() && other.m8 .is_nan()) || self.m8  == other.m8 ) &&
        ((self.m9 .is_nan() && other.m9 .is_nan()) || self.m9  == other.m9 ) &&
        ((self.m10.is_nan() && other.m10.is_nan()) || self.m10 == other.m10) &&
        ((self.m11.is_nan() && other.m11.is_nan()) || self.m11 == other.m11) &&
        ((self.m12.is_nan() && other.m12.is_nan()) || self.m12 == other.m12) &&
        ((self.m13.is_nan() && other.m13.is_nan()) || self.m13 == other.m13) &&
        ((self.m14.is_nan() && other.m14.is_nan()) || self.m14 == other.m14) &&
        ((self.m15.is_nan() && other.m15.is_nan()) || self.m15 == other.m15)
    }
}

impl PartialEq<craymath::Vector4> for raymath::Vector4 {
    fn eq(&self, other: &craymath::Vector4) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.z.is_nan() && other.z.is_nan()) || self.z == other.z ) &&
        ((self.w.is_nan() && other.w.is_nan()) || self.w == other.w )
    }
}

impl PartialEq<raymath::Vector4> for craymath::Vector4 {
    fn eq(&self, other: &raymath::Vector4) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.z.is_nan() && other.z.is_nan()) || self.z == other.z ) &&
        ((self.w.is_nan() && other.w.is_nan()) || self.w == other.w )
    }
}

impl PartialEq<craymath::Vector3> for raymath::Vector3 {
    fn eq(&self, other: &craymath::Vector3) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.z.is_nan() && other.z.is_nan()) || self.z == other.z )
    }
}

impl PartialEq<raymath::Vector3> for craymath::Vector3 {
    fn eq(&self, other: &raymath::Vector3) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.z.is_nan() && other.z.is_nan()) || self.z == other.z )
    }
}

impl PartialEq<raymath::Vector2> for craymath::Vector2 {
    fn eq(&self, other: &raymath::Vector2) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y )
    }
}

impl PartialEq<craymath::Vector2> for raymath::Vector2 {
    fn eq(&self, other: &craymath::Vector2) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y )
    }
}

impl PartialEq<craymath::Rectangle> for raymath::Rectangle {
    fn eq(&self, other: &craymath::Rectangle) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.width.is_nan() && other.width.is_nan()) || self.width == other.width ) &&
        ((self.height.is_nan() && other.height.is_nan()) || self.height == other.height )
    }
}


impl PartialEq<raymath::Rectangle> for craymath::Rectangle {
    fn eq(&self, other: &raymath::Rectangle) -> bool {
        ((self.x.is_nan() && other.x.is_nan()) || self.x == other.x ) &&
        ((self.y.is_nan() && other.y.is_nan()) || self.y == other.y ) &&
        ((self.width.is_nan() && other.width.is_nan()) || self.width == other.width ) &&
        ((self.height.is_nan() && other.height.is_nan()) || self.height == other.height )
    }
}

impl PartialEq<craymath::Ray> for raymath::Ray {
    fn eq(&self, other: &craymath::Ray) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl PartialEq<raymath::Ray> for craymath::Ray {
    fn eq(&self, other: &raymath::Ray) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl PartialEq<craymath::RayCollision> for raymath::RayCollision {
    fn eq(&self, other: &craymath::RayCollision) -> bool {
        self.hit == other.hit && self.distance == other.distance &&
        self.point == other.point && self.normal == other.normal
    }
}

impl PartialEq<raymath::RayCollision> for craymath::RayCollision {
    fn eq(&self, other: &raymath::RayCollision) -> bool {
        self.hit == other.hit &&
        ( self.distance.is_nan() && other.distance.is_nan() ) || self.distance == other.distance &&
        self.point == other.point && self.normal == other.normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use bindings::{Vector4, Vector4Zero};
    use rand::prelude::*;

    fn gen_vec3s() -> Vec<[f32;3]> {
        let mut out = vec![
            [0.,0.,0.],
            [0.,0.,0.],
            [1.,1.,1.],
            [1.,1.,1.],
            [1.,1.,1.],
            [0.,0.,0.],
            [0.,0.,0.],
            [1.,1.,1.]
        ];
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let i: i32 = rng.gen();
            let x: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            out.push([x,y,z]);
        }

        out
    }

    fn gen_vec2_vecs() -> Vec<raymath::Vector2> {
        let mut out = vec![
            [0.,0.,0.].into(),
            [0.,0.,0.].into(),
            [1.,1.,1.].into(),
            [1.,1.,1.].into(),
            [1.,1.,1.].into(),
            [0.,0.,0.].into(),
            [0.,0.,0.].into(),
            [1.,1.,1.].into()
        ];
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let i: i32 = rng.gen();
            let x: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            out.push([x,y,z].into());
        }

        out
    }

    fn gen_vec3_vecs() -> Vec<[raymath::Vector3;3]> {
        let mut out: Vec<[raymath::Vector3;3]> = vec![
            [
                [0.,0.,0.].into(),
                [0.,0.,0.].into(),
                [0.,0.,0.].into(),
            ],
            [
                [1.,1.,1.].into(),
                [1.,1.,1.].into(),
                [1.,1.,1.].into(),
            ],
        ];
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let i1: i32 = rng.gen();
            let x1: f32 = i1 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y1: f32 = i1 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z1: f32 = i1 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let i2: i32 = rng.gen();
            let x2: f32 = i2 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y2: f32 = i2 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z2: f32 = i2 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let i3: i32 = rng.gen();
            let x3: f32 = i3 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y3: f32 = i3 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z3: f32 = i3 as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            out.push(
                [
                    [x1,y1,z1].into(),
                    [x2,y2,z2].into(),
                    [x3,y3,z3].into(),
                ]
            );
        }

        out
    }

    fn gen_vec5s() -> Vec<[f32;5]> {
        let mut out = vec![
            [0.,0.,0.,0.,0.],
            [0.,0.,0.,0.,0.],
            [1.,1.,1.,1.,1.],
            [1.,1.,1.,1.,1.],
            [1.,1.,1.,1.,1.],
            [0.,0.,0.,0.,0.],
            [0.,0.,0.,0.,0.],
            [1.,1.,1.,1.,1.]
        ];
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let i: i32 = rng.gen();
            let v: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let w: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let x: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let y: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            let z: f32 = i as f32 * rng.gen::<f32>(); // generates a float between 0 and 1
            out.push([v,w,x,y,z]);
        }

        out
    }

    fn gen_vec16s() -> Vec<[f32;16]> {
        let mut out = vec![
            [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.],
            [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.],
            [1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.],
            [1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.],
            [1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.],
            [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.],
            [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.],
            [1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.,1.],
        ];
        let mut rng = rand::thread_rng();

        for _ in 0..ITERATIONS {
            let i: i32 = rng.gen();
            let m0: f32 = i as f32 * rng.gen::<f32>();
            let m1: f32 = i as f32 * rng.gen::<f32>();
            let m2: f32 = i as f32 * rng.gen::<f32>();
            let m3: f32 = i as f32 * rng.gen::<f32>();
            let m4: f32 = i as f32 * rng.gen::<f32>();
            let m5: f32 = i as f32 * rng.gen::<f32>();
            let m6: f32 = i as f32 * rng.gen::<f32>();
            let m7: f32 = i as f32 * rng.gen::<f32>();
            let m8: f32 = i as f32 * rng.gen::<f32>();
            let m9: f32 = i as f32 * rng.gen::<f32>();
            let ma: f32 = i as f32 * rng.gen::<f32>();
            let mb: f32 = i as f32 * rng.gen::<f32>();
            let mc: f32 = i as f32 * rng.gen::<f32>();
            let md: f32 = i as f32 * rng.gen::<f32>();
            let me: f32 = i as f32 * rng.gen::<f32>();
            let mf: f32 = i as f32 * rng.gen::<f32>();
            out.push([m0,m1,m2,m3,m4,m5,m6,m7,m8,m9,ma,mb,mc,md,me,mf]);
        }

        out
    }

    #[test]
    fn lerp() {
        let v3 = gen_vec3s();
        for v in v3 {
            let cres = unsafe {craymath::Lerp(v[0], v[1], v[2])};
            let rres = raymath::lerp(v[0], v[1], v[2]);
            assert_eq!(cres, rres);
        }
    }

    #[test]
    fn normalize() {
        let v3 = gen_vec3s();
        for v in v3 {
            let cres = unsafe {craymath::Normalize(v[0], v[1], v[2])};
            let rres = raymath::normalize(v[0], v[1], v[2]);
            if cres.is_nan() && rres.is_nan() { continue }
            assert_eq!(cres, rres);
        }
    }

    #[test]
    fn remap() {
        let v3 = gen_vec5s();
        for v in v3 {
            let cres = unsafe {craymath::Remap(v[0], v[1], v[2], v[3], v[4])};
            let rres = raymath::remap(v[0], v[1], v[2], v[3], v[4]);
            if cres.is_nan() && rres.is_nan() { continue }
            assert_eq!(cres, rres);
        }
    }

    #[test]
    fn wrap() {
        let v3 = gen_vec3s();
        for v in v3 {
            let cres = unsafe {craymath::Wrap(v[0], v[1], v[2])};
            let rres = raymath::wrap(v[0], v[1], v[2]);
            if cres.is_nan() && rres.is_nan() { continue }
            assert_eq!(cres, rres);
        }
    }

    #[test]
    fn float_equals() {
        let v3 = gen_vec3s();
        for (i,v) in v3.iter().enumerate() {
            let cres = unsafe {craymath::FloatEquals(v[0], v[1])};
            let rres = raymath::float_equals(v[0], v[1]);
            assert_eq!(cres, rres as i32, "[{i}]: {:?}", v);
        }
    }

// VEC2 unique ===============================================================================================

    #[test]
    fn vector2_rotate() {
        let v3 = gen_vec3s();
        for input in v3.chunks(2) {
            let cres = unsafe {craymath::Vector2Rotate(input[0].into(), input[1][0])};
            let rres = raymath::vector2_rotate(input[0].into(), input[1][0]);
            assert_eq!(cres.x, rres.x);
            assert_eq!(cres.y, rres.y);
        }
    }

    #[test]
    fn vector2_line_angle() {
        let v3 = gen_vec3s();
        for v in v3.chunks(2) {
            let cres = unsafe {craymath::Vector2LineAngle(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
            let rres = raymath::vector2_line_angle(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
            assert_eq!(cres, rres);
        }
    }

//  VECTOR 2 SHARED =======================================================================

#[test]
fn vector2_zero() {
    let cres = unsafe {craymath::Vector2Zero()};
    let rres = raymath::vector2_zero();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
}

#[test]
fn vector2_one() {
    let cres = unsafe {craymath::Vector2One()};
    let rres = raymath::vector2_one();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
}

#[test]
fn vector2_add() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Add(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{ x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_add(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{ x: v[1][0], y: v[1][1]});
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_add_value() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2AddValue(craymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0])};
        let rres = raymath::vector2_add_value(raymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_subtract() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Subtract(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{ x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_subtract(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{ x: v[1][0], y: v[1][1]});
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_subtract_value() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2SubtractValue(craymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0])};
        let rres = raymath::vector2_subtract_value(raymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_length() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Length(craymath::Vector2{x: v[0][0], y: v[0][1]})};
        let rres = raymath::vector2_length(raymath::Vector2{x: v[0][0], y: v[0][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_length_sqr() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2LengthSqr(craymath::Vector2{x: v[0][0], y: v[0][1]})};
        let rres = raymath::vector2_length_sqr(raymath::Vector2{x: v[0][0], y: v[0][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_dot_product() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2DotProduct(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_dot_product(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_distance() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Distance(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_distance(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_distance_sqr() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2DistanceSqr(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_distance_sqr(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_angle() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Angle(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_angle(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector2_scale() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Scale(craymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0])};
        let rres = raymath::vector2_scale(raymath::Vector2{x: v[0][0], y: v[0][1]}, v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_multiply() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Multiply(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_multiply(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_divide() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Divide(craymath::Vector2{x: v[0][0], y: v[0][1]}, craymath::Vector2{x: v[1][0], y: v[1][1]})};
        let rres = raymath::vector2_divide(raymath::Vector2{x: v[0][0], y: v[0][1]}, raymath::Vector2{x: v[1][0], y: v[1][1]});
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
    }
}

#[test]
fn vector2_negate() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Negate(craymath::Vector2{x: v[0][0], y: v[0][1]})};
        let rres = raymath::vector2_negate(raymath::Vector2{x: v[0][0], y: v[0][1]});
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_normalize() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Normalize(craymath::Vector2{x: v[0][0], y: v[0][1]})};
        let rres = raymath::vector2_normalize(raymath::Vector2{x: v[0][0], y: v[0][1]});
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_transform() {
    let v3 = gen_vec3s();
    let mats = gen_vec16s();
    for i in 0..v3.len() {
        let cres = unsafe {craymath::Vector2Transform(v3[i].into(), mats[i].into())};
        let rres = raymath::vector2_transform(v3[i].into(), mats[i].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_lerp() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {

        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector2Lerp(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector2_lerp(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_reflect() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Reflect(input[0].into(), input[1].into())};
        let rres = raymath::vector2_reflect(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_min() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Min(input[0].into(), input[1].into())};
        let rres = raymath::vector2_min(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_max() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Max(input[0].into(), input[1].into())};
        let rres = raymath::vector2_max(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_move_towards() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector2MoveTowards(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector2_move_towards(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

#[test]
fn vector2_invert() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::Vector2Invert(input.into())};
        let rres = raymath::vector2_invert(input.into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

// #[test]
// fn vector2_clamp() {
//     let v3 = gen_vec3s();
//     for input in v3.chunks(3) {
//         let cres = unsafe {craymath::Vector2Clamp(input[0].into(), input[1].into(), input[2].into())};
//         let rres = raymath::vector2_clamp(input[0].into(), input[1].into(), input[2].into());
//         assert_eq!(cres.x, rres.x);
//         assert_eq!(cres.y, rres.y);
//     }
// }

#[test]
fn vector2_equals() {
    let v3 = gen_vec3s();

    let cres = unsafe {craymath::Vector2Equals(craymath::Vector2Zero(), craymath::Vector2Zero())};
    let rres = raymath::vector2_equals(raymath::vector2_zero(), raymath::vector2_zero());
    assert_eq!(cres, rres as i32);

    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector2Equals(input[0].into(), input[1].into())};
        let rres = raymath::vector2_equals(input[0].into(), input[1].into());
        assert_eq!(cres, rres as i32);
    }
}

#[test]
fn vector2_refract() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector2Refract(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector2_refract(input[0].into(), input[1].into(), v);
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
    }
}

// VEC 3 UNIQUE ====================================================================================

#[test]
fn vector3_cubic_hermite() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(4) {

        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector3CubicHermite(input[0].into(), input[1].into(), input[2].into(), input[3].into(), v)};
        let rres = raymath::vector3_cubic_hermite(input[0].into(), input[1].into(), input[2].into(), input[3].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_cross_product() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3CrossProduct(v[0].into(), v[1].into())};
        let rres = raymath::vector3_cross_product(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_perpendicular() {
    let v3 = gen_vec3s();
    for v in v3 {
        let cres = unsafe {craymath::Vector3Perpendicular(v.into())};
        let rres = raymath::vector3_perpendicular(v.into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_to_floatv() {
    let v3 = gen_vec3s();
    for v in v3 {
        let cres = unsafe {craymath::Vector3ToFloatV(v.into())};
        let rres = raymath::vector3_to_float_v(v.into());
        assert_eq!(cres.v[0], rres.v[0]);
        assert_eq!(cres.v[1], rres.v[1]);
        assert_eq!(cres.v[2], rres.v[2]);
    }
}

#[test]
fn vector3_project() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Project(v[0].into(), v[1].into())};
        let rres = raymath::vector3_project(v[0].into(), v[1].into());
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
    }
}

#[test]
fn vector3_reject() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Reject(v[0].into(), v[1].into())};
        let rres = raymath::vector3_reject(v[0].into(), v[1].into());
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
    }
}

#[test]
fn vector3_ortho_normalize() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let mut v1 = v[0].into();
        let mut v2 = v[1].into();
        unsafe {craymath::Vector3OrthoNormalize(&mut v1, &mut v2)};
        let (rres1, rres2) = raymath::vector3_ortho_normalize(v[0].into(), v[1].into());
        assert_eq!(v1.x, rres1.x);
        assert_eq!(v1.y, rres1.y);
        assert_eq!(v1.z, rres1.z);

        assert_eq!(v2.x, rres2.x);
        assert_eq!(v2.y, rres2.y);
        assert_eq!(v2.z, rres2.z);
    }
}

#[test]
fn vector3_rotate_by_quaternion() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3RotateByQuaternion(v[0].into(), v[1].into())};
        let rres = raymath::vector3_rotate_by_quaternion(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_rotate_by_axis_angle() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {

        let i: i32 = rng.gen();
        let f: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector3RotateByAxisAngle(v[0].into(), v[1].into(), f)};
        let rres = raymath::vector3_rotate_by_axis_angle(v[0].into(), v[1].into(), f);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_rotate_barycenter() {
    let v3 = gen_vec3s();
    for v in v3.chunks(4) {

        let cres = unsafe {craymath::Vector3Barycenter(v[0].into(), v[1].into(), v[2].into(), v[3].into())};
        let rres = raymath::vector3_barycenter(v[0].into(), v[1].into(), v[2].into(), v[3].into());
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
    }
}

// #[test]
// fn vector3_unproject() {
//     let v3 = gen_vec16s();
//     for v in v3.chunks(3) {

//         let cres = unsafe {craymath::Vector3Unproject(v[0].into(), v[1].into(), v[2].into())};
//         let rres = raymath::vector3_unproject(v[0].into(), v[1].into(), v[2].into());
//         if !cres.x.is_nan() && !rres.x.is_nan() {
//             assert_eq!(cres.x, rres.x);
//         }
//         if !cres.y.is_nan() && !rres.y.is_nan() {
//             assert_eq!(cres.y, rres.y);
//         }
//         if !cres.z.is_nan() && !rres.z.is_nan() {
//             assert_eq!(cres.z, rres.z);
//         }
//     }
// }

// VECTOR 3 SHARED =======================================================================

#[test]
fn vector3_zero() {
    let cres = unsafe {craymath::Vector3Zero()};
    let rres = raymath::vector3_zero();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
    assert_eq!(cres.z, rres.z);
}

#[test]
fn vector3_one() {
    let cres = unsafe {craymath::Vector3One()};
    let rres = raymath::vector3_one();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
    assert_eq!(cres.z, rres.z);
}

#[test]
fn vector3_add() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Add(v[0].into(), v[1].into())};
        let rres = raymath::vector3_add(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_add_value() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3AddValue(v[0].into(), v[1][0])};
        let rres = raymath::vector3_add_value(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_subtract() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Subtract(v[0].into(), v[1].into())};
        let rres = raymath::vector3_subtract(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_subtract_value() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3SubtractValue(v[0].into(), v[1][0])};
        let rres = raymath::vector3_subtract_value(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_length() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Length(v[0].into())};
        let rres = raymath::vector3_length(v[0].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_length_sqr() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3LengthSqr(v[0].into())};
        let rres = raymath::vector3_length_sqr(v[0].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_dot_product() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3DotProduct(v[0].into(), v[1].into())};
        let rres = raymath::vector3_dot_product(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_distance() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Distance(v[0].into(), v[1].into())};
        let rres = raymath::vector3_distance(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_distance_sqr() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3DistanceSqr(v[0].into(), v[1].into())};
        let rres = raymath::vector3_distance_sqr(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_angle() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Angle(v[0].into(), v[1].into())};
        let rres = raymath::vector3_angle(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector3_scale() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Scale(v[0].into(), v[1][0])};
        let rres = raymath::vector3_scale(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_multiply() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Multiply(v[0].into(), v[1].into())};
        let rres = raymath::vector3_multiply(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_divide() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Divide(v[0].into(), v[1].into())};
        let rres = raymath::vector3_divide(v[0].into(), v[1].into());
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
    }
}

#[test]
fn vector3_negate() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Negate(v[0].into())};
        let rres = raymath::vector3_negate(v[0].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_normalize() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Normalize(v[0].into())};
        let rres = raymath::vector3_normalize(v[0].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_transform() {
    let v3 = gen_vec3s();
    let mats = gen_vec16s();
    for i in 0..v3.len() {
        let cres = unsafe {craymath::Vector3Transform(v3[i].into(), mats[i].into())};
        let rres = raymath::vector3_transform(v3[i].into(), mats[i].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_lerp() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {

        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector3Lerp(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector3_lerp(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_reflect() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Reflect(input[0].into(), input[1].into())};
        let rres = raymath::vector3_reflect(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_min() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Min(input[0].into(), input[1].into())};
        let rres = raymath::vector3_min(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_max() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Max(input[0].into(), input[1].into())};
        let rres = raymath::vector3_max(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_move_towards() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector3MoveTowards(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector3_move_towards(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

#[test]
fn vector3_invert() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::Vector3Invert(input.into())};
        let rres = raymath::vector3_invert(input.into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
    }
}

// #[test]
// fn vector3_clamp() {
//     let v3 = gen_vec3s();
//     for input in v3.chunks(3) {
//         let cres = unsafe {craymath::Vector3Clamp(input[0].into(), input[1].into(), input[2].into())};
//         let rres = raymath::vector3_clamp(input[0].into(), input[1].into(), input[2].into());
//         assert_eq!(cres.x, rres.x);
//         assert_eq!(cres.y, rres.y);
//     }
// }

#[test]
fn vector3_equals() {
    let v3 = gen_vec3s();

    let cres = unsafe {craymath::Vector3Equals(craymath::Vector3Zero(), craymath::Vector3Zero())};
    let rres = raymath::vector3_equals(raymath::vector3_zero(), raymath::vector3_zero());
    assert_eq!(cres, rres as i32);

    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Equals(input[0].into(), input[1].into())};
        let rres = raymath::vector3_equals(input[0].into(), input[1].into());
        assert_eq!(cres, rres as i32);
    }
}

#[test]
fn vector3_refract() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector3Refract(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector3_refract(input[0].into(), input[1].into(), v);
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
    }
}

// VEC 4 SHARED ================================================================================================

#[test]
fn vector4_zero() {
    let cres = unsafe {craymath::Vector4Zero()};
    let rres = raymath::vector4_zero();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
    assert_eq!(cres.z, rres.z);
    assert_eq!(cres.w, rres.w);
}

#[test]
fn vector4_one() {
    let cres = unsafe {craymath::Vector4One()};
    let rres = raymath::vector4_one();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
    assert_eq!(cres.z, rres.z);
    assert_eq!(cres.w, rres.w);
}

#[test]
fn vector4_add() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Add(v[0].into(), v[1].into())};
        let rres = raymath::vector4_add(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_add_value() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4AddValue(v[0].into(), v[1][0])};
        let rres = raymath::vector4_add_value(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_subtract() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Subtract(v[0].into(), v[1].into())};
        let rres = raymath::vector4_subtract(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_subtract_value() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4SubtractValue(v[0].into(), v[1][0])};
        let rres = raymath::vector4_subtract_value(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_length() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Length(v[0].into())};
        let rres = raymath::vector4_length(v[0].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector4_length_sqr() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4LengthSqr(v[0].into())};
        let rres = raymath::vector4_length_sqr(v[0].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector4_dot_product() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4DotProduct(v[0].into(), v[1].into())};
        let rres = raymath::vector4_dot_product(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector4_distance() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Distance(v[0].into(), v[1].into())};
        let rres = raymath::vector4_distance(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector4_distance_sqr() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4DistanceSqr(v[0].into(), v[1].into())};
        let rres = raymath::vector4_distance_sqr(v[0].into(), v[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn vector4_scale() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Scale(v[0].into(), v[1][0])};
        let rres = raymath::vector4_scale(v[0].into(), v[1][0]);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_multiply() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Multiply(v[0].into(), v[1].into())};
        let rres = raymath::vector4_multiply(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_divide() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Divide(v[0].into(), v[1].into())};
        let rres = raymath::vector4_divide(v[0].into(), v[1].into());
        if !cres.x.is_nan() && !rres.x.is_nan() {
            assert_eq!(cres.x, rres.x);
        }
        if !cres.y.is_nan() && !rres.y.is_nan() {
            assert_eq!(cres.y, rres.y);
        }
        if !cres.z.is_nan() && !rres.z.is_nan() {
            assert_eq!(cres.z, rres.z);
        }
        if !cres.w.is_nan() && !rres.w.is_nan() {
            assert_eq!(cres.w, rres.w);
        }
    }
}

#[test]
fn vector4_negate() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Negate(v[0].into())};
        let rres = raymath::vector4_negate(v[0].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_normalize() {
    let v3 = gen_vec5s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Normalize(v[0].into())};
        let rres = raymath::vector4_normalize(v[0].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_lerp() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {

        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector4Lerp(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector4_lerp(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_min() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Min(input[0].into(), input[1].into())};
        let rres = raymath::vector4_min(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_max() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Max(input[0].into(), input[1].into())};
        let rres = raymath::vector4_max(input[0].into(), input[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_move_towards() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();

        let cres = unsafe {craymath::Vector4MoveTowards(input[0].into(), input[1].into(), v)};
        let rres = raymath::vector4_move_towards(input[0].into(), input[1].into(), v);
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

#[test]
fn vector4_invert() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::Vector4Invert(input.into())};
        let rres = raymath::vector4_invert(input.into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
        assert_eq!(cres.z, rres.z);
        assert_eq!(cres.w, rres.w);
    }
}

// #[test]
// fn vector4_clamp() {
//     let v3 = gen_vec5s();
//     for input in v3.chunks(3) {
//         let cres = unsafe {craymath::Vector4Clamp(input[0].into(), input[1].into(), input[2].into())};
//         let rres = raymath::vector4_clamp(input[0].into(), input[1].into(), input[2].into());
//         assert_eq!(cres.x, rres.x);
//         assert_eq!(cres.y, rres.y);
//     }
// }

#[test]
fn vector4_equals() {
    let v3 = gen_vec5s();

    let cres = unsafe {craymath::Vector4Equals(craymath::Vector4Zero(), craymath::Vector4Zero())};
    let rres = raymath::vector4_equals(raymath::vector4_zero(), raymath::vector4_zero());
    assert_eq!(cres, rres as i32);

    for input in v3.chunks(2) {
        let cres = unsafe {craymath::Vector4Equals(input[0].into(), input[1].into())};
        let rres = raymath::vector4_equals(input[0].into(), input[1].into());
        assert_eq!(cres, rres as i32);
    }
}

// Matrices ===============================================================================================

#[test]
fn matrix_determinant() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixDeterminant(input.into())};
        let rres = raymath::matrix_determinant(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_trace() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixTrace(input.into())};
        let rres = raymath::matrix_trace(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_transpose() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixTranspose(input.into())};
        let rres = raymath::matrix_transpose(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_invert() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixInvert(input.into())};
        let rres = raymath::matrix_invert(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_identity() {

    let cres = unsafe {craymath::MatrixIdentity()};
    let rres = raymath::matrix_identity();
    assert_eq!(cres, rres);
}

#[test]
fn matrix_add() {
    let v3 = gen_vec16s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixAdd(input[0].into(), input[1].into())};
        let rres = raymath::matrix_add(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_subtract() {
    let v3 = gen_vec16s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixSubtract(input[0].into(), input[1].into())};
        let rres = raymath::matrix_subtract(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_multiply() {
    let v3 = gen_vec16s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixMultiply(input[0].into(), input[1].into())};
        let rres = raymath::matrix_multiply(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_translate() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixTranslate(input[0].into(), input[1].into(), input[2].into())};
        let rres = raymath::matrix_translate(input[0].into(), input[1].into(), input[2].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate() {
    let mut rng = rand::thread_rng();
    let v3 = gen_vec3s();
    for input in v3 {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();
        let cres = unsafe {craymath::MatrixRotate(input.into(), v)};
        let rres = raymath::matrix_rotate(input.into(), v);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate_x() {
    let mut rng = rand::thread_rng();
    for _ in 0..ITERATIONS {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();
        let cres = unsafe {craymath::MatrixRotateX(v)};
        let rres = raymath::matrix_rotate_x(v);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate_y() {
    let mut rng = rand::thread_rng();
    for _ in 0..ITERATIONS {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();
        let cres = unsafe {craymath::MatrixRotateY(v)};
        let rres = raymath::matrix_rotate_y(v);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate_z() {
    let mut rng = rand::thread_rng();
    for _ in 0..ITERATIONS {
        let i: i32 = rng.gen();
        let v: f32 = i as f32 * rng.gen::<f32>();
        let cres = unsafe {craymath::MatrixRotateZ(v)};
        let rres = raymath::matrix_rotate_z(v);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate_xyz() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixRotateXYZ(input.into())};
        let rres = raymath::matrix_rotate_xyz(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_rotate_zyx() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixRotateZYX(input.into())};
        let rres = raymath::matrix_rotate_zyx(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_scale() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixScale(input[0].into(), input[1].into(), input[2].into())};
        let rres = raymath::matrix_scale(input[0].into(), input[1].into(), input[2].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_frustum() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixFrustum(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64, input[1][1] as f64, input[1][2] as f64)};
        let rres = raymath::matrix_frustum(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64, input[1][1] as f64, input[1][2] as f64);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_perspective() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixPerspective(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64)};
        let rres = raymath::matrix_perspective(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_ortho() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::MatrixOrtho(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64, input[1][1] as f64, input[1][2] as f64)};
        let rres = raymath::matrix_ortho(input[0][0] as f64, input[0][1] as f64, input[0][2] as f64,input[1][0] as f64, input[1][1] as f64, input[1][2] as f64);
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_look_at() {
    let v3 = gen_vec3s();
    for input in v3.chunks(3) {
        let cres = unsafe {craymath::MatrixLookAt(input[0].into(), input[1].into(), input[2].into())};
        let rres = raymath::matrix_look_at(input[0].into(), input[1].into(), input[2].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn matrix_to_float_v() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::MatrixToFloatV(input.into())};
        let rres = raymath::matrix_to_float_v(input.into());
        assert_eq!(cres.v, rres.v);
    }
}

// QUATERNIONS =============================================================================================

#[test]
fn quaternion_identity() {
    let cres = unsafe {craymath::QuaternionIdentity()};
    let rres = raymath::quaternion_identity();
    assert_eq!(cres, rres);
}

#[test]
fn quaternion_add() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionAdd(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_add(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_subtract() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionSubtract(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_subtract(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_multiply() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionMultiply(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_multiply(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_divide() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionDivide(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_divide(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_equals() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionEquals(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_equals(input[0].into(), input[1].into());
        assert_eq!(cres, rres as i32);
    }
}

#[test]
fn quaternion_length() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionLength(input.into())};
        let rres = raymath::quaternion_length(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_normalize() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionNormalize(input.into())};
        let rres = raymath::quaternion_normalize(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_invert() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionInvert(input.into())};
        let rres = raymath::quaternion_invert(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_to_matrix() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionToMatrix(input.into())};
        let rres = raymath::quaternion_to_matrix(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_from_matrix() {
    let v3 = gen_vec16s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionFromMatrix(input.into())};
        let rres = raymath::quaternion_from_matrix(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_from_vector3_to_vector3() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionFromVector3ToVector3(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_from_vector3_to_vector3(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_from_axis_angle() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionFromAxisAngle(input[0].into(), input[1][0])};
        let rres = raymath::quaternion_from_axis_angle(input[0].into(), input[1][0]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_from_euler() {
    let v3 = gen_vec3s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionFromEuler(input[0].into(), input[1], input[2])};
        let rres = raymath::quaternion_from_euler(input[0].into(), input[1], input[2]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_to_euler() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionToEuler(input.into())};
        let rres = raymath::quaternion_to_euler(input.into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_cubic_hermite_spline() {
    let v3 = gen_vec5s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::QuaternionCubicHermiteSpline(input[0].into(), input[1].into(), input[2].into(), input[3].into(), input[3][4])};
        let rres = raymath::quaternion_cubic_hermite_spline(input[0].into(), input[1].into(), input[2].into(), input[3].into(), input[3][4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_lerp() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionLerp(input[0].into(), input[1].into(), input[1][4])};
        let rres = raymath::quaternion_lerp(input[0].into(), input[1].into(), input[1][4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_nlerp() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionNlerp(input[0].into(), input[1].into(), input[1][4])};
        let rres = raymath::quaternion_nlerp(input[0].into(), input[1].into(), input[1][4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_slerp() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionSlerp(input[0].into(), input[1].into(), input[1][4])};
        let rres = raymath::quaternion_slerp(input[0].into(), input[1].into(), input[1][4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_add_value() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionAddValue(input.into(), input[4])};
        let rres = raymath::quaternion_add_value(input.into(), input[4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_subtract_value() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionSubtractValue(input.into(), input[4])};
        let rres = raymath::quaternion_subtract_value(input.into(), input[4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_scale() {
    let v3 = gen_vec5s();
    for input in v3 {
        let cres = unsafe {craymath::QuaternionScale(input.into(), input[4])};
        let rres = raymath::quaternion_scale(input.into(), input[4]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_transform() {
    let v3 = gen_vec16s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::QuaternionTransform(input[0].into(), input[1].into())};
        let rres = raymath::quaternion_transform(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn quaternion_to_axis_angle() {
    let v3 = gen_vec5s();
    for input in v3 {
        let q1 = input.into();
        let mut v1= unsafe { craymath::Vector3Zero() };
        let mut f1 = 0.0;
        unsafe {craymath::QuaternionToAxisAngle(q1, &mut v1, &mut f1)};
        let (v2, f2) = raymath::quaternion_to_axis_angle(input.into());
        assert_eq!(v1, v2);
        assert_eq!(f1, f2);
    }
}

#[test]
fn matrix_decompose() {
    let v3 = gen_vec16s();
    for input in v3 {
        let m1 = input.into();
        let mut q1 = unsafe { craymath::Vector4Zero() };
        let mut v1 = unsafe { craymath::Vector3Zero() };
        let mut s1 = unsafe { craymath::Vector3Zero() };
        unsafe {craymath::MatrixDecompose(m1, &mut v1, &mut q1, &mut s1)};
        let (v2, q2, s2) = raymath::matrix_decompose(input.into());
        assert_eq!(q1, q2);
        assert_eq!(v1, v2);
        assert_eq!(s1, s2);
    }
}


#[test]
fn check_collision_recs() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionRecs(input[0].into(), input[1].into())};
        let rres = raymath::check_collision_recs(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_circles() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionCircles(input[0].into(), input[0][2], input[1].into(), input[1][2])};
        let rres = raymath::check_collision_circles(input[0].into(), input[0][2], input[1].into(), input[1][2]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_circle_rec() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionCircleRec(input[0].into(), input[0][2], input[1].into())};
        let rres = raymath::check_collision_circle_rec(input[0].into(), input[0][2], input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_point_rec() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionPointRec(input[0].into(), input[1].into())};
        let rres = raymath::check_collision_point_rec(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_point_circle() {
    let v3 = gen_vec3s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionPointCircle(input[0].into(), input[1].into(), input[1][2])};
        let rres = raymath::check_collision_point_circle(input[0].into(), input[1].into(), input[1][2]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_point_triangle() {
    let v3 = gen_vec3s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::CheckCollisionPointTriangle(input[0].into(), input[1].into(), input[2].into(), input[3].into())};
        let rres = raymath::check_collision_point_triangle(input[0].into(), input[1].into(), input[2].into(), input[3].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_point_poly() {
    let mut points = gen_vec3s();
    points.truncate(ITERATIONS / ITER_DIV);

    for point in points {
        let mut poly = gen_vec2_vecs();
        poly.truncate(ITERATIONS / ITER_DIV);
        let mut cpoly = vec![];
        for i in &poly {
            cpoly.push(craymath::Vector2{x: i.x, y: i.y});
        }
        let cres = unsafe {craymath::CheckCollisionPointPoly(point.into(), cpoly.as_mut_ptr(), poly.len() as i32)};
        let rres = raymath::check_collision_point_poly(point.into(), poly);
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_lines() {
    let v3 = gen_vec3s();
    for input in v3.chunks(4) {
        let mut cres2 = craymath::Vector2 {x: 0., y: 0.};
        let cres = unsafe {craymath::CheckCollisionLines(input[0].into(), input[1].into(), input[2].into(), input[3].into(), &mut cres2)};
        let rres = raymath::check_collision_lines(input[0].into(), input[1].into(), input[2].into(), input[3].into());

        match rres {
            None => assert!(cres == false),
            Some(rres) => assert_eq!(cres2, rres),
        }
    }
}

#[test]
fn check_collision_point_line() {
    let v3 = gen_vec3s();
    for input in v3.chunks(3) {
        let mut threshold: i32 = rand::thread_rng().gen();
        if threshold < 0 {
            threshold = -threshold;
        }
        let cres = unsafe {craymath::CheckCollisionPointLine(input[0].into(), input[1].into(), input[2].into(), threshold as i32)};
        let rres = raymath::check_collision_point_line(input[0].into(), input[1].into(), input[2].into(), threshold as usize);
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_circle_line() {
    let v3 = gen_vec3s();
    for input in v3.chunks(3) {
        let cres = unsafe {craymath::CheckCollisionCircleLine(input[0].into(), input[0][2], input[1].into(), input[2].into())};
        let rres = raymath::check_collision_circle_line(input[0].into(), input[0][2], input[1].into(), input[2].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn get_collision_rec() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::GetCollisionRec(input[0].into(), input[1].into())};
        let rres = raymath::get_collision_rec(input[0].into(), input[1].into());
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_spheres() {
    let v3 = gen_vec5s();
    for input in v3.chunks(2) {
        let cres = unsafe {craymath::CheckCollisionSpheres(input[0].into(), input[0][3], input[1].into(), input[1][3])};
        let rres = raymath::check_collision_spheres(input[0].into(), input[0][3], input[1].into(), input[1][3]);
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_boxes() {
    let v3 = gen_vec5s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::CheckCollisionBoxes(craymath::BoundingBox { min: input[0].into(), max: input[1].into() }, craymath::BoundingBox { min: input[2].into(), max: input[3].into() })};
        let rres = raymath::check_collision_boxes(raymath::BoundingBox { min: input[0].into(), max: input[1].into() }, raymath::BoundingBox { min: input[2].into(), max: input[3].into() });
        assert_eq!(cres, rres);
    }
}

#[test]
fn check_collision_box_sphere() {
    let v3 = gen_vec5s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::CheckCollisionBoxSphere(craymath::BoundingBox { min: input[0].into(), max: input[1].into() }, input[2].into(), input[3][0] )};
        let rres = raymath::check_collision_box_sphere(raymath::BoundingBox { min: input[0].into(), max: input[1].into() }, input[2].into(), input[3][0] );
        assert_eq!(cres, rres);
    }
}

#[test]
fn get_ray_collision_sphere() {
    let v3 = gen_vec5s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::GetRayCollisionSphere(craymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3][0] )};
        let rres = raymath::get_ray_collision_sphere(raymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3][0] );
        assert_eq!(cres, rres);
    }
}

#[test]
fn get_ray_collision_box() {
    let v3 = gen_vec5s();
    for input in v3.chunks(4) {
        let cres = unsafe {craymath::GetRayCollisionBox(craymath::Ray { position: input[0].into(), direction: input[1].into() }, craymath::BoundingBox { min: input[2].into(), max: input[3].into() } )};
        let rres = raymath::get_ray_collision_box(raymath::Ray { position: input[0].into(), direction: input[1].into() }, raymath::BoundingBox { min: input[2].into(), max: input[3].into() } );
        assert_eq!(cres, rres);
    }
}

#[test]
fn get_ray_collision_triangle() {
    let v3 = gen_vec5s();
    for input in v3.chunks(10) {
        let cres = unsafe {craymath::GetRayCollisionTriangle(craymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3].into(), input[4].into() )};
        let rres = raymath::get_ray_collision_triangle(raymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3].into(), input[4].into() );
        assert_eq!(cres, rres);
    }
}

#[test]
fn get_ray_collision_quad() {
    let v3 = gen_vec5s();
    for input in v3.chunks(6) {
        let cres = unsafe {craymath::GetRayCollisionQuad(craymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3].into(), input[4].into(), input[5].into() )};
        let rres = raymath::get_ray_collision_quad(raymath::Ray { position: input[0].into(), direction: input[1].into() }, input[2].into(), input[3].into(), input[4].into(), input[5].into() );
        assert_eq!(cres, rres);
    }
}

// #[test]
// fn get_ray_collision_mesh() {
//     let v3 = gen_vec5s();
//     let mats = gen_vec16s();

//     for (i, v) in v3.chunks(2).enumerate() {

//         let mut poly = gen_vec3_vecs();
//         poly.truncate(ITERATIONS / ITER_DIV);
//         let mut cpoly = vec![];
//         for i in &poly {
//             cpoly.push(craymath::Vector3{x: i[0].x, y: i[1].y, z: i[2].z});
//             cpoly.push(craymath::Vector3{x: i[0].x, y: i[1].y, z: i[2].z});
//             cpoly.push(craymath::Vector3{x: i[0].x, y: i[1].y, z: i[2].z});
//         }

//         let cres = unsafe {craymath::GetRayCollisionMesh(craymath::Ray { position: v[0].into(), direction: v[1].into() }, cpoly.as_mut_ptr(), cpoly.len() as u32, mats[i].into() )};
//         let rres = raymath::get_ray_collision_mesh(raymath::Ray { position: v[0].into(), direction: v[1].into() }, poly, mats[i].into() );
//         assert_eq!(cres, rres);
//     }
// }


// RayCollision GetRayCollisionMesh(Ray ray, Vector3 * mesh, unsigned int mesh_len, Matrix transform);                       // Get collision info between ray and mesh




}