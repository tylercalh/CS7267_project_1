
// glam is a Rust library that provides linear algebra functionality. 
use glam::*;
use std::fs::File;
use std::io::prelude::*;

// The cluster maintains the state of the centroid and references to each data point that belong to it.
struct Cluster<'a> {
    centroid: Vec2,
    data: Vec<&'a Vec2>,
}

impl<'a> Cluster<'a> {
    fn new(_centroid: Vec2) -> Self {
	Self {
	    centroid: _centroid,
	    data: Vec::new(),
	}
    }

    fn mean(&self) -> Vec2 {
	let num_points = self.data.len() as f32;
	let total = self.data
	    .iter()
	    .fold(Vec2::ZERO, |acc, &&x| acc + x);
	total / num_points
    }
}

fn main() {
    // Default data from KMTest.csv.
    let data: Vec<Vec2> = vec![
	Vec2::new(2.000000, 4.000000),
	Vec2::new(3.000000, 3.000000),
	Vec2::new(3.000000, 4.000000),
	Vec2::new(3.000000, 5.000000),
	Vec2::new(4.000000, 3.000000),
	Vec2::new(4.000000, 4.000000),
	Vec2::new(9.000000, 4.000000),
	Vec2::new(9.000000, 5.000000),
	Vec2::new(9.000000, 9.000000),
	Vec2::new(9.000000, 10.000000),
	Vec2::new(10.000000, 4.000000),
	Vec2::new(10.000000, 5.000000),
	Vec2::new(10.000000, 9.000000),
	Vec2::new(10.000000, 10.000000),
	Vec2::new(11.000000, 10.000000),
	Vec2::new(15.000000, 4.000000),
	Vec2::new(15.000000, 5.000000),
	Vec2::new(15.000000, 6.000000),
	Vec2::new(16.000000, 4.000000),
	Vec2::new(16.000000, 6.000000)];

    let clusters = k_means(&data);

    // Write the data to the log.
    write_log("logs/clustered.csv", &clusters);
}

fn k_means(data: &Vec<Vec2>) -> Vec<Cluster> {
    // Initialize K clusters.
    let mut clusters: Vec<Cluster> = Vec::new();
    const K: usize = 3;
    clusters.push(Cluster::new(Vec2::new(2.5, 2.5)));
    clusters.push(Cluster::new(Vec2::new(5.0, 5.0)));
    clusters.push(Cluster::new(Vec2::new(16.0, 10.0)));
    
    let mut stop = false;
    while !stop {

	// Remove data points from the clusters (if there are any).
	for cluster in clusters.iter_mut() {
	    cluster.data.clear();
	}

	// Give each data point to the cluster closest to it.
	for point in data.iter() {
	    let min_index = closest_cluster(point, &clusters);
	    clusters[min_index].data.push(point);
	}

	// Calculate the mean value of each cluster. Set the cluster's centroid to its mean.
	let mut max_delta = 0.0;
	for cluster in clusters.iter_mut() {
	    let prev_mean = cluster.centroid;
	    let new_mean = cluster.mean();
	    cluster.centroid = new_mean;

	    // Keep track of the largest centroid movement.
	    let delta = (new_mean - prev_mean).length();
	    if delta > max_delta {max_delta = delta;}
	}
	// Convergence is reached when centroid movement is sufficiently small.
	if max_delta < 0.1 {stop = true;}
    }
    
    clusters
}


// Determine which cluster is closest to a given point using the cluster's centroid.
// The distance function uses euclidean distance between the point and the cluster centroid.
fn closest_cluster(point: &Vec2, clusters: &Vec<Cluster>) -> usize {
    let mut min_index = 0;
    for cur_index in 1..clusters.len() {
	let d0 = point.distance(clusters[min_index].centroid);
	let d1 = point.distance(clusters[cur_index].centroid);
	if d1 < d0 {min_index = cur_index;}
    }
    min_index
}

// Format and write the data to a log file.
fn write_log(path: &str, clusters: &Vec<Cluster>) {
    let colors = vec!["r", "g", "b", "c", "m", "y", "k"];
    let mut file = File::create(path).unwrap();

    for (index, cluster) in clusters.iter().enumerate() {
	for point in cluster.data.iter() {
	    let record = format!("{} {} \"{}\"\n", point.x, point.y, colors[index]);
	    file.write_all(record.as_bytes()).unwrap();
	}
    }
}
