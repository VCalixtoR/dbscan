pub type Point = Vec<f32>; // Each Point is an item of the dataset
pub type PointVector = Vec<Point>; // PointVector is a collection of dataset items

// single cluster
pub struct Cluster {
    pub cluster_name: String,
    pub core_indexes: Vec<usize>,
    pub border_indexes: Vec<usize>,
}
// result from clustering algorithm
pub struct ClusterGroup {
    pub clusters: Vec<Cluster>,
    pub outlier_indexes: Vec<usize>,
}
