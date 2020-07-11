
fn gcd(x:i64,y:i64) -> i64{
    if y != 0 {
        gcd(y, x % y)
    }else {
        x
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    let N = splits[0].parse::<i32>().unwrap();

    let mut a:i64 = 0;
    let mut b:i64 = 1;
    let mut i:i32;
    for i in 0..N {
        let mut scan = String::new();
        std::io::stdin().read_line(&mut scan).unwrap();
        let splits: Vec<&str> = scan.split('/').collect();
        let c:i64 = splits[0].parse::<i64>().unwrap(); //分子
        let d:i64 = splits[1].parse::<i64>().unwrap(); //分母
        let aa:i64 = a*d + b*c;
        let bb:i64 = b*d;
        if aa < 0{
            a =aa/gcd(-aa,bb); 
            b =bb/gcd(-aa,bb); 
        }else {
            a =aa/gcd(aa,bb); 
            b =bb/gcd(aa,bb); 
        }

        
    }

    let x:i64 = a/b;
    let y:i64 = a%b;

    if y == 0 {
        println!("{:?}",x);
    }else {
        if x == 1{
            println!("{:?}",x);
        }
        println!("{:?}/{:?}",y,b);
    }
    return;
}
