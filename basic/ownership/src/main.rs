fn main() {
    let s1 = "string1";
    let s_obj1 = String::from(s1);
    let s_obj2 = s_obj1;
    let s_obj3 = s_obj2.clone();

    print_string(s_obj2);

    println!("s_obj3: {}", s_obj3);

    let s_obj4 = String::from("string4");

    let s_obj4 = take_ownership(s_obj4);

    println!("s_obj4: {}", s_obj4);

    let tup = ("name", 26);

    println!("name: {}, age : {}", tup.0, tup.1);

    reference_and_borrow();
}

fn print_string(mut str: String) {
    str.push_str("string2");

    println!("{}", str);
}

fn take_ownership(str: String) -> String {
    str
}

fn reference_and_borrow() {
    let mut s1 = String::from("hello, ");

    borrow_str_and_do_not_change_value(&s1);

    borrow_str_and_change_value(&mut s1);

    println!("s1 = {}", s1);
}

fn borrow_str_and_do_not_change_value(str: &String) {
    println!("borrow str = {}", str);
}

fn borrow_str_and_change_value(str: &mut String) {
    println!("borrow str = {}", str);

    str.push_str("world")
}
