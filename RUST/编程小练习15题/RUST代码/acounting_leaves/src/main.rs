pub struct Node {
    father:i32,
    flag:i32,
    level:i32,
}

impl Node{
    pub fn new() -> Self {
        Node{father:0,flag:0,level:0}
    }
}

    
fn main(){
    let mut i:i32;
    let mut j:i32;
    let mut num:i32;
    let mut N:usize;
    let mut M:usize;
    let mut ID:usize;
    let mut child:usize;
    let mut maxlevel:i32 = 1;


    let mut node = Vec::with_capacity(205);
    // init
    //cin >> N >> M; C++
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    N = splits[0].to_string().parse::<usize>().unwrap();
    M = splits[1].to_string().parse::<usize>().unwrap();

    for _ in 0..N {
        node.push(Node::new());
    }

    let mut result:[i32;205] = [0;205];

    node[1].level = 1;

    for _ in 1..M {
        let mut input1 = String::new();
        std::io::stdin().read_line(&mut input1).unwrap();
        let splits: Vec<&str> = input1.split(' ').collect();
        ID = splits[0].to_string().parse::<usize>().unwrap();
        num = splits[1].to_string().parse::<i32>().unwrap();

        if num != 0 {
            node[ID].flag = 1;
        }
        for _ in 1..num {
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            let splits: Vec<&str> = input2.split(' ').collect();
            child = splits[0].to_string().parse::<usize>().unwrap();
            node[child].father = ID as i32;
        }
    }
    let mut i:usize;
    let mut j:usize;

    for i in 1..N {
        for j in 1..N {
            if node[j].father == i as i32 {
                node[j].level = node[i].level + 1;
            }
        }
    }

    for i in 1..N {
        if node[i].flag == 0 && node[i].level > 0 {
            result[node[i].level as usize ] = result[node[i].level as usize] + 1;
        }
        if node[i].level > maxlevel {
            maxlevel = node[i].level;
        }
    }

    for i in 1..maxlevel as usize {
        print!("{} ",result[i]);
    }



}
    
    