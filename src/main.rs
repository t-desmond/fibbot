use std::env::args;
fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty(){
        println!("no arguments supplied");
        return;
    } else if args.len() == 1 {
        println!("Fibbot requires two parameters");
    } else if args.len() ==2  {
        let argument_1 = &args[0];
        let argument_2 = &args[1];
        println!("fiboot enabled succesfully with max_threshold: {}", argument_2);
    }
}