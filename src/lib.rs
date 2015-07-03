use std::convert::From;

use self::Node::*;
use self::Affix::*;
use self::FingerTree::*;

#[derive(Debug)]
pub enum Node<A> {
    Branch2([A;2]),
    Branch3([A;3]),
}

impl<A:Clone> Clone for Node<A> {
    fn clone(&self) -> Node<A> {
        match self {
            &Branch2(ref b) => Branch2([b[0].clone(), b[1].clone()]),
            &Branch3(ref b) => Branch3([b[0].clone(), b[1].clone(), b[2].clone()]),
        }
    }
}

impl<A> Node<A> {
    fn to_list(&self) -> &[A] {
        match self {
            &Branch2(ref x) => x,
            &Branch3(ref x) => x,
        }
    }
}

impl<A> From<[A; 2]> for Node<A> {
    fn from(input: [A; 2]) -> Node<A> {
        Branch2(input)
    }
}

impl<A> From<[A; 3]> for Node<A> {
    fn from(input: [A; 3]) -> Node<A> {
        Branch3(input)
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
            &Affix1(ref x) => x,
            &Affix2(ref x) => x,
            &Affix3(ref x) => x,
            &Affix4(ref x) => x,
        }
    }
}

impl<A> From<[A; 1]> for Affix<A> {
    fn from(input: [A; 1]) -> Affix<A> {
        Affix1(input)
    }
}

impl<A> From<[A; 2]> for Affix<A> {
    fn from(input: [A; 2]) -> Affix<A> {
        Affix2(input)
    }
}

impl<A> From<[A; 3]> for Affix<A> {
    fn from(input: [A; 3]) -> Affix<A> {
        Affix3(input)
    }
}

impl<A> From<[A; 4]> for Affix<A> {
    fn from(input: [A; 4]) -> Affix<A> {
        Affix4(input)
    }
}

impl<A:Clone> Affix<A> {
    fn push_front(&mut self, value: A)  {
        *self = match self {
            &mut Affix1(ref b) => Affix2([value, b[0].clone()]),
            &mut Affix2(ref b) => Affix3([value, b[0].clone(), b[1].clone()]),
            &mut Affix3(ref b) => Affix4([value, b[0].clone(), b[1].clone(), b[2].clone()]),
            &mut Affix4(_) => panic!("Can't append to Affix4"),
        };
    }

    fn push_back(&mut self, value: A)  {
        *self = match self {
            &mut Affix1(ref b) => Affix2([b[0].clone(), value]),
            &mut Affix2(ref b) => Affix3([b[0].clone(), b[1].clone(), value]),
            &mut Affix3(ref b) => Affix4([b[0].clone(), b[1].clone(), b[2].clone(), value]),
            &mut Affix4(_) => panic!("Can't append to Affix4"),
        };
    }
}

pub struct AffixData<A> {
    prefix: Affix<A>,
    suffix: Affix<A>,
}

pub enum FingerTree<A> {
    Empty,
    Single(A),
    DeepTwo{
        data: AffixData<A>,
        children: Option<Box<(FingerTree<A>, FingerTree<A>)>>,
    },
    DeepThree{
        data: AffixData<A>,
        children: Option<Box<(FingerTree<A>, FingerTree<A>, FingerTree<A>)>>,
    },
}

/*
#[test]
fn it_works() {
}
*/
