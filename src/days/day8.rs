use crate::prelude::*;

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn parse(mut data: &[u32]) -> Result<(Node, &[u32])> {
        if data.len() < 2 {
            bail!("Unexpected end of data");
        }
        let num_children = data[0];
        let num_metadata = data[1] as usize;
        let mut children = Vec::new();
        data = &data[2..];
        for _ in 0..num_children {
            let (child, tail) = Self::parse(data)?;
            children.push(child);
            data = tail;
        }
        if data.len() < num_metadata {
            bail!("Unexpected end of data");
        }
        let metadata = data[..num_metadata].to_vec();
        Ok((Node { children, metadata }, &data[num_metadata..]))
    }

    fn from_string<S: AsRef<str>>(s: S) -> Result<Node> {
        let data: Vec<u32> = s
            .as_ref()
            .split(" ")
            .map(|v| {
                v.parse()
                    .map_err(|e| format_err!("Failed to parse as u32: '{}' - {}", v, e))
            })
            .collect::<std::result::Result<_, _>>()?;
        let (node, tail) = Self::parse(&data)?;
        if !tail.is_empty() {
            bail!("Unexpected data after root node");
        }
        Ok(node)
    }

    fn sum_metadata(&self) -> u32 {
        self.children
            .iter()
            .map(|n| n.sum_metadata())
            .chain(self.metadata.iter().cloned())
            .sum()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().cloned().sum()
        } else {
            self.metadata
                .iter()
                .map(|&idx| {
                    if idx > 0 && idx as usize <= self.children.len() {
                        self.children[idx as usize - 1].value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

pub fn run(data: &AocData) -> AocResult {
    let tree = Node::from_string(data.to_string()?)?;
    answers(tree.sum_metadata(), tree.value())
}

#[cfg(test)]
#[test]
fn test() {
    let tree = Node::from_string("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2").unwrap();
    assert_eq!(tree.sum_metadata(), 138);
    assert_eq!(tree.value(), 66);
}
