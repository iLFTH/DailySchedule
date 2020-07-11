fn main() {
    let mut prime:[bool;100000] = [false;100000];

    prime[0] = true;
    prime[1] = true;
    let mut i:usize;
    let mut j:usize;
    for i in 0..100000 {
        if prime[i] == false {
            for j in i*2..100000 {
                prime[j]=true;
            }
        }
    }

    let mut N:i32;
    let mut a:usize;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    N = splits[1].parse::<i32>().unwrap();
    let mut M:usize = 4;
    loop {
        if prime[M] == true{
            M = M + 1;
        }
    }
    let table:[i32;4] = [0;4];

    for j in 0..N {
        let mut scan = String::new();
        std::io::stdin().read_line(&mut scan).unwrap();
        let splits: Vec<&str> = scan.split(' ').collect();
        a = splits[1].parse::<usize>().unwrap();
        let k:usize = a % M;
        i = 1;
        loop {
            if i<=M && table[k]!=0 {
                k = (a+i*i) % M;
                i = i + 1;
            }else {
                break;
            }
        }

        if table[k] == 0{
            table[k] = a as i32;
            if  j > 0{
                println!("{}{}"," ",k);
            }else {
                println!("{}{}","",k);
            }
            
            
        }else {
            println!("{}"," ");
        }

        return;

    }

    
}
