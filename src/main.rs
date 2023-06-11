mod configuration;
mod kmeans;
mod pre_processing;
mod utils;
use configuration::{ ClusterGroup, PointVector, KmeansConfig };

fn main() -> () {
    // pre processing
    let point_vector: PointVector = pre_processing::pre_process_database("Simple.csv");
    
    println!("PreProcess Result: ");
    for vector_pos in 0..point_vector.len() {
        print!("{: ^3}[", vector_pos);
        for point_pos in 0..point_vector[vector_pos].len() {
            print!("{: ^6.2}", point_vector[vector_pos][point_pos]);
        }
        println!("]");
    }

    let _result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(
        KmeansConfig {
            centroid_points: vec![point_vector[0].clone(), point_vector[1].clone(), point_vector[2].clone()],
            database_points: point_vector,
        });

    // do you dbscan function bellow, this functions need to have its own modules for organization
}
