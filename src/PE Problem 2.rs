fn main() {
    let mut prev_number1 = 0;
    let mut prev_number2 = 0;
    for i in 1..10 {
        let mut number = 0;
        number = prev_number1+prev_number2;
        println!("{}", number);
    }
}
