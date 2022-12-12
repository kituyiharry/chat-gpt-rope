// Code produced by chatgpt
enum Rope {
  Leaf(String),
  Node(Rope, Rope),
}

// Concatenate two ropes
fn append(r1: Rope, r2: Rope) -> Rope {
  match r1, r2 {
    Leaf(s1), Leaf(s2) => Leaf(format!("{}{}", s1, s2)),
    Leaf(s1), Node(l2, r2) => Node(Leaf(s1), Node(l2, r2)),
    Node(l1, r1), Leaf(s2) => Node(l1, Node(r1, Leaf(s2))),
    Node(l1, r1), Node(l2, r2) => Node(l1, append(r1, Node(l2, r2))),
  }
}

// Split a rope at a given index
fn split(r: Rope, i: usize) -> (Rope, Rope) {
  match r {
    Leaf(s) => {
      let len = s.len();
      if i == 0 {
        (Leaf("".to_string()), r)
      } else if i == len {
        (r, Leaf("".to_string()))
      } else {
        (
          Leaf(s[..i].to_string()),
          Leaf(s[i..len].to_string()),
        )
      }
    }
    Node(l, r) => {
      let len = length(l);
      if i == 0 {
        (Leaf("".to_string()), r)
      } else if i == len {
        (l, r)
      } else if i < len {
        let (l1, l2) = split(l, i);
        (l1, Node(l2, r))
      } else {
        let (r1, r2) = split(r, i - len);
        (Node(l, r1), r2)
      }
    }
  }
}

// Get the length of a rope
fn length(r: Rope) -> usize {
  match r {
    Leaf(s) => s.len(),
    Node(l, r) => length(l) + length(r),
  }
}

fn main() {
  // Use the rope functions here
}

