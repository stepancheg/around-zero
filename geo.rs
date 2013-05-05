use misc::*;

pub struct Coord {
    lat: f32,
    long: f32,
}

impl ToStr for Coord {
    fn to_str(&self) -> ~str {
        self.lat.to_str() + " " + self.long.to_str()
    }
}

impl Coord {
    pub fn dist(&self, that: &Coord) -> f32 {
        f32::sqrt(sqr(self.lat - that.lat) + sqr(self.long - that.long))
    }
}

