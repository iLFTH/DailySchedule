use std::collections::HashMap;

fn main() {
    let mut map:HashMap<i32, i32> = HashMap::new();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    let n:i32 = splits[0].parse::<i32>().unwrap();

    let mut A:[i32;1000] = [0;1000];

    let mut i:i32 = 0;
    for i in 0..n {
        let mut scan = String::new();
        std::io::stdin().read_line(&mut scan).unwrap();
        let splits: Vec<&str> = scan.split(' ').collect();
        A[i as usize] = splits[0].parse::<i32>().unwrap();
        if map.contains_key(&A[i as usize]) == false {
            map.insert(A[i as usize],1);
        } else {
            map.insert(A[i as usize],2);
        }
    }

    for i in 0..n {
        let value = map.get(&A[i as usize]);
        if  value == Some(&1){
            println!("{}",A[i as usize]);
            return;
        }
       
    }
    print!("None");
    return;

    
}
