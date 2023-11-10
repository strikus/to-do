fn main() {
    // Slices are refrences to a cotiguous sequence of elemtns in a collection 
    // contiguous means the elements are next to each other
     let tweet = String::from("This is my tweet and and its very very long ");
     let trimmed_tweet=&tweet[..20]; //string slice  [starting index..ending index]
     println!("{trimmed_tweet}")
 }
 