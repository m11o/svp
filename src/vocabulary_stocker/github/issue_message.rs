pub struct IssueMessage<'a> {
    meaning: &'a String,
    collocations: &'a Vec<String>,
    examples: &'a Vec<String>,
}

impl<'b> IssueMessage<'b> {
    pub fn new(meaning: &'b String, collocations: &'b Vec<String>, examples: &'b Vec<String>) -> Self {
        Self {
            meaning,
            collocations,
            examples
        }
    }

    pub fn build_issue_body(&self) -> String {
        let mut message = self.meaning_message();
        message.push_str("\n");
        message.push_str(self.collocations_message().as_str());
        message.push_str("\n");
        message.push_str(self.examples_message().as_str());
        message
    }

    fn meaning_message(&self) -> String {
        let title = "意味";
        let body = format!("\
            <details>\n\
            <summary>答えを見る</summary>\n\
            \n\
            ```\n\
            {meaning}\n\
            ```\n\
            \n\
            </details>\
            ",
            meaning=*self.meaning
        );
        Self::default_issue_template(title, body.as_str())
    }

    fn collocations_message(&self) -> String {
        let title = "コロケーション";
        let body = format!("\
            {collocations}\
            ",
            collocations=self.collocations.join("\n")
        );
        Self::default_issue_template(title, body.as_str())
    }

    fn examples_message(&self) -> String {
        let title = "例文";
        let body = format!("\
            {examples}\
            ",
            examples=self.examples.join("\n")
        );
        Self::default_issue_template(title, body.as_str())
    }

    fn default_issue_template(title: &str, body: &str) -> String {
        format!("\
            # {title}\n\
            \n\
            {body}\n\
            ",
            title=title,
            body=body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_meaning_message() {
        let instance = IssueMessage {
            meaning: &String::from("meaning"),
            collocations: &vec![String::from("collocations")],
            examples: &vec![String::from("examples")]
        };

        let message = instance.meaning_message();
        let expected = format!("\
            # 意味\n\
            \n\
            <details>\n\
            <summary>答えを見る</summary>\n\
            \n\
            ```\n\
            meaning\n\
            ```\n\
            \n\
            </details>\n\
        ");
        assert_eq!(message, expected);
    }

    #[test]
    fn it_create_collocations_message() {
        let instance = IssueMessage {
            meaning: &String::from("meaning"),
            collocations: &vec![String::from("collocations1"), String::from("collocations2")],
            examples: &vec![String::from("examples")]
        };

        let message = instance.collocations_message();
        let expected = format!("\
            # コロケーション\n\
            \n\
            collocations1\n\
            collocations2\n\
            "
        );
        assert_eq!(message, expected);
    }

    #[test]
    fn it_create_examples_message() {
        let instance = IssueMessage {
            meaning: &String::from("meaning"),
            collocations: &vec![String::from("collocations1")],
            examples: &vec![String::from("examples1"), String::from("examples2")]
        };

        let message = instance.examples_message();
        let expected = format!("\
            # 例文\n\
            \n\
            examples1\n\
            examples2\n\
        ");
        assert_eq!(message, expected);
    }

    #[test]
    fn it_create_issue_message() {
        let instance = IssueMessage {
            meaning: &String::from("meaning"),
            collocations: &vec![String::from("collocations1"), String::from("collocations2")],
            examples: &vec![String::from("examples1"), String::from("examples2")]
        };

        let message = instance.build_issue_body();
        let expected = format!("\
            # 意味\n\
            \n\
            <details>\n\
            <summary>答えを見る</summary>\n\
            \n\
            ```\n\
            meaning\n\
            ```\n\
            \n\
            </details>\n\
            \n\
            # コロケーション\n\
            \n\
            collocations1\n\
            collocations2\n\
            \n\
            # 例文\n\
            \n\
            examples1\n\
            examples2\n\
        ");
        assert_eq!(message, expected);
    }
}
