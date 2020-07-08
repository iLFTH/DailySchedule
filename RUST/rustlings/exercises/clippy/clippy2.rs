// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}


// error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement.
//  --> clippy2.rs:9:14
//   |
// 9 |     for x in option {
//   |              ^^^^^^
//   |
//   = note: `#[deny(clippy::for_loop_over_option)]` on by default
//   = help: consider replacing `for x in option` with `if let Some(x) = option`
//   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#for_loop_over_option
