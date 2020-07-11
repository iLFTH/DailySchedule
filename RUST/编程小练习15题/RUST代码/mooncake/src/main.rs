fn main() {


    let mut profit = 0.0;
    let mut price:[f64;1000] = [0.0;1000];
    let mut stock:[f64;1000] = [0.0;1000];
    let mut i:usize;
    let mut max:usize;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    let n = splits[0].parse::<usize>().unwrap();
    let mut d = splits[1].parse::<f64>().unwrap();

    for i in 0..n {
        let mut scan = String::new();
        std::io::stdin().read_line(&mut scan).unwrap();
        let splits: Vec<&str> = scan.split(' ').collect();
        stock[i] = splits[0].parse::<f64>().unwrap();
    }

    for i in 0..n {
        let mut scan = String::new();
        std::io::stdin().read_line(&mut scan).unwrap();
        let splits: Vec<&str> = scan.split(' ').collect();
        price[i] = splits[0].parse::<f64>().unwrap();
    }


    loop {
        if d > 0.0 {
            max = 0;
            for i in 0..n {
                if price[i] / stock[i] > price[max] / stock[max]{
                    max = i;
                }
            }
            if  d > stock[max] {
                profit = profit + price[max];
                d =  d - stock[max];
                price[max] = 0.0;
            }else {
                profit = profit + d * price[max] / stock[max];
                d = 0.0;
            }



        }else{
            break;
        }
    }

    println!("{:.02}",profit);
    return;
}
