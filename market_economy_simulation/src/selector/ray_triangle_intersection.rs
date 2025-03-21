//! Calculates an intersection of a line with a triangle using the **Möller-Trumbore algorithm**  
//!
//! https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html
//!
//! ## Möller-Trumbore algorithm
//!
//! The Möller-Trumbore algorithm is a fast ray-triangle intersection algorithm
//! that was introduced in 1997 by Tomas Möller and Ben Trumbore in a paper titled
//! "Fast, Minimum Storage Ray/Triangle Intersection". It is still considered today
//! a fast algorithm that is often used in benchmarks to compare the performances of other methods.
//!
//! ```math
//!                    C
//!                    x
//!   O  D            / \
//! --x-->-----------/-x-\--------------
//!                 /  P  \
//!                x-------x
//!                A       B
//! ```
//!
//! ### Calculation of the intersection point in barycentric coordinates
//! ```math
//! (1)   P = (1 - u - v)A + uB + vC
//! ```
//!
//! P: Intersection point  
//! A: First point of the triangle   
//! B: Second point of the triangle   
//! C: Third point of the triangle   
//!
//! ### Ray's parametric equation:   
//! ```math
//! (2)  P = O + tD
//! ```
//!
//! P: Intersection point
//! O: Origin of the ray
//! D: Direction of the tay  
//! t: Distance from the ray's origin to the intersection P  
//!
//! ### Place (2) in (1):
//!
//! ```math
//!                               |t|
//! (3)  [ -D  (B - A)  (C - A) ] |u| = O - A
//!                               |v|
//! ```
//!
//! ### Cramer's Rule
//!
//! ```math
//! |t|      1     | det(M_t) |
//! |u| = -------- | det(M_u) |
//! |v|    det(M)  | det(M_v) |
//!
//! M =   [   -D     (B - A)  (C - A) ]  
//! M_t = [ (O - A)  (B - A)  (C - A) ]  
//! M_u = [   -D     (O - A)  (C - A) ]  
//! M_v = [   -D     (B - A)  (O - A) ]   
//! ```
//!
//! ### Optimize
//!
//! ```math
//! |t|      1      | |  T E1 E2 | |
//! |u| = --------  | | -D T  E2 | |
//! |v|  |-D E1 E2| | | -D E1 T  | |
//!
//! T =  O - A
//! E1 = B - A
//! E2 = C - A
//!
//! |t|         1         | (T x E1) * E2 |
//! |u| = --------------- | (D x E2) * T  |
//! |v|    (D x E2) * E1  | (T x E1) * D  |
//!
//! |t|      1     | Q * E2 |
//! |u| = ---------| P * T  |
//! |v|    P * E1  | Q * D  |
//!
//! P = (D x E2)
//! Q = (T x E1)
//! ```
//!
//!
//!

use cgmath::InnerSpace;

type Vec3 = cgmath::Vector3<f32>;

/// Calculates an intersection of a line with a triangle using the **Möller-Trumbore algorithm**
///
/// ### Input
/// orig: Origin of the ray  
/// dir: Direction of the ray  
/// v0, v1, v2: Three vertices forming a triangle  
///
/// ### Result
/// t: Distance from the origin of the ray to the intersection P  
/// u: Barycentric coordinate 0 to the intersection P  
/// v: Barycentric coordinate 1 to the intersection P
///
pub fn ray_triangle_intersect(
    orig: &Vec3,
    dir: &Vec3,
    v0: &Vec3,
    v1: &Vec3,
    v2: &Vec3,
) -> Option<Vec3> {
    let a = v0;
    let b = v1;
    let c = v2;
    let o = orig;
    let d = dir;

    let t_ = o - a;
    let e1 = b - a;
    let e2 = c - a;

    let p = d.cross(e2);
    let q = t_.cross(e1);

    let det_m = p.dot(e1);

    // check for backface culling
    if det_m <= 0.0 {
        return None;
    }

    let det_t = q.dot(e2);
    let det_u = p.dot(t_);
    let det_v = q.dot(*d);

    let res = 1.0 / (det_m) * Vec3::new(det_t, det_u, det_v);
    {
        // check result
        let t = res[0];
        let u = res[1];
        let v = res[2];

        if t < 0.0 {
            return None;
        }

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        if !(0.0..=1.0).contains(&v) {
            return None;
        }
    }

    Some(res)
}

/// Calculates an intersection coordinate P from the ray
///
/// ### Input
/// orig: Origin of the ray  
/// dir: Direction of the ray  
/// t: Distance from the origin of the ray to the intersection P  
///
/// ### Result
/// Intersection P  
///
pub fn get_intersection_from_ray(orig: &Vec3, dir: &Vec3, t: f32) -> Vec3 {
    orig + t * dir
}

