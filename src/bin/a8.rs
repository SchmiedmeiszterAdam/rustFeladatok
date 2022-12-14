enum Flavor{
    Orange,
    Lemon,
    Mango,
}
struct Drink{
    flavor:Flavor,
    ounce:f64
}
fn main(){
    let drink = Drink{
        flavor:Flavor::Orange,
        ounce:1.5
    };
    print_drink(drink);
}
fn print_drink(drink:Drink){
    match drink.flavor{
        Flavor::Orange => println!("Flavor: Orange, Ounce: {:?}",drink.ounce),
        Flavor::Lemon => println!("Flavor: Lemon, Ounce: {:?}",drink.ounce),
        Flavor::Mango => println!("Flavor: Mango, Ounce: {:?}",drink.ounce),
    }
    
}