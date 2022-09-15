use proconio::input;
use std::vec::Vec;

fn main() {
    input! {
        n: usize,
        mut t: [[i64; 3]; n]
    }

    let mut turns: Vec<Vec<i64>> = vec![vec![0, 0, 0]];
    turns.append(&mut t);

    for i in 1..n + 1 {
        let prev = &turns[i - 1];
        let current = &turns[i];

        let time = current[0] - prev[0];
        let x_move = (current[1] - prev[1]).abs();
        let y_move = (current[2] - prev[2]).abs();
        let total_move = x_move + y_move;

        if time < total_move {
            println!("No");
            return;
        }

        if (time & 0x1) != (total_move & 0x1) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
