fn main() {
   let age:i32=34;
   // allow u to compare a value with seires of path wwhich is match to execute
   // it can be literals variables wildcards etc
   match age {
    1=>println!("Happy first bday"),
    13..19=>println!("you are a teenager "),
   
   x=>println!("you arr x {x}year old ")
//    _=>println!("")
   }
}



// / match a control  flow operator 
// / In Rust, pattern matching is a powerful and flexible feature that allows you to destructure and match values against patterns, making it easier to handle various cases and perform different actions based on the structure and content of data. Pattern matching is typically used with the match keyword and can be applied to various data types, including enums, structs, tuples, and more
// / 