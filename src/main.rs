use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let git_user:&String = &args[1];
    let mut url:String = String::from("https://api.github.com/users/");
    url.push_str(git_user);
    url.push_str("/repos");
    
    println!("estoy pidiendo esto: {}", url);
    let repos = get_github_repos(url).await;
    println!("{:#?}", repos);
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
