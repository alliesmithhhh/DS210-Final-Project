use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, Write};
use plotters::prelude::*;

fn load_graph(file_path: &str) -> DiGraph<(), ()> {
    let mut graph = DiGraph::new();
    let file = File::open(file_path).expect("Cannot open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let nodes: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nodes.len() == 2 {
            let node1 = graph.add_node(());
            let node2 = graph.add_node(());
            graph.add_edge(node1, node2, ());
        }
    }

    println!("Graph loaded with {} nodes and {} edges", graph.node_count(), graph.edge_count());
    graph
}

fn compute_distances(graph: &DiGraph<(), ()>) -> Vec<usize> {
    let mut all_distances = Vec::new();

    for node in graph.node_indices() {
        let mut queue = VecDeque::new();
        let mut node_distances = HashMap::new();
        queue.push_back(node);
        node_distances.insert(node, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = *node_distances.get(&current).unwrap();
            for neighbor in graph.neighbors(current) {
                if !node_distances.contains_key(&neighbor) {
                    node_distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                    all_distances.push(current_distance + 1);
                }
            }
        }
    }

    println!("Collected {} distances", all_distances.len());
    all_distances
}

fn save_distances_to_file(distances: &[usize], file_path: &str) {
    let mut file = File::create(file_path).expect("Cannot create file");
    for distance in distances {
        writeln!(file, "{}", distance).expect("Cannot write to file");
    }
    println!("Distances saved to {}", file_path);
}

fn plot_histogram(data: &[usize], output_file: &str, title: &str) {
    if data.is_empty() {
        println!("No data to plot for {}", title);
        return;
    }

    let min_value = *data.iter().filter(|&&x| x > 0).min().unwrap_or(&0);
    let max_value = *data.iter().max().unwrap_or(&1);
    println!("Histogram range: min = {}, max = {}", min_value, max_value);

    let mut histogram = vec![0; (max_value - min_value + 1) as usize];
    for &value in data {
        histogram[(value - min_value) as usize] += 1;
    }

    println!("Histogram data: {:?}", histogram);

    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_value..max_value, 0..*histogram.iter().max().unwrap())
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            histogram.iter().enumerate().map(|(idx, &count)| {
                Rectangle::new(
                    [(min_value + idx, 0), (min_value + idx, count)],
                    RED.filled(),
                )
            }),
        )
        .unwrap();

    root.present().unwrap();
    println!("Histogram saved to {}", output_file);
}

fn main() {
    // Analyze Twitter graph
    let twitter_file_path = "/Users/alliesmith/ds210/finalproject/twitter_combined.txt";
    println!("Loading Twitter graph...");
    let twitter_graph = load_graph(twitter_file_path);

    println!("Calculating distances for Twitter graph...");
    let twitter_distances = compute_distances(&twitter_graph);

    println!("Saving Twitter distances to file...");
    save_distances_to_file(&twitter_distances, "twitter_distances.txt");

    println!("Plotting Twitter histogram...");
    plot_histogram(
        &twitter_distances,
        "twitter_histogram.png",
        "Twitter Distance Distribution",
    );

    // Analyze Facebook graph
    let facebook_file_path = "/Users/alliesmith/ds210/finalproject/facebook_combined.txt";
    println!("\nLoading Facebook graph...");
    let facebook_graph = load_graph(facebook_file_path);

    println!("Calculating distances for Facebook graph...");
    let facebook_distances = compute_distances(&facebook_graph);

    println!("Saving Facebook distances to file...");
    save_distances_to_file(&facebook_distances, "facebook_distances.txt");

    println!("Plotting Facebook histogram...");
    plot_histogram(
        &facebook_distances,
        "facebook_histogram.png",
        "Facebook Distance Distribution",
    );
}
