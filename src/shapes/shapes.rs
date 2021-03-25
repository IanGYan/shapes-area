use crate::shapes::calculable::AreaCalculable;

pub struct Triangle {
    pub bot_edge: f32,
    pub height: f32,
}

impl AreaCalculable for Triangle {
    fn area(&self) -> f32 {
        self.bot_edge * self.height * 0.5
    }
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl AreaCalculable for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Round {
    pub radius: f32,
}

impl AreaCalculable for Round {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.1415926
    }
}