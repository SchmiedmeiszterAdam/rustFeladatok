enum Color{
    Blue,
    Red,
    Brown
}

impl Color{
    fn print(&self){
        match self {
            Color::Blue => println!("Blue"),
            Color::Red => println!("Red"),
            Color::Brown => println!("Brown"),
        }
    }
}
struct Dimensions{
    width:f64,
    height:f64,
    depth:f64,
}

impl Dimensions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
struct ShippingBox{
    color:Color,
    dimensions: Dimensions,
    weight:f64,
}

impl ShippingBox{
    fn create_new_box(weight:f64,color:Color,dimensions:Dimensions) -> Self{
        Self { weight,color,dimensions}
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}
fn main(){
    let small_dimensions =  Dimensions{
        width:1.0,
        height:2.0,
        depth:3.0
    };
    let small_box = ShippingBox::create_new_box(5.0,Color::Brown,small_dimensions);
    small_box.print();
}