pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String;
}

pub struct WeiBo {
    author: String,
    content: String,
    text_a: String,
}

pub struct Tweet {
    author: String,
    content: String,
    text_b: String,
}

impl Summary for WeiBo {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.author, self.content);
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.author(), self.content);
    }
}
