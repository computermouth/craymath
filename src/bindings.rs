/* automatically generated by rust-bindgen 0.65.1 */

pub const PI: f64 = 3.141592653589793;
pub const EPSILON: f64 = 0.000001;
pub const DEG2RAD: f64 = 0.017453292519943295;
pub const RAD2DEG: f64 = 57.29577951308232;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl From<[f32; 3]> for Vector2 {
    fn from(f: [f32; 3]) -> Self {
        Vector2{x: f[0], y: f[1]}
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Vector3 {
    fn from(f: [f32; 5]) -> Self {
        Vector3{x: f[0], y: f[1], z: f[2]}
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Vector3 {
    fn from(f: [f32; 16]) -> Self {
        Vector3{x: f[0], y: f[1], z: f[2]}
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Quaternion {
    fn from(f: [f32; 5]) -> Self {
        Quaternion{x: f[0], y: f[1], z: f[2], w: f[3]}
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Quaternion {
    fn from(f: [f32; 16]) -> Self {
        Quaternion{x: f[0], y: f[1], z: f[2], w: f[3]}
    }
}

#[test]
fn bindgen_test_layout_Vector2() {
    const UNINIT: ::std::mem::MaybeUninit<Vector2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector2>(),
        8usize,
        concat!("Size of: ", stringify!(Vector2))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector2>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector2),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl From<[f32; 3]> for Vector3 {
    fn from(f: [f32; 3]) -> Self {
        Vector3{x: f[0], y: f[1], z: f[2]}
    }
}

#[test]
fn bindgen_test_layout_Vector3() {
    const UNINIT: ::std::mem::MaybeUninit<Vector3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector3>(),
        12usize,
        concat!("Size of: ", stringify!(Vector3))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector3>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector3),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_Vector4() {
    const UNINIT: ::std::mem::MaybeUninit<Vector4> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Vector4>(),
        16usize,
        concat!("Size of: ", stringify!(Vector4))
    );
    assert_eq!(
        ::std::mem::align_of::<Vector4>(),
        4usize,
        concat!("Alignment of ", stringify!(Vector4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Vector4),
            "::",
            stringify!(w)
        )
    );
}
pub type Quaternion = Vector4;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}

impl From<[f32; 16]> for Matrix {
    fn from(f: [f32; 16]) -> Self {
        Matrix{
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

#[test]
fn bindgen_test_layout_Matrix() {
    const UNINIT: ::std::mem::MaybeUninit<Matrix> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Matrix>(),
        64usize,
        concat!("Size of: ", stringify!(Matrix))
    );
    assert_eq!(
        ::std::mem::align_of::<Matrix>(),
        4usize,
        concat!("Alignment of ", stringify!(Matrix))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m4) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m8) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m12) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m5) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m9) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m13) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m13)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m2) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m6) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m10) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m14) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m14)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m3) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m7) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m11) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m11)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m15) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(Matrix),
            "::",
            stringify!(m15)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float3 {
    pub v: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_float3() {
    const UNINIT: ::std::mem::MaybeUninit<float3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<float3>(),
        12usize,
        concat!("Size of: ", stringify!(float3))
    );
    assert_eq!(
        ::std::mem::align_of::<float3>(),
        4usize,
        concat!("Alignment of ", stringify!(float3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(float3), "::", stringify!(v))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct float16 {
    pub v: [f32; 16usize],
}
#[test]
fn bindgen_test_layout_float16() {
    const UNINIT: ::std::mem::MaybeUninit<float16> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<float16>(),
        64usize,
        concat!("Size of: ", stringify!(float16))
    );
    assert_eq!(
        ::std::mem::align_of::<float16>(),
        4usize,
        concat!("Alignment of ", stringify!(float16))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(float16),
            "::",
            stringify!(v)
        )
    );
}
extern "C" {
    pub fn Clamp(value: f32, min: f32, max: f32) -> f32;
}
extern "C" {
    pub fn Lerp(start: f32, end: f32, amount: f32) -> f32;
}
extern "C" {
    pub fn Normalize(value: f32, start: f32, end: f32) -> f32;
}
extern "C" {
    pub fn Remap(
        value: f32,
        inputStart: f32,
        inputEnd: f32,
        outputStart: f32,
        outputEnd: f32,
    ) -> f32;
}
extern "C" {
    pub fn Wrap(value: f32, min: f32, max: f32) -> f32;
}
extern "C" {
    pub fn FloatEquals(x: f32, y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Vector2Zero() -> Vector2;
}
extern "C" {
    pub fn Vector2One() -> Vector2;
}
extern "C" {
    pub fn Vector2Add(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2AddValue(v: Vector2, add: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Subtract(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2SubtractValue(v: Vector2, sub: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Length(v: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2LengthSqr(v: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2DotProduct(v1: Vector2, v2: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2Distance(v1: Vector2, v2: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2DistanceSqr(v1: Vector2, v2: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2Angle(v1: Vector2, v2: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2LineAngle(start: Vector2, end: Vector2) -> f32;
}
extern "C" {
    pub fn Vector2Scale(v: Vector2, scale: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Multiply(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Negate(v: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Divide(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Normalize(v: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Transform(v: Vector2, mat: Matrix) -> Vector2;
}
extern "C" {
    pub fn Vector2Lerp(v1: Vector2, v2: Vector2, amount: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Reflect(v: Vector2, normal: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Min(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Max(v1: Vector2, v2: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Rotate(v: Vector2, angle: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2MoveTowards(v: Vector2, target: Vector2, maxDistance: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Invert(v: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2Clamp(v: Vector2, min: Vector2, max: Vector2) -> Vector2;
}
extern "C" {
    pub fn Vector2ClampValue(v: Vector2, min: f32, max: f32) -> Vector2;
}
extern "C" {
    pub fn Vector2Equals(p: Vector2, q: Vector2) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Vector2Refract(v: Vector2, n: Vector2, r: f32) -> Vector2;
}
extern "C" {
    pub fn Vector3Zero() -> Vector3;
}
extern "C" {
    pub fn Vector3One() -> Vector3;
}
extern "C" {
    pub fn Vector3Add(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3AddValue(v: Vector3, add: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3Subtract(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3SubtractValue(v: Vector3, sub: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3Scale(v: Vector3, scalar: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3Multiply(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3CrossProduct(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Perpendicular(v: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Length(v: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3LengthSqr(v: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3DotProduct(v1: Vector3, v2: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3Distance(v1: Vector3, v2: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3DistanceSqr(v1: Vector3, v2: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3Angle(v1: Vector3, v2: Vector3) -> f32;
}
extern "C" {
    pub fn Vector3Negate(v: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Divide(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Normalize(v: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Project(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Reject(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3OrthoNormalize(v1: *mut Vector3, v2: *mut Vector3);
}
extern "C" {
    pub fn Vector3Transform(v: Vector3, mat: Matrix) -> Vector3;
}
extern "C" {
    pub fn Vector3RotateByQuaternion(v: Vector3, q: Quaternion) -> Vector3;
}
extern "C" {
    pub fn Vector3RotateByAxisAngle(v: Vector3, axis: Vector3, angle: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3MoveTowards(v: Vector3, target: Vector3, maxDistance: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3Lerp(v1: Vector3, v2: Vector3, amount: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3CubicHermite(
        v1: Vector3,
        tangent1: Vector3,
        v2: Vector3,
        tangent2: Vector3,
        amount: f32,
    ) -> Vector3;
}
extern "C" {
    pub fn Vector3Reflect(v: Vector3, normal: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Min(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Max(v1: Vector3, v2: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Barycenter(p: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3;
}
extern "C" {
    pub fn Vector3ToFloatV(v: Vector3) -> float3;
}
extern "C" {
    pub fn Vector3Invert(v: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3Clamp(v: Vector3, min: Vector3, max: Vector3) -> Vector3;
}
extern "C" {
    pub fn Vector3ClampValue(v: Vector3, min: f32, max: f32) -> Vector3;
}
extern "C" {
    pub fn Vector3Equals(p: Vector3, q: Vector3) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Vector3Refract(v: Vector3, n: Vector3, r: f32) -> Vector3;
}
extern "C" {
    pub fn Vector4Zero() -> Vector4;
}
extern "C" {
    pub fn Vector4One() -> Vector4;
}
extern "C" {
    pub fn Vector4Add(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4AddValue(v: Vector4, add: f32) -> Vector4;
}
extern "C" {
    pub fn Vector4Subtract(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4SubtractValue(v: Vector4, add: f32) -> Vector4;
}
extern "C" {
    pub fn Vector4Length(v: Vector4) -> f32;
}
extern "C" {
    pub fn Vector4LengthSqr(v: Vector4) -> f32;
}
extern "C" {
    pub fn Vector4DotProduct(v1: Vector4, v2: Vector4) -> f32;
}
extern "C" {
    pub fn Vector4Distance(v1: Vector4, v2: Vector4) -> f32;
}
extern "C" {
    pub fn Vector4DistanceSqr(v1: Vector4, v2: Vector4) -> f32;
}
extern "C" {
    pub fn Vector4Scale(v: Vector4, scale: f32) -> Vector4;
}
extern "C" {
    pub fn Vector4Multiply(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Negate(v: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Divide(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Normalize(v: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Min(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Max(v1: Vector4, v2: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Lerp(v1: Vector4, v2: Vector4, amount: f32) -> Vector4;
}
extern "C" {
    pub fn Vector4MoveTowards(v: Vector4, target: Vector4, maxDistance: f32) -> Vector4;
}
extern "C" {
    pub fn Vector4Invert(v: Vector4) -> Vector4;
}
extern "C" {
    pub fn Vector4Equals(p: Vector4, q: Vector4) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MatrixDeterminant(mat: Matrix) -> f32;
}
extern "C" {
    pub fn MatrixTrace(mat: Matrix) -> f32;
}
extern "C" {
    pub fn MatrixTranspose(mat: Matrix) -> Matrix;
}
extern "C" {
    pub fn MatrixInvert(mat: Matrix) -> Matrix;
}
extern "C" {
    pub fn MatrixIdentity() -> Matrix;
}
extern "C" {
    pub fn MatrixAdd(left: Matrix, right: Matrix) -> Matrix;
}
extern "C" {
    pub fn MatrixSubtract(left: Matrix, right: Matrix) -> Matrix;
}
extern "C" {
    pub fn MatrixMultiply(left: Matrix, right: Matrix) -> Matrix;
}
extern "C" {
    pub fn MatrixTranslate(x: f32, y: f32, z: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixRotate(axis: Vector3, angle: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixRotateX(angle: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixRotateY(angle: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixRotateZ(angle: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixRotateXYZ(angle: Vector3) -> Matrix;
}
extern "C" {
    pub fn MatrixRotateZYX(angle: Vector3) -> Matrix;
}
extern "C" {
    pub fn MatrixScale(x: f32, y: f32, z: f32) -> Matrix;
}
extern "C" {
    pub fn MatrixFrustum(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        nearPlane: f64,
        farPlane: f64,
    ) -> Matrix;
}
extern "C" {
    pub fn MatrixPerspective(fovY: f64, aspect: f64, nearPlane: f64, farPlane: f64) -> Matrix;
}
extern "C" {
    pub fn MatrixOrtho(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        nearPlane: f64,
        farPlane: f64,
    ) -> Matrix;
}
extern "C" {
    pub fn MatrixLookAt(eye: Vector3, target: Vector3, up: Vector3) -> Matrix;
}
extern "C" {
    pub fn MatrixToFloatV(mat: Matrix) -> float16;
}
extern "C" {
    pub fn QuaternionAdd(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionAddValue(q: Quaternion, add: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionSubtract(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionSubtractValue(q: Quaternion, sub: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionIdentity() -> Quaternion;
}
extern "C" {
    pub fn QuaternionLength(q: Quaternion) -> f32;
}
extern "C" {
    pub fn QuaternionNormalize(q: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionInvert(q: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionMultiply(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionScale(q: Quaternion, mul: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionDivide(q1: Quaternion, q2: Quaternion) -> Quaternion;
}
extern "C" {
    pub fn QuaternionLerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionNlerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionSlerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionCubicHermiteSpline(
        q1: Quaternion,
        outTangent1: Quaternion,
        q2: Quaternion,
        inTangent2: Quaternion,
        t: f32,
    ) -> Quaternion;
}
extern "C" {
    pub fn QuaternionFromVector3ToVector3(from: Vector3, to: Vector3) -> Quaternion;
}
extern "C" {
    pub fn QuaternionFromMatrix(mat: Matrix) -> Quaternion;
}
extern "C" {
    pub fn QuaternionToMatrix(q: Quaternion) -> Matrix;
}
extern "C" {
    pub fn QuaternionFromAxisAngle(axis: Vector3, angle: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionToAxisAngle(q: Quaternion, outAxis: *mut Vector3, outAngle: *mut f32);
}
extern "C" {
    pub fn QuaternionFromEuler(pitch: f32, yaw: f32, roll: f32) -> Quaternion;
}
extern "C" {
    pub fn QuaternionToEuler(q: Quaternion) -> Vector3;
}
extern "C" {
    pub fn QuaternionTransform(q: Quaternion, mat: Matrix) -> Quaternion;
}
extern "C" {
    pub fn QuaternionEquals(p: Quaternion, q: Quaternion) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MatrixDecompose(
        mat: Matrix,
        translation: *mut Vector3,
        rotation: *mut Quaternion,
        scale: *mut Vector3,
    );
}
