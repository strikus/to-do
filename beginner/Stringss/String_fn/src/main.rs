fn main() {
    let s = "Hello s";
    let s2 = String::from("hello rohit");
let _d=my_func(s);
println!("{}",_d);
let _swe=my_func(&s2);
println!("{}",_swe);
println!("{s2}");
println!("{s}")
}

fn my_func(a: &str)->String{
    return format!("{}",a);
}