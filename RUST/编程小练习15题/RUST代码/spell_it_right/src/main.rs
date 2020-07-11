// input "12345"

fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn main() {
    let mut s:String;
    let temp:[&str;10] = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    
    let b = splits[0].as_bytes();

    let mut sum:i32 = 0;
    let mut i:i32 = 0;
    for i in 0..char_counter(splits[0]) {

        sum = sum + (b[i] as i32 - 48);
    }
    s = sum.to_string();
    let byte = s.as_bytes();
    for i in 0..byte.len() {
        print!("{:?} ",temp[(byte[i] as i32 -48) as usize]);
    }


}
