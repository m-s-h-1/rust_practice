use std::io;
#[derive(Debug)]
struct map {
    index: usize,
    times: usize
}

impl map {
    pub fn new(index:usize, times:usize ) -> Self {
        map {
            index,
            times
        }
    }
}


fn main(){
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u32 = t.trim().parse().expect("Error");
    for i in 1..test_cases+1 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let split1 = input.split_whitespace();
        let vec1 = split1.collect::<Vec<&str>>();
        let no_of_people = vec1[0].parse::<usize>().unwrap();
        let max_amount = vec1[1].parse::<usize>().unwrap();
        let mut times = vec![0; no_of_people];
        let mut queue = String::new();
        io::stdin().read_line(&mut queue).expect("Error");
        let split2 = queue.split_whitespace();
        let vec2 = split2.collect::<Vec<&str>>();
        let mut leave_ans = vec![0; no_of_people];
        let mut mapped_queue;

        for i in 0..no_of_people {
            let a = vec2[i].parse::<usize>().unwrap() / max_amount;
            times[i] = a + 1;
            mapped_queue = vec![
                map::new(i, times[i])
            ];
            if i == no_of_people {
                mapped_queue.sort_by(|a, b| b.times.cmp(&a.times));
                
            }
        }

        println!("");
    }
}
