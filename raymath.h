
#ifndef RAYMATH_H
#define RAYMATH_H

//----------------------------------------------------------------------------------
// Defines and Macros
//----------------------------------------------------------------------------------
#define PI 3.14159265358979323846264338327950288

#define EPSILON 1.1920929E-7f

#ifndef DEG2RAD
    #define DEG2RAD (PI/180.0f)
#endif

#ifndef RAD2DEG
    #define RAD2DEG (180.0f/PI)
#endif

// Get float vector for Matrix
#ifndef MatrixToFloat
    #define MatrixToFloat(mat) (MatrixToFloatV(mat).v)
#endif

// Get float vector for Vector3
#ifndef Vector3ToFloat
    #define Vector3ToFloat(vec) (Vector3ToFloatV(vec).v)
#endif

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------
#if !defined(RL_VECTOR2_TYPE)
// Vector2 type
typedef struct Vector2 {
    float x;
    float y;
} Vector2;
#define RL_VECTOR2_TYPE
#endif

#if !defined(RL_VECTOR3_TYPE)
// Vector3 type
typedef struct Vector3 {
    float x;
    float y;
    float z;
} Vector3;
#define RL_VECTOR3_TYPE
#endif

#if !defined(RL_VECTOR4_TYPE)
// Vector4 type
typedef struct Vector4 {
    float x;
    float y;
    float z;
    float w;
} Vector4;
#define RL_VECTOR4_TYPE
#endif

#if !defined(RL_QUATERNION_TYPE)
// Quaternion type
typedef Vector4 Quaternion;
#define RL_QUATERNION_TYPE
#endif

#if !defined(RL_MATRIX_TYPE)
// Matrix type (OpenGL style 4x4 - right handed, column major)
typedef struct Matrix {
    float m0, m4, m8, m12;      // Matrix first row (4 components)
    float m1, m5, m9, m13;      // Matrix second row (4 components)
    float m2, m6, m10, m14;     // Matrix third row (4 components)
    float m3, m7, m11, m15;     // Matrix fourth row (4 components)
} Matrix;
#define RL_MATRIX_TYPE
#endif

// NOTE: Helper types to be used instead of array return types for *ToFloat functions
typedef struct float3 {
    float v[3];
} float3;

typedef struct float16 {
    float v[16];
} float16;
																																			 
