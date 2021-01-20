fn main(){
    let mut num_of_prime = 0;
    let mut num = 3;
    loop {
        for i in 2..num+1{
            if num%1 == 0 && num%num==0 || num%i!=0 {
                num_of_prime +=1;
            } else if num_of_prime == 6 {
                println!("{}", num);
                break;
            }
            num+=1;
        }
        break;
    }
}