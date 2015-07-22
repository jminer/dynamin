
use std::sync::Arc;


enum Node {
	Internal {
		left: Arc<Node>,
		right: Arc<Node>,
		// cache
		height: i32,
		len: uint,
	},
	Leaf(String),
}

impl Node {
	fn new_internal(left: Arc<Node>, right: Arc<Node>) -> Node {
		Node::Internal {
			left: left,
			right: right,
			height: left.height() + right.height(),
			len: left.len() + right.len()
		}
	}
	
	unsafe fn new_unchecked<T: Iterator<u8>>(string: T, chunk_target_len: uint) {
		
		// TODO: search backwards using is_char_boundary() looking for a place to break
	}
	
	fn len(&self) -> uint {
		match self {
			&Node::Internal { ref len, .. } => *len,
			&Node::Leaf(ref s) => s.len()
		}
	}
	
	fn height(&self) -> i32 {
		match self {
			&Node::Internal { ref height, .. } => *height,
			&Node::Leaf(ref s) => 1
		}
	}
	
	fn insert_str(&self, idx: uint, string: &str, balance_factor: i32,
	              chunk_target_len: uint) -> Node {
		match self {
			&Node::Internal { ref height, .. } => {
				
			}
			&Node::Leaf(ref s) => {
				let (before, after) = s.split_at(idx);
				Node::new_unchecked(before.as_bytes()
				                    .chain(string.as_bytes())
				                    .chain(after.as_bytes()), chunk_target_len);
			}
		}
	}
}

pub struct ImmRope {
	root: Arc<Node>,
}

impl ImmRope {
	fn new() -> ImmRope {
		ImmRope {
			root: Arc::new(Node::Leaf(String::new()))
		}
	}
	
	fn insert(&self, idx: uint, ch: char) -> ImmRope {
		
	}
	
	fn bytes(&self) {
		
	}
	
	fn len(&self) -> uint {
		self.root.len()
	}
	
	fn chunk_target_len() -> int {
		3000
	}
}

fn main() {
	let x = ImmRope::new();
	println!("{}", x.len());
}
