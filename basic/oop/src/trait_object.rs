trait Draw {
    fn draw(&self);
}

// struct Screen<T: Draw> {
//     components: Vec<T>,
// }
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("@@@");
    }
}

struct Text;

impl Draw for Text {
    fn draw(&self) {
        println!("@@@");
    }
}

pub fn test_trait_object() {
    let mut screen = Screen { components: vec![] };

    // screen.components.push(Button);
    // screen.components.push(Text);

    screen.components.push(Box::new(Button));
    screen.components.push(Box::new(Text));
}
