fn main(){
    #![allow(unused)]
    // creation 
    // we use let keyword for var 
    let a=20; // 32 bit integer
    let x: i16=3; //16 bit 
    // let a: i64=1.0; // mismatch type 
    // mutability 
    let b=34;
 // change value of b
//  b=2;cannot assign twice to immutable variable
// by default var in rust is immutable
// we have to use mut keyword to make it mutable
let mut c =23;
c =231;

// shadowing 
let d =12;
let d =112;
println!("d is {d},d");
println!("d is: {}", d);
// second d is shadowing first one 
// scope 
// interscope 
{
    /*
    let d=40;
    */
    let d =3434;
    println!("inner d is :{}",d); // if d is not declared in this block the value of d will fetch from outside if there is d inside it it will shadow outside d value 
}
}