#[allow(unused)]

mod class;
use class::*;
use std::time::Instant;

fn print_coordinate<T>(p: &T, name: &str) where T: Coordinate {
    println!("The coordinate of {} is {:?};", name, p.co());
}


// Main.
fn main() {
    let time_start = Instant::now();
    let _pi = std::f64::consts::PI;

    // Define points.
    let mut p_a = Point3d::new(2.0, 3.0, -2.0);
    let mut p_b = Point3d::new(-1.0, 0.0, 4.0);

    // Define vectors.
    let mut v_a = Vector3d::from_point(&p_a);
    let mut v_b = Vector3d::from_point(&p_b);

    // Define lines.
    let mut line_1 = Line::new(p_a, p_b);

    // Tests.
    print_coordinate(&p_a, "p_a");
    print_coordinate(&p_b, "p_b");
    println!("Length of v_a is {};", v_a.len());
    println!("Length of line is {};", line_1.len());
    println!("Middle point of line is {:?};", line_1.point_at(0.5).co());
    println!("Addition of v_a and v_b is {:?};", add(&v_a, &v_b).co());
    println!("Dot product of v_a and v_b is {};", v_a.dot(&v_b));
    println!("Cross product of v_a and v_b is {:?};", v_a.cross(&v_b).co());

    println!("\n****************************************\n");

    // Varialble changes.
    v_b.scale(2.0);
    p_a.move_along(&v_b);

    // Tests.
    print_coordinate(&v_b, "v_b");
    print_coordinate(&p_a, "p_a");
    println!("Length of line is {};", line_1.len());

    line_1.point_at_start.move_along(&v_b);
    println!("Distance of p_a and p_b is {};", p_a.dist_to(&p_b));
    println!("Length of line is {};", line_1.len());

    println!("\n****************************************\n");

    let vec_global = Vector3d::new(2.2222e6, -3.3333e6, -1.257e-5);
    let vec_local = Vector3d::new(-1.09975e-8, -4.00617e6, -2.8982e-5);

    println!("|vec_global| / |vec_local| = {}", vec_global.len() / vec_local.len());

    println!("\n****************************************\n");
    let axis = Vector3d::new(0.0, 1.0, 2.0);
    let mut v_r_test = Vector3d::new(5.0, 1.4, 3.0);
    print_coordinate(&rotate(&v_r_test, &axis, 2.0), "v_rotate");
    v_r_test.rotate(&axis, 2.0);
    print_coordinate(&v_r_test, "v_rotate");

    println!("\n****************************************\n");
    let plane_o = Point3d::new(1.0, 1.0, 1.0);
    let plane_x = Vector3d::new(1.0, 3.0, 2.0);
    let plane_y = Vector3d::new(-1.0, 1.0, -1.0);
    let plane = Plane::new(plane_o, plane_x, plane_y);
    let circle = Circle::new(plane, 2.0);
    println!("The area of the circle is {}", circle.area());
    print_coordinate(&circle.point_at(0.38), "start point");

    let time_end = Instant::now();
    println!("Time = {:?}", time_end.duration_since(time_start));

    println!("");

    let v_01 = Point3d::new(1.0,   1.0, 0.0);
    let v_02 = Point3d::new(1.0,  -1.0, 0.0);
    let v_03 = Point3d::new(-1.0, -1.0, 0.0);
    let v_04 = Point3d::new(-1.0,  1.0, 0.0);
    let mut pl_a = Polyline::new(vec!(v_01, v_02, v_03, v_04), false);
    println!("The length of polyline is: {}", pl_a.len());
    pl_a.append_node(Point3d::new(0.0, 2.0, 0.0));
    println!("The length of polyline is: {}", pl_a.len());
    pl_a.closed();
    println!("The length of polyline is: {}", pl_a.len());
}
