use std::collections::HashSet;

fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let mut set: HashSet<i64> = HashSet::new();
    for i in s.split_whitespace() {
        let i = i.parse().unwrap();
        set.insert(i);
    }

    println!("{}", set.len());
}
