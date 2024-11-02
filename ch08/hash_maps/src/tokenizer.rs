use regex::Regex;

pub struct Tokenizer {
    source: String,
}

impl Tokenizer {
    pub fn new(source: String) -> Tokenizer {
        Tokenizer { source }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.source != "" {
            let token = self.tokenize_one_token()?;
            self.source = self.source[token.value.len()..].trim().to_string();
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn tokenize_one_token(&self) -> Result<Token, String> {
        let type_regexes = [
            (
                TokenType::Quit,
                Regex::new(r"(?i)\A(?:quit|exit)\b").unwrap(),
            ),
            (TokenType::Add, Regex::new(r"(?i)\Aadd\b").unwrap()),
            (TokenType::To, Regex::new(r"(?i)\Ato\b").unwrap()),
            (TokenType::Show, Regex::new(r"(?i)\Ashow\b").unwrap()),
            (TokenType::ShowAll, Regex::new(r"(?i)\Ashowall\b").unwrap()),
            (TokenType::Identifier, Regex::new(r"\A\w+").unwrap()),
        ];

        for type_regex in type_regexes.iter() {
            match self.find_expression(type_regex) {
                None => continue,
                Some(token) => return Ok(token),
            }
        }
        Err(format!("Could not match a token to: \"{}\"", &self.source))
    }

    fn find_expression(&self, (token_type, regex): &(TokenType, Regex)) -> Option<Token> {
        let value = regex.find(&self.source)?.as_str().to_string();
        Some(Token {
            token_type: token_type.clone(),
            value,
        })
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Add,
    Identifier,
    Show,
    ShowAll,
    To,
    Quit,
}
