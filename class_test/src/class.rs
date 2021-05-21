pub trait Coordinate {
    fn co(&self) -> (f64, f64, f64);
}

const PI: f64 = std::f64::consts::PI;

/********** Function *********/
pub fn add<T: Coordinate>(a: &T, b: &T) -> Vector3d {
    return Vector3d::new(
        a.co().0 + b.co().0,
        a.co().1 + b.co().1,
        a.co().2 + b.co().2,
    );
}

pub fn mult<T: Coordinate>(a: &T, b: f64) -> Vector3d {
    return Vector3d::new(a.co().0 * b, a.co().1 * b, a.co().2 * b);
}

pub fn rotate(v: &Vector3d, axis: &Vector3d, angle: f64) -> Vector3d {
    let y_dir = axis.cross(v);
    if y_dir.len() == 0.0 {
        return *v;
    } else {
        let z_axis_local = axis.unit();
        let y_axis_local = y_dir.unit();
        let x_axis_local = y_axis_local.cross(&z_axis_local);

        let a = v.dot(&x_axis_local);
        let b = v.dot(&y_axis_local);
        let c = v.dot(&z_axis_local);

        let d = a * angle.cos() - b * angle.sin();
        let e = a * angle.sin() + b * angle.cos();

        return Vector3d::new(
            d * x_axis_local.x + e * y_axis_local.x + c * z_axis_local.x,
            d * x_axis_local.y + e * y_axis_local.y + c * z_axis_local.y,
            d * x_axis_local.z + e * y_axis_local.z + c * z_axis_local.z,
        );
    }
}

/********** Class ************/
// BEGIN class: Vector3d;
#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate for Vector3d {
    fn co(&self) -> (f64, f64, f64) {
        return (self.x, self.y, self.z);
    }
}

impl Vector3d {
    pub fn new(x_co: f64, y_co: f64, z_co: f64) -> Vector3d {
        Vector3d {
            x: x_co,
            y: y_co,
            z: z_co,
        }
    }

    pub fn from_point(point: &Point3d) -> Vector3d {
        Vector3d {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }

    pub fn from_2points(from_point: &Point3d, to_point: &Point3d) -> Vector3d {
        Vector3d {
            x: to_point.x - from_point.x,
            y: to_point.y - from_point.y,
            z: to_point.z - from_point.z,
        }
    }

    pub fn add(&mut self, vector: &Vector3d) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
    }

    pub fn sub(&mut self, vector: &Vector3d) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }

    pub fn scale(&mut self, mult: f64) {
        self.x *= mult;
        self.y *= mult;
        self.z *= mult;
    }

    pub fn len(&self) -> f64 {
        let dist: f64 = ((self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)).sqrt();
        return dist;
    }

    pub fn rotate(&mut self, axis: &Vector3d, angle: f64) {
        *self = rotate(self, &axis, angle);
    }

    pub fn dot(&self, vector: &Vector3d) -> f64 {
        return self.x * vector.x + self.y * vector.y + self.z * vector.z;
    }

    pub fn cross(&self, vector: &Vector3d) -> Vector3d {
        return Vector3d::new(
            self.y * vector.z - self.z * vector.y,
            self.z * vector.x - self.x * vector.z,
            self.x * vector.y - self.y * vector.x,
        );
    }

    pub fn unit(&self) -> Vector3d {
        if self.len() != 0.0 {
            return mult(self, 1.0 / self.len());
        } else {
            panic!("Zero vector does not have unit vector!")
        }
    }

    pub fn unitize(&mut self) {
        if self.len() != 0.0 {
            self.scale(1.0 / self.len());
        } else {
            panic!("Zero vector cannot be unitize!")
        }
    }

    pub fn to_point(&self) -> Point3d {
        return Point3d::new(self.x, self.y, self.z);
    }
}
// END class: Vector3d.

// BEGIN class: Point3d;
#[derive(Copy, Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate for Point3d {
    fn co(&self) -> (f64, f64, f64) {
        return (self.x, self.y, self.z);
    }
}

