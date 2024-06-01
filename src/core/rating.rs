use colored::*;
pub fn colorize_rating(rating: &str) -> ColoredString {
    match rating {
        "Platinum" => rating.on_truecolor(180, 199, 220),
        "Gold" => rating.on_truecolor(207, 181, 59),
        "Silver" => rating.on_truecolor(166, 166, 166),
        "Bronze" => rating.on_truecolor(205, 127, 50),
        "Borked" => rating.on_red(),
        _ => panic!("Unexcepted ERROR!"),
    }
}
