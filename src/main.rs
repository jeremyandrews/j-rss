use rss::Channel;
use serde::{Deserialize};

// This `derive` requires the `serde` dependency.
#[derive(Deserialize, Debug)]
struct Response {
    code: usize,
    lang: String,
    text: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let base = "https://translate.yandex.net/api/v1.5/tr.json/translate";
    //let api_key = "get from https://translate.yandex.com/developers/keys";
    let lang = "it-en";
    let format = "plain";

    let channel = match Channel::from_url("https://www.serchioindiretta.it/cronaca/rss") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing URL: {}", e);
            std::process::exit(1);
        }
    };
    println!("Parsing '{}'...", channel.title());
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
        println!("{}\n--", text);

        // Translate
        let url = format!("{}?key={}&text={}&lang={}&format={}", base, api_key, text, lang, format);
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

    // Translation
    // From: `en` to `it`
    // @TODO: before open-sourcing: https://tech.yandex.com/translate/doc/dg/concepts/design-requirements-docpage/
    // 
    // https://tech.yandex.com/translate/doc/dg/reference/translate-docpage/
    //
    // https://translate.yandex.net/api/v1.5/tr.json/translate
    //   ? key=<API key>
    //   & text=<text to translate>
    //   & lang=<translation direction>      |   it-en
    //   & [format=<text format>]            | plain, or html
    //  NOT USED  & [options=<translation options>]
    //  NOT USED  & [callback=<name of the callback function>]

    Ok(())
}
