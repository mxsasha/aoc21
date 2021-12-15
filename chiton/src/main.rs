use std::io::{self, Read};

use petgraph::{
    algo::astar,
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};

fn calculate(input: &str, repetitions: u32) -> u32 {
    let mut graph = UnGraph::<u32, ()>::new_undirected();

    let initial_grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut expanded_grid: Vec<Vec<u32>> =
        vec![vec![0; initial_grid[0].len() * 5]; initial_grid.len() * 5];
    for y_repetition in 0..repetitions {
        for x_repetition in 0..repetitions {
            for y in 0..initial_grid.len() {
                for x in 0..initial_grid[0].len() {
                    let mut new_value = initial_grid[y][x] + y_repetition + x_repetition;
                    let new_x = x + x_repetition as usize * initial_grid[0].len();
                    let new_y = y + y_repetition as usize * initial_grid.len();
                    while new_value > 9 {
                        new_value -= 9;
                    }
                    expanded_grid[new_y][new_x] = new_value;
                }
            }
        }
    }

    let graph_grid: Vec<Vec<NodeIndex<u32>>> = expanded_grid
        .iter()
        .map(|row| row.iter().map(|c| graph.add_node(*c)).collect())
        .collect();

    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];
    for row in graph_grid.iter() {
        for column_idx in 0..row.len() - 1 {
            edges.push((row[column_idx], row[column_idx + 1]));
        }
    }
    for row_idx in 0..graph_grid.len() - 1 {
        for column_idx in 0..graph_grid[0].len() {
            edges.push((
                graph_grid[row_idx][column_idx],
                graph_grid[row_idx + 1][column_idx],
            ));
        }
    }
    graph.extend_with_edges(edges);

    let node_map = astar(
        &graph,
        graph_grid[0][0],
        |finish| finish == graph_grid[graph_grid.len() - 1][graph_grid[0].len() - 1],
        |e| *graph.node_weight(e.target()).unwrap(),
        |_| 0,
    );
    let (cost, _vni) = node_map.unwrap();
    cost
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input, 5);
    println!("result: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_calculate() {
        let count = calculate(
            "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
",
            5,
        );
        assert_eq!(count, 315);
    }
}
