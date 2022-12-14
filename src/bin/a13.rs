fn main(){
    let numbers = vec![10,20,30,40];
    println!("length: {:?}", numbers.len());
    for number in numbers{
        match number {
            30 => println!("thirty"),
            _ => println!("{:?}",number),
        }
    }
}