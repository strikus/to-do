fn main() {
  // fn keyword followed by function name() 
  my_function(22);
 let z= my_function1(22);
 println!("my fn returnedd :{}",z)
}
fn my_function(x: i32){
    println!("my fn called with :{}",x)
}
// r3eturn fn with arrow fn 
fn my_function1(x: i32)->i32{
    println!("my fn called with :{}",x);
    let y=10;
   return  y;
   // or only y
}