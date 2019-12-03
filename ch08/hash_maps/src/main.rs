extern crate regex;

use std::collections::HashMap;
use std::io::{self, Write};

mod nodes;
mod parser;
mod tokenizer;

use crate::nodes::*;
use crate::parser::*;
use crate::tokenizer::*;

fn main() {
    let mut company: Company = Company::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Could not write prompt.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input.");
        if input.trim().is_empty() {
            continue;
        }

        let tokenize_result = Tokenizer::new(input).tokenize();
        if let Err(message) = tokenize_result {
            println!("Error: {}", message);
            continue;
        }
        let parse_result = Parser::new(tokenize_result.unwrap()).parse();
        if let Err(message) = parse_result {
            println!("Error: {}", message);
            continue;
        }

        match parse_result.unwrap() {
            CommandNode::Add(add_node) => company.add_name(add_node),
            CommandNode::Show(show_node) => company.print_department(&show_node),
            CommandNode::ShowAll => company.print_company(),
            CommandNode::Quit => break,
        }
    }
}

struct Company {
    data: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            data: HashMap::new(),
        }
    }

    fn add_name(&mut self, add_node: AddNode) {
        self.data
            .entry(add_node.department)
            .or_insert(vec![])
            .push(add_node.name);
    }

    fn print_company(&self) {
        println!("{:#?}", self.data);
    }

    fn print_department(&self, show_node: &ShowNode) {
        if let Some(names) = self.data.get(&show_node.department) {
            let mut display_vector = names.clone();
            display_vector.sort();
            println!("{:#?}", display_vector);
        } else {
            println!("Department \"{}\" does not exist.", &show_node.department);
        }
    }
}
