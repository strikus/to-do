fn main() {
  // Srting slices and functions
  let tweet = String::from("Strign is veryt od sdrohit sdkj ksf");
  let trimmed_tweet= trim_tweet(&tweet);
  let tweet2="Strign is veryt od sdrohit sdkj ksf";
  let trimmed_tweet2=trim_tweet(tweet2);
  println!("{trimmed_tweet}");
  println!("{trimmed_tweet2}");

  let a:[i32;6]=[1,2,3,4,5,6];
  let a_slice=&a[..3];
  println!("{:?}",a_slice);
}

fn trim_tweet(tweetdf:&str)->&str{
&tweetdf[..20]
}