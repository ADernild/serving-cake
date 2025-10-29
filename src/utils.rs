use rand::seq::SliceRandom;

pub fn get_random_surprise() -> Option<String> {
    if rand::random::<bool>() {
        // 50% chance of getting a surprise
        let surprises = vec![
            // Sweet surprises
            "Lucky you, you got a strawberry! 🍓",
            "There's a cherry on top! 🍒",
            "Extra chocolate sprinkles for you! 🍫",
            "This slice has rainbow frosting! 🌈",
            "You found a hidden candy in your slice! 🍬",
            "Caramel drizzle surprise! 🍯",
            "A golden chocolate coin! 🪙",
            "Mini marshmallows hidden inside! 🧸",

            // Funny/weird surprises
            "Eew, a severed toe! 🩸",
            "A tiny plastic dinosaur! 🦖",
            "A fortune cookie with your slice! 🥠",
            "A single gummy bear! 🐻",
            "A mystery flavor (hope you like adventure)! 🤔",

            // Pop culture references
            "A 'Live Long and Prosper' sprinkle! 🖖",
            "A 'Cake or Death?' note (choose wisely)! ☠️",
            "A 'You shall pass!' icing message! 🏰",

            // Interactive surprises
            "A riddle: What has to be broken before you can use it? (Answer: An egg!)",
            "A mini puzzle piece! 🧩",
            "A QR code to a cat video! 🐱",

            // Wholesome surprises
            "A compliment: You're awesome! 🌟",
            "A hug coupon (redeemable anytime)! 🤗",
            "A 'You made someone smile today' note! 😊",

            // Absurd surprises
            "A single sock (where's its pair?)... 🧦",
            "A tiny umbrella (for your tiny cake)! ☔",
            "A 'This cake is a lie' note! 🎂",

            // Nostalgic surprises
            "A retro candy (like Pop Rocks)! 💥",
            "A '90s throwback: a Tamagotchi pixel! 🎮",
            "A 'You've been chosen by the cake fairy!' note! 🧚",
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
