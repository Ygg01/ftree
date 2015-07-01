use std::convert::From;

#[derive(Debug)]
pub enum Node<A> {
    Branch2([A;2]),
    Branch3([A;3]),
}

impl<A> Node<A> {
    fn to_list(&self) -> &[A] {
        match self {
            &Node::Branch2(ref x) => x,
            &Node::Branch3(ref x) => x,
        }
    }
}

impl<A> From<[A; 2]> for Node<A> {
    fn from(input: [A; 2]) -> Node<A> {
        Node::Branch2(input)
    }
}

impl<A> From<[A; 3]> for Node<A> {
    fn from(input: [A; 3]) -> Node<A> {
        Node::Branch3(input)
    }
}

#[derive(Debug)]
pub enum Affix<A> {
    Affix1([A; 1]),
    Affix2([A; 2]),
    Affix3([A; 3]),
    Affix4([A; 4]),
}

impl<A> Affix<A> {
    fn to_list(&self) -> &[A] {
        match self {
            &Affix::Affix1(ref x) => x,
            &Affix::Affix2(ref x) => x,
            &Affix::Affix3(ref x) => x,
            &Affix::Affix4(ref x) => x,
        }
    }
}

impl<A> From<[A; 1]> for Affix<A> {
    fn from(input: [A; 1]) -> Affix<A> {
        Affix::Affix1(input)
    }
}

impl<A> From<[A; 2]> for Affix<A> {
    fn from(input: [A; 2]) -> Affix<A> {
        Affix::Affix2(input)
    }
}

impl<A> From<[A; 3]> for Affix<A> {
    fn from(input: [A; 3]) -> Affix<A> {
        Affix::Affix3(input)
    }
}

impl<A> From<[A; 4]> for Affix<A> {
    fn from(input: [A; 4]) -> Affix<A> {
        Affix::Affix4(input)
    }
}

//TODO append/prepend


pub enum FingerTree<A> {
    Empty,
    Single(A),
    Deep(DeepTree<A>),
}


pub struct DeepTree<A> {
    prefix: Affix<A>,
    deeper: Box<FingerTree<Node<A>>>,
    suffix: Affix<A>,
}
/*
#[test]
fn it_works() {
}
*/
