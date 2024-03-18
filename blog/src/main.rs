use blog::Post;

fn main() {
    let text = "I ate a salad for lunch today";
    let mut post = Post::new();

    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
}
