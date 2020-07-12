fn main() {
    let mut n:i32;
    let mut current:i32 = 0;
    let mut sumTime:i32 = 0;
    let mut a:i32;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    n = splits[0].parse::<i32>().unwrap();

    loop {
          let mut scan = String::new();
          std::io::stdin().read_line(&mut scan).unwrap();
          let splits: Vec<&str> = scan.split(' ').collect();
          a = splits[0].parse::<i32>().unwrap();
          
          if a>current {
              sumTime =sumTime + 6*(a-current) + 5;
          } else if a < current {
              sumTime = sumTime + 4*(current-a) + 5;
          } else {
              sumTime = sumTime + 5;
          }
          current = a;
          println!("{:?}",sumTime);
    }
   
   
}
