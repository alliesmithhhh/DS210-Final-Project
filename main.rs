use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;

fn main() -> Result<(), Box<dyn Error>> {
    let train_file = "fashion-mnist_train.csv";
    let test_file = "fashion-mnist_test.csv";

    let train_data = read_csv(train_file)?;
    let test_data = read_csv(test_file)?;

    println!("Train Data: {:?}", train_data);
    println!("Test Data: {:?}", test_data);

    // Ensure the graph type is consistent (Undirected)
    let mut graph1 = Graph::<&str, (), Undirected>::new_undirected();
    let mut graph2 = Graph::<&str, f32, Undirected>::new_undirected();

    // Add nodes and edges
    let shirt1 = graph1.add_node("Shirt1");
    let shirt2 = graph1.add_node("Shirt2");
    let pants1 = graph1.add_node("Pants1");
    graph1.add_edge(shirt1, shirt2, ());
    graph1.add_edge(shirt1, pants1, ());

    let shoe1 = graph2.add_node("Shoe1");
    let shoe2 = graph2.add_node("Shoe2");
    let shoe3 = graph2.add_node("Shoe3");
    graph2.add_edge(shoe1, shoe2, 0.9);
    graph2.add_edge(shoe1, shoe3, 0.8);

    println!("\nGraph 1 - Category-Based Details:");
    print_graph(&graph1);

    println!("\nGraph 2 - Similarity-Based Details:");
    print_graph(&graph2);

    // Compute and print average distances
    let avg_distance_graph1 = compute_avg_distance(&graph1, None); 
    println!("\nGraph 1 Average Distance: {:.2}", avg_distance_graph1);

    let avg_distance_graph2 = compute_avg_distance(&graph2, Some(|&weight| weight)); 
    println!("\nGraph 2 Average Distance: {:.2}", avg_distance_graph2);

    // Print node degrees
    println!("\nGraph 1 Node Degrees:");
    printer_node_degrees(&graph1);

    println!("\nGraph 2 Node Degrees:");
    printer_node_degrees(&graph2);

    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let headers = reader.headers()?.clone();
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut row = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row.insert(header.to_string(), value.to_string());
        }
        data.push(row);
    }

    Ok(data)
}

fn compute_avg_distance<N, E, F>(graph: &Graph<N, E, Undirected>, weight_fn: Option<F>) -> f32
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
    F: Fn(&E) -> f32,
{
    if graph.node_count() == 0 {
        println!("Graph is empty, skipping distance calculation.");
        return 0.0;
    }

    let mut total_distance = 0.0;
    let mut pair_count = 0;

    let nodes: Vec<NodeIndex> = graph.node_indices().collect();

    for &start_node in &nodes {
        let distances = match &weight_fn {
            Some(f) => dijkstra(graph, start_node, None, |edge| f(edge.weight())),
            None => dijkstra(graph, start_node, None, |_| 1.0),
        };

        for &end_node in &nodes {
            if start_node != end_node {
                if let Some(&distance) = distances.get(&end_node) {
                    total_distance += distance;
                    pair_count += 1;
                }
            }
        }
    }

    if pair_count == 0 {
        0.0
    } else {
        total_distance / pair_count as f32
    }
}

fn print_graph<N, E>(graph: &Graph<N, E, Undirected>)
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
{
    for edge in graph.edge_references() {
        println!(
            "Edge from {:?} to {:?} with weight {:?}",
            graph[edge.source()],
            graph[edge.target()],
            edge.weight()
        );
    }
}

fn printer_node_degrees<N, E>(graph: &Graph<N, E, Undirected>)
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
{
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        println!("Node {:?} has degree {}", graph[node], degree);
    }
}
