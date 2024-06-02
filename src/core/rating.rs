use colored::*;
pub fn colorize_rating(rating: &str) -> ColoredString {
    match rating {
        "Platinum" => rating.on_truecolor(180, 199, 220).black(),
        "Gold" => rating.on_truecolor(207, 181, 59).black(),
        "Silver" => rating.on_truecolor(166, 166, 166).black(),
        "Bronze" => rating.on_truecolor(205, 127, 50).black(),
        "Borked" => rating.on_red().black(),
        _ => panic!("Unexcepted ERROR!"),
    }
}
