use reqwest::{Proxy, Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use std::sync::Arc;
use serde::{Deserialize,Serialize};
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize,  Debug)]
pub struct OAIbody {
    pub model: String,
    pub messages: Vec<Message>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OAIChoice {
    pub index: u8,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    choices: Vec<OAIChoice>
}

pub async fn sentmessages(oaibody: & OAIbody) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://api.openai.com/v1/chat/completions";

    let oai_token: String = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in env");
    let auth_header_val = format!("Bearer {}", oai_token);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header_val).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let proxy = "http://127.0.0.1:7890";

    let client = Client::builder()
        .proxy(Proxy::all(proxy)?)
        .build()?;

    let  resp = client
        .post(uri)
        .headers(headers.clone())
        .json(& oaibody)
        .send()
        .await?;
    
    let resp = resp.json::<OAIResponse>().await.expect("Failed to parse response");

    let choices = resp.choices;
    // println!("{:?}", choices);
    let message =  choices[0].message.clone();
    // println!("{:?}", message);
    Ok(message)
}



#[derive(Deserialize, Debug)]
pub struct Token {
   pub refresh_token: String,
   pub access_token: String,
   pub expires_in: u32,
}

lazy_static! {
    static ref TOKEN: Arc<Mutex<Option<Token>>> = Arc::new(Mutex::new(None));
}

fn is_token_valid(token: &Token) -> bool {
    true
}

pub async fn get_token() -> Result<Token, Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id=u0O3E6vTdt4NRFmGICsKGxfr&client_secret=PKbMcCoralHnSGC4wklhpm0bneLpSwGG";
    let resp = Client::new()
        .post(uri)
        .send()
        .await?
        .json::<Token>()
        .await?;
    // println!("{:?}", resp);
    Ok(resp)
}
pub async fn get_or_refresh_token() -> Result<Arc<Mutex<Option<Token>>>, Box<dyn std::error::Error + Send + Sync>> {
    let mut token_lock = TOKEN.lock().unwrap();

    if token_lock.is_none() || !is_token_valid(token_lock.as_ref().unwrap()) {
        *token_lock = Some(get_token().await?);
    }
    Ok(Arc::clone(&TOKEN))
}
// pub async fn refresh_token() -> Result<Token, Box<dyn std::error::Error + Send + Sync>> {
//     pass;
// }

#[derive(Deserialize, Debug)]
pub struct BaiduResponse {
    pub result: String,
}

pub async fn sentmessages_baidu(messages: &Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions";
    
    let access_token = get_or_refresh_token().await?.lock().unwrap().as_ref().unwrap().access_token.clone();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let params = [("access_token", access_token)];
    let resp = Client::new()
        .post(uri)
        .headers(headers.clone())
        .query(&params)
        .json(messages)
        .send()
        .await?
        .json::<BaiduResponse>()
        .await?;
    println!("{:?}", resp);
    Ok(resp.result)
}