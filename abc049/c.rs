use proconio::input;

fn main() {
    let parts: Vec<&str> = vec!["maerd", "remaerd", "esare", "resare"];

    input! {
        s: String,
    }

    let mut s: String = s.chars().rev().collect();

    while s.len() > 0 {
        let mut find_some = false;

        for p in &parts {
            match s.find(p) {
                Some(pos) => {
                    if pos == 0 {
                        s.replace_range(0..p.len(), "");
                        find_some = true;
                    } else {
                        // found but it is not head pos, ignore
                    }
                }
                None => {
                    // do nothing
                }
            }
        }

        if find_some == false && s.len() > 0 {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
