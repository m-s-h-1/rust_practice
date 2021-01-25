use std::io;

fn main(){
    let mut t = String::with_capacity(10001);
    io::stdin().read_line(&mut t).expect("Error");
    let test_cases:u8 = t.trim().parse().expect("Some error");
    let mut i = 1;
    
    while i <= test_cases {
        let mut lucky = 0;
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Something!");
        let s = String::from(input);
        for index in 0..s.len()-1-4 {
            let check_kick = &s[index..index+4];
            if check_kick == "KICK" {
                for ind in index+3..s.len()-5 {
                    let check_start = &s[ind..ind+5];
                    if check_start == "START" {
                        lucky +=1;
                    }
                }
            }
        }
        println!("Case #{}: {}", i, lucky);
        i +=1;
    }
}