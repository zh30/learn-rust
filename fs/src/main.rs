use std::fs;

fn main() {
    let content = fs::read_to_string("la.txt").expect("不能加载文件");

    let counts = content
        .split("\n\n")
        .map(|chunk| -> usize { chunk.split('\n').map(|row| row.parse().unwrap_or(0)).sum() });

    let mut v = counts.collect::<Vec<_>>();
    v.sort();

    let last_idx = v.len() - 1;

    println!("the highest count is {}", v[last_idx]);
}
