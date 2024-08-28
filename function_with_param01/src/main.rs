fn main() {
  print_labeled_measurement(5,'h');
}

fn print_labeled_measurement(value: i32, unit_lable: char){
    println!(" The measurement is: {value}{unit_lable}");
}
