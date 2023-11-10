use std::{string, fmt::format};

fn main() {
  let mut s = String::from("foo");
  s.push_str("bAR");
  println!("{}",s);
  // replace range

  s.replace_range(.., "baz");
  println!("{}",s);
// foobAR
// baz
// concat tjhe string
let s4=String::from("hello");
let s5=String::from("world");
// let s6 = &s4+&s5;string concatenation requires an owned `String` on the left
let s6 = s4+&s5;
let s7=String::from("tic");
let s8=String::from("tac");
let s9=String::from("toe");
println!("{}",s6);
println!("{}",s5);
// can add string slice in format! it less efficient than + operator 
let _s43= format!("{}-{}-{}",s7,s8,s9);
println!("{}",_s43);
// let _s43= format!("{}-{}-{}",s7,s8,"toe");
// println!("{}",_s43);
let s3434=concat!("first","second");
println!("{}",s3434);
let s2323= ["string", "addewd"].concat(); 
let we=format!("{}{}","first","seocnd"
);
let s23= String::from("test");
let swe=s4+"okok"; // string type be must first
}
