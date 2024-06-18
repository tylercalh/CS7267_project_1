
// glam is a Rust library that provides linear algebra functionality. 
use glam::*;
use csv::*;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Configurations:
    const NORMALIZE: bool = true;
    const DATASET: usize = 1; // 0 for kmtest.csv, 1 for iris.csv
    
    // Default data from KMTest.csv.
    let mut data: Vec<Vec2> = Vec::new();
    data = match DATASET {
	0 => read_km_test(),
	1 => read_iris(),
	_ => read_km_test(),
    };

    data = match NORMALIZE {
	true => normalize_data(&data),
	false => data,
    };

    normalize_data(&data);

    let clusters = k_means(&data);

    let mut sse = 0.0;
    for cluster in clusters.iter() {
	sse += cluster.sse();
    }
    println!("{}", sse);

    // Write the data to the log.
    write_log("logs/2b_worst.csv", &clusters);
}

fn k_means(data: &Vec<Vec2>) -> Vec<Cluster> {
    // Initialize K clusters.
    let mut clusters: Vec<Cluster> = Vec::new();

    //1a_k2
    clusters.push(Cluster::new(Vec2::new(4.0, 4.0)));
    clusters.push(Cluster::new(Vec2::new(12.0, 7.0)));
    
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
	if max_delta < 1.0 {stop = true;}
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

// Normalize the dataset with z-score normalization.
fn normalize_data(data: &Vec<Vec2>) -> Vec<Vec2> {
    let n = data.len() as f32;
    let total = data.iter().fold(Vec2::ZERO, |acc, x| acc + *x);
    let mu = total / n;

    let sigma_square = data
	.iter()
	.map(|x| (*x - mu) * (*x - mu))
	.fold(Vec2::ZERO, |acc, x| acc + x) / n;

    let sigma = Vec2::new(sigma_square.x.sqrt(), sigma_square.y.sqrt());

    data.iter().map(|x| (*x - mu) / sigma).collect::<Vec<Vec2>>()
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
	let record = format!("{} {} \"{}\"\n", cluster.centroid.x, cluster.centroid.y, colors[6]);
	file.write_all(record.as_bytes()).unwrap();
    }
}

fn read_km_test() -> Vec<Vec2> {
    // Read data from the csv.
    let path = "data/kmtest.csv";
    let mut reader = csv::ReaderBuilder::new()
	.has_headers(false)
	.flexible(true)
        .delimiter(b' ')
        .from_path(path)
        .unwrap();

    let mut km_test_records: Vec<KMTestRecord> = Vec::new();
    for result in reader.records() {
	let record = result.unwrap();
	let cleaned_record = record
	    .iter()
	    .filter(|x| *x != "")
	    .collect::<StringRecord>();
	let km_test_record: KMTestRecord = cleaned_record.deserialize(None).unwrap();
	km_test_records.push(km_test_record);
    }

    // Convert the record type to generic Vec2 data.
    km_test_records.iter().map(|r| Vec2::new(r.x, r.y)).collect::<Vec<Vec2>>()
}

fn read_iris() -> Vec<Vec2> {
    let path = "data/iris.csv";
    let mut reader = csv::ReaderBuilder::new()
	.has_headers(false)
	.from_path(path)
	.unwrap();

    let mut iris_records = Vec::new();
    for result in reader.deserialize() {
	let record: IrisRecord = result.unwrap();
	iris_records.push(record);
    }
    iris_records.iter().map(|r| Vec2::new(r.c, r.d)).collect::<Vec<Vec2>>()
}

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

    fn sse(&self) -> f32 {
	let mut sse = 0.0;
	for point in self.data.iter() {
	    let distance = point.distance(self.centroid);
	    sse += distance * distance;
	}
	sse
    }
}

#[derive(Debug, Deserialize)]
struct KMTestRecord {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize)]
struct IrisRecord {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: String,
}
