use super::area::Area;
#[allow(dead_code)]
pub(crate) struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}