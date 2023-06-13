use crate::configuration::{ClusterGroup, Point, PointVector, PointType};
use crate::utils::euclidean_distance;
use std::collections::VecDeque;

#[derive(Debug)]
struct ClusterPoint {
    cluster_label: ClusterLabel,
    position: Point,
    point_type: PointType,
}

#[derive(Debug)]
enum ClusterLabel {
    Undefined,
    Outlier,
    ClusterId(u32),
}

#[derive(Debug)]
struct ClusteringDataset(Vec<ClusterPoint>);

// public interface for dbscan
pub fn dbscan_clusterization(dataset: &PointVector, eps: f32, min: usize) -> ClusterGroup {

    println!("Starting DBSCAN");

    let mut c_dataset: ClusteringDataset = parse_dataset_for_dbscan(dataset); // parse dataset
    do_dbscan_clusterization(&mut c_dataset, eps, min); // do clusterization

    println!();

    return parse_dbscan_output(& c_dataset); // parse dataset output
}

/* 
    Creates clustering dataset format to do clustering efficiently
        Used because of the configuration structure uses indexes and not points(ClusterGroup)
        It is not used in algorithm because of the quadratic access that would be less efficiently 
            for derefering indexes
*/
fn parse_dataset_for_dbscan(dataset: &PointVector) -> ClusteringDataset {

    // each item is a undefined cluster in start
    let c_dataset = ClusteringDataset(
        dataset.into_iter().map(|e|
            ClusterPoint {
                cluster_label: ClusterLabel::Undefined,
                point_type: PointType::Undefined,
                position: e.clone() 
            }
        ).collect());
    
    return c_dataset;
}

/* 
    Does the clustering
        The algorithm is based on creating and expanding clusters to their maximum size based on their neighborhood
        The neighbourhood is defined by the nearest min points in a radius of eps
        If the cluster has no neighboors the expansion ends
        If cluster size is one, it is a outlier(does not have min neighboors in eps)
*/
fn do_dbscan_clusterization(dataset: &mut ClusteringDataset, eps: f32, min: usize) {
    let mut current_cluster_id = 0;

    // foreach item in dataset
    for i in 0..dataset.0.len() {
        match dataset.0[i].cluster_label {
            // if its not classified expand that item as a new cluster
            ClusterLabel::Undefined => {
                if expand_cluster(dataset, i, current_cluster_id, eps, min) {
                    current_cluster_id += 1;
                    println!("Cluster expansion: {}", current_cluster_id);
                }
            }
            // else ignores (it is a neighboor classified in previous expansions)
            _ => {}
        }
    }
}

// expand a cluster with initial size of 1 point based on it neighbourhood, eps and min
fn expand_cluster(dataset: &mut ClusteringDataset, index: usize, cluster_id: u32, eps: f32, min: usize) -> bool {
    
    let mut expansion = neighbourhood(dataset, index, eps);

    // if the neighbourhood is not at least min, it is an outlier
    if expansion.len() < min {
        dataset.0[index].cluster_label = ClusterLabel::Outlier;
        dataset.0[index].point_type = PointType::Outlier;
        return false;
    }

    // else, sets actual point as a cluster core point
    dataset.0[index].cluster_label = ClusterLabel::ClusterId(cluster_id);
    dataset.0[index].point_type = PointType::Core;

    // assign neighbourhood to the current cluster
    while expansion.len() != 0 {
        let current = expansion.pop_front().unwrap();
        match dataset.0[current].cluster_label {
            ClusterLabel::Undefined => {
                dataset.0[current].cluster_label = ClusterLabel::ClusterId(cluster_id);
                let mut current_expansion = neighbourhood(dataset, current, eps);
                // if it has more than min neighbourhood, it is a Core point
                if current_expansion.len() >= min {
                    expansion.append(&mut current_expansion);
                    dataset.0[current].point_type = PointType::Core;
                }
                // if not it is a Border point
                else{
                    dataset.0[current].point_type = PointType::Border;
                }
            }
            ClusterLabel::Outlier => {
                // point classified to border point because it was preview classified as outlier
                dataset.0[current].cluster_label = ClusterLabel::ClusterId(cluster_id);
                dataset.0[current].point_type = PointType::Border;
            }
            ClusterLabel::ClusterId(_) => {}
        }
    }
    true
}

// calculates neighbourhood for a point base on eps
fn neighbourhood(dataset: &ClusteringDataset, index: usize, eps: f32) -> VecDeque<usize> {
    (0..dataset.0.len()).filter(|neighbour_index| {
        let dist = euclidean_distance(&dataset.0[index].position, &dataset.0[*neighbour_index as usize].position);
        dist <= eps && dist > 0.0
    }).collect()
}

// parse 
fn parse_dbscan_output(dataset: &ClusteringDataset) -> ClusterGroup {

    let mut cluster_group: ClusterGroup;
    let mut max_cluster_label: u32 = 0;

    // get the number of clusters O(n) to create ClusterGroup
    for i in 0..dataset.0.len() {
        match dataset.0[i].cluster_label {
            ClusterLabel::ClusterId(cluster_id) => {
                if cluster_id > max_cluster_label {
                    max_cluster_label = cluster_id;
                }
            },
            _ => {},
        }
    }
    cluster_group = ClusterGroup::new((max_cluster_label + 1) as usize);

    // assign each point index to its cluster in ClusterGroup
    for i in 0..dataset.0.len() {
        let cluster_index: u32 = get_point_cluster_group(dataset, i);

        // core and border
        if cluster_index != u32::MAX {
            if dataset.0[i].point_type != PointType::Undefined {
                cluster_group.add_index_to_cluster(dataset.0[i].point_type.clone(), i as usize, cluster_index as usize);
            }
            else {
                panic!("Error: Undefined point output in dbscan");
            }
        }
        // outliers
        else if dataset.0[i].point_type == PointType::Outlier {
            cluster_group.add_index_to_cluster(PointType::Outlier, i as usize, 0);
        }
    }
    return cluster_group;
}

// get points cluster group, returns u32::MAX if is not a ClusterId enum
fn get_point_cluster_group(dataset: &ClusteringDataset, index: usize) -> u32 {

    match dataset.0[index].cluster_label {
        ClusterLabel::ClusterId(cluster_id) => {
            return cluster_id;
        },
        _ => {
            return u32::MAX;
        },
    }
}