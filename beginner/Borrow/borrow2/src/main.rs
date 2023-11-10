fn main(){
    let mut s1= String::from("rust");
    let r1 = &s1;
    // let _r2=&mut s1;cannot borrow `s1` as mutable because it is also borrowed as immutable
print_string(r1);
let _r2=&mut s1;
add_to_string(_r2);
println!("{s1}");
let _s2=generate_string();
println!("{_s2}")
}
fn generate_string()->String{
    String::from("ferris")
    // we return direcly string if we return a refercne of a owned var of this block the owned var get droped and refrence will give nothjing so dont use it 
}
fn add_to_string(p1:&mut String){
p1.push_str("is awesome");
}
fn print_string(p1: &String){
    println!("{p1}");
}




// fn main() {
//     let mut s1=String::from("rust");
//     let r1 = &s1;
//     print_string(r1);
//    // let r2=&mut s1;cannot borrow `s1` as mutable because it is also borrowed as immutable
//    let r2=&mut s1;
//    let s1=add_to_string(p1)
    
//    }
   
//    fn add_to_string(p1:&mut String){
//        p1.push_str("is awesome!");
//    }
   
//    fn print_string(p1:&String){
//        println!("{p1}");
//    }