struct  Product{
    name:String,
    price:f32,
    in_stock:bool
}
// implelmentation block allow us to implement functionality for a given type we can create a new implementation vlock usingf impl keytword
//impl keyword 
// self is keyword that refers to the instance of product this fnction is being called

//new acts as construcotr 
impl Product{
    // only true consturicor 
    // 
    fn new(name:String, price:f32)-> Product{
        Product{
            name:name,
            price:price,
            in_stock:true
        }
    }
    // it is without parameter fn called associatie function 
    fn get_default_sales_tax() -> f32{
        0.1
    }
    fn sales_tax(&self)->f32{
        self.price*Product::get_default_sales_tax()
        // self.price*0.1
    }
    fn set_price(&mut self,price:f32){
        self.price=price;
    }
    // owned form of self // usuaully wehen u want to transform one type to another while also preventing 
    // alsp color from using the original instance
     fn buy(self)->i32{
let name=self.name;
println!("{name} was bought");
123
     }
      // return fake receipt num
     // by take the owned version of seld which means ownershhip of the instance will be passed to this method we getting name from self and end of the method self will dropped 

}
fn main() {
   let mut book=Product::new(String::from("Book"),30.0);
println!("{}",book.name);
println!("{}",book.price);
   
//    let mut book=Product{
//     name:String::from("book"),
//     price:243.3,
//     in_stock:true
//    };
   let sales_tax = book.sales_tax();
//    let sales_tax = sales_tax(&book);
   println!("{}",sales_tax);
   book.set_price(1.0);
   println!("{}",book.price)//    book.buy(); // book was bought
//    book.set_price(2.0); // book instance will no longer valid
//    
//  |    book.set_price(2.0);
//    |    ^^^^^^^^^^^^^^^^^^^ value borrowed here after move
//    |
// note: `Product::buy` takes ownership of the receiver `self`, which moves `book`
}
