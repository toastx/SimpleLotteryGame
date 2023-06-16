use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("welcome");
    let secretnum = rand::thread_rng().gen_range(1..10);
    let mut count= 0u32;
    let mut remaining = 5;
    while count<5{
        count+=1;
        remaining -= 1;
    println!("enter a number you choose:");
    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("invalid");
    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=> continue
    };
    println!("you guess {guess}");
    match guess.cmp(&secretnum) {
        Ordering::Less => println!("You have {remaining} tries remaining"),
        Ordering::Greater => println!("You have {remaining} tries remaining"),
        Ordering::Equal => {println!("You win");
        break;
    }
    }
}
if count==5{
    println!("Better luck next time");
}
}


