use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EmojisEnum {
    CryingFace,
    WorriedFace,
    NeutralFace,
    SlightlySimilingFace,
    GrinningFaceWithBigEyes,
    FaceGlasses,
    MoneyMouthFace,
    ExplodingHead,
    RedQuestionMark,
}

lazy_static! {
    pub static ref EMOJIS: HashMap<EmojisEnum, char> = {
        let mut emojis: HashMap<EmojisEnum, char> = HashMap::new();
        emojis.insert(EmojisEnum::CryingFace, '\u{1F622}');
        emojis.insert(EmojisEnum::WorriedFace, '\u{1F61F}');
        emojis.insert(EmojisEnum::NeutralFace, '\u{1F610}');
        emojis.insert(EmojisEnum::SlightlySimilingFace, '\u{1F642}');
        emojis.insert(EmojisEnum::GrinningFaceWithBigEyes, '\u{1F603}');
        emojis.insert(EmojisEnum::FaceGlasses, '\u{1F60E}');
        emojis.insert(EmojisEnum::MoneyMouthFace, '\u{1F911}');
        emojis.insert(EmojisEnum::ExplodingHead, '\u{1F92F}');
        emojis.insert(EmojisEnum::RedQuestionMark, '\u{2753}');

        emojis
    };
}

pub fn get_emoji (percentage_of_salary: f32) -> &'static char {
    let emoji = match percentage_of_salary {
        x if x <= 3.0 => EMOJIS.get(&EmojisEnum::CryingFace),
        x if x <= 5.0 => EMOJIS.get(&EmojisEnum::WorriedFace),
        x if x <= 10.0 => EMOJIS.get(&EmojisEnum::NeutralFace),
        x if x <= 15.0 => EMOJIS.get(&EmojisEnum::SlightlySimilingFace),
        x if x <= 25.0 => EMOJIS.get(&EmojisEnum::GrinningFaceWithBigEyes),
        x if x <= 40.0 => EMOJIS.get(&EmojisEnum::FaceGlasses),
        x if x <= 80.0 => EMOJIS.get(&EmojisEnum::MoneyMouthFace),
        x if x <= 100.0 => EMOJIS.get(&EmojisEnum::ExplodingHead),
        _ => EMOJIS.get(&EmojisEnum::RedQuestionMark),
    };

    return emoji.unwrap()
}