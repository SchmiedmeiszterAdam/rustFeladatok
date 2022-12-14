struct Person{
    age:i32,
    name:String,
    favorite_color:String,
}
fn main(){
    let people = vec![
        Person{
            age:20,
            name:String::from("Ádám"),
            favorite_color:String::from("white"),
        },
        Person{
            age:32,
            name:String::from("Éva"),
            favorite_color:String::from("black"),
        },
        Person{
            age:6,
            name:"Noel".to_owned(),
            favorite_color:"blue".to_owned(),
        },
    ];

    for person in people{
        if person.age <= 10{
            println!("age: {:?}",person.age);
            print(&person.name);
            print(&person.favorite_color);
        }

    }
    fn print(data:&str){
        println!("{:?}",data);
    }
}