fn tuple() -> (i32,i32){
    (1,4)   
}
fn main(){
    let (x,y) = tuple();
    if y < 5{
        println!("less than 5");
    }
    else if y > 5{
        println!("bigger tan 5");
    }
    else{
        println!("its 5");
    }
}