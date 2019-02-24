use crate::nodes::*;
use crate::tokenizer::*;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<CommandNode, String> {
        match self.peek_type() {
            TokenType::Add => self.parse_add(),
            TokenType::Show => self.parse_show(),
            TokenType::ShowAll => self.parse_show_all(),
            TokenType::Quit => self.parse_quit(),
            _ => Err(format!("Syntax error: {}", &self.tokens[0].value)),
        }
    }

    fn parse_add(&mut self) -> Result<CommandNode, String> {
        self.consume(&TokenType::Add)?;
        let name_token = self.consume(&TokenType::Identifier)?;
        self.consume(&TokenType::To)?;
        let department_token = self.consume(&TokenType::Identifier)?;
        Ok(CommandNode::Add(AddNode {
            name: name_token.value,
            department: department_token.value,
        }))
    }

    fn parse_show(&mut self) -> Result<CommandNode, String> {
        self.consume(&TokenType::Show)?;
        let token = self.consume(&TokenType::Identifier)?;
        Ok(CommandNode::Show(ShowNode {
            department: token.value,
        }))
    }

    fn parse_show_all(&mut self) -> Result<CommandNode, String> {
        self.consume(&TokenType::ShowAll)?;
        Ok(CommandNode::ShowAll)
    }

    fn parse_quit(&mut self) -> Result<CommandNode, String> {
        self.consume(&TokenType::Quit)?;
        Ok(CommandNode::Quit)
    }

    fn consume(&mut self, expected_type: &TokenType) -> Result<Token, String> {
        if self.tokens.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let token = self.tokens.drain(..1).next().unwrap();
        if &token.token_type == expected_type {
            Ok(token)
        } else {
            Err(format!(
                "Expected token type {:?}, but got {:?}",
                expected_type, &token.token_type
            ))
        }
    }

    fn peek_type(&self) -> &TokenType {
        &self.tokens[0].token_type
    }
}
