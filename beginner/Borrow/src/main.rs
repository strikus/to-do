// fn main() {
 
// let x:String=String::from("rust");

// print_string(x);
// println!("{x}");


// //note: consider changing this parameter type in function `print_string` to borrow instead if owning the value isn't necessary
// }

// fn print_string(p1:String){
//     println!("{p1}")
// }//

fn main() {
 
    let x:String=String::from("rust");
    let r1=&x;
    print_string(r1);
    println!("{x}");
    
    
    //note: consider changing this parameter type in function `print_string` to borrow instead if owning the value isn't necessary
    }
    
    fn print_string(p1:&String){
        println!("{p1}")
    }