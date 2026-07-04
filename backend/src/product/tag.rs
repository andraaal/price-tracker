use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize)]
#[sqlx(type_name = "tag")]
pub enum Tag {
    Organic,
    Vegan,
    Vegetarian,
    GlutenFree,
    LactoseFree,
    FairTrade,
    AMASigil,
    Cooled,
    Frozen,
}

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bio" => Ok(Tag::Organic),
            "vegan" => Ok(Tag::Vegan),
            "vegetarisch" => Ok(Tag::Vegetarian),
            "glutenfrei" => Ok(Tag::GlutenFree),
            "laktosefrei" => Ok(Tag::LactoseFree),
            "fairtrade" => Ok(Tag::FairTrade),
            "ama gütesiegel" => Ok(Tag::AMASigil),
            "gekühlt" => Ok(Tag::Cooled),
            "tiefgekühlt" => Ok(Tag::Frozen),
            e => {
                if !e.is_empty() {
                    println!("Error parsing tag: {}", e);
                }
                Err(())
            }
        }
    }
}
