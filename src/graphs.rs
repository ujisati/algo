use std::collections::VecDeque;

type WeightedAdjacenyMatrix = Vec<Vec<f64>>;
type AdjacencyList = Vec<Vec<Node>>;

#[derive(Copy, Clone)]
pub struct Node {
    pub to: usize,
    pub weight: f64,
}

fn breadth_first_search(matrix: WeightedAdjacenyMatrix, source: i64, target: i64) -> Vec<i64> {
    let mut seen = vec![false; matrix.len()];
    let mut prev = vec![-1; matrix.len()];

    let mut queue = VecDeque::new();
    seen[source as usize] = true;
    queue.push_front(source);

    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();
        if curr == target as i64 {
            break;
        }
        seen[curr as usize] = true;
        for (i, num) in matrix[curr as usize].iter().enumerate() {
            if num == &0. {
                continue;
            }
            if seen[i] == true {
                continue;
            }
            seen[i] = true;
            prev[i] = curr;
            queue.push_back(i as i64);
        }
    }

    let mut path = Vec::new();
    let mut tmp = target;
    while tmp != -1 {
        path.push(tmp);
        tmp = prev[tmp as usize];
    }
    path
}

pub fn depth_first_search(
    g: &AdjacencyList,
    curr: usize,
    target: usize,
    seen: &mut Vec<bool>,
    path: &mut Vec<usize>,
) -> bool {
    // pre-curse
    if seen[curr] {
        return false;
    }
    seen[curr] = true;

    path.push(curr);
    if curr == target {
        return true;
    }

    // recurse
    let edges = &g[curr];
    for edge in edges {
        return depth_first_search(g, edge.to, target, seen, path);
    }

    // post-curse
    path.pop();
    false
}

// TODO: use a min heap for sorting of shortest path
pub fn djikstras_shortest_path(g: &AdjacencyList, source: usize, target: usize) -> Vec<usize> {
    let mut seen = vec![false; g.len()];
    let mut dists = vec![f64::INFINITY; g.len()];
    let mut prev = vec![-1 as i64; g.len()];
    dists[source] = 0.;

    while has_unvisited(&seen, &dists) {
        let curr = get_lowest_unvisited(&seen, &dists);
        seen[curr] = true;

        let adjs = &g[curr];
        for edge in adjs {
            if seen[edge.to] {
                continue;
            }
            let dist = dists[curr] + edge.weight;
            if dist < dists[edge.to] {
                dists[edge.to] = dist;
                prev[edge.to] = curr as i64;
            }
        }
    }

    let mut out = Vec::new();
    let mut curr = target as i64;
    while prev[curr as usize] != -1 {
        out.push(curr as usize);
        curr = prev[curr as usize];
    }
    out.push(source);
    out.reverse();
    return out;
}

pub fn has_unvisited(seen: &Vec<bool>, dists: &Vec<f64>) -> bool {
    let mut has_unvisited = false;
    for (i, e) in seen.iter().enumerate() {
        if !e && dists[i] < f64::INFINITY {
            has_unvisited = true;
            break;
        }
    }
    return has_unvisited;
}

pub fn get_lowest_unvisited(seen: &Vec<bool>, dists: &Vec<f64>) -> usize {
    let mut lowest_idx = -1 as i64;
    let mut lowest_distance = f64::INFINITY;

    for i in 0..seen.len() {
        if seen[i] {
            continue;
        }
        if lowest_distance > dists[i] {
            lowest_distance = dists[i]
        }
        lowest_idx = i as i64;
    }

    return lowest_idx as usize;
}

#[cfg(test)]
mod tests {
    use super::{breadth_first_search, depth_first_search, Node};

    #[test]
    fn test_bfs() {
        let mut m = vec![vec![0.; 4]; 4];
        m[0][1] = 1.;
        m[0][2] = 1.;
        m[0][3] = 1.;
        m[1][2] = 1.;
        m[2][3] = 1.;
        let path = breadth_first_search(m, 0, 3);
        assert_eq!(vec![3, 0], path)
    }

    #[test]
    fn test_dfs() {
        let g = vec![
            vec![Node { to: 1, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 4, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 3, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 3, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 5, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 0, weight: 0. }, Node { to: 0, weight: 0. }],
            vec![Node { to: 0, weight: 0. }, Node { to: 0, weight: 0. }],
        ];
        let mut seen = vec![false; g.len()];
        let mut path: Vec<usize> = Vec::new();
        depth_first_search(&g, 0, 5, &mut seen, &mut path);
        assert_eq!(path, vec![0, 1, 4, 5]);
    }
}
