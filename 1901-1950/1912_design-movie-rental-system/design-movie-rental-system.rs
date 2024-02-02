
use std::collections::{HashMap, BTreeSet};

struct MovieRentingSystem {
    avail: HashMap<i32, BTreeSet<(i32, i32)>>,  // movie -> (price, shop)
    prices: HashMap<(i32, i32), i32>,  // (shop, movie) -> price

    rented: BTreeSet<(i32, i32, i32)>,  // (price, shop, movie)
}

impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut avail = HashMap::new();
        let mut prices = HashMap::new();
        let rented = BTreeSet::new();

        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            prices.insert((shop, movie), price);
            avail.entry(movie).or_insert(BTreeSet::new()).insert((price, shop));
        }

        MovieRentingSystem { avail, prices, rented }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.avail.get(&movie).map_or(Vec::new(), |set| {
            set.iter().take(5).map(|&(_, shop)| shop).collect()
        })
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        if let Some(price) = self.prices.get(&(shop, movie)) {
            self.avail.get_mut(&movie).map(|set| {
                set.remove(&(*price, shop));
            });
            self.rented.insert((*price, shop, movie));
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        if let Some(price) = self.prices.get(&(shop, movie)) {
            self.avail.entry(movie).or_insert(BTreeSet::new()).insert((*price, shop));
            self.rented.remove(&(*price, shop, movie));
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented.iter().take(5).map(|&(_, shop, movie)| vec![shop, movie]).collect()
    }
}
