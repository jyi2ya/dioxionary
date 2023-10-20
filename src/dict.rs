//! Look up words from the Internet.
use crate::error::{Error, Result};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use reqwest;
use rodio::{Decoder, OutputStream, Sink};
use scraper::{Html, Selector};
use std::fmt;
use std::io::Cursor;

/// Generate url for looking up.
fn gen_url(word: &str) -> String {
    format!("https://www.youdao.com/result?word={}&lang=en", word)
}

/// Is an English word?
fn is_enword(word: &str) -> bool {
    word.as_bytes()
        .iter()
        .all(|x| x.is_ascii_alphabetic() || x.is_ascii_whitespace())
}

/// Get web dictionary html by word.
async fn get_html(word: &str) -> Result<Html> {
    static APP_USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let url = gen_url(word);
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(Html::parse_document(&body))
}

/// Lookup words by Chinese meaning.
fn zh2en(html: &Html) -> Result<String> {
    let mut res = String::new();
    let trans = Selector::parse("ul.basic").map_err(|_| Error::HtmlParsingError)?;
    html.select(&trans).for_each(|x| {
        x.text().collect::<Vec<_>>().iter().for_each(|x| {
            res.push_str(x);
            res.push('\n');
        });
    });
    Ok(res)
}

/// Lookup words by English word.
fn en2zh(html: &Html) -> Result<String> {
    let mut res = String::new();
    let phonetic = Selector::parse(".per-phone").map_err(|_| Error::HtmlParsingError)?;
    html.select(&phonetic).for_each(|x| {
        x.text().collect::<Vec<_>>().iter().for_each(|x| {
            res.push_str(x);
            res.push(' ');
        });
    });
    res.push('\n');
    let mut pos_text: Vec<&str> = Vec::new();
    let pos = Selector::parse(".pos").map_err(|_| Error::HtmlParsingError)?;
    html.select(&pos).for_each(|x| {
        x.text().collect::<Vec<_>>().iter().for_each(|x| {
            pos_text.push(*x);
        });
    });
    let mut trans_text: Vec<&str> = Vec::new();
    let trans = Selector::parse(".trans").map_err(|_| Error::HtmlParsingError)?;
    html.select(&trans).for_each(|x| {
        x.text().collect::<Vec<_>>().iter().for_each(|x| {
            trans_text.push(*x);
        });
    });
    for i in pos_text
        .iter()
        .zip_longest(trans_text.iter())
        .map(|x| match x {
            Both(a, b) => (a, b),
            Left(a) => (a, &""),
            Right(b) => (&"", b),
        })
    {
        res.push_str(format!("{} {}\n", i.0, i.1).as_str());
    }
    Ok(res)
}

/// Get the diffculty level of the word from html.
fn get_exam_type(html: &Html) -> Result<Vec<String>> {
    let types = Selector::parse(".exam_type-value").map_err(|_| Error::HtmlParsingError)?;
    let mut res: Vec<String> = Vec::new();
    html.select(&types).for_each(|x| {
        x.text()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|x| res.push(x.to_string()))
    });
    Ok(res)
}

/// Word item from the web dictionary.
pub struct WordItem {
    /// The word being looked up.
    pub word: String,
    /// Is an English word?
    pub is_en: bool,
    /// The meaning or the translation of the word.
    pub trans: String,
    /// The diffculty level of the word(can be none or more than one).
    pub types: Option<Vec<String>>,
}

impl WordItem {
    /// Build a word item by looking up from the web dictionary.
    pub fn lookup(word: &str) -> Result<WordItem> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let html = get_html(word).await?;
            let is_en = is_enword(word);
            let dirction = if is_en { en2zh } else { zh2en };
            let trans = dirction(&html)?.trim().to_string();
            // cannot find the word
            if trans.is_empty() {
                Err(Error::WordNotFound("online".to_string()))
            } else {
                let types = if is_en {
                    Some(get_exam_type(&html)?)
                } else {
                    None
                };
                let word = word.to_owned();
                Ok(WordItem {
                    word,
                    is_en,
                    trans,
                    types,
                })
            }
        })
    }
}

impl fmt::Display for WordItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut types_contents = String::new();
        if let Some(types) = &self.types {
            types_contents.push('\n');
            types
                .iter()
                .for_each(|x| types_contents.push_str(&format!("<{}> ", x)))
        };
        write!(f, "{}\n{}{}", self.word, self.trans.trim(), types_contents)
    }
}

/// Play word pronunciation.
pub fn read_aloud(word: &str) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let url = format!("https://dict.youdao.com/dictvoice?audio={}&type=1", word);
    let response = reqwest::blocking::get(url)?;
    let inner = response.bytes()?;
    if let Ok(source) = Decoder::new(Cursor::new(inner)) {
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            sink.append(source);
            sink.sleep_until_end();
        }
    }
    Ok(())
}
