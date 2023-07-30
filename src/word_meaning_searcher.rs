mod request;
mod response;
mod parser;

pub struct WordMeaningSearcher {
    request: request::Request
}

impl WordMeaningSearcher {
    pub fn new(word: String) -> Self {
        Self {
            request: request::Request::new(word)
        }
    }

    pub async fn fetch_meaning(&self) -> response::MeaningResponse {
        self.request.fetch_meaning().await
    }
}