mod aggregator;

use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people,"),
        reply: false,
        report: false,
    };

    println!("1 new post: {}", post.summarise());
}
