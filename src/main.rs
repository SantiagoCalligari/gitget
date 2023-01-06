use std::args;
use std::collections::HashSet;

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
        serde_json::from_str(results).unwrap();
    json_results
}

fn show_results(json_results: serde_json::Value) -> () {
    for item in json_results.as_array().unwrap().iter() {
        let name = item["html_url"].as_str().unwrap();
        let description = item["description"].as_str().unwrap_or_else(|| "This repo has no description");
        println!("{} \n {} \n",name, description)
    }
}

fn check_args(args: &Vec<String>) -> bool {
    args.len() > 1
}

fn show_usage() {
    println!("\nusage: gitget [options] <githubUsername>
[options]
{{-L --limit}}
{{-V --version}}
");
}

fn show_version() {
    println!("gitget version: 1.0");
} 

fn take_limit(args: Vec<String>) -> u32{
    for (i, element) in args.iter().enumerate() {
        if element == "-L" {
            return args[i+1];
        }
    }
    30
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = args::args().collect();
    let url = if check_args(&args) { make_url(&args[1])} 
        else { show_usage(); String::from("")};
    println!(take_limit(args))
    //let json_results = json_results(url).await;
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
