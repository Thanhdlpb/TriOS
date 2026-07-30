use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Fact { subject: String, predicate: String, object: String, tense: Option<String>, negation: bool },
    Rule { condition: Vec<Condition>, conclusion: Condition, relation: RuleRelation },
    Query { question_type: QuestionType, subject: String, predicate: String, object: String },
    Command { command_type: CommandType, action: String, target: Option<String> },
    Print(Expression),
    Assign { name: String, value: Expression },
    IfElse { condition: Expression, then_body: Vec<Statement>, else_body: Option<Vec<Statement>> },
    ForLoop { var: String, start: Expression, end: Expression, body: Vec<Statement> },
    Function { name: String, params: Vec<String>, body: Vec<Statement> },
    ChuongTrinh { name: String, body: Vec<Statement> },
    UseModule { path: String }, // <<< THÊM MODULE
}

// ... (giữ nguyên các enum khác)
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Simple { subject: String, predicate: String, object: String },
    Negation(Box<Condition>),
    Conjunction(Box<Condition>, Box<Condition>),
    Disjunction(Box<Condition>, Box<Condition>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleRelation { Implication, Equivalence, Transitive }

#[derive(Debug, Clone, PartialEq)]
pub enum QuestionType { YesNo, What, Where, Who, When, How }

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType { Do, Dont }

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64), String(String), Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Fact { subject, predicate, object, .. } => write!(f, "{} {} {}.", subject, predicate, object),
            Statement::Query { subject, predicate, object, .. } => write!(f, "{} {} {}?", subject, predicate, object),
            Statement::Print(expr) => write!(f, "in {:?}", expr),
            Statement::Assign { name, value } => write!(f, "{} = {:?}", name, value),
            Statement::ChuongTrinh { name, .. } => write!(f, "chương trình '{}'", name),
            Statement::UseModule { path } => write!(f, "dùng \"{}\"", path),
            _ => write!(f, "..."),
        }
    }
}
