use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_tree: BTreeMap<char, i32> = BTreeMap::new();

    for (value, characters) in h {
        for c in characters {
            new_tree.insert(c.to_ascii_lowercase(), *value);
        }
    }

    println!("new tree = {:?}", new_tree);

    new_tree
}