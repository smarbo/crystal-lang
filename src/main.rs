use colored::*;
use std::{
    collections::HashMap,
    env::args,
    fs::{self, read_to_string},
    process::exit,
};

use lexer::{Lexer, MathToken, Token};
use memories::{ast_to_memory, binary_op, Memory};
use parser::{ASTNode, Parser};

mod lexer;
mod memories;
mod parser;

fn get_args() -> Vec<String> {
    let run_args: Vec<String> = args().collect();
    run_args[1..run_args.len()].to_vec()
}

fn run(path: String) {
    let file = read_to_string(path.clone());
    if let Result::Err(_) = file {
        println!(
            "{}",
            format!(
                "{ce}{path}{ca}",
                ce = "CRYSTAL.Error: File '".bright_red(),
                ca = "' not found.".bright_red(),
                path = path.clone().bright_yellow(),
            )
        );
        exit(1)
    }
    let mut lexer = Lexer::new(file.expect("CRYSTAL.Error: File error."));
    let mut tokens = Vec::new();
    while let token = lexer.next_token() {
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let mut virtual_brain: HashMap<String, Box<Memory>> = HashMap::new();
    if let ASTNode::Program(nodes) = &ast {
        for node in nodes {
            match node {
                ASTNode::Let(ident, val) => {
                    virtual_brain.insert(
                        ident.to_string(),
                        Box::new(ast_to_memory(*val.clone(), true, &virtual_brain)),
                    );
                }
                ASTNode::Final(ident, val) => {
                    virtual_brain.insert(
                        ident.to_string(),
                        Box::new(ast_to_memory(*val.clone(), false, &virtual_brain)),
                    );
                }
                ASTNode::CompoundAssign { ident, op, value } => {
                    if let Some(mem) = virtual_brain.get(ident) {
                        if let Memory::Number(n, is_mut) = &**mem {
                            if *is_mut {
                                println!("{op:?}");
                                let new_value = match op {
                                    Token::Arithmetic(MathToken::PlusEq) => {
                                        n + binary_op(*value.clone(), &virtual_brain)
                                    }
                                    Token::Arithmetic(MathToken::MinusEq) => {
                                        n - binary_op(*value.clone(), &virtual_brain)
                                    }
                                    Token::Arithmetic(MathToken::MultiplyEq) => {
                                        n * binary_op(*value.clone(), &virtual_brain)
                                    }
                                    Token::Arithmetic(MathToken::DivideEq) => {
                                        n / binary_op(*value.clone(), &virtual_brain)
                                    }
                                    _ => panic!("CRY.ERROR: Invalid binary operation"),
                                };

                                virtual_brain.insert(
                                    ident.to_string(),
                                    Box::new(Memory::Number(new_value, *is_mut)),
                                );
                            } else {
                                panic!("CRY.ERROR: Cannot modify a final variable");
                            }
                        } else {
                            panic!("CRY.ERROR: Invalid memory type found for compound assignment");
                        }
                    } else {
                        panic!("CRY.ERROR: Memory '{ident}' not found");
                    }
                }
                _ => {}
            }
        }
    }

    println!("{virtual_brain:#?}");
    println!("{ast:#?}");
}

fn unknown_cmd(cmd: String) {
    println!(
        "{}",
        format!("CRYSTAL.Error: Command {cmd} not found! Run 'crystal help' for help on commands.")
            .bright_red()
    );
}

fn new_project(name: String) {
    println!("{}", format!("Creating new project '{name}'").cyan());
    fs::create_dir(format!("./{name}")).unwrap();
    fs::write(format!("./{name}/app.cry"), "let x = 5;\n").unwrap();
}

fn help() {
    let help_text = format!(
        "
{title}
Command List:

{run_cmd} {path_q}
- run a .cry file. if path unspecified, runs ./app.cry

{new_cmd} {name_q}
- create a new CRYSTAL project. 
- if name unspecified, creates an 'untitled_app'

{help_cmd}
- shows this help menu. 
- for more info, 
- head to https://github.com/smarbo/crystal-lang
",
        run_cmd = "crystal run".bold().green(),
        new_cmd = "crystal new".bold().blue(),
        help_cmd = "crystal help".bold().yellow(),
        path_q = "?PATH?".bold().blink(),
        name_q = "?NAME?".bold().blink(),
        title = "Welcome to CRYSTAL-Lang.".bold().cyan(),
    );
    println!("{}", format!("{help_text}"));
}

#[derive(Debug)]
enum Command {
    Run(String),
    New(String),
    None,
    Unknown,
}

fn main() {
    let run_args = get_args();
    let cmd = if run_args.len() >= 1 {
        match run_args[0].as_str() {
            "run" => Command::Run(if run_args.len() > 1 {
                run_args[1].clone()
            } else {
                String::from("app.cry")
            }),
            "new" => Command::New(if run_args.len() > 1 {
                run_args[1].clone()
            } else {
                String::from("untitled_app")
            }),
            "help" => Command::None,
            _ => Command::Unknown,
        }
    } else {
        Command::None
    };

    match cmd {
        Command::Run(f) => run(f),
        Command::New(name) => new_project(name),
        Command::Unknown => unknown_cmd(run_args[0].clone()),
        Command::None => help(),
    }
}
