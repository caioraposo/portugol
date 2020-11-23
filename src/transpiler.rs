use crate::ast::{Expression, Program, Statement};

const SPACES: u8 = 4;

pub struct Transpiler {
    program: Program,
}

impl Transpiler {
    pub fn new(program: Program) -> Self {
        Transpiler { program }
    }

    pub fn transpile(&self) {
        for stmt in &self.program.statements {
            self.parse_statement(stmt.clone(), 0);
        }
    }

    pub fn parse_statement(&self, stmt: Statement, indent_level: usize) {
        let mut spaces = String::new();
        for _ in 0..indent_level {
            for _ in 0..SPACES {
                spaces.push(' ');
            }
        }
        match stmt {
            Statement::Let(ident, value) => println!("{}{} = {}", spaces, ident, value),
            Statement::Return(None) => println!("{}return", spaces),
            Statement::Return(Some(exp)) => println!("{}return {}", spaces, exp),
            Statement::Print(None) => println!("{}print()", spaces),
            Statement::Print(Some(exp)) => println!("{}print({})", spaces, exp),
            Statement::Expression(exp) => match exp {
                Expression::Assign(name, value) => println!("{}{} = {}", spaces, name, value),
                Expression::If(condition, consequence, alternative) => {
                    println!("{}if {}:", spaces, condition);
                    for cons in &consequence.statements {
                        self.parse_statement(cons.clone(), indent_level + 1);
                    }
                    if let Some(alts) = alternative {
                        println!("{}else:", spaces);
                        for alt in &alts.statements {
                            self.parse_statement(alt.clone(), indent_level + 1);
                        }
                    }
                }
                Expression::While(condition, consequence) => {
                    println!("{}while {}:", spaces, condition);
                    for cons in &consequence.statements {
                        self.parse_statement(cons.clone(), indent_level + 1);
                    }
                }
                _ => {}
            },
        }
    }
}
