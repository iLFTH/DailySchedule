fn main()  {
    let mut data:[char;10000] = ['0';10000];
    let mut Next:[i32;10000] = [0;10000];
    let mut hashTable:[bool;10000] = [false;10000];




    let mut scan = String::new();
    std::io::stdin().read_line(&mut scan).unwrap();
    let splits: Vec<&str> = scan.split(' ').collect();
    let mut begin1:i32 = splits[0].parse::<i32>().unwrap();
    let mut begin2:i32 = splits[1].parse::<i32>().unwrap();
    let mut N:i32 = splits[2].parse::<i32>().unwrap();

    loop {
        if N != 0{
            let mut a:usize;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let splits: Vec<&str> = input.split(' ').collect();
            a = splits[0].parse::<usize>().unwrap();
            data[a] = splits[1].parse::<char>().unwrap();
            Next[a] = splits[2].parse::<i32>().unwrap();
            N = N - 1;
        } else {
            break ;
        }
    }
    loop {
        if begin1!=-1 {
            hashTable[begin1 as usize]=true;
            begin1=Next[begin1 as usize];
        } else {
            break;
        }
    }
    loop {
        if begin2!=-1 {
            if hashTable[begin2 as usize] == true {
                println!("{:05}",begin2);
                return;
            }
        } else {
            break;
        }
    }
    println!("-1");
    return;

}
