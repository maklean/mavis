// (x, y)
pub type Coordinate = (u16, u16);

pub fn euclidean_distance(p1: Coordinate, p2: Coordinate) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (x1, y1) = (x1 as f64, y1 as f64);
    let (x2, y2) = (x2 as f64, y2 as f64);

    return ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
}

pub fn manhattan_distance(p1: Coordinate, p2: Coordinate) -> u16 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (x1, y1) = (x1 as i32, y1 as i32);
    let (x2, y2) = (x2 as i32, y2 as i32);

    return ((x1 - x2).abs() + (y1 - y2).abs()) as u16;
}