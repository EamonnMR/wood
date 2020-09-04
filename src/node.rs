#[derive(Clone)]
pub enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32),
    Nil,
    Function{
        params: Vec<ParseTreeNode>,
        proc: Vec<ParseTreeNode>
    }
}
