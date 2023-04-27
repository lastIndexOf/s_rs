use hello_macro::{arr, hello_marco_proc::HelloMarco, HelloMacroTrait};

#[derive(Debug, HelloMarco)]
struct HelloMacroStruct;

pub fn run_marco() {
    let lst = arr![1, 2, 3];

    println!("lst = {:?}", lst);

    HelloMacroStruct::signature();
}
