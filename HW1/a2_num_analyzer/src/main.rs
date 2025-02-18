fn is_even(n: i32) -> bool{
    if n%2 == 0 {
        return true;
    }else{return false}
}

fn main() {
    let num=[5,8,1,23,45,77,12,81,53,67];
    for x in num {
        if is_even(x){println!("{} is even",x)}else{println!("{} is odd", x)}
    }
    println!("");
    for x in num{
        let by_3 = x%3 == 0;
        let by_5 = x%5 == 0;
        if by_3 && by_5 {
            println!("{} is FizzBuzz",x);
        }else if by_3 && !by_5{
            println!("{} is Fizz",x);
        }else if !by_3 && by_5 {
            println!("{} is Buzz",x);     
        }else {
            print!("");
        }
    }

    println!("");
    let mut i = (num.len() - 1) as isize;
    let mut sum = 0;
    while i >= 0 {
        sum += num[i as usize];
        i -= 1;  
    }
    println!("{} is sum of the array",sum);

    


}
