use std::collections::VecDeque;

#[derive(Debug, Clone)]

pub struct Edge {
    src: usize,
    dest: usize,
}

impl Edge {
    fn new(src: usize, dest: usize) -> Self {
        Self { src, dest }
    }
}

pub fn create_graph(graph: &mut Vec<Vec<Edge>>) {
    graph[0].push(Edge::new(0, 1));
    graph[0].push(Edge::new(0, 2));


    graph[1].push(Edge::new(1, 0));
    graph[1].push(Edge::new(1, 3));

    graph[2].push(Edge::new(2, 0));
    graph[2].push(Edge::new(2, 4));

    graph[3].push(Edge::new(3, 1));
    graph[3].push(Edge::new(3, 4));
    graph[3].push(Edge::new(3, 5));

    graph[4].push(Edge::new(4, 2));
    graph[4].push(Edge::new(4, 3));
    graph[4].push(Edge::new(4, 5));

    graph[5].push(Edge::new(5, 3));
    graph[5].push(Edge::new(5, 4));
    graph[5].push(Edge::new(5, 6));
    
    graph[5].push(Edge::new(6, 5));

}

fn bfs(graph: &Vec<Vec<Edge>>, start: usize, V: usize) {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut vis = vec![false; V];

    q.push_back(start);
    vis[start] = true;

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        print!("{} ", curr);
        for edge in &graph[curr] {
            let neighbor = edge.dest;
            if !vis[neighbor] {
                vis[neighbor] = true;
                q.push_back(neighbor);
            }
        }
    }
}

fn dfs(graph: &Vec<Vec<Edge>>, curr: usize, Vis: &mut Vec<bool>) {
    print!("{} ",curr);
    Vis[curr as usize] = true;

    for e in &graph[curr] {
        if !Vis[e.dest] {
            dfs(graph, e.dest, Vis);
        }
    }
}

pub fn test_bfs() {
    let V = 7;
    let mut Vis:Vec<bool> = vec![false; V];
    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); V];
    create_graph(&mut graph);
    bfs(&graph, 0, V);
    println!();
    dfs(&graph, 0, &mut Vis);
    println!();
}
