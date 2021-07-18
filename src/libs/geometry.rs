pub trait Figure {
    fn area(&self) -> f64;
    fn volume(&self) -> f64;
}

pub struct Circle { pub radius: f64 }
impl Figure for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * &self.radius.powf(2.0)
    }

    fn volume(&self) -> f64 {
        0.0
    }
}

pub struct Rectangle { pub length: f64, pub width: f64 }
impl Figure for Rectangle {
    fn area(&self) -> f64 {
        &self.length * &self.width
    }

    fn volume(&self) -> f64 {
        0.0
    }
}

pub struct Sphere { pub radius: f64 }
impl Figure for Sphere {
    fn area(&self) -> f64 {
        0.0
    }

    fn volume(&self) -> f64 {
        (std::f64::consts::PI * &self.radius.powf(3.0) * 4.0) / 3.0
    }
}

pub struct Parallelepiped { pub length: f64, pub width: f64, pub height: f64 }
impl Figure for Parallelepiped {
    fn area(&self) -> f64 {
        0.0
    }

    fn volume(&self) -> f64 {
        &self.length * &self.width * &self.height
    }
}

// sphere and parallelepiped
pub fn calculate_area(t: &dyn Figure) -> f64 {
    return t.area();
}

pub fn calculate_volume(t: &dyn Figure) -> f64 {
    return t.volume();
}