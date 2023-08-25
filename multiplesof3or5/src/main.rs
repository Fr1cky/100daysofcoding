// Write code to find the sum of all multiples of 3 or 5 below 1000
fn main() { 
    //filter to keep the numbers that satisfy the condition
    // sum of the filtered numbers
    let total: i32 = (1..1000).filter(|&num| num % 3 == 0 || num % 5 == 0).sum();
    println!("Sum: {}", total);
    // let mut sum = 0;
    // for n in 1..1000 { 
    //     if n % 3 == 0 { 
    //         sum += n;
    //     } 
    //     else if n % 5 == 0 { 
    //         sum += n;
    //     }
    // }
    // println!("Total sum, {}", sum)
}
