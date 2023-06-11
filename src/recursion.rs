pub fn walk_to(
    maze: &Vec<String>,
    to: (usize, usize),
    seen: &mut Vec<(usize, usize)>,
    path: &mut Vec<(usize, usize)>,
) -> bool {
    // 1. Check if we are at a wall
    if maze[to.0].chars().nth(to.1).unwrap() == '#' {
        return false;
    }
    // 2. Check if we are off the map
    if to.0 >= maze.len() || to.0 < 0 || to.1 >= maze[0].len() || to.1 < 0 {
        return false;
    }
    // 3. Check if we have seen this spot before
    if seen.contains(&to) {
        return false;
    }
    // 4. Check if we are at the end
    if maze[to.0].chars().nth(to.1).unwrap() == 'E' {
        path.push(to);
        return true;
    }
    seen.push(to);
    path.push(to);
    let directions: Vec<(i64, i64)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for direction in directions {
        let new_to = (
            (to.0 as i64 + direction.0) as usize,
            (to.1 as i64 + direction.1) as usize,
        );
        if walk_to(maze, new_to, seen, path) {
            return true;
        }
    }
    path.pop();
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_to() {
        let mut maze = vec![
            String::from("########"),
            String::from("#.....E#"),
            String::from("#.######"),
            String::from("#..#...#"),
            String::from("#..#...#"),
            String::from("#..#...#"),
            String::from("#..#...#"),
            String::from("#......#"),
            String::from("########"),
        ];
        let mut seen = Vec::<(usize, usize)>::new();
        let mut path = Vec::<(usize, usize)>::new();
        let result = walk_to(&maze, (7, 6), &mut seen, &mut path);
        assert_eq!(result, true);
        assert_eq!(
            path,
            vec![
                (7, 6),
                (6, 6),
                (5, 6),
                (4, 6),
                (3, 6),
                (3, 5),
                (4, 5),
                (5, 5),
                (6, 5),
                (7, 5),
                (7, 4),
                (7, 3),
                (7, 2),
                (6, 2),
                (5, 2),
                (4, 2),
                (3, 2),
                (3, 1),
                (2, 1),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6)
            ]
        );

        maze = vec![
            String::from("########"),
            String::from("########"),
            String::from("########"),
            String::from("########"),
            String::from("########"),
        ];
        let mut seen = Vec::<(usize, usize)>::new();
        let mut path = Vec::<(usize, usize)>::new();
        let result = walk_to(&maze, (0, 0), &mut seen, &mut path);
        assert_eq!(result, false);
        assert_eq!(path, vec![]);
    }
}
