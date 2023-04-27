use futures::executor::block_on;

use super::bro::bro_child::BRO_1_CHILD;
use crate::bro::BRO_1;

pub(crate) const BRO_2: i32 = 2;

pub fn print_in_bro_2() {
    println!("print_in_bro_2, {BRO_2} {}", BRO_1);
    println!("print_in_bro_3, {}", BRO_1_CHILD);

    let x = block_on(async {
        let s = test_1().await?;
        Ok::<&str, &str>(s)
    });

    println!("in print in bro 2 x = {}", x.unwrap());
}

async fn test_1() -> Result<&'static str, &'static str> {
    Ok("test 1")
}
