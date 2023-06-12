mod configuration;
mod kmeans;
mod pre_processing;
mod post_processing;
mod utils;
use configuration::{ ClusterGroup, PointVector };

fn main() -> () {

    println!("\n---------------------");
    // IrisReduced - based on iris, removed major elements
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("IrisReduced.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[1].clone(), parsed_database[2].clone()];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // post processing
        post_processing::print_silhouette_coefficient(&result_cluster_group, &parsed_database);
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "IrisReduced_sepalXpetal.png");
    }
    println!("\n---------------------");
    // Iris https://www.kaggle.com/datasets/uciml/iris
    {
        // pre processing
        let used_collumns = vec![1, 3];
        let parsed_database: PointVector = pre_processing::pre_process_database("Iris.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![parsed_database[0].clone(), parsed_database[50].clone(), parsed_database[100].clone()];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // post processing
        post_processing::print_silhouette_coefficient(&result_cluster_group, &parsed_database);
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "SepalLengthCm", "PetalLengthCm", "Iris_sepalXpetal.png");
    }
    println!("\n---------------------");
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
            parsed_database[40].clone(),
        ];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // post processing
        post_processing::print_silhouette_coefficient(&result_cluster_group, &parsed_database);
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "Weight", "Height", "Clustering_gmm_WeightXHeight.png");
    }
    println!("\n---------------------");
    // Mall_Customers https://www.kaggle.com/datasets/vjchoudhary7/customer-segmentation-tutorial-in-python
    {
        // pre processing 
        let used_collumns = vec![2, 3, 4];
        let parsed_database: PointVector = pre_processing::pre_process_database("Mall_Customers.csv", &used_collumns);
        // data mining - kmeans
        let mut cluster_group: PointVector = vec![
            parsed_database[0].clone(),
            parsed_database[10].clone(),
            parsed_database[20].clone()
        ];
        let result_cluster_group: ClusterGroup = kmeans::kmeans_clusterization(&mut cluster_group, &parsed_database, false);
        // post processing
        post_processing::print_silhouette_coefficient(&result_cluster_group, &parsed_database);
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 1, "Age", "Annual Income", "Mall_Customers_AgeXAnnualIncome.png");
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 0, 2, "Age", "Spending Score", "Mall_Customers_AgeXSpendingScore.png");
        post_processing::parse_and_plot_cartesion_2d(&result_cluster_group, &parsed_database, 1, 2, "Annual Income", "Spending Score", "Mall_Customers_AnnualIncomeXSpendingScore.png");
    }
    println!("\n---------------------");
}
