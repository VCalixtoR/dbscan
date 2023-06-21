mod configuration;
mod dbscan;
mod kmeans;
mod pre_processing;
mod post_processing;
mod utils;
use configuration::{ ClusterGroup, PointVector };
use std::io;

fn main() -> () {
    // Each code block represents a different database
    println!("\n--------------------- IrisReduced ---------------------");
    // IrisReduced - based on iris, removed major elements
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("IrisReduced.csv", &used_collumns);
        
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[2].clone()];
        let kmeans_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // data mining - dbscan
        let dbscan_cluster_group: ClusterGroup = dbscan::dbscan_clusterization(&parsed_database, 0.3, 3);
        
        // post processing
        post_processing::print_silhouette_coefficient(&kmeans_cluster_group, &parsed_database, "Kmeans");
        post_processing::print_silhouette_coefficient(&dbscan_cluster_group, &parsed_database, "DBSCAN");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Km_IrisReduced_sepalXpetal.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Db_IrisReduced_sepalXpetal.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "IrisReduced_sepalXpetal.png");
    }
    println!("\n--------------------- Iris ---------------------");
    // Iris https://www.kaggle.com/datasets/uciml/iris
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("Iris.csv", &used_collumns);
        
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[50].clone()];
        let kmeans_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // data mining - dbscan
        let dbscan_cluster_group: ClusterGroup = dbscan::dbscan_clusterization(&parsed_database, 0.15, 7);
        
        // post processing
        post_processing::print_silhouette_coefficient(&kmeans_cluster_group, &parsed_database, "Kmeans");
        post_processing::print_silhouette_coefficient(&dbscan_cluster_group, &parsed_database, "DBSCAN");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Km_Iris_sepalXpetal.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Db_Iris_sepalXpetal.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Iris_sepalXpetal.png");
    }
    println!("\n--------------------- Clustering gmm ---------------------");
    // Clustering_gmm https://www.kaggle.com/datasets/ankit8467/dataset-for-dbscan
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
        ];
        let kmeans_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // data mining - dbscan
        let dbscan_cluster_group: ClusterGroup = dbscan::dbscan_clusterization(&parsed_database, 0.035, 7);

        // post processing
        post_processing::print_silhouette_coefficient(&kmeans_cluster_group, &parsed_database, "Kmeans");
        post_processing::print_silhouette_coefficient(&dbscan_cluster_group, &parsed_database, "DBSCAN");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 1, "Weight", "Height", "Km_Clustering_gmm_WeightXHeight.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 1, "Weight", "Height", "Db_Clustering_gmm_WeightXHeight.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Clustering_gmm.png");
    }
    println!("\n--------------------- Mall_Customers ---------------------");
    // Mall_Customers https://www.kaggle.com/datasets/vjchoudhary7/customer-segmentation-tutorial-in-python
    {
        // pre processing 
        let used_collumns = vec![2, 3, 4];
        let parsed_database: PointVector = pre_processing::pre_process_database("Mall_Customers.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![
            parsed_database[0].clone(),
            parsed_database[10].clone(),
            parsed_database[20].clone(),
            parsed_database[30].clone(),
        ];
        let kmeans_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        let dbscan_cluster_group: ClusterGroup = dbscan::dbscan_clusterization(&parsed_database, 0.151, 4);
        
        // post processing
        post_processing::print_silhouette_coefficient(&kmeans_cluster_group, &parsed_database, "Kmeans");
        post_processing::print_silhouette_coefficient(&dbscan_cluster_group, &parsed_database, "DBSCAN");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 1, "Age", "Annual Income", "Km_Mall_Customers_AgeXAnnualIncome.png");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 2, "Age", "Spending Score", "Km_Mall_Customers_AgeXSpendingScore.png");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 1, 2, "Annual Income", "Spending Score", "Km_Mall_Customers_AnnualIncomeXSpendingScore.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 1, "Age", "Annual Income", "Db_Mall_Customers_AgeXAnnualIncome.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 2, "Age", "Spending Score", "Db_Mall_Customers_AgeXSpendingScore.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 1, 2, "Annual Income", "Spending Score", "Db_Mall_Customers_AnnualIncomeXSpendingScore.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 1, "Age", "Annual Income", "Db_Mall_Customers_AgeXAnnualIncome.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 2, "Age", "Spending Score", "Db_Mall_Customers_AgeXSpendingScore.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 1, 2, "Annual Income", "Spending Score", "Db_Mall_Customers_AnnualIncomeXSpendingScore.png");
    }
    println!("\n--------------------- Circular Generated ---------------------");
    // Circular_Generated - Used functions to generate database - https://oralytics.com/2021/10/18/dbscan-clustering-in-python/
    {
        // pre processing 
        let used_collumns = vec![0, 1];
        let parsed_database: PointVector = pre_processing::pre_process_database("Circular_Generated.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![
            parsed_database[0].clone(),
            parsed_database[1000].clone(),
            parsed_database[2000].clone(),
        ];
        let kmeans_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        let dbscan_cluster_group: ClusterGroup = dbscan::dbscan_clusterization(&parsed_database, 0.025, 8);
        // post processing
        post_processing::print_silhouette_coefficient(&kmeans_cluster_group, &parsed_database, "Kmeans");
        post_processing::print_silhouette_coefficient(&dbscan_cluster_group, &parsed_database, "DBSCAN");
        post_processing::parse_and_plot_cartesion_2d(&kmeans_cluster_group, &parsed_database, 0, 1, "x_axis", "y_axis", "Km_Circular_Generated_xXy.png");
        post_processing::parse_and_plot_cartesion_2d(&dbscan_cluster_group, &parsed_database, 0, 1, "x_axis", "y_axis", "Db_Circular_Generated_xXy.png");
        post_processing::parse_database_and_plot_cartesion_2d(&parsed_database, 0, 1, "x_axis", "y_axis", "Db_Circular_Generated_xXy.png");

    }

    println!("\nPress enter to exit: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}