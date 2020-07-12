// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)


// error: strict comparison of `f32` or `f64`
//   --> clippy1.rs:14:8
//    |
// 14 |     if y != x {
//    |        ^^^^^^ help: consider comparing them within some error: `(y - x).abs() > error`


fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    // if y != x {
    //     println!("Success!");
    // }

    if (x - y).abs() < f64::EPSILON{ //https://en.wikipedia.org/wiki/Machine_epsilon
        println!("Success!");

    }
    
}