/// Calculates an intersection coordinate P from the triangle
///
/// ### Input
/// v0, v1, v2: Three vertices forming a triangle  
/// u: Barycentric coordinate 0 to the intersection P  
/// v: Barycentric coordinate 1 to the intersection P
///
/// ### Result
/// Intersection P  
///
#[allow(dead_code)]
pub fn get_intersection_from_triangle(v0: &Vec3, v1: &Vec3, v2: &Vec3, u: f32, v: f32) -> Vec3 {
    ((1.0 - u - v) * v0) + (u * v1) + (v * v2)
}

// ############## Testing ##################

#[allow(dead_code)]
fn is_less(v0: Vec3, v1: &Vec3) -> bool {
    v0.x < v1.x && v0.y < v1.y && v0.z < v1.z
}

#[allow(dead_code)]
fn is_greater(v0: Vec3, v1: &Vec3) -> bool {
    v0.x > v1.x && v0.y > v1.y && v0.z > v1.z
}

#[allow(dead_code)]
fn is_equal(v0: &Vec3, v1: &Vec3) -> bool {
    let eps = 0.001;
    let eps = Vec3::new(eps, eps, eps);

    let res0 = is_less(v0 - eps, v1);
    let res1 = is_greater(v0 + eps, v1);

    res0 && res1
}

/// Tests a simple (none edge) case
#[test]
fn test_simple() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let dir = Vec3::new(1.0, 0.0, 0.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_some());
    let res = res.unwrap();

    let t = res[0];
    let u = res[1];
    let v = res[2];
    println!("t: {}, u: {}, v: {}", t, u, v);

    let p0 = get_intersection_from_ray(&orig, &dir, t);
    let p1 = get_intersection_from_triangle(&v0, &v1, &v2, u, v);

    println!("p0: {:?}", p0);
    println!("p1: {:?}", p1);

    assert!(is_equal(&p0, &p1));
}

/// Intersection with exactly the first point
#[test]
fn test_first_edge() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);
    let dir = v0 - orig;

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_some());
    let res = res.unwrap();

    let t = res[0];
    let u = res[1];
    let v = res[2];
    println!("t: {}, u: {}, v: {}", t, u, v);

    let p0 = get_intersection_from_ray(&orig, &dir, t);
    let p1 = get_intersection_from_triangle(&v0, &v1, &v2, u, v);

    println!("p0: {:?}", p0);
    println!("p1: {:?}", p1);

    assert!(is_equal(&p0, &p1));
}

/// Intersection with exactly the second point
#[test]
fn test_second_edge() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);
    let dir = v1 - orig;

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_some());
    let res = res.unwrap();

    let t = res[0];
    let u = res[1];
    let v = res[2];
    println!("t: {}, u: {}, v: {}", t, u, v);

    let p0 = get_intersection_from_ray(&orig, &dir, t);
    let p1 = get_intersection_from_triangle(&v0, &v1, &v2, u, v);

    println!("p0: {:?}", p0);
    println!("p1: {:?}", p1);

    assert!(is_equal(&p0, &p1));
}

/// Intersection with exactly the third point
#[test]
fn test_third_edge() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);
    let dir = v2 - orig;

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_some());
    let res = res.unwrap();

    let t = res[0];
    let u = res[1];
    let v = res[2];
    println!("t: {}, u: {}, v: {}", t, u, v);

    let p0 = get_intersection_from_ray(&orig, &dir, t);
    let p1 = get_intersection_from_triangle(&v0, &v1, &v2, u, v);

    println!("p0: {:?}", p0);
    println!("p1: {:?}", p1);

    assert!(is_equal(&p0, &p1));
}

#[test]
fn test_back_face() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let dir = Vec3::new(1.0, 0.0, 0.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v2 = Vec3::new(3.0, 0.0, 3.0);
    let v1 = Vec3::new(3.0, 1.0, 0.0);

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_none());
}

#[test]
fn test_back_face_2() {
    let orig = Vec3::new(5.0, 0.0, 1.0);
    let dir = Vec3::new(-1.0, 0.0, 0.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_none());
}

#[test]
fn test_outside() {
    let orig = Vec3::new(-1.0, 0.0, 1.0);
    let dir = Vec3::new(1.0, 1.0, 0.0);
    let v0 = Vec3::new(3.0, -1.0, 0.0);
    let v1 = Vec3::new(3.0, 0.0, 3.0);
    let v2 = Vec3::new(3.0, 1.0, 0.0);

    let res = ray_triangle_intersect(&orig, &dir, &v0, &v1, &v2);
    assert!(res.is_none());
}
