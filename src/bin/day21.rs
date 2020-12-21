use structopt::StructOpt;
use shared::{ FileContentOpts, regex };
use std::collections::{ HashSet, HashMap };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let foods: Vec<_> = parse_input(&opts.file).collect();

    // Find out relationships between allergens and ingredients that may contain them:
    let mut atoi: HashMap<&str, HashSet<&str>> = HashMap::new();
    for food in &foods {
        let ingredient_set: HashSet<_> = food.ingredients.iter().map(|&s| s).collect();
        for allergen in &food.allergens {
            let curr_ingredients = atoi.entry(allergen).or_insert_with(|| ingredient_set.clone());
            *curr_ingredients = curr_ingredients.intersection(&ingredient_set).map(|&s| s).collect();
        }
    }

    // Squash that into a big set of "possibly-contains-allergen" ingredients
    let risky_ingredients: HashSet<&str> = atoi.values().flat_map(|s| s.iter()).map(|&s| s).collect();
    let ok_count = foods.iter().flat_map(|f| f.ingredients.iter()).filter(|&i| !risky_ingredients.contains(i)).count();
    println!("Star 1: {}", ok_count);

    // Reduce the set of allergens to one per food (assumes that there is exactly 1 per food)
    let mut dangerous_ingredients = vec![];
    while let Some((a,i)) = atoi.iter().find(|(_, ingredients)| ingredients.len() == 1) {
        let allergen = *a;
        let ingredient = *i.iter().next().unwrap();
        dangerous_ingredients.push((allergen,ingredient));
        atoi = atoi.into_iter()
            .filter(|(a,_)| *a != allergen)
            .map(|(a,mut i)| { i.remove(ingredient); (a,i) })
            .collect();
    }
    dangerous_ingredients.sort_by_key(|(a,_)| *a);
    let danger_list = dangerous_ingredients.into_iter().map(|(_,i)| i).collect::<Vec<_>>().join(",");
    println!("Star 2: {}", danger_list);

    Ok(())
}

#[derive(Debug,Clone)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>
}

fn parse_input<'a>(s: &'a str) -> impl Iterator<Item=Food<'a>> + 'a {
    s.lines().filter_map(|line| {
        let caps = regex!(r#"([a-z ]+) \(contains ([a-z, ]+)\)"#).captures(line.trim())?;
        let ingredients = caps.get(1)?.as_str().split(" ").filter(|s| !s.is_empty()).collect();
        let allergens = caps.get(2)?.as_str().split(", ").filter(|s| !s.is_empty()).collect();
        Some(Food { ingredients, allergens })
    })
}