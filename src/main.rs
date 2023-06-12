mod configuration;
mod kmeans;
mod pre_processing;
mod post_processing;
mod utils;
use configuration::{ ClusterGroup, PointVector };

fn main() -> () {
    // pre processing
    let parsed_database: PointVector = pre_processing::pre_process_database("Simple.csv");
    
    /*
    println!("PreProcess Result: ");
    for vector_pos in 0..parsed_database.len() {
        print!("{: ^3}[", vector_pos);
        for point_pos in 0..parsed_database[vector_pos].len() {
            print!("{: ^6.2}", parsed_database[vector_pos][point_pos]);
        }
        println!("]");
    }
    */

    // kmeans
    let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[1].clone(), parsed_database[2].clone()];
    let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database);

    // post processing
    post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "petal_length", "sepal_length", "petalXsepal.png");

    // do you dbscan function bellow, this functions need to have its own modules for organization
}
