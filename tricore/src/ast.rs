use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Sự kiện
    Fact { subject: String, predicate: String, object: String, tense: Option<String>, negation: bool },
    // Luật suy luận
    Rule { condition: Vec<Condition>, conclusion: Condition, relation: RuleRelation },
    // Câu hỏi
    Query { question_type: QuestionType, subject: String, predicate: String, object: String },
    // Lệnh
    Command { command_type: CommandType, action: String, target: Option<String> },
    // Cấu trúc điều khiển
    Print(Expression),
    Assign { name: String, value: Expression },
    IfElse { condition: Expression, then_body: Vec<Statement>, else_body: Option<Vec<Statement>> },
    ForLoop { var: String, start: Expression, end: Expression, body: Vec<Statement> },
    Function { name: String, params: Vec<String>, body: Vec<Statement> },
    Program { name: String, body: Vec<Statement> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Simple { subject: String, predicate: String, object: String },
    Negation(Box<Condition>),
    Conjunction(Box<Condition>, Box<Condition>),
    Disjunction(Box<Condition>, Box<Condition>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleRelation {
    Implication,      // nếu...thì
    Equivalence,      // nếu và chỉ nếu
    Transitive,       // bắc cầu
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuestionType {
    YesNo,            // có...không?
    What,             // là gì?
    Where,            // ở đâu?
    Who,              // ai?
    When,             // khi nào?
    How,              // như thế nào?
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Do,               // hãy
    Dont,             // đừng
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Variable(String),
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
            _ => write!(f, "..."),
        }
    }
}
