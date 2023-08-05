mod parser;
mod request;
mod response;

pub struct WordMeaningSearcher<'b> {
    request: request::Request<'b>,
}

impl<'b> WordMeaningSearcher<'b> {
    pub fn new(word: &'b String) -> Self {
        Self {
            request: request::Request::new(word),
        }
    }

    pub async fn fetch_meaning(&self) -> response::MeaningResponse {
        self.request.fetch_meaning().await
    }
}
