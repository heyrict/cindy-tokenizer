use mecab::{Node, Tagger};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

fn strip_symbols<'a>(s: &'a str) -> Cow<'a, str> {
    lazy_static! {
        static ref SYMBOL_RE: Regex =
            Regex::new(r#"[a-zA-Z\s`~!@#$%^&*()-_=+\[\]{}|\\'":<>,./?]+"#).unwrap();
    }
    SYMBOL_RE.replace_all(s, " ")
}

fn simplify_features<'a>(s: &'a str) -> (String, String) {
    lazy_static! {
        static ref POS_RE: Regex = Regex::new(
            r#"^(?P<pos>[^,]+),(?P<pos_detail_1>[^,]+),[^,]+,[^,]+,"
                       "[^,]+,[^,]+,(?P<orig>[^,]+)"#
        )
        .unwrap();
    }
    let caps = POS_RE.captures(&s).unwrap();
    (
        format!("{},{}", &caps["pos"], &caps["pos_detail_1"]),
        (&caps["orig"]).to_owned(),
    )
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    poc: String,
}

impl From<Node> for Token {
    fn from(node: Node) -> Token {
        let (text, poc) = simplify_features(&node.feature);
        Token { text, poc }
    }
}

impl Token {
    pub fn get_poc(&self) -> Poc {
        let poc_str: &str = self.poc.split(',').next().unwrap();
        match poc_str {
            "名詞" => Poc::Noun,
            "動詞" => Poc::Verb,
            "形容詞" => Poc::Adjactive,
            "副詞" => Poc::Adverb,
            "助詞" => Poc::Partical,
            "接続詞" => Poc::Conjunctive,
            "助動詞" => Poc::Partical,
            "連体詞" => Poc::AuxilliaryVerb,
            "感動詞" => Poc::Determiner,
            _ => Poc::Others,
        }
    }

    pub fn get_poc_detail(&self) -> &str {
        let mut iter = self.poc.split(',');

        // Drop poc
        iter.next();
        // Returns poc_detail_1
        iter.next().unwrap_or("*")
    }
}

pub enum Poc {
    Noun,
    Verb,
    Adjactive,
    Adverb,
    Partical,
    Conjunctive,
    AuxilliaryVerb,
    Determiner,
    Interjection,
    Others,
}

pub fn tokenize<'a>(text: &'a str) -> Vec<Token> {
    let text = strip_symbols(text);

    let mut tagger = Tagger::new("");

    tagger
        .parse_to_node(text.as_ref())
        .iter_next()
        .map(|node| match node.stat as i32 {
            mecab::MECAB_BOS_NODE => None,
            mecab::MECAB_EOS_NODE => None,
            _ => Some(Token::from(node)),
        })
        .flatten()
        .collect::<Vec<Token>>()
}
