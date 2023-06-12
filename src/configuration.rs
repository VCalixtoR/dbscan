use std::clone::Clone;

pub type Point = Vec<f32>; // Each Point is an item of the dataset
pub type PointVector = Vec<Point>; // PointVector is a collection of dataset items

// enumerate point indexes types
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IndexType {
    CoreIndex,
    BorderIndex,
    OutlierIndex,
}
// single cluster
#[derive(Clone)]
#[allow(dead_code)]
pub struct Cluster {
    pub core_indexes: Vec<usize>,
    pub border_indexes: Vec<usize>,
}
// result from clustering algorithm
#[derive(Clone)]
#[allow(dead_code)]
pub struct ClusterGroup {
    pub clusters: Vec<Cluster>,
    pub outlier_indexes: Vec<usize>,
}
impl ClusterGroup {
    // initialize a cluster group with cluster_quantity clusters
    pub fn new(cluster_quantity: usize) -> ClusterGroup {
        
        let mut cluster_group: ClusterGroup = ClusterGroup {
            clusters: Vec::new(),
            outlier_indexes: Vec::new()
        };
        for _ in 0..cluster_quantity {
            cluster_group.clusters.push( Cluster {
                core_indexes: Vec::new(),
                border_indexes: Vec::new(),
            })
        };
        return cluster_group;
    }
    // add a index with IndexType to a cluster, it not handles index duplicity for eficiency
    pub fn add_index_to_cluster(&mut self, index_type: IndexType, index: usize, cluster_index: usize) -> () {

        if index_type != IndexType::OutlierIndex && cluster_index >= self.clusters.len() {
            panic!("Error: Invalid cluster index while trying to add a index to a cluster: len={} index={}", self.clusters.len(), cluster_index);
        }

        match index_type {
            IndexType::OutlierIndex => {
                self.outlier_indexes.push(index);
            },
            IndexType::CoreIndex => {
                self.clusters[cluster_index].core_indexes.push(index);
            },
            IndexType::BorderIndex =>{
                self.clusters[cluster_index].border_indexes.push(index);
            },
        }
    }
    /* 
        Returns if the self cluster is equal to another: O(n) considering each index as input
        The ClusterGroups must have the same number of clusters, be ordened in clusters and in cluster indices
        Efficient comparison
    */
    pub fn is_ordened_equals_to(&self, clg_b: &ClusterGroup) -> bool {
        // compares cluster length and outlier_indexes
        if self.clusters.len() != clg_b.clusters.len() || self.outlier_indexes.len() != clg_b.outlier_indexes.len() {
            return false;
        }
        for outlier_pos in 0..self.outlier_indexes.len() {
            if self.outlier_indexes[outlier_pos] != clg_b.outlier_indexes[outlier_pos] {
                return false;
            }
        }
        for cluster_pos in 0..self.clusters.len() {
            // compares core_indexes
            if self.clusters[cluster_pos].core_indexes.len() != clg_b.clusters[cluster_pos].core_indexes.len() {
                return false;
            }
            for core_pos in 0..self.clusters[cluster_pos].core_indexes.len() {
                if self.clusters[cluster_pos].core_indexes[core_pos] != clg_b.clusters[cluster_pos].core_indexes[core_pos] {
                    return false;
                }
            }
            // compares border_indexes
            if self.clusters[cluster_pos].border_indexes.len() != clg_b.clusters[cluster_pos].border_indexes.len() {
                return false;
            }
            for border_pos in 0..self.clusters[cluster_pos].border_indexes.len() {
                if self.clusters[cluster_pos].border_indexes[border_pos] != clg_b.clusters[cluster_pos].border_indexes[border_pos] {
                    return false;
                }
            }
        }
        return true;
    }
    // formated prints
    pub fn print(&self) -> () {
        println!("----");
        for cluster_pos in 0..self.clusters.len() {

            if self.clusters[cluster_pos].core_indexes.len() > 0 {
                print!("    C{}:CoreI:", cluster_pos);
                for core_pos in 0..self.clusters[cluster_pos].core_indexes.len() {
                    print!(" {: ^3}", self.clusters[cluster_pos].core_indexes[core_pos]);
                }
                println!();
            }
            
            if self.clusters[cluster_pos].border_indexes.len() > 0 {
                print!("    C{}:BordI:", cluster_pos);
                for border_pos in 0..self.clusters[cluster_pos].border_indexes.len() {
                    print!(" {: ^3}", self.clusters[cluster_pos].border_indexes[border_pos]);
                }
                println!();
            }
        }
        if self.outlier_indexes.len() > 0 {
            print!("    CN:OutlI:");
            for outlier_pos in 0..self.outlier_indexes.len() {
                print!(" {: ^3}", self.outlier_indexes[outlier_pos]);
            }
            println!();
        }
        println!("----");
    }
}