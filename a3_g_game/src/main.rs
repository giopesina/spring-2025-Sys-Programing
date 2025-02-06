
fn check_guess(guess: i32,secret: i32) -> i32{
    if guess == secret{
        return 0;
    }else if guess > secret{
        return 1;
    }else{return -1;}
}
fn main() {
    let mut ans: i32 = 57;
    let mut guess = [97, 32,56, 74,65, 59,57];
    let mut counter = 0;
    loop{
        if check_guess(guess[counter], ans) == 0{
            println!("Correct");
            counter += 1;
            break;
        }else if check_guess(guess[counter], ans) == 1{
            println!("Too High, guess lower");
            counter += 1;
            
        }else{
            println!("Too Low, guess Higher");
            counter += 1;
        }
    }
    println!("{} guesses",counter);
}
