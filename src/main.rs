use std::env;
use colored::Colorize;

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
    println!("{}\n │",format!("{}",git_user).magenta());
    url.push_str(git_user);
    url.push_str("/repos");
    url
}

async fn json_results(url:String) -> serde_json::Value {
    let results:&str = &unwrap_or_return!(get_github_repos(url).await)[..];
    let json_results: serde_json::Value = 
        serde_json::from_str(results).unwrap();
    json_results

}

fn show_results(json_results: serde_json::Value, limit:u32){
    let limit = if limit==0 {json_results.as_array().unwrap().len() as u32} else {4};
    let last = limit-1;
    for (i,item) in json_results.as_array().unwrap().iter().enumerate() {
        if (i as u32)<limit {
            let name = item["html_url"].as_str().unwrap();
            let description = item["description"].as_str().unwrap_or_else(|| "This repo has no description");
            if (i as u32) != last{
                println!(" ├┬{}\n │└─{}\n │",format!("{}",name).blue(), format!("{}",description).bright_cyan());
            }else{
                println!(" └┬{}\n  └─{}\n",format!("{}",name).blue(), format!("{}",description).bright_cyan());
            }
        }
    }
}

fn check_args(args: &Vec<String>) -> bool {
    args.len() > 1
}

fn show_usage() {
    println!("\nusage: gitget [options] <githubUsername>
[options]
{{-L --limit}}
");
}

fn take_limit(args: Vec<String>) -> u32{
    for (i, element) in args.iter().enumerate() {
        if element == "-L" || element == "--limit" {
            return args[i+1].parse().unwrap();
        }
    }
    0
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = if check_args(&args) { make_url(&args[1])} 
        else { show_usage(); String::from("")};
    if !(url == String::from("")) {
        let json_results = json_results(url).await;
        show_results(json_results, take_limit(args));
    }
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
