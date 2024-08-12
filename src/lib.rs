#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
#[allow(unused_imports)]
use bindings as craymath;
#[allow(unused_imports)]
use raymath;

const ITERATIONS: usize = 100000000;

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}

// VECTOR 3 SHARED =======================================================================

#[test]
fn vector3_zero() {
    let cres = unsafe {craymath::Vector3Zero()};
    let rres = raymath::vector3_zero();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
}

#[test]
fn vector3_one() {
    let cres = unsafe {craymath::Vector3One()};
    let rres = raymath::vector3_one();
    assert_eq!(cres.x, rres.x);
    assert_eq!(cres.y, rres.y);
}

#[test]
fn vector3_add() {
    let v3 = gen_vec3s();
    for v in v3.chunks(2) {
        let cres = unsafe {craymath::Vector3Add(v[0].into(), v[1].into())};
        let rres = raymath::vector3_add(v[0].into(), v[1].into());
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
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
        assert_eq!(cres.x, rres.x);
        assert_eq!(cres.y, rres.y);
    }
}


}
