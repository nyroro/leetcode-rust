
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let mut available_recipes = HashSet::new();
        let mut recipe_map = HashMap::new();
        
        for (i, recipe) in recipes.iter().enumerate() {
            recipe_map.insert(recipe.clone(), ingredients[i].clone());
        }
        
        let mut available_supplies = HashSet::new();
        for supply in supplies {
            available_supplies.insert(supply);
        }
        
        let mut new_recipe_created = true;
        while new_recipe_created {
            new_recipe_created = false;
            for (recipe, required_ingredients) in &recipe_map {
                if available_recipes.contains(recipe) {
                    continue;
                }
                let mut can_create = true;
                for ingredient in required_ingredients {
                    if !available_supplies.contains(ingredient) && !available_recipes.contains(ingredient) {
                        can_create = false;
                        break;
                    }
                }
                if can_create {
                    available_recipes.insert(recipe.clone());
                    for ingredient in required_ingredients {
                        available_supplies.insert(ingredient.clone());
                    }
                    new_recipe_created = true;
                }
            }
        }
        
        available_recipes.into_iter().collect()
    }
}
