use std::time::Instant;


fn main() {
let start = Instant::now();

let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop{
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count ==2 {
            break 'counting_up;
        }
        remaining -=1;
    }
    count +=1;
}
println!("End Count = {count}");



let duration = start.elapsed();
println!("Time elapsed is: {:?}",duration);

}
