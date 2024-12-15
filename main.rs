use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let train_file = "fashion-mnist_train.csv";
    let test_file = "fashion-mnist_test.csv";

    let train_file = read_csv(train_file)?;
    let test_data = read_csv(test_file)?;

    println!("Train Data: {:?}", train_data);
    println!("Test Data: {:?}", test_data);

    let mut graph1 = Graph::<&str, ()>::new_undirected();

    let shirt1 = graph1.add_node("Shirt1");
    let shirt2 = graph1.add_node("Shirt2");
    let pants1 = graph1.add_node("Pants1");

    graph1.add_edge(shirt1, shirt2, ());

    let mut graph2 = Graph::&str, f32>::new_undirected();

    let shoe1 = graph2.add_node("Shoe1");
    let shoe2 = graph2.add_node("Shoe2");
    let shoe3 = graph2.add_node("Shoe3");

    graph2.add_edge(shoe1, shoe2, 0.9);
    graph2.add_edge(shoe1, shoe2, 0.8);

    println!("\nGraph 1 - Category-Based Details;");
    print_graph(&graph1);

    println!("\nGraph 2 - Similarity-Based Details;");
    print_graph(&graph2);

    println!("\nGraph 1 - Category-Based Details;");
    let avg_distance_graph1 = compute_avg_distance(&graph1, None);
    println!("Average Distance: {:.2}", avg_distance_graph1);

    println!("\nGraph 2 - Similarity-Based Details;");
    let avg_distance_graph2 = compute_avg_distance(&graph2, None);
    println!("Average Distance: {:.2}", avg_distance_graph2);

    println!("n\Comparison");
    println!("Graph 1 (Category-Based) Average Distance: {:.2}", avg_distance_graph1);
    println!("Graph 2 (Similarity-Based) Average Distance: {:.2}", avg_distance_graph2);
    
    println!("n\Graph 1 Node Degrees:");
    print_node_degrees(&graph1);

    println!("n\Graph 2 Node Degrees:");
    print_node_degrees(&graph2);

    Ok(())
}

fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, Boxdyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let headers = reader.headers()?.clone();
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut row = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row.insert(header.to_string(), value.string());
        }
        data.push(row);
    }

    Ok(data)
}

fn compute_avg_distance<N, E, F>(graph: &Graph<N, E>, weight_fn: Option<F>) -> f32
where 
    F: Fn(&E) -> f32, 
{
    let mut total_distance = 0.0;
    let mut pair_count = 0;

    let nodes: Vec<Nodeindex> = graph.node_indices().collect();

    for &start_node in &nodes {
        let distances = match &weight_fn {
            Some(f) => dijkstras(graph, start_node, None, |_| 1.0), 
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

    total_distance / pair_count as f32
}

fn print_graph <N, E>(graph: &Graph <N, E>)
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
{
    println!("Graph:");
    for edge in graph.edge_references() {
        println!(
            "Edge from {:?} to {:?} with weight {:?}",
            graph[edge.source()],
            graph[edge.source()],
            edge.weight()
        );
    }
}

fn printer_node_degrees<N, E>(graph: &Graph<N, E>)
where 
    N: std::fmt::Debug, 
{
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        println!("Node {:?} has degree {}", graph[node], degree);
    }
}
