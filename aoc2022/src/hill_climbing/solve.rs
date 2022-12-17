use std::fs;
use itertools::Itertools;
use ndarray::{Array, Array2, ArrayBase, ArrayView, Axis, Ix2, OwnedRepr};
use petgraph::graph::{NodeIndex};
use petgraph::algo::{dijkstra};
use petgraph::{Directed, Graph};


pub fn solve_first_star() -> u32 {
    let (graph, start, end) = parse_input("src/hill_climbing/input.txt");

    let node_map = dijkstra(&graph, start, Some(end), |x| *x.weight());

    let shortest_path = *node_map.get(&end).unwrap();

    shortest_path
}


pub fn solve_second_star() -> u32 {
    let (mut graph, start, end) = parse_input("src/hill_climbing/input.txt");

    // reverse edges so we can find all shortest paths from End to all other nodes in the graph
    graph.reverse();

    // find shortest path to all nodes in graph from End
    let node_map = dijkstra(&graph, end, None, |x| *x.weight());

    // filter shortest path results for 'a' starting positions
    let starting_positions = node_map.iter().filter(|&(a, b)| graph[*a] == 'a');

    // sort shortest paths in ascending order
    let mut shortest_path = starting_positions.sorted_by(|&(_,a), &(_,b)| a.cmp(b) );

    *shortest_path.next().unwrap().1
}


fn parse_input(file_path: &str) -> (Graph<char, u32, Directed>, NodeIndex, NodeIndex) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    // construct new directed graph
    let mut graph = Graph::<char, u32>::new();

    let grid = construct_hill_data(contents);

    // construct graph nodes, capture node indexes in order of original grid
    let graph_nodes = grid.map(|x| graph.add_node(*x));

    // find start and end points in graph
    let start = graph.node_indices().find(|&x| graph[x] == 'S').unwrap();
    let end = graph.node_indices().find(|&x| graph[x] == 'E').unwrap();

    // add edges to graph
    construct_edges(&mut graph, grid, &graph_nodes);


    (graph, start, end)
}


fn construct_edges(graph: &mut Graph<char, u32>, grid: ArrayBase<OwnedRepr<char>, Ix2>, graph_nodes: &Array<NodeIndex, Ix2>) {
    let (height, width) = grid.dim();

    // add edges to graph according to climbing rules
    for ((row, col), node) in graph_nodes.indexed_iter() {
        // construct  indices as int so we can filter out negatives
        let indices: Vec<(i32, i32)> = vec![
            (row as i32 - 1, col as i32),
            (row as i32, col as i32 - 1),
            (row as i32, col as i32 + 1),
            (row as i32 + 1, col as i32),
        ];

        // filter out invalid indices
        let neighbours = indices
            .iter()
            .filter(|&(r, c)|
                *r >= 0 && *c >= 0 && *r < height as i32 && *c < width as i32)
            .collect::<Vec<_>>();

        for (neighbour_row, neighbour_col) in neighbours {
            let r = *neighbour_row as usize;
            let c = *neighbour_col as usize;


            let dest = graph_nodes[[r,c]];

            // only add an edge if curr = dest + 1 or dest < curr
            // S has elevation a
            // E has elevation z
            let mut curr_elevation = graph[*node];
            let mut dest_elevation = graph[dest];

            if curr_elevation == 'S' {
                curr_elevation = 'a'
            } else if curr_elevation == 'E' {
                curr_elevation = 'z'
            }

            if dest_elevation == 'E' {
                dest_elevation = 'z'
            } else if dest_elevation == 'S' {
                dest_elevation = 'a'
            }

            if dest_elevation as usize <= curr_elevation as usize + 1 {
                // add edge
                graph.add_edge(*node, dest, 1);
            }

        }
    }
}


fn construct_hill_data(contents: String) -> ArrayBase<OwnedRepr<char>, Ix2> {
    // process input file data into an Array to support construction of graph
    let width = contents.lines().next().unwrap().chars().count();

    let mut grid = Array2::<char>::from_elem((0, width), 'a');

    for line in contents.lines() {
        let grid_row = line.
            chars()
            .collect::<Vec<_>>();

        grid.push(Axis(0), ArrayView::from(&grid_row)).unwrap();
    }
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(481, solve_first_star());
        assert_eq!(480, solve_second_star());
    }

    #[test]
    fn test_shortest_path() {
        let (graph, start, end) = parse_input("src/hill_climbing/test_input.txt");

        let node_map = dijkstra(&graph, start, Some(end), |x| *x.weight());

        let shortest_path = *node_map.get(&end).unwrap();

        assert_eq!(31, shortest_path);
    }
}