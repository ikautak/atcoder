fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let i: u32 = u32::from_str_radix(&s, 2).unwrap();
    println!("{}", i.count_ones());
}
