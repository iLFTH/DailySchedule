pub struct Student {
    id:i32,
    grade:[i32;4],
}

impl Student{
    pub fn new() -> Self {
        Student{id:0,grade:[0,0,0,0]}
    }
}


fn cmp(a:Student,b:Student,now:i32) -> bool{
    return a.grade[now as usize]>b.grade[now as usize];
}
    
fn main(){
    let mut course:[char;4]=['A','C','M','E'];  //按照优先级排列 
    let mut Rank:[[i32;10000000];4] = [[0;10000000];4];

    let mut m:usize;
    let mut n:usize;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let splits: Vec<&str> = input.split(' ').collect();
    n = splits[0].to_string().parse::<usize>().unwrap();
    m = splits[1].to_string().parse::<usize>().unwrap();

    let mut stu = Vec::with_capacity(30010);
    for _ in 0..30010 {
        stu.push(Student::new());
    }

    let mut i:usize;
    let mut j:usize;
    for i in 0..n {
        let mut scanline = String::new();
        std::io::stdin().read_line(&mut scanline).unwrap();
        let splits: Vec<&str> = scanline.split(' ').collect();
        stu[i].id = splits[0].to_string().parse::<i32>().unwrap();
        stu[i].grade[1] = splits[1].to_string().parse::<i32>().unwrap();
        stu[i].grade[2] = splits[0].to_string().parse::<i32>().unwrap();
        stu[i].grade[3] = splits[0].to_string().parse::<i32>().unwrap();

        stu[i].grade[0] = stu[i].grade[1]+stu[i].grade[2]+stu[i].grade[3];
    }

    for i in 0..4 {
        Rank[stu[0].id as usize][i]=1;
        for j in 1..n {
            if stu[i].grade[i]==stu[i-1].grade[i]{
                Rank[stu[j].id as usize][i]=Rank[stu[j-1].id as usize][i];
            }else {
                Rank[stu[j].id as usize][i]=j as i32 +1;
            }
        }

    }

    let mut query:i32;
    for i in 0..m {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let splits: Vec<&str> = line.split(' ').collect();
        query = splits[0].to_string().parse::<i32>().unwrap();

        if Rank[query as usize][0]==0 {
            println!("N/A");
        } else {
            let mut k:usize = 0;
            for j in 0..4 {
                if Rank[query as usize][j]<Rank[query as usize][k] {
                    k = j;
                }
            }
            println!("{} {}",Rank[query as usize][k],course[k]);
        }

    }

}
    