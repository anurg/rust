trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headlines: String,
    content: String,
}
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} ...", &self.content[0..50])
    }
}
fn main() {
    let article = Article {
        headlines: String::from("New AI Model"),
        content: String::from(
            "New AI Model--Lorem Ipsum,New AI Model--Lorem Ipsum,
        New AI Model--Lorem Ipsum,New AI Model--Lorem Ipsum,New AI Model--Lorem Ipsum,
        New AI Model--Lorem Ipsum,New AI Model--Lorem Ipsum,New AI Model--Lorem Ipsum,New AI 
        Model--Lorem Ipsum,New AI Model--Lorem Ipsum,",
        ),
    };
    println!("{}", article.headlines);
    println!("{}", article.summarize());
}
