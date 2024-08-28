fn main() {

    // rust is a statically typed language i.e., it must be know the types of variable at compile
    // time.
    
    let guess: u32 = "42".parse().expect("not a number");
    println!("{guess}");
    
    //Scaler data type
    
    //: scaler type represents a single value: Rust has primary 4 scalar type
    //i.e., 1. integers, floating-point numbers, booleans and character.
    
    // Integer   ==> without fractional components
      
      //  Length Signed Unsigned
      //  8  bit   i8     u8
      //  16 bit   i16    u16
      //  32 bit   i32    u32   --> primarily we use this i.e., default
      //  64 bit   i64    u64
      // 128 bit   i128   u128
      // arch      isize  usize  --> this will used when indexing some sort of collection.
      

    //floating-Point Types
    // --> rust also have two types of floating-point numbers i.e, f32 and f64 which is 32 and 64
    // bits in size respectively.
    // default is f64 because of modern CPU. --> speed is same as f32 but it has more precision.
    //
    //
    println!();
    println!("floating-poing types");
    println!();

    let t = 2.0; //f64
    println!("{t}");
    let u: f32 = 3.0; //f32
    println!("{u}");
 
    println!(); 
    println!("Numeric Operators!!!!");
    println!();

    println!("Addition");
    let sum = 5 + 10;

    println!("Addition: {sum}");
    println!("Subtraction");
    let difference = 95.5 - 4.3;

    println!("Subtraction: {difference}");
    println!("multiplication");
    let product = 4 * 40;

    println!("muliplication: {product}");
    let quotient = 56.7/32.2;

    println!("quotient: {quotient}");
    let truncated = -5/3; //result in -1

    println!("truncated: {truncated}");
    let remainder = 43%5;
    
    println!("remainder: {remainder}");




   println!();
   println!("Boolean Type");
   println!();

   let e = true;
   println!("{e}");
   
   let f: bool = false; //with explicit type annotation

   println!("{f}");



   println!();
   println!("Character Type"); // rust char type is 4 bytes in size, represents unicode scalar
                               // value. chinese, japanese and korean; emoji, zero-width space
                               // everyting are valid in rust. U+0000 --> U+DD7FF
   println!();

   let c = 'z';
   println!("{c}");
   let z: char = 'Z'; // with explicit type annotation
   println!("{z}");
   let heart_eyed_cat = "~_~";
   println!("{heart_eyed_cat}");

   
   println!();
   println!(" ### COMPOUND TYPES ###"); // compound types can group multiple values into one type.
                                        // Two Types:: 
                                        // 1. Tuples
                                        // 2. Arrays
   println!();

   println!(" ## COMPOUND TYPE###: 1. Tuples");
   // tuples are general way of grouping together a number of values with a variety of types into
   // one compound type.
   println!();
   println!("one-way of calling tuple");
   let tup: (i32,f64,u8)=(500,4.8,1);
   let (x,y,z) = tup;
  println!("tuple data be like: {x},{y},{z}");

   println!();
   println!("Other way");

   let tupl: (i32,f64,u8)=(700,6.4,1);

   let seven_hundred = tupl.0;
   println!("{seven_hundred}");

   let six_point_four = tupl.1;
   println!("{six_point_four}");

   let one = tupl.2;
   println!("{one}");
   
   println!();
   println!(" ## COMPOUND TYPE ## 2. ArrayType");
   // array are useful when you want data allocated on stack rather than heap
   println!();



}
