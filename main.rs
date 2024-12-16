use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;
use petgraph::algo::{connected_components, dijkstra};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use rand::seq::SliceRandom;
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
            let node1 = nodes[0];
            let node2 = nodes[1];
            
            let node1_idx = *graph.node_indices().find(|&n| n.index() == node1).get_or_insert_with(|| graph.add_node(()));
            let node2_idx = *graph.node_indices().find(|&n| n.index() == node2).get_or_insert_with(|| graph.add_node(()));
            
            graph.add_edge(node1_idx, node2_idx, ());
        }
    }

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
                }
            }
        }

        all_distances.extend(node_distances.values().cloned());
    }

    all_distances
}

fn remove_high_degree_nodes(graph: &mut DiGraph<(), ()>, degree_threshold: usize) {
    let nodes_to_remove: Vec<_> = graph
        .node_indices()
        .filter(|&node| graph.edges(node).count() > degree_threshold)
        .collect();

    for node in nodes_to_remove {
        graph.remove_node(node);
    }
}

fn graph_diameter(graph: &DiGraph<(), ()>) -> usize {
    let mut max_distance = 0;
    for node in graph.node_indices() {
        let distances = dijkstra(graph, node, None, |_| 1);
        if let Some(&max) = distances.values().max() {
            max_distance = max_distance.max(max);
        }
    }
    max_distance
}

fn extract_ego_network(graph: &DiGraph<(), ()>, center: NodeIndex, depth: usize) -> DiGraph<(), ()> {
    let mut subgraph = DiGraph::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((center, 0));
    visited.insert(center);

    while let Some((current, current_depth)) = queue.pop_front() {
        if current_depth < depth {
            for neighbor in graph.neighbors(current) {
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, current_depth + 1));
                    subgraph.add_edge(current, neighbor, ());
                }
            }
        }
    }

    subgraph
}

fn random_subgraph(graph: &DiGraph<(), ()>, sample_size: usize) -> DiGraph<(), ()> {
    let mut rng = rand::thread_rng();
    let sampled_nodes: Vec<_> = graph
        .node_indices()
        .collect::<Vec<_>>()
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect();

    graph.filter_map(
        |node, _| if sampled_nodes.contains(&node) { Some(()) } else { None },
        |_, edge| Some(()), 
    )        
}

fn plot_histogram(data: &[usize], output_file: &str, title: &str) {
    let min_value = *data.iter().min().unwrap_or(&0);
    let max_value = *data.iter().max().unwrap_or(&1);
    let histogram = (min_value..=max_value)
        .map(|bin| {
            let count = data.iter().filter(|&&x| x == bin).count();
            (bin, count)
        })
        .collect::<Vec<_>>();

    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_count = histogram.iter().map(|&(_, count)| count).max().unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_value..max_value, 0..max_count)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            histogram.iter().map(|&(bin, count)| {
                Rectangle::new(
                    [(bin, 0), (bin + 1, count)],
                    RED.filled(),
                )
            }),
        )
        .unwrap();

    root.present().unwrap();
    println!("Histogram saved to {}", output_file);
}

fn main() {
    let twitter_file_path = "/Users/alliesmith/ds210/finalproject/twitter_combined.txt";
    let mut twitter_graph = load_graph(twitter_file_path);

    remove_high_degree_nodes(&mut twitter_graph, 100); 
    let twitter_distances = compute_distances(&twitter_graph);
    
    let diameter = graph_diameter(&twitter_graph);
    println!("Twitter Graph Diameter: {}", diameter);

    let facebook_file_path = "/Users/alliesmith/ds210/finalproject/facebook_combined.txt";
    let mut facebook_graph = load_graph(facebook_file_path);

    remove_high_degree_nodes(&mut facebook_graph, 100);
    let facebook_distances = compute_distances(&facebook_graph);
    
    plot_histogram(
        &twitter_distances,
        "/Users/alliesmith/ds210/finalproject/twitter_histogram.png",
        "Twitter Distance Distribution",
    );
    
    let diameter = graph_diameter(&facebook_graph);
    println!("Facebook Graph Diameter: {}", diameter);

    plot_histogram(
        &facebook_distances,
        "/Users/alliesmith/ds210/finalproject/facebook_histogram.png",
        "Facebook Distance Distribution",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_temp_file() -> String {
        let temp_file_path = "/tmp/test_graph.txt";
        let mut file = File::create(temp_file_path).expect("Cannot create temp file");
        writeln!(file, "0 1").expect("Cannot write to temp file");
        writeln!(file, "1 2").expect("Cannot write to temp file");
        writeln!(file, "2 3").expect("Cannot write to temp file");
        writeln!(file, "3 4").expect("Cannot write to temp file");
        temp_file_path.to_string()
    }

    #[test]
    fn test_load_graph() {
        let file_path = create_temp_file();
        let graph = load_graph(&file_path);

        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 4);
        
        assert!(graph.node_indices().any(|n| n.index() == 0));
        assert!(graph.node_indices().any(|n| n.index() == 1));
        assert!(graph.node_indices().any(|n| n.index() == 2));
        assert!(graph.node_indices().any(|n| n.index() == 3));
        assert!(graph.node_indices().any(|n| n.index() == 4));
    }

    #[test]
    fn test_compute_distances() {
        let file_path = create_temp_file();
        let graph = load_graph(&file_path);
        
        let distances = compute_distances(&graph);

        assert!(!distances.is_empty());

        let max_distance = *distances.iter().max().unwrap();
        assert!(max_distance <= 4);
}
