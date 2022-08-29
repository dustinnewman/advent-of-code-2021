use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn is_lower(str: &str) -> bool {
    str.chars().next().unwrap().is_ascii_lowercase()
}

// We can actually use a vec for the `seen` counter because our paths are short
fn dfs<'a>(graph: &Graph<'a>, v: &'a str, seen: &mut Vec<&'a str>, mut twice: bool) -> usize {
    if v == "end" {
        return 1;
    }
    if seen.contains(&v) && is_lower(v) {
        if twice || v == "start" {
            return 0;
        }
        twice = true;
    }
    seen.push(v);
    let num_paths = graph[v].iter().map(|child| dfs(graph, child, seen, twice)).sum();
    seen.pop();
    num_paths
}

fn main() {
    // We could do a clever thing here since we do not have that many vertices
    // and use bitstrings as vertex names but that is more confusing. Perhaps
    // another time if I really want to code get_key() and visit() functions.
    let mut graph = HashMap::new();
    include_str!("input.txt")
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(s, d)| {
            graph.entry(s).or_insert(vec![]).push(d);
            graph.entry(d).or_insert(vec![]).push(s);
        });
    println!("Part 1: {}", dfs(&graph, "start", &mut vec![], true));
    println!("Part 2: {}", dfs(&graph, "start", &mut vec![], false));
}
