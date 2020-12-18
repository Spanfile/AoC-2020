use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Peekable;

#[derive(Clone, Copy)]
struct PrecedenceRules {
    mul: i32,
    add: i32,
    group: i32,
}

#[derive(Debug)]
pub enum Token {
    Numeric(u64),
    Addition,
    Multiplication,
    OpenParentheses,
    CloseParentheses,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Value(u64),
    Addition { lhs: Box<Expression>, rhs: Box<Expression> },
    Multiplication { lhs: Box<Expression>, rhs: Box<Expression> },
    Group(Box<Expression>),
}

impl Expression {
    fn parse<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        precedence: i32,
        rules: PrecedenceRules,
    ) -> Expression {
        let mut lhs = match tokens.next() {
            Some(Token::Numeric(v)) => Expression::Value(*v),
            Some(Token::OpenParentheses) => {
                let group = Expression::Group(Box::new(Expression::parse(tokens, 0, rules)));
                if let Some(Token::CloseParentheses) = tokens.next() {
                    group
                } else {
                    panic!("missing closing parentheses");
                }
            }
            t => panic!(format!("unexpected token {:?}", t)),
        };

        while precedence
            < match tokens.peek() {
                Some(Token::Addition) => rules.add,
                Some(Token::Multiplication) => rules.mul,
                Some(Token::OpenParentheses) => rules.group,
                _ => 0,
            }
        {
            lhs = match tokens.peek() {
                Some(Token::Addition) => Expression::Addition {
                    lhs: Box::new(lhs),
                    rhs: {
                        tokens.next();
                        Box::new(Expression::parse(tokens, rules.add, rules))
                    },
                },
                Some(Token::Multiplication) => Expression::Multiplication {
                    lhs: Box::new(lhs),
                    rhs: {
                        tokens.next();
                        Box::new(Expression::parse(tokens, rules.mul, rules))
                    },
                },
                _ => lhs,
            };
        }

        lhs
    }

    fn eval(self) -> u64 {
        match self {
            Expression::Value(v) => v,
            Expression::Addition { lhs, rhs } => lhs.eval() + rhs.eval(),
            Expression::Multiplication { lhs, rhs } => lhs.eval() * rhs.eval(),
            Expression::Group(g) => g.eval(),
        }
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    '+' => Some(Token::Addition),
                    '*' => Some(Token::Multiplication),
                    '(' => Some(Token::OpenParentheses),
                    ')' => Some(Token::CloseParentheses),
                    c if c.is_digit(10) => Some(Token::Numeric(c.to_digit(10).unwrap() as u64)),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<Token>]) -> u64 {
    input
        .iter()
        .map(|t| {
            let mut tokens = t.iter().peekable();
            Expression::parse(
                &mut tokens,
                0,
                PrecedenceRules {
                    mul: 1,
                    add: 1,
                    group: 2,
                },
            )
            .eval()
        })
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vec<Token>]) -> u64 {
    input
        .iter()
        .map(|t| {
            let mut tokens = t.iter().peekable();
            Expression::parse(
                &mut tokens,
                0,
                PrecedenceRules {
                    mul: 1,
                    add: 2,
                    group: 3,
                },
            )
            .eval()
        })
        .sum()
}
