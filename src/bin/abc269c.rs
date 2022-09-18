use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut find_list = vec![0u64];

    for i in 0..64 {
        let a = 0x1 << i;

        if n & a != 0 {
            let current_list = find_list.clone();
            for l in current_list {
                find_list.push(l | a);
            }
        }
    }

    for l in find_list {
        println!("{}", l);
    }
}
