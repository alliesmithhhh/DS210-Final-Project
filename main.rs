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

    println!
    
