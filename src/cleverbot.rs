use std::sync::Arc;
use ringbuffer::{RingBuffer, AllocRingBuffer};
use crate::{cookie_generation::get_cookie, helpers::pythonic_encode};
use std::str;
use tracing;
use parking_lot::RwLock;  // these bastards lied to me, theres no holding-across-an-await-point detection here


#[derive(Debug, Clone)]
pub struct Cleverbot {
    pub cookie: Arc<RwLock<String>>,
    pub history: Arc<RwLock<AllocRingBuffer<String>>>,
    pub client: reqwest::Client,
    pub with_retries: bool,
}


impl Cleverbot {
    const BAD_RESPONSES: &'static [&'static str] = &["Hello from Cleverbot\n", "<html"];

    /// get a response from cleverbot
    pub async fn get_response(&self, message: &str) -> Result<String, crate::Error> {
        let payload = self.build_payload(message).await;

        match self.send_and_handle_response(&payload, message).await {
            Ok(answer) => return Ok(answer),
            Err(why) if !self.with_retries => return Err(why),
            _ => {}  // when there's an err and with_retries is true
        }

        let new_cookie = get_cookie(&self.client).await?;

        *self.cookie.write() = new_cookie;

        self.send_and_handle_response(&payload, message).await
            .map_err(|e| match e {
                crate::Error::BadResponse(why) => crate::Error::BadResponseAfterRetrying(why),
                other => other
            })
    }

    /// returns the history as a vec of strings
    pub fn get_history(&self) -> Vec<String> {
        (*self.history.read()).iter().cloned().collect::<Vec<_>>()
    }

    /// ok nah i aint making this 💀
    // pub fn get_history_iter(&self) -> impl Iterator<Item = String> + '_ {
    //     // let history = (*self.history.read()).iter().cloned().collect::<Vec<_>>();
    //     // history.into_iter()
    //     (*self.history.read()).iter().cloned()
    // }

    /// manual reset cookie
    pub async fn recookie(&self) -> Result<(), crate::Error> {
        let new_cookie = get_cookie(&self.client).await?;
        *self.cookie.write() = new_cookie;
        Ok(())
    }

    /// helper for sending and handling responses from the api
    async fn send_and_handle_response(&self, payload: &str, message: &str) -> Result<String, crate::Error> {
        let answer = self.send_cleverbot_request(payload).await?;

        if Self::BAD_RESPONSES.contains(&&*answer) {
            return Err(crate::Error::BadResponse(answer.clone()))
        }

        let mut history = self.history.write();
        history.push(message.to_string());
        history.push(answer.clone());

        Ok(answer)
    }

    /// stimulus - user's input message, question to cleverbot
    async fn build_payload(&self, stimulus: &str) -> String {
        let stimulus_str = format!("stimulus={}", pythonic_encode(stimulus));

        let context_str = self.history.read()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, text)| format!("&vText{}={}", i + 2, pythonic_encode(text)))
            .collect::<String>();

        let cb_settings_str = "&cb_settings_scripting=no&islearning=1&icognoid=wsf&icognocheck=";

        let partial_payload = format!("{}{}{}", stimulus_str, context_str, cb_settings_str);

        // i dont know why, i dont wanna know why, but this is just needed at the end (the name speaks for itself)
        let magic_ingredient = format!("{:x}", md5::compute(&partial_payload[7..33]));

        let payload = format!("{}{}", partial_payload, magic_ingredient);

        tracing::debug!("payload: {payload}");

        payload
    }

    async fn send_cleverbot_request(&self, payload: &str) -> Result<String, crate::Error> {
        let cookie = self.cookie.read().clone();
        let bytes_res = self.client.post("https://www.cleverbot.com/webservicemin?uc=UseOfficialCleverbotAPI")
            .body(payload.to_string())
            .header("cookie", cookie)
            // .header("accept-encoding", "gzip, deflate")
            .header("user-agent", "python-requests/2.32.3")
            .send()
            .await?
            .bytes()
            .await?;

        tracing::debug!("bytes_res: {:?}", bytes_res);
        
        let text = str::from_utf8(&bytes_res)?;
        let response = text.split('\r').next().ok_or(crate::Error::InvalidResponseFromCleverbotApi)?;

        Ok(response.into())
    }
}

pub struct CleverbotBuilder {
    client: reqwest::Client,
    with_retries: bool,
    history_size: usize,
}

impl Default for CleverbotBuilder {
    fn default() -> Self {
        Self { 
            client: reqwest::Client::new(), 
            with_retries: true, 
            history_size: Self::DEFAULT_HISTORY_SIZE 
        }
    }
}

impl CleverbotBuilder {
    const DEFAULT_HISTORY_SIZE: usize = 50;

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// by default `with_retries` is set to `true`
    pub fn with_retries(mut self, with_retries: bool) -> Self {
        self.with_retries = with_retries;
        self
    }

    pub fn with_custom_history_size(mut self, history_size: usize) -> Self {
        self.history_size = history_size;
        self
    }

    pub async fn build(self) -> Result<Cleverbot, crate::Error> {
        let cookie = get_cookie(&self.client).await?;
        tracing::debug!("cookie: {cookie}");

        Ok(Cleverbot {
            cookie: Arc::new(RwLock::new(cookie)),
            history: Arc::new(RwLock::new(AllocRingBuffer::<String>::new(self.history_size))),
            client: self.client,
            with_retries: self.with_retries
        })
    }
}