float Clamp(float value, float min, float max)                                                                                         ;
float Lerp(float start, float end, float amount)                                                                                       ;
float Normalize(float value, float start, float end)                                                                                   ;
float Remap(float value, float inputStart, float inputEnd, float outputStart, float outputEnd)                                         ;
float Wrap(float value, float min, float max)                                                                                          ;
int FloatEquals(float x, float y)                                                                                                      ;
Vector2 Vector2Zero(void)                                                                                                              ;
Vector2 Vector2One(void)                                                                                                               ;
Vector2 Vector2Add(Vector2 v1, Vector2 v2)                                                                                             ;
Vector2 Vector2AddValue(Vector2 v, float add)                                                                                          ;
Vector2 Vector2Subtract(Vector2 v1, Vector2 v2)                                                                                        ;
Vector2 Vector2SubtractValue(Vector2 v, float sub)                                                                                     ;
float Vector2Length(Vector2 v)                                                                                                         ;
float Vector2LengthSqr(Vector2 v)                                                                                                      ;
float Vector2DotProduct(Vector2 v1, Vector2 v2)                                                                                        ;
float Vector2Distance(Vector2 v1, Vector2 v2)                                                                                          ;
float Vector2DistanceSqr(Vector2 v1, Vector2 v2)                                                                                       ;
float Vector2Angle(Vector2 v1, Vector2 v2)                                                                                             ;
float Vector2LineAngle(Vector2 start, Vector2 end)                                                                                     ;
Vector2 Vector2Scale(Vector2 v, float scale)                                                                                           ;
Vector2 Vector2Multiply(Vector2 v1, Vector2 v2)                                                                                        ;
Vector2 Vector2Negate(Vector2 v)                                                                                                       ;
Vector2 Vector2Divide(Vector2 v1, Vector2 v2)                                                                                          ;
Vector2 Vector2Normalize(Vector2 v)                                                                                                    ;
Vector2 Vector2Transform(Vector2 v, Matrix mat)                                                                                        ;
Vector2 Vector2Lerp(Vector2 v1, Vector2 v2, float amount)                                                                              ;
Vector2 Vector2Reflect(Vector2 v, Vector2 normal)                                                                                      ;
Vector2 Vector2Min(Vector2 v1, Vector2 v2)                                                                                             ;
Vector2 Vector2Max(Vector2 v1, Vector2 v2)                                                                                             ;
Vector2 Vector2Rotate(Vector2 v, float angle)                                                                                          ;
Vector2 Vector2MoveTowards(Vector2 v, Vector2 target, float maxDistance)                                                               ;
Vector2 Vector2Invert(Vector2 v)                                                                                                       ;
Vector2 Vector2Clamp(Vector2 v, Vector2 min, Vector2 max)                                                                              ;
Vector2 Vector2ClampValue(Vector2 v, float min, float max)                                                                             ;
int Vector2Equals(Vector2 p, Vector2 q)                                                                                                ;
Vector2 Vector2Refract(Vector2 v, Vector2 n, float r)                                                                                  ;
Vector3 Vector3Zero(void)                                                                                                              ;
Vector3 Vector3One(void)                                                                                                               ;
Vector3 Vector3Add(Vector3 v1, Vector3 v2)                                                                                             ;
Vector3 Vector3AddValue(Vector3 v, float add)                                                                                          ;
Vector3 Vector3Subtract(Vector3 v1, Vector3 v2)                                                                                        ;
Vector3 Vector3SubtractValue(Vector3 v, float sub)                                                                                     ;
Vector3 Vector3Scale(Vector3 v, float scalar)                                                                                          ;
Vector3 Vector3Multiply(Vector3 v1, Vector3 v2)                                                                                        ;
Vector3 Vector3CrossProduct(Vector3 v1, Vector3 v2)                                                                                    ;
Vector3 Vector3Perpendicular(Vector3 v)                                                                                                ;
float Vector3Length(const Vector3 v)                                                                                                   ;
float Vector3LengthSqr(const Vector3 v)                                                                                                ;
float Vector3DotProduct(Vector3 v1, Vector3 v2)                                                                                        ;
float Vector3Distance(Vector3 v1, Vector3 v2)                                                                                          ;
float Vector3DistanceSqr(Vector3 v1, Vector3 v2)                                                                                       ;
float Vector3Angle(Vector3 v1, Vector3 v2)                                                                                             ;
Vector3 Vector3Negate(Vector3 v)                                                                                                       ;
Vector3 Vector3Divide(Vector3 v1, Vector3 v2)                                                                                          ;
Vector3 Vector3Normalize(Vector3 v)                                                                                                    ;
Vector3 Vector3Project(Vector3 v1, Vector3 v2)                                                                                         ;
Vector3 Vector3Reject(Vector3 v1, Vector3 v2)                                                                                          ;
void Vector3OrthoNormalize(Vector3 *v1, Vector3 *v2)                                                                                   ;
Vector3 Vector3Transform(Vector3 v, Matrix mat)                                                                                        ;
Vector3 Vector3RotateByQuaternion(Vector3 v, Quaternion q)                                                                             ;
Vector3 Vector3RotateByAxisAngle(Vector3 v, Vector3 axis, float angle)                                                                 ;
Vector3 Vector3MoveTowards(Vector3 v, Vector3 target, float maxDistance)                                                               ;
Vector3 Vector3Lerp(Vector3 v1, Vector3 v2, float amount)                                                                              ;
Vector3 Vector3CubicHermite(Vector3 v1, Vector3 tangent1, Vector3 v2, Vector3 tangent2, float amount)                                  ;
Vector3 Vector3Reflect(Vector3 v, Vector3 normal)                                                                                      ;
Vector3 Vector3Min(Vector3 v1, Vector3 v2)                                                                                             ;
Vector3 Vector3Max(Vector3 v1, Vector3 v2)                                                                                             ;
Vector3 Vector3Barycenter(Vector3 p, Vector3 a, Vector3 b, Vector3 c)                                                                  ;
Vector3 Vector3Unproject(Vector3 source, Matrix projection, Matrix view)                                                               ;
float3 Vector3ToFloatV(Vector3 v)                                                                                                      ;
Vector3 Vector3Invert(Vector3 v)                                                                                                       ;
Vector3 Vector3Clamp(Vector3 v, Vector3 min, Vector3 max)                                                                              ;
Vector3 Vector3ClampValue(Vector3 v, float min, float max)                                                                             ;
int Vector3Equals(Vector3 p, Vector3 q)                                                                                                ;
Vector3 Vector3Refract(Vector3 v, Vector3 n, float r)                                                                                  ;
Vector4 Vector4Zero(void)                                                                                                              ;
Vector4 Vector4One(void)                                                                                                               ;
Vector4 Vector4Add(Vector4 v1, Vector4 v2)                                                                                             ;
Vector4 Vector4AddValue(Vector4 v, float add)                                                                                          ;
Vector4 Vector4Subtract(Vector4 v1, Vector4 v2)                                                                                        ;
Vector4 Vector4SubtractValue(Vector4 v, float add)                                                                                     ;
float Vector4Length(Vector4 v)                                                                                                         ;
float Vector4LengthSqr(Vector4 v)                                                                                                      ;
float Vector4DotProduct(Vector4 v1, Vector4 v2)                                                                                        ;
float Vector4Distance(Vector4 v1, Vector4 v2)                                                                                          ;
float Vector4DistanceSqr(Vector4 v1, Vector4 v2)                                                                                       ;
Vector4 Vector4Scale(Vector4 v, float scale)                                                                                           ;
Vector4 Vector4Multiply(Vector4 v1, Vector4 v2)                                                                                        ;
Vector4 Vector4Negate(Vector4 v)                                                                                                       ;
Vector4 Vector4Divide(Vector4 v1, Vector4 v2)                                                                                          ;
Vector4 Vector4Normalize(Vector4 v)                                                                                                    ;
Vector4 Vector4Min(Vector4 v1, Vector4 v2)                                                                                             ;
Vector4 Vector4Max(Vector4 v1, Vector4 v2)                                                                                             ;
Vector4 Vector4Lerp(Vector4 v1, Vector4 v2, float amount)                                                                              ;
Vector4 Vector4MoveTowards(Vector4 v, Vector4 target, float maxDistance)                                                               ;
Vector4 Vector4Invert(Vector4 v)                                                                                                       ;
int Vector4Equals(Vector4 p, Vector4 q)                                                                                                ;
float MatrixDeterminant(Matrix mat)                                                                                                    ;
float MatrixTrace(Matrix mat)                                                                                                          ;
Matrix MatrixTranspose(Matrix mat)                                                                                                     ;
Matrix MatrixInvert(Matrix mat)                                                                                                        ;
Matrix MatrixIdentity(void)                                                                                                            ;
Matrix MatrixAdd(Matrix left, Matrix right)                                                                                            ;
Matrix MatrixSubtract(Matrix left, Matrix right)                                                                                       ;
Matrix MatrixMultiply(Matrix left, Matrix right)                                                                                       ;
Matrix MatrixTranslate(float x, float y, float z)                                                                                      ;
Matrix MatrixRotate(Vector3 axis, float angle)                                                                                         ;
Matrix MatrixRotateX(float angle)                                                                                                      ;
Matrix MatrixRotateY(float angle)                                                                                                      ;
Matrix MatrixRotateZ(float angle)                                                                                                      ;
Matrix MatrixRotateXYZ(Vector3 angle)                                                                                                  ;
Matrix MatrixRotateZYX(Vector3 angle)                                                                                                  ;
Matrix MatrixScale(float x, float y, float z)                                                                                          ;
Matrix MatrixFrustum(double left, double right, double bottom, double top, double nearPlane, double farPlane)                          ;
Matrix MatrixPerspective(double fovY, double aspect, double nearPlane, double farPlane)                                                ;
Matrix MatrixOrtho(double left, double right, double bottom, double top, double nearPlane, double farPlane)                            ;
Matrix MatrixLookAt(Vector3 eye, Vector3 target, Vector3 up)                                                                           ;
float16 MatrixToFloatV(Matrix mat)                                                                                                     ;
Quaternion QuaternionAdd(Quaternion q1, Quaternion q2)                                                                                 ;
Quaternion QuaternionAddValue(Quaternion q, float add)                                                                                 ;
Quaternion QuaternionSubtract(Quaternion q1, Quaternion q2)                                                                            ;
Quaternion QuaternionSubtractValue(Quaternion q, float sub)                                                                            ;
Quaternion QuaternionIdentity(void)                                                                                                    ;
float QuaternionLength(Quaternion q)                                                                                                   ;
Quaternion QuaternionNormalize(Quaternion q)                                                                                           ;
Quaternion QuaternionInvert(Quaternion q)                                                                                              ;
Quaternion QuaternionMultiply(Quaternion q1, Quaternion q2)                                                                            ;
Quaternion QuaternionScale(Quaternion q, float mul)                                                                                    ;
Quaternion QuaternionDivide(Quaternion q1, Quaternion q2)                                                                              ;
Quaternion QuaternionLerp(Quaternion q1, Quaternion q2, float amount)                                                                  ;
Quaternion QuaternionNlerp(Quaternion q1, Quaternion q2, float amount)                                                                 ;
Quaternion QuaternionSlerp(Quaternion q1, Quaternion q2, float amount)                                                                 ;
Quaternion QuaternionCubicHermiteSpline(Quaternion q1, Quaternion outTangent1, Quaternion q2, Quaternion inTangent2, float t)          ;
Quaternion QuaternionFromVector3ToVector3(Vector3 from, Vector3 to)                                                                    ;
Quaternion QuaternionFromMatrix(Matrix mat)                                                                                            ;
Matrix QuaternionToMatrix(Quaternion q)                                                                                                ;
Quaternion QuaternionFromAxisAngle(Vector3 axis, float angle)                                                                          ;
void QuaternionToAxisAngle(Quaternion q, Vector3 *outAxis, float *outAngle)                                                            ;
Quaternion QuaternionFromEuler(float pitch, float yaw, float roll)                                                                     ;
Vector3 QuaternionToEuler(Quaternion q)                                                                                                ;
Quaternion QuaternionTransform(Quaternion q, Matrix mat)                                                                               ;
int QuaternionEquals(Quaternion p, Quaternion q)                                                                                       ;
void MatrixDecompose(Matrix mat, Vector3 *translation, Quaternion *rotation, Vector3 *scale)                                           ;


