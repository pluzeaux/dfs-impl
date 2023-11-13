use dfs_implementation::dfs_correct::{
    from_pairs,
    topological_sort,
    times, parents,
    forest,
    Vertex
};

fn main() {
    let pairs: Vec<(Vertex, Vertex)> = [
        (4, 1),
        (6, 3),
        (8, 6),
        (9, 8),
        (9, 7),
        (7, 5)
    ].into_iter().collect();
    let g = from_pairs(pairs.into_iter());
    
    println!("{:?}", g);
    println!("Topological sort: {:?}", topological_sort(&g));
    println!("Parents: {:?}", parents(&g));
    println!("Times: {:?}", times(&g));
    println!("Forest: {:?}", forest(&g));
}