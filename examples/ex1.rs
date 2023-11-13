use dfs_implementation::dfs_init::{
    from_pairs,
    dfs,
    topological_sort
};

fn main() {
    // let pairs = [(1, 2), (2, 1)];
    let pairs = [
        (4, 1),
        (6, 3),
        (8, 6),
        (9, 8),
        (9, 7),
        (7, 5)
    ];
    let g = from_pairs(&pairs);
    println!("{:?}", g);
    println!("{:?}", dfs(&g));
    println!("{:?}", topological_sort(&g));
}