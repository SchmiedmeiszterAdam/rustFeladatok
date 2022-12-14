enum Ticket{
    Backstage(String,f64),
    VIP(String,f64),
    Standard(f64),
}
fn main(){
    let tickets = vec![
        Ticket::Backstage("Anna".to_owned(),50.0),
        Ticket::VIP("Enu".to_owned(),500.0),
        Ticket::Standard(10.0)
    ];
    for ticket in tickets{
        match ticket {
            Ticket::Backstage(name,price) => println!("name: {:?}, price: {:?}",name,price),
            Ticket::VIP(name,price) => println!("name: {:?}, price: {:?}",name,price),
            Ticket::Standard(price) => println!("price: {:?}",price),
        }
    }
}