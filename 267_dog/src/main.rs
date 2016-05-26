use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() != 2 {
       try!(io::stderr().write(b"You must provide exactly one argument"));
       process::exit(1);
   }

   let position: u16 = args[1].parse::<u16>().unwrap();

   for i in 1..101 {
       if i == position {
           continue;
       }
       let suffix: &str;
       if i.to_string().ends_with("11") || i.to_string().ends_with("12") || i.to_string().ends_with("13") {
           suffix = "th";
       } else if i.to_string().ends_with("1") {
           suffix = "st";
       } else if i.to_string().ends_with("2") {
           suffix = "nd";
       } else if i.to_string().ends_with("3") {
           suffix = "rd";
       } else {
           suffix = "th";
       }

       println!("Position: {}{}", i, suffix);
   }
}
