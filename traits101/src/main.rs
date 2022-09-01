use traits101::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "the future is now old man".to_string(),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Penguins in Central park zoo".to_string(),
        location: "New York".to_string(),
        author: "Chuck Charles".to_string(),
        content: "Group of mysterious penguins infiltraited local grocery store in search of food and treats".to_string()
    };

    println!("New article available! {}", article.summarize());
}
