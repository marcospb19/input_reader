# Input reader

This is my personal input parser for competing in `codeforces`.

```rust
// https://www.beecrowd.com.br/judge/pt/problems/view/1611

use input_reader::InputReader;

fn main() {
    let mut reader = InputReader::from_leaked_stdin().unwrap();

    let tests = reader.read::<i32>();

    for _ in 0..tests {
        let [n, c, m] = reader.read_n::<usize, 3>();
        println!("{n} {c} {m}");

        let levels = reader.read_vec::<i32>(m);
        println!("{levels:?}");
    }
}
```

To those who are unaware, in competitive programming contests like ICPC you don't want to waste any time treating your input, programs are defined and samples of `stdin` and expected `stdout` are given, it's a race against time to produce the required algorithm.
