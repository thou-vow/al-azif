use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid character when tokenizing expression: {0}")]
    InvalidCharacter(char),
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Unmatched opening parenthesis")]
    UnmatchedOpeningParenthesis,
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    OpenParen,
    CloseParen,
}

pub enum ASTNode {
    Number(f64),
    UnaryAdd(Box<ASTNode>),
    BinaryAdd(Box<ASTNode>, Box<ASTNode>),
    UnarySub(Box<ASTNode>),
    BinarySub(Box<ASTNode>, Box<ASTNode>),
    Mul(Box<ASTNode>, Box<ASTNode>),
    Div(Box<ASTNode>, Box<ASTNode>),
    Exp(Box<ASTNode>, Box<ASTNode>),
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut chars = expr.char_indices().peekable();

    while let Some(&(i, ch)) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut decimal_point_seen = false;

                while let Some(&(_, ch)) = chars.peek() {
                    match ch {
                        '0'..='9' => {
                            chars.next();
                        },
                        '.' | ',' if !decimal_point_seen => {
                            chars.next();
                            decimal_point_seen = true;
                        },
                        _ => {
                            break;
                        }
                    }
                }

                let end = chars.peek().map_or(expr.len(), |&(j, _)| j);

                let number_str= &expr[i..end];
                tokens.push(Token::Number(number_str.parse().unwrap()));
            },
            '+' => {
                tokens.push(Token::Add);
                chars.next();
            },
            '-' => {
                tokens.push(Token::Sub);
                chars.next();
            },
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
            },
            '/' => {
                tokens.push(Token::Div);
                chars.next();
            },
            '^' => {
                tokens.push(Token::Exp);
                chars.next();
            },
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            },
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            },
            _ => return Err(Error::InvalidCharacter(ch))
        }
    }

    Ok(tokens)
}

pub fn parse(tokens: &[Token]) -> Result<ASTNode, Error> {
    fn parse_lvl_0(tokens: &[Token]) -> Result<(ASTNode, &[Token]), Error> {
        let (mut left, mut tokens) = parse_lvl_1(tokens)?;

        while let Some(token) = tokens.first() {
            match token {
                Token::Add => {
                    let (right, new_tokens) = parse_lvl_1(&tokens[1..])?;
                    left = ASTNode::BinaryAdd(Box::new(left), Box::new(right));
                    tokens = new_tokens;
                },
                Token::Sub => {
                    let (right, new_tokens) = parse_lvl_1(&tokens[1..])?;
                    left = ASTNode::BinarySub(Box::new(left), Box::new(right));
                    tokens = new_tokens;
                },
                _ => break,
            };
        }

        Ok((left, tokens))
    }

    fn parse_lvl_1(tokens: &[Token]) -> Result<(ASTNode, &[Token]), Error> {
        let (mut left, mut tokens) = parse_lvl_2(tokens)?;

        while let Some(token) = tokens.first() {
            match token {
                Token::Mul => {
                    let (right, new_tokens) = parse_lvl_2(&tokens[1..])?;
                    left = ASTNode::Mul(Box::new(left), Box::new(right));
                    tokens = new_tokens;
                },
                Token::Div => {
                    let (right, new_tokens) = parse_lvl_2(&tokens[1..])?;
                    left = ASTNode::Div(Box::new(left), Box::new(right));
                    tokens = new_tokens;
                },
                _ => break,
            };
        }

        Ok((left, tokens))
    }

    fn parse_lvl_2(tokens: &[Token]) -> Result<(ASTNode, &[Token]), Error> {
        let (mut left, mut tokens) = parse_lvl_3(tokens)?;

        while let Some(token) = tokens.first() {
            match token {
                Token::Exp => {
                    let (right, new_tokens) = parse_lvl_3(&tokens[1..])?;
                    left = ASTNode::Exp(Box::new(left), Box::new(right));
                    tokens = new_tokens;
                },
                _ => break,
            };
        }

        Ok((left, tokens))
    }

    fn parse_lvl_3(tokens: &[Token]) -> Result<(ASTNode, &[Token]), Error> {
        if let Some(token) = tokens.first() {
            match token {
                Token::Add => {
                    let (node, tokens) = parse_lvl_3(&tokens[1..])?;
                    return Ok((ASTNode::UnaryAdd(Box::new(node)), tokens));
                },
                Token::Sub => {
                    let (node, tokens) = parse_lvl_3(&tokens[1..])?;
                    return Ok((ASTNode::UnarySub(Box::new(node)), tokens));
                },
                _ => {},
            };
        }

        parse_lvl_4(tokens)
    }

    fn parse_lvl_4(tokens: &[Token]) -> Result<(ASTNode, &[Token]), Error> {
        if let Some(token) = tokens.first() {
            match token {
                Token::Number(n) => {
                    return Ok((ASTNode::Number(*n), &tokens[1..]));
                },
                Token::OpenParen => {
                    let (node, tokens) = parse_lvl_0(&tokens[1..])?;
                    if let Some(Token::CloseParen) = tokens.first() {
                        return Ok((node, &tokens[1..]));
                    } else {
                        return Err(Error::UnmatchedOpeningParenthesis);
                    }
                },
                _ => {},
            }
        }
        
        Err(Error::UnexpectedToken)
    }

    let (tree, _) = parse_lvl_0(tokens)?;

    Ok(tree)
}

pub fn evaluate(node: &ASTNode) -> Result<f64, Error> {
    match node {
        ASTNode::Number(n) => Ok(*n),
        ASTNode::UnaryAdd(node) => Ok(evaluate(node)?),
        ASTNode::BinaryAdd(left, right) => Ok(evaluate(left)? + evaluate(right)?),
        ASTNode::UnarySub(node) => Ok(-evaluate(node)?),
        ASTNode::BinarySub(left, right) => Ok(evaluate(left)? - evaluate(right)?),
        ASTNode::Mul(left, right) => Ok(evaluate(left)? * evaluate(right)?),
        ASTNode::Div(left, right) => Ok(evaluate(left)? / evaluate(right)?),
        ASTNode::Exp(left, right) => Ok(evaluate(left)?.powf(evaluate(right)?)),
    }
}