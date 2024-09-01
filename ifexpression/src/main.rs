fn main() {
 let number = 3;
 if number < 5 {
     println!("condition was true");
 }
 else{
     println!("condition was false");
 }

if true {
    println!("number was 3");
}

if number > 1{
    println!("number is greater than 1");
}

let number = 6;

if number% 4 ==0{
    println!("number is divisible by 4");
}else if number% 3 ==0{
    println!("nubmer is divisible by 3");
}
else if number % 2 == 0 {
    println!("number is divisible by 2");
}
else{
    println!("number is notdivisible by 4,3 or 2");

}

let condition = true;
let number = if condition {5} else {6};

println!("The value of number is: {number}");


let condition = true;
let number1 = if condition {"five"} else {"six"}; // in rust variable must have similar types; also
                                                  // you can't use println! for printing this
let data = number1;

println!("The value of number is: {data}");

let condition = true;
let hello = if condition{"5"} else {"6"};

println!("the value of number is: {hello}");
}