// Ray, ray for raycasting
typedef struct Ray {
    Vector3 position;       // Ray position (origin)
    Vector3 direction;      // Ray direction (normalized)
} Ray;

// RayCollision, ray hit information
typedef struct RayCollision {
    bool hit;               // Did the ray hit something?
    float distance;         // Distance to the nearest hit
    Vector3 point;          // Point of the nearest hit
    Vector3 normal;         // Surface normal of hit
} RayCollision;

// BoundingBox
typedef struct BoundingBox {
    Vector3 min;            // Minimum vertex box-corner
    Vector3 max;            // Maximum vertex box-corner
} BoundingBox;

// Rectangle, 4 components
typedef struct Rectangle {
    float x;                // Rectangle top-left corner position x
    float y;                // Rectangle top-left corner position y
    float width;            // Rectangle width
    float height;           // Rectangle height
} Rectangle;

bool CheckCollisionRecs(Rectangle rec1, Rectangle rec2);                                           // Check collision between two rectangles
bool CheckCollisionCircles(Vector2 center1, float radius1, Vector2 center2, float radius2);        // Check collision between two circles
bool CheckCollisionCircleRec(Vector2 center, float radius, Rectangle rec);                         // Check collision between circle and rectangle
bool CheckCollisionPointRec(Vector2 point, Rectangle rec);                                         // Check if point is inside rectangle
bool CheckCollisionPointCircle(Vector2 point, Vector2 center, float radius);                       // Check if point is inside circle
bool CheckCollisionPointTriangle(Vector2 point, Vector2 p1, Vector2 p2, Vector2 p3);               // Check if point is inside a triangle
bool CheckCollisionPointPoly(Vector2 point, const Vector2 *points, int pointCount);                // Check if point is within a polygon described by array of vertices
bool CheckCollisionLines(Vector2 startPos1, Vector2 endPos1, Vector2 startPos2, Vector2 endPos2, Vector2 *collisionPoint); // Check the collision between two lines defined by two points each, returns collision point by reference
bool CheckCollisionPointLine(Vector2 point, Vector2 p1, Vector2 p2, int threshold);                // Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
bool CheckCollisionCircleLine(Vector2 center, float radius, Vector2 p1, Vector2 p2);               // Check if circle collides with a line created betweeen two points [p1] and [p2]
Rectangle GetCollisionRec(Rectangle rec1, Rectangle rec2);  

bool CheckCollisionSpheres(Vector3 center1, float radius1, Vector3 center2, float radius2);   // Check collision between two spheres
bool CheckCollisionBoxes(BoundingBox box1, BoundingBox box2);                                 // Check collision between two bounding boxes
bool CheckCollisionBoxSphere(BoundingBox box, Vector3 center, float radius);                  // Check collision between box and sphere
RayCollision GetRayCollisionSphere(Ray ray, Vector3 center, float radius);                    // Get collision info between ray and sphere
RayCollision GetRayCollisionBox(Ray ray, BoundingBox box);                                    // Get collision info between ray and box
RayCollision GetRayCollisionMesh(Ray ray, Vector3 * mesh, unsigned int mesh_len, Matrix transform);                       // Get collision info between ray and mesh
RayCollision GetRayCollisionTriangle(Ray ray, Vector3 p1, Vector3 p2, Vector3 p3);            // Get collision info between ray and triangle
RayCollision GetRayCollisionQuad(Ray ray, Vector3 p1, Vector3 p2, Vector3 p3, Vector3 p4);    // Get collision info between ray and quad


#endif
