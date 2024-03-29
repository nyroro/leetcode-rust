
use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x

    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_index: HashMap<String, usize> = HashMap::new();
        let mut email_to_name: HashMap<String, String> = HashMap::new();
        let mut uf = UnionFind::new(accounts.len());

        for (i, account) in accounts.iter().enumerate() {
            let name = &account[0];
            for j in 1..account.len() {
                let email = &account[j];
                email_to_name.insert(email.clone(), name.clone());
                if let Some(&index) = email_to_index.get(email) {
                    uf.union(index, i);
                } else {
                    email_to_index.insert(email.clone(), i);
                }
            }
        }

        let mut index_to_email: HashMap<usize, Vec<String>> = HashMap::new();
        for (email, &index) in email_to_index.iter() {
            let root = uf.find(index);
            index_to_email.entry(root).or_insert(Vec::new()).push(email.clone());
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for (index, emails) in index_to_email.iter_mut() {
            emails.sort();
            let name = &email_to_name.get(&emails[0]).unwrap();
            let mut account: Vec<String> = Vec::new();
            account.push(name.to_string());
            account.append(emails);
            result.push(account);
        }

        result

    }
}
