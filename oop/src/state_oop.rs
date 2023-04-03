#[derive(Debug, PartialEq)]
enum PostState {
    Draft,
    PendingReview,
    Published,
}

#[derive(Debug)]
struct Post {
    content: String,
    state: PostState,
}

impl Post {
    fn new() -> Self {
        Self {
            content: String::new(),
            state: PostState::Draft,
        }
    }

    fn content(&self) -> &str {
        match self.state {
            PostState::Published => &self.content,
            _ => "",
        }
    }

    fn add_text(&mut self, text: &str) {
        match self.state {
            PostState::Draft => {
                self.content.push_str(text);
            }
            _ => {
                panic!("State Error");
            }
        };
    }

    fn request_review(&mut self) {
        match self.state {
            PostState::Draft => {
                self.state = PostState::PendingReview;
            }
            _ => {
                panic!("State Error");
            }
        };
    }

    fn approve(&mut self) {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::Published;
            }
            _ => {
                panic!("State Error");
            }
        };
    }
}

pub fn test_state_oop() {
    let mut post = Post::new();

    post.add_text("hello");

    println!("post: {:?}", post.content());
    post.add_text(" world");
    println!("post: {:?}", post.content());

    post.request_review();
    println!("post: {:?}", post.content());
    post.approve();

    println!("post: {:?}", post.content());
}
