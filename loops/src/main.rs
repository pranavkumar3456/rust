use std::time::Instant;


fn main() {
let start = Instant::now();
let mut counter = 0;

let result = loop{
     counter +=1;
     
     if counter == 10{
         break counter * 2;}
};

println!("The result is {result}");

let duration = start.elapsed();
println!("Time elapsed is: {:?}", duration);
}


