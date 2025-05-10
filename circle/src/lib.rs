use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) ->f64 {
        self.radius.powf(2.0) * PI

    }
    pub fn intersect(&self, other:Circle) -> bool {
        let dx = self.center.0 - other.center.0;
        let dy = self.center.1 - other.center.1;
        let distance = (dx * dx + dy * dy).sqrt();
        let r1 = self.radius;
        let r2 = other.radius;
        distance <= r1 + r2 && distance >= (r1 - r2).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self , p : Point)-> f64{
        let num = (self.1 - p.1).powf(2.0) + (self.1 - p.1).powf(2.0);
        num.sqrt()
    }
}
