use std::io::{self, Read};

use petgraph::{
    algo::astar,
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};

fn calculate(input: &str) -> u32 {
    let mut graph = UnGraph::<u32, ()>::new_undirected();

    let grid: Vec<Vec<NodeIndex<u32>>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| graph.add_node(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];
    for row in grid.iter() {
        for column_idx in 0..row.len() - 1 {
            edges.push((row[column_idx], row[column_idx + 1]));
        }
    }
    for row_idx in 0..grid.len() - 1 {
        for column_idx in 0..grid[0].len() {
            edges.push((grid[row_idx][column_idx], grid[row_idx + 1][column_idx]));
        }
    }
    graph.extend_with_edges(edges);

    let node_map = astar(
        &graph,
        grid[0][0],
        |finish| finish == grid[grid.len() - 1][grid[0].len() - 1],
        |e| *graph.node_weight(e.target()).unwrap(),
        |_| 0,
    );
    let (cost, _vni) = node_map.unwrap();
    cost
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input);
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
        );
        assert_eq!(count, 40);
    }
}
