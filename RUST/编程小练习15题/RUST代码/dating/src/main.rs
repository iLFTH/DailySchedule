use std::collections::HashMap;

fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count() -2
}

fn min(a:usize,b:usize)-> usize{
    if a <= b {
        a
    } else {
        b
    }
}

fn main() {
    // let  week:HashMap<char, String> = HashMap::new();
    // week.insert('A',"MON".to_string());
    // week.insert('B',"TUE".to_string());
    // week.insert('C',"WED".to_string());
    // week.insert('D',"THU".to_string());
    // week.insert('E',"FRI".to_string());
    // week.insert('F',"SAT".to_string());
    // week.insert('G',"SUN".to_string());

    // let mut hour:HashMap<char, i32> = HashMap::new();
    // let mut i:i32 = 0 ;
    // for i in 0..24 {
    //     if i < 10 {
    //         hour.insert(i+)
    //     } 
    // }
    let day:[&str;7] = ["MON ","TUE ","WED ","THU ","FRI ","SAT ","SUN "];

    let mut a1 = String::new();
    let mut a2 = String::new();
    let mut a3 = String::new();
    let mut a4 = String::new();
    std::io::stdin().read_line(&mut a1).unwrap();
    std::io::stdin().read_line(&mut a2).unwrap();
    std::io::stdin().read_line(&mut a3).unwrap();
    std::io::stdin().read_line(&mut a4).unwrap();
    let s1 = a1.as_bytes();
    let s2 = a2.as_bytes();
    let s3 = a3.as_bytes();
    let s4 = a4.as_bytes();
    let l1:usize = char_counter(a1.as_str());
    let l2:usize = char_counter(a2.as_str());
    let l3:usize = char_counter(a3.as_str());
    let l4:usize = char_counter(a4.as_str());
    // print!("{}{}{}{}",l1,l2,l3,l4);
    let mut count:usize = 0;
    let mut c:u8 = 0;
    let mut a:u8 = 0;
    let mut b:u8 = 0;
    let mut i:usize = 0;
    for i in 0..min(l1,l2) {
        if s1[i] == s2[i] && s1[i] >= 65 && s1[i] <= 71 {
            count = i;
            a = s1[i];
            break;
        }
    }


    for i in count+1..min(l1,l2) {
        if s1[i]==s2[i] &&  (s1[i] >= 65 && s1[i] <= 78) || (s1[i] >= 48 && s1[i] <= 57) {
            b = s1[i];
            break;
        }
    }

    for i in 0..min(l3,l4) {
        if s3[i]==s4[i] && ((s3[i] >= 65  && s3[i] <= 90 )||(s3[i] >= 97&&s3[i] <= 122)) {
            c = i as u8;
            break;
        }
    }
    let index:usize  =  (a - 65) as usize; 
    print!("{}",day[index]);
    let mut m:u8 = b - 38;

    if b < 45 && b > 57 {
        m = b - 65 + 10;
    }
    println!("{:02}:{:02}",m,c);
    return;



}
