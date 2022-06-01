use std::collections;
use std::io;
use std::io::BufRead;

extern crate anyhow;

const START_CAVE: &str = "start";
const END_CAVE: &str = "end";

fn read_adj_matrix() -> anyhow::Result<collections::HashMap<String, Vec<String>>> {
    let mut adj_matrix = collections::HashMap::new();

    io::stdin()
        .lock()
        .lines()
        .take_while(|line| {
            if let Ok(line) = &line {
                !line.is_empty()
            } else {
                false
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|line| line.split('-'))
        .map(|caves| caves.map(|cave| cave.to_owned()))
        .map(|mut caves| (caves.next().unwrap(), caves.next().unwrap()))
        .for_each(|(i, j)| {
            adj_matrix
                .entry(i.clone())
                .or_insert(vec![])
                .push(j.clone());
            adj_matrix.entry(j).or_insert(vec![]).push(i)
        });

    Ok(adj_matrix)
}

fn find_all_paths(
    adj_matrix: &collections::HashMap<String, Vec<String>>,
    cave: &str,
    small_visited: &mut collections::HashSet<String>,
    num_paths: &mut usize,
) -> () {
    if cave.chars().all(|c| c.is_lowercase()) {
        small_visited.insert(cave.to_owned());
    }

    if cave == END_CAVE {
        *num_paths += 1;
    } else {
        adj_matrix[cave]
            .iter()
            .map(|adj| adj.as_str())
            .for_each(|adj| {
                if !small_visited.contains(adj) {
                    find_all_paths(adj_matrix, adj, small_visited, num_paths)
                }
            });
    }

    if small_visited.contains(cave) {
        small_visited.remove(cave);
    }
}

fn part_one(adj_matrix: &collections::HashMap<String, Vec<String>>) -> usize {
    let mut small_visited = collections::HashSet::new();
    let mut num_paths = 0;

    find_all_paths(adj_matrix, START_CAVE, &mut small_visited, &mut num_paths);

    num_paths
}

fn main() -> anyhow::Result<()> {
    let adj_matrix = read_adj_matrix()?;

    // println!("{:?}", adj_matrix);

    println!("Part one: {}", part_one(&adj_matrix));

    Ok(())
}
