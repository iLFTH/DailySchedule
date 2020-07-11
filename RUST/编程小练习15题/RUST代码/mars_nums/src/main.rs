use std::collections::HashMap;

fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count() -2
}


fn main() {
    let low:[&str;13] = ["tret", "jan", "feb", "mar", "apr", "may", "jun",
                        "jly",  "aug", "sep", "oct", "nov", "dec"];
    let high:[&str;13] =["tret", "tam", "hel", "maa", "huh", "tou", "kes",
                        "hei",  "elo", "syy", "lok", "mer", "jou"];                  
    let mut temp:HashMap<&str, usize> = HashMap::new();
    let mut i:usize;
    for i in 0..13 {
        temp.insert(low[i], i);
        temp.insert(high[i], i*13);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    let n = splits[0].parse::<i32>().unwrap();

    loop {
        if n != 0{
            let mut scan = String::new();
            std::io::stdin().read_line(&mut scan).unwrap();
            let splits: Vec<&str> = scan.split(' ').collect();
            let s = splits[0].as_bytes();
            if s[0] >= 48 && s[0] <=57{
                let k = splits[0].parse::<usize>().unwrap();
                if k/13 != 0{
                    print!("{}",high[k/13]);
                }
                if k/13 != 0 && k % 13 != 0{
                    print!(" ");
                }
                if k / 13 == 0 || k % 13 != 0{
                    print!("{}",low[k % 13]);
                }
                println!("");
            }else{
                let k:usize = 0;
                

            }

        }else{
            break;
        }
    }

}