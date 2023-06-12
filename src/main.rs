mod configuration;
mod kmeans;
mod pre_processing;
mod post_processing;
mod utils;
use configuration::{ ClusterGroup, PointVector };

fn main() -> () {

    // IrisReduced
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("IrisReduced.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[1].clone(), parsed_database[2].clone()];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database);
        // post processing
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "IrisReduced_sepalXpetal.png");
    }
    // Iris
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("Iris.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[50].clone(), parsed_database[100].clone()];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database);
        // post processing
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Iris_sepalXpetal.png");
    }
    // Clustering_gmm
    {
        // pre processing
        let used_collumns = vec![0, 1];
        let parsed_database: PointVector = pre_processing::pre_process_database("Clustering_gmm.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![
            parsed_database[0].clone(),
            parsed_database[10].clone(),
            parsed_database[20].clone(),
            parsed_database[30].clone(),
            parsed_database[40].clone(),
        ];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database);
        // post processing
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "Weight", "Height", "Clustering_gmm_WeightXHeight.png");
    }
}
