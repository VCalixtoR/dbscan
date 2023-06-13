use crate::configuration::Point;

// calculates the euclidian distance between two multi dimensional points, the debug part is commented
pub fn euclidean_distance(p1: &Point, p2: &Point) -> f32 {
    let mut total_square_sum: f32 = 0.0;

    if p1.len() != p2.len() {
        panic!("Error: Different dimensions for two points");
    }

    for pos in 0..p1.len() {
        total_square_sum += (p1[pos] - p2[pos]).powf(2.0);
    }

    return total_square_sum.sqrt();
}
