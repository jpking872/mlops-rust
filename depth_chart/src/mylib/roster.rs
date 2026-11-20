use std::fs;

pub async fn get_lineup(team: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Team: {}", team);
    let players = get_roster_from_file(&team);
    Ok(players)
}

use reqwest::Client;
use serde_json::json;

fn get_roster_from_file(team: &str) -> Vec<String> {

    let lines: Vec<String> = fs::read_to_string("src/mylib/steelers.txt")
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect();

    println!("{}", team);

    return lines

}

async fn get_roster(team: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI").unwrap();

    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": format!(
                        "Return the current roster for the {} as a JSON array of player names.",
                        team
                    )
                }
            ]
        }))
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    println!("Response: {}", body.to_string());

    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");

    let roster: Vec<String> = serde_json::from_str(content)?;

    Ok(roster)
}