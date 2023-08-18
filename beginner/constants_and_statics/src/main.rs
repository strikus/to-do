const MAX_PLAYERS: u8 =10;
// cant be mutated as in variable 
// all letter will be in caps + _ snake case  and declare in global scope inside scope 
// value of const will be a constant expresssion  value has to be computed as time of compile 
static CASINO_NAME: &str="ROHit";
// can be declare in any scope 
// can mutated by using mut keyword
// unsafe later on
// when using const the value of const
// diiffernce 

fn main() {
    // when using constant variables 
// value of const wil be inline 
let a: u8=MAX_PLAYERS;
let b = MAX_PLAYERS;

let c: &str=CASINO_NAME;
let d: &str = CASINO_NAME;
  
}
