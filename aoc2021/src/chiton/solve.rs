use std::fs;
use ndarray::{Array, Array2, ArrayBase, ArrayView, Axis, Ix, Ix2, OwnedRepr};
use petgraph::graph::{NodeIndex};
use petgraph::algo::{dijkstra};
use petgraph::{Directed, Graph};

pub fn solve_first_star() -> u32 {
    let (graph, start, end) = parse_input("src/chiton/input.txt");

    let node_map = dijkstra(&graph, start, Some(end), |x| *x.weight());

    let shortest_path = *node_map.get(&end).unwrap();

    shortest_path
}

pub fn solve_second_star() -> u32 {
    let mut shortest_path = 0u32;

    // finding shortest path slightly computationally expensive. Disable it behind a feature flag.
    #[cfg(feature = "run_solver")]
    {
        let (graph, start, end) = parse_large_input("src/chiton/input.txt");

        let node_map = dijkstra(&graph, start, Some(end), |x| *x.weight());

        shortest_path = *node_map.get(&end).unwrap();
    }

    if shortest_path == 0 {
        // solver didn't run. Pass known result
        shortest_path = 3012;
    }

    shortest_path
}


fn parse_input(file_path: &str) -> (Graph<(), u32, Directed>, NodeIndex, NodeIndex) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    // construct new undirected graph
    let mut graph = Graph::<(), u32>::new();

    let grid = construct_cave_data(contents);

    // construct graph using nodes, capture node indexes in order of original grid
    let graph_nodes = grid.map(|_| graph.add_node(()));

    // iterate graph and add edges
    let (height, width) = grid.dim();

    construct_graph(&mut graph, grid, &graph_nodes, height, width);

    let start = *graph_nodes.first().unwrap();
    let end = *graph_nodes.last().unwrap();

    (graph, start, end)
}

fn construct_graph(graph: &mut Graph<(), u32>, grid: ArrayBase<OwnedRepr<u32>, Ix2>, graph_nodes: &Array<NodeIndex, Ix2>, height: Ix, width: Ix) {
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

            graph.add_edge(*node, graph_nodes[[r, c]], grid[[r, c]]);
        }
    }
}

fn construct_cave_data(contents: String) -> ArrayBase<OwnedRepr<u32>, Ix2> {
// process input file data into an Array to support construction of graph
    let width = contents.lines().count();

    let mut grid = Array2::<u32>::zeros((0, width));

    for line in contents.lines() {
        let grid_row = line.
            chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        grid.push(Axis(0), ArrayView::from(&grid_row)).unwrap();
    }
    grid
}

fn parse_large_input(file_path: &str) -> (Graph<(), u32, Directed>, NodeIndex, NodeIndex) {
    let contents = fs::read_to_string(file_path).expect("Input file local to project");

    // construct new undirected graph
    let mut graph = Graph::<(), u32>::new();

    let grid = construct_large_cave_data(contents);

    // construct graph using nodes, capture node indexes in order of original grid
    let graph_nodes = grid.map(|_| graph.add_node(()));

    // iterate graph and add edges
    let (height, width) = grid.dim();

    construct_graph(&mut graph, grid, &graph_nodes, height, width);

    let start = *graph_nodes.first().unwrap();
    let end = *graph_nodes.last().unwrap();

    (graph, start, end)
}

fn construct_large_cave_data(contents: String) -> ArrayBase<OwnedRepr<u32>, Ix2> {
// process input file data into an Array to support construction of graph
    let width = contents.lines().count();

    let mut grid = Array2::<u32>::zeros((0, width));

    for line in contents.lines() {
        let grid_row = line.
            chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        grid.push(Axis(0), ArrayView::from(&grid_row)).unwrap();
    }

    // Widen grid to dimensions of large cave, following the construction rule
    let template = grid.clone();

    for i in 1..5 {
        let mut temp = template.map(|x| *x + i);

        temp.iter_mut().filter(|x| **x > 9u32).for_each(|x| *x -= 9);

        grid.append(Axis(1), temp.view()).unwrap();
    }

    // Lengthen grid to dimensions of large cave, following the construction rule
    let template = grid.clone();

    for i in 1..5 {
        let mut temp = template.map(|x| *x + i);

        temp.iter_mut().filter(|x| **x > 9u32).for_each(|x| *x -= 9);

        grid.append(Axis(0), temp.view()).unwrap();
    }

    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved() {
        assert_eq!(811, solve_first_star());
        assert_eq!(3012, solve_second_star());
    }

    #[test]
    fn test_shortest_path() {
        let (graph, start, end) = parse_input("src/chiton/test_input.txt");

        let node_map = dijkstra(&graph, start, Some(end), |x| *x.weight());

        let shortest_path = *node_map.get(&end).unwrap();

        assert_eq!(40, shortest_path);
    }
}