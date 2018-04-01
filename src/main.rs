extern crate petgraph;
pub use matrices::*;

use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;
use petgraph::visit::Data;
use petgraph::visit::GraphBase;
use petgraph::visit::NodeCount;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::EdgeRef;
use petgraph::visit::GraphProp;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut string = String::new();
    let mut string2 = String::new();
    let currentDatetime = "2017-11-01T09:41:22+00:00";
    let mut graph = Graph::<String, f32>::new();
    let mut v = Vec::new();
    let mut i = 0;

    for line in stdin.lock().lines() {
        input = line.unwrap();
        println!("{}", input);
        let updateVec: Vec<&str> = input.split_whitespace().collect();
        let b = isLatestDateTime(updateVec[0].to_string(), currentDatetime.to_string());
        if b {
            string = [updateVec[1], updateVec[2]].join(" ");
            string2 = [updateVec[1], updateVec[3]].join(" ");
            if i == 0 {
                println!("first update");
                v.push(graph.add_node(string));
                v.push(graph.add_node(string2));
                let forwardFactor: f32 = updateVec[4].parse().unwrap();
                let backwardFactor: f32 = updateVec[5].parse().unwrap();
                graph.add_edge(v[i], v[i+1], forwardFactor);
                graph.add_edge(v[i+1], v[i], backwardFactor);
                println!("{}",forwardFactor);
                println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
                i+= 2;
            } else {
                println!("else");
                for z in 0..i {
                    if &string == &graph[v[z]]{
                        println!("this is already here we need to update: {}", &graph[v[z]]);

                    }
                    if &string2 == &graph[v[z]]{
                        println!("this is already here we need to update: {}", &graph[v[z]]);
                    }
                }
                println!("we need to create a new node and edges if the edges match existin we need 1 to 1 edges to there also");
            }
            println!("end of string processing");
        }
    }
}



fn isLatestDateTime(newDateTime: String, currentDateTime: String) -> bool {

    let newDateTimeVec: Vec<&str> = newDateTime.split(&['-', 'T', ':', '+'][..]).collect();
    let currentDateTimeVec: Vec<&str> = currentDateTime.split(&['-', 'T', ':', '+'][..]).collect();
    println!("I need to be updating the current time on each pair");
    for i in 0..7 {
        let new = newDateTimeVec[i];
        let newInt: i32 = new.parse().unwrap();
        let current = currentDateTimeVec[i];
        let currentInt: i32 = current.parse().unwrap();
        println!("current: {}", currentInt);
        println!("new: {}", newInt);
        if newInt > currentInt {
            return true;
        }
    }
    println!("false");
    return false;

}

