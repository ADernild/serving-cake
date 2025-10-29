use rand::seq::SliceRandom;

pub fn get_random_surprise() -> Option<String> {
    if rand::random::<bool>() {
        // 50% chance of getting a surprise
        let surprises = vec![
            "Lucky you, you got a strawberry!",
            "There's a cherry on top!",
            "Extra chocolate sprinkles for you!",
            "This slice has rainbow frosting!",
            "You found a hidden candy in your slice!",
        ];
        Some(
            surprises
                .choose(&mut rand::thread_rng())
                .unwrap_or(&"Enjoy your cake!")
                .to_string(),
        )
    } else {
        None
    }
}
