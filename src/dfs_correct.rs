use std::collections::{HashMap, HashSet};

pub type Vertex = u64;
type Graph = HashMap<Vertex, HashSet<Vertex>>;

/// Constructs a graph from a sequence of parent child pairs
pub fn from_pairs<I>(pairs: I) -> Graph
where
    I: IntoIterator<Item = (Vertex, Vertex)>,
{
    let mut graph = Graph::new();
    for (parent, child) in pairs {
        graph
            .entry(parent)
            .or_insert_with(HashSet::new)
            .insert(child);
    }
    graph
}

#[allow(unused_variables)]
pub trait DfsAction {
    fn initial_visit(&mut self, graph: &Graph, vertex: Vertex) {}
    fn start(&mut self, graph: &Graph, vertex: Vertex) {}
    fn pre_visit(&mut self, graph: &Graph, current: Vertex, next: Vertex) {}
    fn end(&mut self, graph: &Graph, vertex: Vertex) {}
}

pub fn dfs<A>(graph: &Graph, action: &mut A)
where
    A: DfsAction,
{
    fn dfs_visit<A>(graph: &Graph, vertex: Vertex, seen: &mut HashSet<Vertex>, action: &mut A)
    where
        A: DfsAction,
    {
        action.start(graph, vertex);

        seen.insert(vertex);

        for neighbor in graph.get(&vertex).unwrap_or(&HashSet::new()) {
            if !seen.contains(neighbor) {
                action.pre_visit(graph, vertex, *neighbor);
                dfs_visit(graph, *neighbor, seen, action);
            }
        }

        action.end(graph, vertex);
    }

    let mut seen = HashSet::new();

    for vertex in graph.keys() {
        if !seen.contains(vertex) {
            action.initial_visit(graph, *vertex);
            dfs_visit(graph, *vertex, &mut seen, action);
        }
    }
}

#[derive(Debug, Default)]
struct TopologicalSort(Vec<Vertex>);

impl DfsAction for TopologicalSort {
    fn end(&mut self, _: &Graph, vertex: Vertex) {
        self.0.push(vertex);
    }
}

pub fn topological_sort(graph: &Graph) -> Vec<Vertex> {
    let mut topo = TopologicalSort::default();
    dfs(graph, &mut topo);
    topo.0.reverse();
    topo.0
}

type Times = HashMap<Vertex, u64>;

#[derive(Debug, Default)]
struct Timer {
    time: u64,
    starting_times: Times,
    finishing_times: Times,
}

impl DfsAction for Timer {
    fn start(&mut self, _: &Graph, vertex: Vertex) {
        self.time += 1;
        self.starting_times.insert(vertex, self.time);
    }

    fn end(&mut self, _: &Graph, vertex: Vertex) {
        self.time += 1;
        self.finishing_times.insert(vertex, self.time);
    }
}

pub fn times(graph: &Graph) -> (Times, Times) {
    let mut times = Timer::default();
    dfs(graph, &mut times);
    (times.starting_times, times.finishing_times)
}

#[derive(Debug, Default)]
pub struct Parents(HashMap<Vertex, Option<Vertex>>);

impl Parents {
    fn new(graph: &Graph) -> Self {
        let mut parents = HashMap::new();

        for vertex in graph.keys() {
            parents.insert(*vertex, None);
        }

        Parents(parents)
    }
}

impl DfsAction for Parents {
    fn pre_visit(&mut self, _: &Graph, current: Vertex, next: Vertex) {
        self.0.insert(next, Some(current));
    }
}

pub fn parents(graph: &Graph) -> HashMap<Vertex, Option<Vertex>> {
    let mut parents = Parents::new(graph);
    dfs(graph, &mut parents);
    parents.0
}

#[derive(Debug, Default)]
pub struct Forest(Vec<HashSet<Vertex>>);

impl DfsAction for Forest {
    fn initial_visit(&mut self, _: &Graph, _: Vertex) {
        self.0.push(HashSet::new());
    }

    fn start(&mut self, _: &Graph, vertex: Vertex) {
        self.0
            .last_mut()
            .expect("The Forest should not be empty!")
            .insert(vertex);
    }
}

pub fn forest(graph: &Graph) -> Vec<HashSet<Vertex>> {
    let mut forest = Forest::default();
    dfs(graph, &mut forest);
    forest.0
}