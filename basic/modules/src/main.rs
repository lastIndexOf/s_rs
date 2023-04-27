use outter_module_a::outter_module_b;
use outter_module_a::outter_module_b::OUTTER_MODULE_B_CONST as CONST_B;

mod module_a {
    pub const MODULE_A_CONST: char = 'A';

    pub mod module_b {
        pub const MODULE_B_CONST: char = 'B';

        pub const MODULE_B_CONST_B: char = super::MODULE_A_CONST;
        // println!("MODULE_A_CONST = {}", MODULE_A_CONST);
    }

    pub enum Color {
        Transparent,
        RGB(u8, u8, u8),
        RGBA(u8, u8, u8, u8),
        HEX(String),
        Test { x: u8 },
    }
}

mod outter_module_a;

fn main() {
    println!("module_a::MODULE_A_CONST = {}", module_a::MODULE_A_CONST);
    println!(
        "module_a::module_b::MODULE_B_CONST = {}",
        module_a::module_b::MODULE_B_CONST
    );
    println!(
        "outter_module_b = {}",
        outter_module_b::OUTTER_MODULE_B_CONST
    );
    println!("OUTTER_MODULE_B_CONST = {}", CONST_B);
    println!(
        "OUTTER_MODULE_B_CONST = {}",
        crate::outter_module_a::outter_module_b::OUTTER_MODULE_B_CONST
    );
    println!("OUTTER_MODULE_B_CONST = {}", crate::CONST_B == CONST_B);
}

fn check_color(color: module_a::Color) {
    let color = module_a::Color::HEX(String::from("ffffff"));

    let _c = if let module_a::Color::HEX(hex) = color {
        hex.parse::<u8>().unwrap()
    } else {
        0
    };

    println!("c = {}", _c);
}
