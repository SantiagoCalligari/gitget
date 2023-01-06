use std::env;

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => String::from(""),
        }
    }
}

fn make_url(git_user: &String) -> String {
    let mut url:String = String::from("https://api.github.com/users/");
    url.push_str(git_user);
    url.push_str("/repos");
    url
}
async fn json_results(url:String) -> serde_json::Value {
    let results:&str = &unwrap_or_return!(get_github_repos(url).await)[..];
    let json_results: serde_json::Value = 
        serde_json::from_str(results).expect("JSON was not well-formated");
    json_results
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = make_url(&args[1]);
    println!("{:#?}", json_results(url).await);
}

async fn get_github_repos(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).header("User-Agent", "GitGet/0.1")
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
