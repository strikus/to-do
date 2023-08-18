fn main() {
    #![allow(unused)]
    // data types

// brief overview
    // boolean
    let b1: bool = true;

    // unsigned integers(positive int)
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i6: u128 = 1;

    // signed integers(positive or negative)
    let i7: i8 = 1;
    let i8: i16 = 1;
    let i9: i32 = 1;
    let i10: i64 = 1;
    let i11: i128 = 1;

    // floating point numbers either ///32 or 64
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;  // The pointer-sized unsigned integer type.
    let p2: isize = 1;  //The pointer-sized signed integer type.

    // characters, &str, and String
    let c1: char = 'c';
    // two types of string 1.string slices  and 2. String 
    let s1: &str = "hello";
    let s2: String = String::from("hello");
// compound datatype store more than one value 
    // arrays
    // array hold same datatype value 
    let a1 = [1, 2, 3, 4, 5];

    let i1 = a1[4];

    // tuples
    // hold different type data 
    let t1 = (1, 2, 3);
    let t1 = (5, 5.0, "5");

    let s1 = t1.2;
    let (i1, f1, s1) = t1;

    let unit = ();

    // Type aliasing
    type Age = u8;

    let a1: Age = 57;
    
}

