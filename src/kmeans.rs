use crate::configuration::{ ClusterGroup, IndexType, Point, PointVector };
use crate::utils::euclidian_distance;

pub fn calc_centroid_by_mean(database_points: &PointVector, centroid_points: &mut PointVector, cluster_group: &ClusterGroup) -> () {

    let mut cluster_sum: Point;

    for cluster_pos in 0..cluster_group.clusters.len() {
        // sum all values from a cluster
        cluster_sum = vec![0.0; database_points[0].len() as usize];
        for core_pos in 0..cluster_group.clusters[cluster_pos].core_indexes.len() {
            let index = cluster_group.clusters[cluster_pos].core_indexes[core_pos];
            for value_pos in 0..database_points[index].len() {
                cluster_sum[value_pos] += database_points[index][value_pos];
            }
        }
        // assign average value to centroid
        for value_pos in 0..cluster_sum.len() {
            centroid_points[cluster_pos][value_pos] = cluster_sum[value_pos] / (cluster_group.clusters[cluster_pos].core_indexes.len() as f32);
        }
    }
}

pub fn kmeans_clusterization(centroid_points: &mut PointVector, database_points: &PointVector, debug: bool) -> ClusterGroup {
    
    let mut cluster_group: ClusterGroup = ClusterGroup::new(centroid_points.len());
    let mut min_centroid_distance: f32;
    let mut min_centroid_index: usize;
    let mut tmp_float: f32;
    let mut iteration: u32 = 0;
    
    println!("\nStarting Kmeans");

    // while centroid changes
    let mut cluster_group_has_changed: bool = true;
    while cluster_group_has_changed {
        iteration += 1;

        // temporary cluster group
        let mut tmp_cluster_group = ClusterGroup::new(centroid_points.len());

        // foreach point
        for point_index in 0..database_points.len() {
            min_centroid_distance = f32::MAX;
            min_centroid_index = 0;

            // calculate the distance to the centroids and group it to the nearest centroid
            for centroid_index in 0..centroid_points.len() {
                tmp_float = euclidian_distance(&centroid_points[centroid_index], &database_points[point_index]);
                if min_centroid_distance > tmp_float {
                    min_centroid_distance = tmp_float;
                    min_centroid_index = centroid_index;
                }
            }
            tmp_cluster_group.add_index_to_cluster(IndexType::CoreIndex, point_index, min_centroid_index);
        }
        cluster_group_has_changed = !cluster_group.is_ordened_equals_to(&tmp_cluster_group);

        if cluster_group_has_changed {
            cluster_group = tmp_cluster_group; // move pointer to cluster_group and drops tmp_cluster_group
            calc_centroid_by_mean(database_points, centroid_points, &cluster_group);
        }

        println!("Iteration: {}", iteration);
        println!("Cluster Group changed = {}", cluster_group_has_changed);
        if debug {
            println!("Centroids             = {:?}", &centroid_points);
            println!("Cluster Group         : ");
            cluster_group.print();
        }
    }
    println!();
    return cluster_group;
}