fn main() {
  let s1 = String::from("Rust");
//   let s2=s1;
  let _s2=s1.clone();
//   println!("{}",s1);println!("{}",s1);
//   |                 ^^ value borrowed here after move
println!("{}",s1);
let x=10;
let y=x;
println!("x is:{}",y);

// in integer character boolean the var clone by default and in string we have to clone by using .clone()
}
