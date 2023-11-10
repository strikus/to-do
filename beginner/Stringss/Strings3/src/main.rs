
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let s1="🦀🦀🦀🦀";
    // let s2= s1[0]; // cant index a string by integer 
    // first index means first byte  in our string in utf8 a char between 1-4 bytes
    // first byte doesnt mean anything
    // it allows to string slice over specific set of bytes
    // crab is 4byte long
    let s2=&s1[0..4];
    println!("{}",s2);

    for b in "लोसोेूाी".bytes(){
        println!("{}",b);
    }

    for c in "ोे्िु्िसले्किे््ितक".chars(){
        println!("{}",c);
    }
    for g in "्रुु्िरकुकिकररुतक्".graphemes{
println!()
    }

}
// bytes fn give bytes of each element

// chars give each character

// some character cant display 
// its the iterator over the unicode scalar value of the string 
// scalar value are a basic unit in unicode and some user perceived char made up of multiple scaler values
// user perceived in utf8 knowns as graphene clusters



fn kid() {
    let my_string = "café";

    // Using UnicodeSegmentation to iterate over graphemes
    for grapheme in UnicodeSegmentation::graphemes(my_string, true) {
        println!("{}", grapheme);
    }

    // Get the nth grapheme
    if let Some(grapheme) = UnicodeSegmentation::grapheme_indices(my_string, true).nth(1) {
        println!("Second grapheme: {}", grapheme.1);
    }
}
