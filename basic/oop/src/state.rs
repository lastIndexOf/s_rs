struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingPost {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn text(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingPost {
        PendingPost {
            content: self.content,
        }
    }
}

impl PendingPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

pub fn test_state() {
    let mut post = Post::new();

    post.add_text("hello");
    post.add_text(" world");

    let post = post.request_review();
    let post = post.approve();

    println!("post: {}", post.text());
}
