/*
[syntax]
expr = mul ("+" mul | "-" mul)*
mul = primary ("*" primary | "/" primary)*
primary = num | "(" expr ")"
*/

pub mod parser {
    pub enum NodeKind {
        Add,
        Sub,
        Mul,
        Div,
        Num,
    }

    pub struct Node {
        kind: NodeKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
        value: u32
    }

    fn new_node() {}
    fn expr() {}
    fn mul() {}
    fn primary() {}
}
