#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect_a = Rect::square(32);
    let rect_b = Rect {
        width: 20,
        height: 15,
    };

    dbg!(&rect_a.area());

    println!("rect_b is {:#?}", rect_b);
}
