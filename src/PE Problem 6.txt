fn main(){
    let mut square_sum = 0;
    let mut sum_square = 0;
    let mut sum = 0;
    loop{
        for i in 1..101 {
            let a = i*i;
            square_sum +=a;
        }
        for i in 1..101 {
            sum = sum + i;
        }
        sum_square = sum*sum;
        println!("{}", sum_square-square_sum);
        break;
    }
}