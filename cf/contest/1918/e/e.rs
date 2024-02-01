/* Generated by powerful Codeforces Tool
 * You can download the binary file in here https://github.com/xalanq/cf-tool (Windows, macOS, Linux)
 * Author: maijing
 * Time: 2024-01-31 20:43:53
**/
use std::io::{self, BufRead, Write};

#[allow(dead_code)]
struct Random {
    state: usize,
}

impl Random {
    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn main() {
    let t = read_line().trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}

fn query(i: usize) -> char {
    let mut stdout = io::stdout();
    println!("? {}", i + 1);
    stdout.flush().unwrap();
    read_line().trim().parse().unwrap()
}

fn solve() {
    let n = read_line().trim().parse().unwrap();
    let a: Vec<usize> = (0..n).collect();
    let mut rnd = Random::new(0xdeadbeef);
    let ord = quicksort(&a, &mut rnd);
    let mut ans: Vec<usize> = vec![0; n];
    for i in 0..n {
        ans[ord[i]] = i + 1;
    }
    print!("! ");
    for i in 0..n {
        print!("{} ", ans[i]);
    }
    println!("");
}

fn quicksort(a: &Vec<usize>, rnd: &mut Random) -> Vec<usize> {
    if a.len() == 0 {
        return vec![];
    }
    let mid = rnd.next_in_range(0, a.len());

    while query(a[mid]) != '=' {}
    let mut l: Vec<usize> = vec![];
    let mut r: Vec<usize> = vec![];
    for i in 0..a.len() {
        if i == mid {
            continue;
        }
        let res = query(a[i]);
        if res == '<' {
            l.push(a[i]);
        } else {
            r.push(a[i]);
        }
        query(a[mid]);
    }
    let ord_l = quicksort(&l, rnd);
    let ord_r = quicksort(&r, rnd);
    let mut ord = vec![];
    ord.extend(ord_l);
    ord.push(a[mid]);
    ord.extend(ord_r);
    ord
}