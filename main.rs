use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

/// Load the graph 
fn load_graph(file_path: &str) -> DiGraph<(), ()> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    let file = File::open(file_path).expect("Cannot open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let nodes: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if nodes.len() == 2 {
            let node1 = *node_map.entry(nodes[0]).or_insert_with(|| graph.add_node(()));
            let node2 = *node_map.entry(nodes[1]).or_insert_with(|| graph.add_node(()));
            graph.add_edge(node1, node2, ());
        }
    }

    graph
}

/// Compute all pairs shortest path lengths
fn compute_all_pair_shortest_paths(graph: &DiGraph<(), ()>) -> HashMap<NodeIndex, HashMap<NodeIndex, usize>> {
    let mut all_distances = HashMap::new();

    for node in graph.node_indices() {
        let mut node_distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);
        node_distances.insert(node, 0); // Distance to itself is 0

        while let Some(current) = queue.pop_front() {
            let current_distance = node_distances[&current];

            for neighbor in graph.neighbors(current) {
                if !node_distances.contains_key(&neighbor) {
                    node_distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        all_distances.insert(node, node_distances);
    }

    all_distances
}

/// Calculate the average distance between nodes
fn average_distance(distances: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>) -> f64 {
    let mut total_distance = 0;
    let mut num_pairs = 0;

    for (_, targets) in distances {
        for (_, &dist) in targets {
            if dist > 0 { 
                total_distance += dist;
                num_pairs += 1;
            }
        }
    }

    if num_pairs == 0 {
        0.0 // Avoid division by zero
    } else {
        total_distance as f64 / num_pairs as f64
    }
}

fn main() {
    let file_path = "/Users/alliesmith/ds210/finalproject/twitter_combined.txt";
    let graph = load_graph(file_path);
    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());

    let distances = compute_all_pair_shortest_paths(&graph);
    let avg_distance = average_distance(&distances);
    println!("Average distance: {}", avg_distance);

    let file_path = "/Users/alliesmith/ds210/finalproject/facebook_combined.txt";
    let graph = load_graph(file_path);
    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());

    let distances = compute_all_pair_shortest_paths(&graph);
    let avg_distance = average_distance(&distances);
    println!("Average distance: {}", avg_distance);
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        let edge_list = "0 1\n1 2\n2 3\n3 0\n";
        let file_path = "test_graph.txt";
        std::fs::write(file_path, edge_list).expect("Failed to write test file");

        let graph = load_graph(file_path);
        std::fs::remove_file(file_path).expect("Failed to delete test file");

        assert_eq!(graph.node_count(), 4, "Graph should have 4 nodes");
        assert_eq!(graph.edge_count(), 4, "Graph should have 4 edges");
    }

    #[test]
    fn test_compute_all_pair_shortest_paths() {
        let mut graph = DiGraph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());

        graph.add_edge(n0, n1, ());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n0, ());

        let distances = compute_all_pair_shortest_paths(&graph);

        assert_eq!(distances[&n0][&n2], 2, "Shortest path from n0 to n2 should be 2");
        assert_eq!(distances[&n1][&n3], 2, "Shortest path from n1 to n3 should be 2");
    }
   
    #[test]
    fn test_average_distance() {
        let mut graph = DiGraph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());

        graph.add_edge(n0, n1, ());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n0, ());

        let distances = compute_all_pair_shortest_paths(&graph);
        let avg_distance = average_distance(&distances);

        assert!((avg_distance - 1.5).abs() < 1e-6, "Average distance should be approximately 1.5");
    }
}
