//custom data type in formm of objects 
struct Product{
    name:String,
    price:f32,
    stock:bool
}

fn main() {
    // productr here is instance of struct product
  let mut book = Product{
name:String::from("book"),
price:28.85,
stock:true
  };
//   let price=book.price;
//   book.stock=false; // we are mutating the book 
let sales_tax=calc_sales_tax(&book);
println!("{sales_tax}")
}

fn calc_sales_tax(product:&Product)->f32{
    product.price*0.1
}