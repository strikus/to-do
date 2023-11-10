struct Product{
    name:String,
    // category:String, 
    category:ProductCategory,
    price:f32,
    in_stock:bool
}

//Enums allows us to define a type by enumerating its variance variance like electrics clothing biijs 
enum ProductCategory {
    Books,
    Clothing,
    Electrics
}
fn main(){
let category = ProductCategory::Electrics;
let product =Product{
    name:String::from("tv"),
    category,
    price:200.32,
    in_stock:true
};
}
