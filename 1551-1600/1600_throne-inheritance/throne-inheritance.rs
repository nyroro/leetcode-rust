
use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    children: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
}
