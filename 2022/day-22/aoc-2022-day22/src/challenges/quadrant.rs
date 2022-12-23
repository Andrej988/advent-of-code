use super::coordinates::Coordinates;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Quadrant {
    A,
    B,
    C,
    D,
    E,
    F,
}

pub fn get_quadrant(coords: &Coordinates) -> Quadrant {
    if coords.x >= 1 && coords.x <= 50 && coords.y >= 51 && coords.y <= 100 {
        Quadrant::A
    } else if coords.x >= 1 && coords.x <= 50 && coords.y >= 101 && coords.y <= 150 {
        Quadrant::B
    } else if coords.x >= 51 && coords.x <= 100 && coords.y >= 51 && coords.y <= 100 {
        Quadrant::C
    } else if coords.x >= 101 && coords.x <= 150 && coords.y >= 1 && coords.y <= 50 {
        Quadrant::D
    } else if coords.x >= 101 && coords.x <= 150 && coords.y >= 51 && coords.y <= 100 {
        Quadrant::E
    } else {
        Quadrant::F
    }
}
