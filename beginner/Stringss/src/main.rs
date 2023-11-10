fn main() {
   let s = "we can type here in hindi ";
   let s1= String::from("hindi");
   let s2= "kid".to_string();
   let s3 = "kid".to_owned(); // owned data from &str to string 
   let s4= &s3[..];
   println!("{s4}");
}
