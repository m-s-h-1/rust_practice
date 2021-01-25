fn main(){
    let mut ans:u128 = 1;
    loop {
        if ans%7==0 && ans%11==0 && ans%12==0 && ans%13==0 && ans%14==0 && ans%15==0 && ans%16==0 && ans%17==0 && ans%18==0 && ans%19==0 && ans%20==0 {
            println!("{}", ans);
            break;
        }
        if ans%100_000_000 == 0 {
            println!("{}", ans);
        
        }
        ans +=1
    }
}

/*






7 = 7 x 1



11 = 11 x 1
12 = 2 x 6
13 = 13 x 1
14 = 2 x 7
15 = 3 x 5
16 = 2 x 8
17 = 17 x 1
18 = 2 x 9
19 = 19 x 1
20 = 2 x 10

*/