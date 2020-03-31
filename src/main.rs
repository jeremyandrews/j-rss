use ammonia::Builder;
use chrono::{DateTime, Duration};
use maplit::hashset;
use rss::Channel;
use serde::Deserialize;

// This `derive` requires the `serde` dependency.
#[derive(Deserialize, Debug)]
struct Response {
    code: usize,
    text: Vec<String>,
}

fn strip_anchor(text: &str) -> String {
    str::replace(text, "#", " ")
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Set these up now, as we'll use the same values for all channel items
    let base = "https://translate.yandex.net/api/v1.5/tr.json/translate";
    //let api_key = "get from https://translate.yandex.com/developers/keys";
    let lang = "it-en";
    let format = "plain";

    // allowed tags is an empty set, strip all HTML
    let tags = hashset![];

    let feeds_with_timestamp: Vec<String> = vec![
        "http://www.noitv.it/localita/valle-del-serchio/rss/".to_string(),
        "https://www.lagazzettadelserchio.it/rss/articoli/".to_string(),
        "https://www.lagazzettadelserchio.it/rss/brevi/".to_string(),
    ];
    for feed in feeds_with_timestamp {
        // Feed includes timestamps, only include articles from the past-12 hours
        let channel = match Channel::from_url(&feed) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error parsing URL: {}", e);
                std::process::exit(1);
            }
        };
        println!(">> Parsing '{}'...", channel.title());
        for item in channel.items() {
            let pub_date = match item.pub_date() {
                Some(p) => p,
                None => {
                    "unknown"
                }
            };
            let timestamp = match DateTime::parse_from_rfc2822(pub_date) {
                Ok(t) => t,
                Err(e) => {
                    // timestamp missing? for now ignore article
                    eprintln!("failed to parse timestamp ({:?}): {}", pub_date, e);
                    DateTime::parse_from_rfc2822("Mon, 30 Mar 2020 05:32:05 +0000").unwrap()
                }
            };
            let day_before_now = chrono::offset::Utc::now().checked_sub_signed(Duration::hours(12)).unwrap();
            if timestamp > day_before_now {
                let mut text = String::new();
                let title = match item.title() {
                    Some(t) => t,
                    None => {
                        "[EMPTY TITLE]"
                    }
                };
                println!("-----");
                text.push_str(title);
                text.push_str(": ");
                let description = match item.description() {
                    Some(t) => t,
                    None => {
                        "[EMPTY DESCRIPTION]"
                    }
                };
                text.push_str(description);

                // allowed tags is an empty set, strip all HTML
                let tags = hashset![];
                let cleaned_text = Builder::new()
                    .tags(tags.clone())
                    .clean(&text)
                    .to_string();
                println!("{}\n--\n{}\n--", cleaned_text, pub_date);

                // Translate
                let url = format!("{}?key={}&text={}&lang={}&format={}", base, api_key, strip_anchor(&cleaned_text), lang, format);
                let response = reqwest::get(&url)
                    .await?
                    .json::<Response>()
                    .await?;
                println!("{}\n--", response.text[0]);

                let link = match item.link() {
                    Some(t) => t,
                    None => {
                        "[EMPTY LINK]"
                    }
                };
                println!("{}\n-", link);
                println!("https://translate.yandex.com/translate?url={}&lang=it-en", link);
            }
        }
    }

    // @This feed doesn't include timestamps, so we just include everything:
    let channel = match Channel::from_url("https://www.serchioindiretta.it/cronaca/rss") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing URL: {}", e);
            std::process::exit(1);
        }
    };
    println!(">> Parsing '{}'...", channel.title());
    for item in channel.items() {
        let mut text = String::new();
        let title = match item.title() {
            Some(t) => t,
            None => {
                "[EMPTY TITLE]"
            }
        };
        println!("-----");
        text.push_str(title);
        text.push_str(": ");
        let description = match item.description() {
            Some(t) => t,
            None => {
                "[EMPTY DESCRIPTION]"
            }
        };
        text.push_str(description);

        let cleaned_text = Builder::new()
            .tags(tags.clone())
            .clean(&text)
            .to_string();
        println!("{}\n--", cleaned_text);

        // Translate
        let url = format!("{}?key={}&text={}&lang={}&format={}", base, api_key, strip_anchor(&text), lang, format);
        let response = reqwest::get(&url)
            .await?
            .json::<Response>()
            .await?;
        println!("{}\n--", response.text[0]);

        let link = match item.link() {
            Some(t) => t,
            None => {
                "[EMPTY LINK]"
            }
        };
        println!("{}\n-", link);
        println!("https://translate.yandex.com/translate?url={}&lang=it-en", link);
    }

    Ok(())
}
