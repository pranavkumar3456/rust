//fn five() -> i32{
//    5
//}
//
//
//fn main() {
//
//    let x = five();
//   println!("The value of x is: {x}");
//
//
//
//}

fn main()
{
    let x = plus_one(5);

    println!("the value ofx is: {x}");
}

fn plus_one(x: i32) -> i32 {
            x + 1
        }
