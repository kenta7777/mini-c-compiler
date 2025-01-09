/*
[syntax]
expr = mul ("+" mul | "-" mul)*
mul = primary ("*" primary | "/" primary)*
primary = num | "(" expr ")"
*/

pub mod parser {
    use crate::tokenizer::tokenizer::TokenKind;

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
    fn expr(tokens: Vec<TokenKind>) -> Box<Node> {
        let node = mul();
        let mut tokens = tokens.iter().peekable();
        //TODO: update node recursively
        while let Some(&token) = tokens.peek() {
            match *token {
                TokenKind::Plus => {
                    let node = new_node(NodeKind::Add, node, mul());
                }
                TokenKind::Minus => {
                    let node = new_node(NodeKind::Sub, node, mul());
                }
                _ => return node,
            }
        }

        new_node_number(3)
    }

    fn mul() -> Box<Node> {
        new_node_number(1)
    }
    fn primary() {}
}
