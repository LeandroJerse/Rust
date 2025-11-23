#[derive(Debug)]
struct Rectangle{
    width: u32,
    heigth: u32,
}

impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.heigth
    }
    fn grow_widht(&mut self){
        self.width+=1;
    }
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
    fn new_rectangle(width:u32,heigth:u32) -> Rectangle {
        Rectangle {
            width,
            heigth,
        }
    }

}




fn main() {

    


    let mut my_rectangle = Rectangle::new_rectangle(30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        my_rectangle.area()
    );
    my_rectangle.heigth = 75;
    my_rectangle.grow_widht();
    println!("Rectangle is {:?}",my_rectangle);

    let react1 = Rectangle::new_rectangle(30,50);
    let react2 = Rectangle::new_rectangle(10,40);
    let react3 = Rectangle::new_rectangle(60,45);
    
    println!("Can react1 hold react2? {}",react1.can_hold(&react2));
    println!("Can react1 hold react3? {}",react1.can_hold(&react3));
    println!("Can react2 hold react3? {}",react2.can_hold(&react3));

    
}


