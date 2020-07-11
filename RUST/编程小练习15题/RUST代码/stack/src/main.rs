use std::collections::LinkedList;


fn update(x:i32, y:i32, v:&mut Vec<i32>) {
    for (int i = x; i < v.size(); i += lowbit(i))
        v[i] += y;
}


fn main() {
    let n:i32;
    let mut a:i32;
    let mut s: LinkedList<i32> = LinkedList::new();
    let mut v:Vec<i32> = Vec::new();

    let mut command:String;
    let mut scan = String::new();
    std::io::stdin().read_line(&mut scan).unwrap();
    let splits: Vec<&str> = scan.split(' ').collect();
    n = splits[0].parse::<i32>().unwrap();

    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let splits: Vec<&str> = input.split(' ').collect();
        command = splits[0].to_string();

        if command == "Push" {
            let mut in_a = String::new();
            std::io::stdin().read_line(&mut in_a).unwrap();
            let splits: Vec<&str> = in_a.split(' ').collect();
            a = splits[0].parse::<i32>().unwrap();
            s.push_back(a);
            update(a,1,&mut v);
            

        }



    }
}
