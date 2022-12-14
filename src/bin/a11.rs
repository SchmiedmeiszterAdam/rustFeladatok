struct Grocery{
    quantity:i32,
    id:i32
}
fn print_quantity(grocery : &Grocery){
    println!("quantity: {:?}",grocery.quantity);
}
fn print_id(grocery : &Grocery){
    println!("id: {:?}",grocery.id);
}
fn main(){
    let grocery_item = Grocery{
        quantity:32,
        id:22
    };
    print_quantity(&grocery_item);
    print_id(&grocery_item);
}