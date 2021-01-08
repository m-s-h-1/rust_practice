fn main(){
    let number:u64 = 600851475143;
    for i in 1..number {
        if i%2==0 { continue };
        if i < 10000 {
            if number%i==0 {
                print!("{} ", i);
            }
        } else {
            break;
        }
    }
}