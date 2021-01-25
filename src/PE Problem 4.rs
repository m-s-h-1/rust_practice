fn main(){
    let mut num1 = 999;
    let mut a:bool = true;
    while num1>900{
        for num2 in 900..999 {
            let product = num1*num2;
            let digits: Vec<_> = product.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
            if digits[0]==digits[5] {
                if digits[1]==digits[4] {
                    if digits[2]==digits[3] {
                        println!("{} is a palindrome", product);
                        println!("{} x {} is the answer", num1, num2);
                        a = false;
                    }
                }
            }
        }
        if a==false {
            break;
        }
        num1-=1;
    }
}