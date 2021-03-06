use std::cmp::Ordering;
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This reduces runtime by ~20%.
        state.write_i32(self.x ^ self.y ^ self.z);
    }
}

// Since I implemented Hash, I also need to implement these...
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y, self.z).cmp(&(other.x, other.y, other.z))
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x, y, z, }
    }

    // TODO: Should I be taking 'other' as a copy or a reference?
    // Before change: 5.7s
    // After change: 5.5s
    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn minus(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(&self, other: &Vector) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn negate(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}

pub fn manhatten_distance(a: &Vector, b: &Vector) -> i32 {
    i32::abs(a.x - b.x)
        + i32::abs(a.y - b.y)
        + i32::abs(a.z - b.z)
}

#[derive(Debug, PartialEq, Eq)]
pub struct VectorTransform {
    // Yes, I suppose this could also just be called "Matrix".
    x: Vector,
    y: Vector,
    z: Vector,
}

impl VectorTransform {
    pub fn new(x: Vector, y: Vector, z: Vector) -> VectorTransform {
        VectorTransform { x, y, z }
    }

    pub fn apply(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x.dot(v),
            y: self.y.dot(v),
            z: self.z.dot(v),
        }
    }
}

lazy_static! {
    pub static ref AXES: Vec<Vector> = vec![
        Vector::new( 1,  0,  0),
        Vector::new(-1,  0,  0),
        Vector::new( 0,  1,  0),
        Vector::new( 0, -1,  0),
        Vector::new( 0,  0,  1),
        Vector::new( 0,  0, -1),
    ];

    pub static ref ROTATIONS: Vec<VectorTransform> = {
        let mut v = Vec::new();

        for x in AXES.iter() {
            for y in AXES.iter() {

                if x == y || *x == y.negate() {
                    continue;
                }

                let z = x.cross(y);

                v.push(VectorTransform::new(*x, *y, z))
            }
        }

        v
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform() {
        let scale = VectorTransform::new(
            Vector::new(2, 0, 0),
            Vector::new(0, 3, 0),
            Vector::new(0, 0, 4),
        );

        assert_eq!(Vector::new(4, 6, 8), scale.apply(&Vector::new(2, 2, 2)));
    }

    #[test]
    fn test_rotations() {
        assert_eq!(24, ROTATIONS.len());

        let id = VectorTransform::new(
            Vector::new(1, 0, 0),
            Vector::new(0, 1, 0),
            Vector::new(0, 0, 1),
        );

        assert!(ROTATIONS.contains(&id));

        let rotation = VectorTransform::new(
            Vector::new(1, 0, 0),
            Vector::new(0, -1, 0),
            Vector::new(0, 0, -1),
        );
        assert!(ROTATIONS.contains(&rotation));

        let rotation = VectorTransform::new(
            Vector::new(1, 0, 0),
            Vector::new(0, 0, 1),
            Vector::new(0, -1, 0),
        );
        assert!(ROTATIONS.contains(&rotation));

        // This would not obey the right hand rule.
        let bad_rotation = VectorTransform::new(
            Vector::new(1, 0, 0),
            Vector::new(0, 0, 1),
            Vector::new(0, 1, 0),
        );
        assert!(!ROTATIONS.contains(&bad_rotation));
    }
}