impl Point3d {
    pub fn new(x_co: f64, y_co: f64, z_co: f64) -> Point3d {
        Point3d {
            x: x_co,
            y: y_co,
            z: z_co,
        }
    }

    pub fn move_along(&mut self, vec: &Vector3d) {
        self.x += vec.x;
        self.y += vec.y;
        self.z += vec.z;
    }

    pub fn copy_along(&self, vec: &Vector3d) -> Point3d {
        return Point3d::new(self.x + vec.x, self.y + vec.y, self.z + vec.z);
    }

    pub fn dist_to(&self, to_point: &Point3d) -> f64 {
        let dist: f64 = ((self.x - to_point.x).powi(2)
            + (self.y - to_point.y).powi(2)
            + (self.z - to_point.z).powi(2))
        .sqrt();
        return dist;
    }
}
// END class: Point3d.

// BEGIN class: Line;
pub struct Line {
    pub point_at_start: Point3d,
    pub point_at_end: Point3d,
}

impl Line {
    pub fn new(a: Point3d, b: Point3d) -> Line {
        Line {
            point_at_start: a,
            point_at_end: b,
        }
    }

    pub fn len(&self) -> f64 {
        return self.point_at_start.dist_to(&self.point_at_end);
    }

    pub fn point_at(&self, prm: f64) -> Point3d {
        let f = |x: f64, y: f64| (1.0 - y) * x + y * x;
        return Point3d::new(
            f(self.point_at_start.x, prm),
            f(self.point_at_start.y, prm),
            f(self.point_at_start.z, prm),
        );
    }
}
// END class: Line.

// BEGIN class: Plane;
#[derive(Copy, Clone)]
pub struct Plane {
    pub O: Point3d,
    pub X: Vector3d,
    pub Y: Vector3d,
    pub Z: Vector3d,
}

impl Plane {
    pub fn new(o: Point3d, x_axis: Vector3d, y_axis: Vector3d) -> Plane {
        if x_axis.dot(&y_axis) <= 1e-5 {
            let mut x = x_axis.unit();
            let mut y = y_axis.unit();
            Plane {
                O: o,
                X: x,
                Y: y,
                Z: x.cross(&y),
            }
        } else {
            panic!("x_axis and y_axis must be perpandicular!");
        }
    }
}
// END class: Plane.

// BEGIN class: Circle;
pub struct Circle {
    pub plane: Plane,
    pub center: Point3d,
    pub radius: f64,
}

impl Circle {
    pub fn new(p: Plane, r: f64) -> Circle {
        Circle {
            plane: p,
            center: p.O,
            radius: r,
        }
    }

    pub fn circum(&self) -> f64 {
        return 2.0 * PI * self.radius;
    }

    pub fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }

    pub fn point_at(&self, prm: f64) -> Point3d {
        let base_vec = mult(&self.plane.X, self.radius);
        let prm_vec = rotate(&base_vec, &self.plane.Z, 2.0 * prm * PI);
        return self.center.copy_along(&prm_vec);
    }
}
// END class: Circle.

// START class: Polyline;
pub struct Polyline {
    pub vertices: Vec<Point3d>,
    pub is_closed: bool,
}

impl Polyline {
    pub fn new(vertices: Vec<Point3d>, is_closed: bool) -> Self {
        Self {
            vertices,
            is_closed,
        }
    }

    pub fn len(&self) -> f64 {
        let mut length = 0_f64;
        for _i in 0..(self.vertices.len() - 1) {
            length += self.vertices[_i].dist_to(&self.vertices[_i + 1]);
        }
        if self.is_closed == true {
            length + self.vertices[0].dist_to(&self.vertices.last().unwrap())
        } else {
            length
        }
    }

    pub fn append_node(&mut self, pt: Point3d) {
        self.vertices.push(pt);
    }

    pub fn append_polyline(&mut self, pl: Polyline) {
        if self.vertices.last().unwrap().dist_to(&pl.vertices[0]) <= 1e-5 {
            for i in 1..pl.vertices.len() {
                self.vertices.push(pl.vertices[i]);
            }
        }
    }

    pub fn closed(&mut self) {
        if !self.is_closed {
            self.is_closed = true
        }
    }
}
// END class: Polyline.
