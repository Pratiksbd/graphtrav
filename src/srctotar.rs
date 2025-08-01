use crate::bfs::Edge;

use crate::bfs::create_graph;

pub fn srctotar() {
    let V = 7;
    let mut Vis: Vec<bool> = vec![false; V];
    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); V];
    create_graph(&mut graph);

    let src = 0;
    let tar = 3;

    print_all_path(&graph, &mut Vis, 0, String::from("0"), tar);
}

pub fn print_all_path(
    graph: &Vec<Vec<Edge>>,
    Vis: &mut Vec<bool>,
    curr: usize,
    path: String,
    tar: usize,
) {
    if curr == tar {
        println!("{}", path);
        return;
    }
    for e in &graph[curr] {
        if !Vis[e.dest] {
            Vis[curr] = true;
            print_all_path(graph, Vis, e.dest, format!("{}{}", path, e.dest), tar);
            Vis[curr] = false;
        }
    }
}
