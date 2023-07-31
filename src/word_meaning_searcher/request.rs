use crate::word_meaning_searcher::response::MeaningResponse;

use async_openai::{
    Client,
    types::{
        CreateChatCompletionRequestArgs,
        ChatCompletionRequestMessage,
        Role,
        CreateChatCompletionRequest
    },
    error::OpenAIError
};

pub struct Request <'a> {
    word: &'a String,
    client: Client,
}

impl <'a> Request <'a> {
    pub fn new(word: &'a String) -> Self {
        Self {
            word,
            client: Client::new(),
        }
    }

    pub async fn fetch_meaning(&self) -> MeaningResponse {
        let request = match self.generate_request_args() {
                Ok(request) => request,
                Err(e) => panic!("Error: {}", e)
        };
        let response = match self.client.chat().create(request).await {
            Ok(response) => response,
            Err(e) => panic!("Error: {}", e)
        };
        let result = response.choices[0].message.content.clone();
        MeaningResponse::parse(result)
    }

    fn generate_request_args(&self) -> Result<CreateChatCompletionRequest, OpenAIError> {
        let messages = vec![
            Self::generate_system_message(),
            self.generate_user_message(),
            Self::generate_assistant_message(),
        ];
        CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo-16k")
            .messages(messages)
            .temperature(0.0)
            .user("async-openai")
            .build()
    }

    fn generate_system_message() -> ChatCompletionRequestMessage {
        Self::build_request_message(Role::System, "あなたは良き英語翻訳者であり、語彙も豊富です".to_string())
    }

    fn generate_user_message(&self) -> ChatCompletionRequestMessage {
        Self::build_request_message(Role::User, format!("下記のフォーマットに従って右記の単語について教えてください: {}", self.word))
    }

    fn generate_assistant_message() -> ChatCompletionRequestMessage {
        let message: String = "\
          意味: <簡潔に意味だけ記載>,\n\
          頻度: <ネイティブが会話で使用する頻度を1~10(heads-upを8とした基準)で表すとして次の４つから選択(low(1-2), medium(3-5), high(6-8), very high(9-10))>,\n\
          例文: <改行して回答>,\n\
          collocations: <改行して回答(今回の語彙+よく一緒に使用される語彙で記載)>\
        ".to_string();

        Self::build_request_message(Role::Assistant, message)
    }

    fn build_request_message(role: Role, content: String) -> ChatCompletionRequestMessage {
        ChatCompletionRequestMessage {
            role,
            content,
            name: None,
        }
    }
}