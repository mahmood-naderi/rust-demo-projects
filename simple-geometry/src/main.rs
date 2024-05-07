use rand::Rng;
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }

    fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut rectangle = Rectangle{
        width: 22.0,
        height: 69.0,
    };
    let rand_width = rand::thread_rng().gen::<f32>();
    rectangle.width = 100.0 * rand_width;
    println!("height: {}, width: {}", rectangle.height, rectangle.width);
    println!("{:?}", rectangle);
    println!("{}", rectangle.area());
    println!("{}", rectangle.perimeter());
    println!("{:?}", Rectangle::square(69.0));


}
