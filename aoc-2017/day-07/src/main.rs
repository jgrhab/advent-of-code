use std::collections::{HashMap, HashSet};

struct NodeData {
    weight: u32,
    children: Vec<String>,
}

/// Parses one line of the input.
/// Returns the name and data of the node defined by the line.
fn parse_input_line(line: &str) -> (String, NodeData) {
    let mut iter = line.split_whitespace();

    let name = iter.next().unwrap().to_string();

    let weight = iter
        .next()
        .unwrap()
        .trim_matches(['(', ')'])
        .parse()
        .unwrap();

    let _ = iter.next(); // skip "->"

    let children = iter.fold(Vec::new(), |mut vec, s| {
        vec.push(s.trim_end_matches(',').to_string());
        vec
    });

    (name, NodeData { weight, children })
}

/// Structure containing the result of evaluating a subtree.
///
/// A subtree is balanced if all of its children subtrees have the same weight.
/// For a balanced subtree, the `value` contains the weight of the subtree,
/// which is the sum of the weight of the subtree root and the weights of all
/// children subtrees.
/// For an imbalanced subtree, the value contains the weight that the (unique)
/// imbalanced node should have to make the subtree balanced.
/// The imbalanced node is the node causing the imbalance, it is itself balanced.
struct SubTreeResult {
    is_balanced: bool,
    value: u32,
}

struct Tree {
    nodes: HashMap<String, NodeData>,
}

impl Tree {
    /// Finds the root of the tree.
    fn root(&self) -> &str {
        let nodes: HashSet<_> = self.nodes.keys().collect();
        let nodes_with_parent = self.nodes.values().fold(HashSet::new(), |mut set, data| {
            set.extend(data.children.iter());
            set
        });

        nodes.difference(&nodes_with_parent).next().unwrap()
    }

    /// Evaluates a subtree recursively.
    fn evaluate_subtree(&self, node: &str) -> SubTreeResult {
        let data = self.nodes.get(node).unwrap();

        // A leaf is always balanced and the weight of its subtree is the leaf weight.
        if data.children.is_empty() {
            return SubTreeResult {
                is_balanced: true,
                value: data.weight,
            };
        }

        // compute the SubTreeResult for all children nodes
        let mut children_res = Vec::with_capacity(data.children.len());

        for child in data.children.iter() {
            let child_res = self.evaluate_subtree(child);

            // if we have already found the imbalanced node, propagate the result
            // all the way up the call stack
            if !child_res.is_balanced {
                return child_res;
            }

            children_res.push(child_res);
        }

        // count the number of occurences of each weight amongst the children
        let weight_count = children_res.iter().fold(HashMap::new(), |mut map, res| {
            *map.entry(res.value).or_insert(0) += 1;
            map
        });

        // If the subtree is balanced (all children subtrees have the same weight),
        // return the weight of the subtree
        if weight_count.keys().len() == 1 {
            let (weight, count) = weight_count.into_iter().next().unwrap();

            return SubTreeResult {
                is_balanced: true,
                value: data.weight + weight * count,
            };
        }

        // At this point, not all children subtrees have the same weight.

        // There are two distinct weight values.
        // One value appears only once (the minority weight) and is the
        // weight of the child node that needs adjusting to balance the
        // current node. All other child nodes have the same weight
        // (the majority weight).

        let majority_weight: u32 = *weight_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0;

        let minority_weight: u32 = *weight_count
            .iter()
            .min_by_key(|&(_, count)| count)
            .unwrap()
            .0;

        // Find the node which needs adjusting. This is the node
        // whose weight is the minority weight.
        let (minority_idx, _) = children_res
            .iter()
            .enumerate()
            .find(|(_, res)| res.value == minority_weight)
            .unwrap();

        let imbalanced_child = &data.children[minority_idx];

        // Find the weight of the imbalanced child node and update it to match
        // the weight of the other child nodes.
        let imbalanced_weight = self.nodes.get(imbalanced_child).unwrap().weight as i32;
        let weight_delta = majority_weight as i32 - minority_weight as i32;

        SubTreeResult {
            is_balanced: false,
            value: (imbalanced_weight + weight_delta) as u32,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let nodes =
        input
            .lines()
            .map(parse_input_line)
            .fold(HashMap::new(), |mut map, (name, data)| {
                map.insert(name, data);
                map
            });

    let tree = Tree { nodes };

    // --- Part One --- //

    let part_one = tree.root();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let result = tree.evaluate_subtree(tree.root());

    let part_two = result.value;

    println!("Part Two: {}", part_two);
}
