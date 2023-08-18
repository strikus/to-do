fn main() {
    
   // if/else
   // condition in if statements must be booleans
   let a=5;
//    let a; //error 
   if a>5{
    println!("bigger than 5");
   }
   else if a>3{
    println!("biggetr than 3")
   }
   else{
    println!("smalkler thabn or eq to 3")
   }
   // use in let statement 
   let _b = if a>5 {1} else {-1};
   //{} we can write without return keyword if the return thing in last of line of its own block
   println!("value of b is {}",_b);
   // type of bothj return statement should be compatible 

//    loop exectue forever without break 
// labeling syntax is 'lablename: loop
   'outer: loop{
    println!("loop forever");
    // break;
    {
        break 'outer;
    }
}
loop {
    println!("loop forever");
    // break;
    {
        break; // this will break from innner loop not the outertloop u can use labeling 
    }
}
let _x: i32=loop{
    break 5;
};
// ; necessary in end 
// while 
let mut  b: i128= 2323; // made mutable
while b<2325{
    println!("a is {}",b);
    b=b+1;
}
// for loop 
// allow loop through collection
let _a: [i32;5]=[1,2,3,4,5];
for element in _a{
    println!("{}",element);
}
   // looop will forever 
   // so we use inner loop as break
}
