pub fn day_one(contents: &String) {
    let lines_iter = contents.split("\n").filter(|x| !x.is_empty());
    let mut ans: u32 = 0;
    for line in lines_iter {
        let mut num_map = line.chars().filter_map(|c| c.to_digit(10));
        let first = num_map.next().unwrap();
        let last = num_map.next_back().unwrap_or(first);
        ans += first * 10 + last;
    }
    print!("Total is {}", ans);
}
