use colored::Colorize;
use clap::{App, Arg};

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => String::from(""),
        }
    }
}

fn make_url(git_user: &str) -> String {
    let mut url:String = String::from("https://api.github.com/users/");
    println!("{}\n│",format!("{}",git_user).magenta());
    url.push_str(git_user);
    url.push_str("/repos");
    url
}

async fn json_results(url:String) -> serde_json::Value {
    let results:&str = &unwrap_or_return!(get_api_info(url).await)[..];
    let json_results: serde_json::Value = 
        serde_json::from_str(results).unwrap();
    json_results

}

async fn fork(repo:String) {
    let mut url = String::from("https://api.github.com/repos/");
    url.push_str(&repo);
    url.push_str("/forks");
    let fork_json_results = json_results(url).await;
    println!("{:#?}", fork_json_results);
}

fn show_results(json_results: serde_json::Value, limit:i32){
    let limit = if limit==-1 {json_results.as_array().unwrap().len() as i32} else {limit};
    let last = limit-1;
    for (i,item) in json_results.as_array().unwrap().iter().enumerate() {
        if (i as i32)<limit {
            let name = item["html_url"].as_str().unwrap();
            let description = item["description"].as_str().unwrap_or_else(|| "This repo has no description");
            if (i as i32) != last{
                println!("├┬{}\n│└─{}\n│",format!("{}",name).blue(), format!("{}",description).bright_cyan());
            }else{
                println!("└┬{}\n └─{}\n",format!("{}",name).blue(), format!("{}",description).bright_cyan());
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let matches = App::new("gitget")
        .version("1.0")
        .author("Santiago Calligari. <santiago@calligari.ar>")
        .about("Shows all repositories or repositories forks in GitHub and prints the results")
        .arg(Arg::with_name("repo_user")
             .required(true)
             .help("You need to give the repo that want to see the forks or the github user to see their repos"))
        .arg(Arg::with_name("limit")
             .short("l")
             .long("limit")
             .value_name("LIMIT")
             .help("Sets the limit of the number of results"))
        .get_matches();

    let repo_user = matches.value_of("repo_user").unwrap();
    let limit:i32 = matches.value_of("limit").unwrap_or("").parse().unwrap_or(-1);

    if repo_user.contains("/"){
        println!("repo: {:?}, limit: {}",repo_user , limit);
    }else{
        println!("user: {:?}, limit: {}",repo_user , limit);
        let url = make_url(&repo_user);
        let json_results = json_results(url).await;
        show_results(json_results, limit);
    }

    /*let url = if check_args(&args) { make_url(&args[1])} 
        else { show_usage(); String::from("")};
    if !(url == String::from("")) {
        let json_results = json_results(url).await;
        show_results(json_results, take_limit(args));
    }*/
}


async fn get_api_info(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).header("User-Agent", "GitGet/0.1")
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
