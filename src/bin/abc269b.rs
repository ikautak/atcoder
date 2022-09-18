use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut a = 0usize;
    let mut b = 0usize;
    let mut c = 0usize;
    let mut d = 0usize;

    for i in 0..10 {
        for j in 0..10 {
            let cn = s[i][j];
            if cn == '#' {
                if a == 0 {
                    a = i + 1;
                }

                if c == 0 {
                    c = j + 1;
                }
            }
        }
    }

    for i in (0..10).rev() {
        for j in (0..10).rev() {
            let cn = s[i][j];
            if cn == '#' {
                if b == 0 {
                    b = i + 1;
                }

                if d == 0 {
                    d = j + 1;
                }
            }
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
