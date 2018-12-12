use std::{io, str, time};
use std::io::Read;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let mut edges = Vec::new();
	for line in input.lines() {
		edges.push(line.parse::<Edge>().expect(line));
	}

	let instant1 = time::Instant::now();
	let order = topo_sort(&edges);
	let duration1 = instant1.elapsed();
	println!("Order of assembly is {}. Took {:?}.", order, duration1);

}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Edge {
	finish: char, // from
	before: char, // to
}
impl str::FromStr for Edge {
	type Err = ();
	fn from_str(s: &str) -> Result<Edge, ()> {
		let finish = s.as_bytes()[5] as char;
		let before = s.as_bytes()[36] as char;
		Ok(Edge { finish, before })
	}
}

fn topo_sort(edges: &[Edge]) -> String {
	// Build a list of nodes from the edges
	// For each node keep track of the number of incoming edges
	let mut nodes = HashMap::new();
	for edge in edges {
		nodes.entry(edge.finish).or_insert(0);
		*nodes.entry(edge.before).or_insert(0) += 1;
	}
	// Maintain a list of 'active' nodes with no incoming edges
	// Keep this list of active nodes sorted alphabetically
	let mut active = BinaryHeap::new();
	// Populate the initial state with nodes with no incoming edges
	for (&chr, &value) in &nodes {
		if value == 0 {
			active.push(Reverse(chr));
		}
	}
	// The resulting order of nodes and track the number of removed edges
	let mut result = String::new();
	let mut removed_edges = 0;
	// While we have nodes with no incoming edges
	// Remove the nodes in alphabetical order
	while let Some(Reverse(node)) = active.pop() {
		result.push(node);
		// Remove all edges coming from this node
		for edge in edges {
			if edge.finish == node {
				// Remove the edge and decrement its incoming edge count
				removed_edges += 1;
				let x = nodes.get_mut(&edge.before).unwrap();
				*x -= 1;
				// If the number of incoming edges is zero
				// Add the node to the active nodes
				if *x <= 0 {
					active.push(Reverse(edge.before));
				}
			}
		}
	}
	// Sanity check if the graph is not a DAG
	if removed_edges != edges.len() {
		panic!("Graph is not a DAG!");
	}
	result
}

#[cfg(test)]
static TEST_EDGES: [Edge; 7] = [
	Edge { finish: 'C', before: 'A' },
	Edge { finish: 'C', before: 'F' },
	Edge { finish: 'A', before: 'B' },
	Edge { finish: 'A', before: 'D' },
	Edge { finish: 'B', before: 'E' },
	Edge { finish: 'D', before: 'E' },
	Edge { finish: 'F', before: 'E' },
];

#[test]
fn test_order() {
	assert_eq!("CABDFE", topo_sort(&TEST_EDGES));
}
