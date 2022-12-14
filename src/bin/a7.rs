enum Color{
    Red,
    Blue,
    Green,
    Yellow,
}
fn main(){
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Green);
    print_color(Color::Yellow);
}
fn print_color(color: Color){
    match color {
        Color::Red => println!("it's red"),
        Color::Blue => println!("it's blue"),
        Color::Green => println!("it's green"),
        Color::Yellow => println!("it's yellow"),
    }
}