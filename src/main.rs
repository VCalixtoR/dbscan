mod configuration;
mod pre_processing;
mod utils;
use configuration::PointVector;

fn main() -> () {
    let point_vector: PointVector = pre_processing::pre_process_database("Iris.csv");

    println!("Resulting PointVector: ");
    for point in point_vector.clone() {
        print!("[");
        for point_pos in 0..point.len() {
            print!("{: ^6.2}", point[point_pos]);
        }
        println!("]");
    }

    println!(
        "euclidian_distance({:?}, {:?}) = {}",
        point_vector[0],
        point_vector[1],
        utils::euclidian_distance(&point_vector[0], &point_vector[1])
    );
    // do you dbscan function bellow, this functions need to have its own modules for organization
}
