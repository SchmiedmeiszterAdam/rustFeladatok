
fn print_massage(exp:bool){
    match exp{
        true => println!("it's big"),
        false => println!("small"),
    }
}
fn main(){
    let var = 70;
    let exp = var > 100;
    print_massage(exp)
}