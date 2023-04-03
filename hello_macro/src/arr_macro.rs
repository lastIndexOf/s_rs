#[macro_export]
macro_rules! arr {
    ($( $x: expr ),*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}