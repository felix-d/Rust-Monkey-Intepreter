use std::fmt;

pub struct Program {
    pub statements: Vec<Statement>,
}

pub type Identifier = String;

#[derive(PartialEq)]
pub enum Statement {
    Let { name: Identifier, value: Expression },
    Return(Expression),
    Expression(Expression),
}

impl Statement {
    pub fn new_let(identifier: &str, value: Expression) -> Self {
        Statement::Let {
            name: String::from(identifier),
            value,
        }
    }

    pub fn new_return(return_value: Expression) -> Self {
        Statement::Return(return_value)
    }

    pub fn new_expression(expression: Expression) -> Self {
        Statement::Expression(expression)
    }
}

#[derive(PartialEq)]
pub enum Expression {
    IntegerLiteral(i64),
    Identifier(Identifier),
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: String,
    },
    Boolean(bool),
    IfExpression {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FunctionLiteral {
        parameters: Vec<Identifier>,
        body: BlockStatement,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    pub fn new_integer(value: i64) -> Expression {
        Expression::IntegerLiteral(value)
    }

    pub fn new_identifier(value: &str) -> Self {
        Expression::Identifier(String::from(value))
    }

    pub fn new_prefix(operator: &str, right: Expression) -> Self {
        Expression::Prefix {
            operator: String::from(operator),
            right: Box::new(right),
        }
    }

    pub fn new_infix(operator: &str, left: Expression, right: Expression) -> Self {
        Expression::Infix {
            left: Box::new(left),
            right: Box::new(right),
            operator: String::from(operator),
        }
    }

    pub fn new_boolean(value: bool) -> Self {
        Expression::Boolean(value)
    }

    pub fn new_if(
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Expression::IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }
    }

    pub fn new_function_literal(parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Expression::FunctionLiteral { parameters, body }
    }

    pub fn new_call_expression(function: Expression, arguments: Vec<Expression>) -> Self {
        Expression::CallExpression {
            function: Box::new(function),
            arguments,
        }
    }
}

#[derive(PartialEq)]
pub struct BlockStatement(Vec<Statement>);

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        BlockStatement(statements)
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{:?}", statement)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {} = {:?};", name, value),
            Statement::Return(expression) => write!(f, "return {:?};", expression),
            Statement::Expression(expression) => write!(f, "{:?}", expression),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::IntegerLiteral(literal) => write!(f, "{}", literal),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::Prefix { operator, right } => write!(f, "({}{:?})", operator, right),
            Expression::Infix {
                left,
                right,
                operator,
            } => write!(f, "({:?} {} {:?})", left, operator, right),
            Expression::Boolean(value) => write!(f, "{}", value),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if ({:?}) {:?}", condition, consequence)?;
                match alternative {
                    Some(ref alternative) => write!(f, "else {:?}", alternative),
                    None => Ok(()),
                }
            }
            Expression::FunctionLiteral { parameters, body } => {
                let params = parameters
                    .iter()
                    .map(|param| format!("{:?}", param))
                    .collect::<Vec<String>>();
                write!(f, "fn ({}) {:?}", params.join(", "), body)
            }
            Expression::CallExpression {
                function,
                arguments,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|arg| format!("{:?}", arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{:?}({})", function, arguments)
            }
        }
    }
}

impl fmt::Debug for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let statements = self
            .0
            .iter()
            .map(|statement| format!("{:?}", statement))
            .collect::<Vec<String>>()
            .join("; ");

        write!(f, "{{{}}}", statements)
    }
}
