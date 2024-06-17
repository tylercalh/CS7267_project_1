use glam::*;
use std::fs::File;
use std::io::prelude::*;

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
    // read the data
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

    // initilize clusters

    let mut clusters: Vec<Cluster> = Vec::new();
    const K: usize = 3;
    clusters.push(Cluster::new(Vec2::new(2.5, 2.5)));
    clusters.push(Cluster::new(Vec2::new(5.0, 5.0)));
    clusters.push(Cluster::new(Vec2::new(16.0, 10.0)));

    // stop criteria
    let mut stop = false;
    while !stop {

	// all points are available
	for cluster in clusters.iter_mut() {
	    cluster.data.clear();
	}

	// clusters take closest points
	for point in data.iter() {
	    let mut min_index = 0;
	    for cur_index in 1..clusters.len() {
		let d1 = point.distance(clusters[min_index].centroid);
		let d2 = point.distance(clusters[cur_index].centroid);
		if d2 < d1 {min_index = cur_index;}
	    }

	    clusters[min_index].data.push(point);
	}

	// means are recalculated.
	let mut max_delta = 0.0;
	for cluster in clusters.iter_mut() {
	    let prev_mean = cluster.centroid;
	    let new_mean = cluster.mean();
	    cluster.centroid = new_mean;

	    let delta = (new_mean - prev_mean).length();
	    if delta > max_delta {max_delta = delta;}
	}
	if max_delta < 0.1 {stop = true;}
    }

    let colors = vec!["r", "g", "b", "c", "m", "y", "k"];
    let mut file = File::create("logs/clustered.csv").unwrap();

    for (index, cluster) in clusters.iter().enumerate() {
	for point in cluster.data.iter() {
	    let record = format!("{} {} \"{}\"\n", point.x, point.y, colors[index]);
	    file.write_all(record.as_bytes()).unwrap();
	}
    }
}
