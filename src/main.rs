// fixed from gpt output
#[derive(Debug,Clone)]
pub enum Rope<'a> {
  Leaf(Cow<'a, str>),
  Node(Box<Rope<'a>>, Box<Rope<'a>>),
}

use std::borrow::Cow;

use Rope::*;

// Concatenate two ropes
fn append<'a>(r1: Rope<'a>, r2: Rope<'a>) -> Rope<'a> {
    match (r1, r2) {
      (Leaf(s1), Leaf(s2)) => {
         let d = format!("{}{}", s1, s2);
         Leaf(Cow::Owned(d))
      },
      (Leaf(s1), Node(l2, r2)) => Node(Box::new(Leaf(s1)), Box::new(Node(l2, r2))),
      (Node(l1, r1), Leaf(s2)) => Node(l1, Box::new(Node(r1, Box::new(Leaf(s2))))),
      (Node(l1, r1), Node(l2, r2)) => Node(l1, Box::new(append(*r1, Node(l2, r2)))),
  }
}

// Split a rope at a given index
fn split<'a>(r: Rope<'a>, i: usize) -> (Rope<'a>, Rope<'a>) {
  match r {
    Leaf(ref s) => {
      let len = s.len();
      if i == 0 {
        (Leaf(Cow::Borrowed("")), r)
      } else if i == len {
        (r, Leaf(Cow::Borrowed("")))
      } else {
        (
          Leaf(Cow::Owned((&s[..i]).to_owned())),
          Leaf(Cow::Owned((&s[i..len]).to_owned())),
        )
      }
    }
    Node(l, r) => {
      let len = length(&l);
      if i == 0 {
        (Leaf(Cow::Borrowed("")), *r)
      } else if i == len {
        (*l, *r)
      } else if i < len {
        let (l1, l2) = split(*l, i);
        (l1, Node(Box::new(l2), r))
      } else {
        let (r1, r2) = split(*r, i - len);
        (Node(l, Box::new(r1)), r2)
      }
    }
  }
}

// Get the length of a rope
fn length(r: &Rope) -> usize {
  match r {
    Leaf(s) => s.len(),
    Node(l, r) => length(l) + length(r),
  }
}

fn main() {
  // Use the rope functions here
  use Rope::*;
  let ff = r#"
  Lorem ipsum dolor sit amet, consetetur sadipscing elitr,
  ed diam nonumy eirmod tempor invidunt ut labore et dolore
  magna aliquyam erat, sed diam voluptua. At vero eos et accusam
  et justo duo dolores et ea rebum. Stet clita kasd gubergren,
  no sea takimata sanctus est Lorem ipsum dolor sit amet."#;
  let xx = r#"
  the quick brown fox jumped over the lazy dog
    "#;
  let gg = Cow::Borrowed(ff);
  let hh = Cow::Borrowed(xx);
  let r =  Node(Box::new(Leaf(gg)), Box::new(Leaf(Cow::Owned("".to_owned()))));
  println!("{:#?}",r);
  let (g, f) = split(r, 10);
  println!("{:#?}", g);
  let b = append(f, Leaf(hh));
  println!("{:#?}", b);
  let p = append(g, b);
  println!("{:#?}", p);
}

