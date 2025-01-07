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

    pub enum Node {
        Operator {
            kind: NodeKind,
            lhs: Box<Node>,
            rhs: Box<Node>,
        },
        Number {
            value: u32,
        },
    }

    fn new_node(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        let node = Node::Operator {
            kind: kind,
            lhs: lhs,
            rhs: rhs,
        };

        Box::new(node)
    }

    fn new_node_number(number: u32) -> Box<Node> {
        let node = Node::Number { value: number };

        Box::new(node)
    }
    fn expr() {}
    fn mul() {}
    fn primary() {}
}
