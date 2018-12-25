use crate::prelude::*;
use std::i32;

pub fn run(data: &AocData) -> AocResult {
    let bots = parse_input(data)?;
    let tree = tree_from_bots(&bots);
    println!("Tree size: {}", tree.len());
    answer(num_in_range_of_strongest(&bots))
}

struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

impl Bot {
    fn dist(&self, x: i32, y: i32, z: i32) -> u32 {
        (x - self.x).abs() as u32 + (y - self.y).abs() as u32 + (z - self.z).abs() as u32
    }

    fn in_range(&self, x: i32, y: i32, z: i32) -> bool {
        self.dist(x, y, z) <= self.r
    }
}

fn parse_input(data: &AocData) -> Result<Vec<Bot>> {
    let mut bots = Vec::new();
    let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    for line in data.lines()? {
        if let Some(caps) = re.captures(&line) {
            bots.push(Bot {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
                r: caps[4].parse().unwrap(),
            });
        } else {
            bail!("Failed to parse line: {}", line);
        }
    }
    if bots.is_empty() {
        bail!("No bots found in input");
    }
    Ok(bots)
}

fn num_in_range_of_strongest(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter()
        .filter(|b| strongest.in_range(b.x, b.y, b.z))
        .count()
}

fn tree_from_bots(bots: &[Bot]) -> BspTree {
    let mut tree = new_tree();
    for (index, bot) in bots.iter().enumerate() {
        let new_box = bot_to_box(bot);
        add_box(&mut tree, &new_box);
        println!("Tree after {} bots: {}", index + 1, tree.len());
    }
    tree
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    A, // +x +y +z
    B, // +x +y -z
    C, // +x -y +z
    D, // +x -y -z
}

enum BspNode {
    Leaf {
        num_in_range: usize,
    },
    Branch {
        axis: Axis,
        split_at: i32,
        children: u32,
    },
}

type BspTree = Vec<BspNode>;

#[derive(Clone)]
struct Box4d {
    a_min: i32,
    a_max: i32,
    b_min: i32,
    b_max: i32,
    c_min: i32,
    c_max: i32,
    d_min: i32,
    d_max: i32,
}

fn bot_to_box(bot: &Bot) -> Box4d {
    let a = bot.x + bot.y + bot.z;
    let b = bot.x + bot.y - bot.z;
    let c = bot.x - bot.y + bot.z;
    let d = bot.x - bot.y - bot.z;
    let r = bot.r as i32;
    Box4d {
        a_min: a - r,
        a_max: a + r + 1,
        b_min: b - r,
        b_max: b + r + 1,
        c_min: c - r,
        c_max: c + r + 1,
        d_min: d - r,
        d_max: d + r + 1,
    }
}

fn new_tree() -> BspTree {
    vec![BspNode::Leaf { num_in_range: 0 }]
}

fn add_box(tree: &mut BspTree, new_box: &Box4d) {
    fn add_box_node(tree: &mut BspTree, node: usize, node_box: Box4d, new_box: &Box4d) {
        loop {
            match tree[node] {
                BspNode::Leaf {
                    ref mut num_in_range,
                } => {
                    if new_box.a_min > node_box.a_min && new_box.a_min < node_box.a_max {
                        split_node(tree, node, Axis::A, new_box.a_min);
                        continue;
                    } else if new_box.a_max > node_box.a_min && new_box.a_max < node_box.a_max {
                        split_node(tree, node, Axis::A, new_box.a_max);
                        continue;
                    } else if new_box.b_min > node_box.b_min && new_box.b_min < node_box.b_max {
                        split_node(tree, node, Axis::B, new_box.b_min);
                        continue;
                    } else if new_box.b_max > node_box.b_min && new_box.b_max < node_box.b_max {
                        split_node(tree, node, Axis::B, new_box.b_max);
                        continue;
                    } else if new_box.c_min > node_box.c_min && new_box.c_min < node_box.c_max {
                        split_node(tree, node, Axis::C, new_box.c_min);
                        continue;
                    } else if new_box.c_max > node_box.c_min && new_box.c_max < node_box.c_max {
                        split_node(tree, node, Axis::C, new_box.c_max);
                        continue;
                    } else if new_box.d_min > node_box.d_min && new_box.d_min < node_box.d_max {
                        split_node(tree, node, Axis::D, new_box.d_min);
                        continue;
                    } else if new_box.d_max > node_box.d_min && new_box.d_max < node_box.d_max {
                        split_node(tree, node, Axis::D, new_box.d_max);
                        continue;
                    }
                    *num_in_range += 1;
                }
                BspNode::Branch {
                    axis,
                    split_at,
                    children,
                } => {
                    let children = children as usize;
                    let mut left_box = node_box.clone();
                    let mut right_box = node_box;
                    match axis {
                        Axis::A => {
                            if new_box.a_min < split_at {
                                left_box.a_max = split_at;
                                add_box_node(tree, children, left_box, new_box);
                            }
                            if new_box.a_max > split_at {
                                right_box.a_min = split_at;
                                add_box_node(tree, children + 1, right_box, new_box);
                            }
                        }
                        Axis::B => {
                            if new_box.b_min < split_at {
                                left_box.b_max = split_at;
                                add_box_node(tree, children, left_box, new_box);
                            }
                            if new_box.b_max > split_at {
                                right_box.b_min = split_at;
                                add_box_node(tree, children + 1, right_box, new_box);
                            }
                        }
                        Axis::C => {
                            if new_box.c_min < split_at {
                                left_box.c_max = split_at;
                                add_box_node(tree, children, left_box, new_box);
                            }
                            if new_box.c_max > split_at {
                                right_box.c_min = split_at;
                                add_box_node(tree, children + 1, right_box, new_box);
                            }
                        }
                        Axis::D => {
                            if new_box.d_min < split_at {
                                left_box.d_max = split_at;
                                add_box_node(tree, children, left_box, new_box);
                            }
                            if new_box.d_max > split_at {
                                right_box.d_min = split_at;
                                add_box_node(tree, children + 1, right_box, new_box);
                            }
                        }
                    }
                }
            }
            break;
        }
    }

    fn split_node(tree: &mut BspTree, node: usize, axis: Axis, split_at: i32) {
        let num_in_range = match tree[node] {
            BspNode::Leaf { num_in_range } => num_in_range,
            BspNode::Branch { .. } => panic!("Tried to split branch node"),
        };
        let children = tree.len() as u32;
        tree.push(BspNode::Leaf { num_in_range });
        tree.push(BspNode::Leaf { num_in_range });
        tree[node] = BspNode::Branch {
            axis,
            split_at,
            children,
        };
    }

    add_box_node(
        tree,
        0,
        Box4d {
            a_min: i32::MIN,
            a_max: i32::MAX,
            b_min: i32::MIN,
            b_max: i32::MAX,
            c_min: i32::MIN,
            c_max: i32::MAX,
            d_min: i32::MIN,
            d_max: i32::MAX,
        },
        new_box,
    );
}
