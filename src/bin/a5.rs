fn main(){
    let mut my_integer = 1;
    loop{
        println!("{:?}", my_integer);
        if my_integer == 4{
            break;
        }
        my_integer+=1;
    }
}