#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

//Add an entity with Player component tag to World
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

//Add a monster entity with Enemy component tag to World
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=7 => goblin(),
        8 => ettin(),
        9 => ogre(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MovingRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

fn ettin() -> (i32, String, FontCharType) {
    (1, "Ettin".to_string(), to_cp437('E'))
}

fn ogre() -> (i32, String, FontCharType) {
    (2, "Ogre".to_string(), to_cp437('O'))
}
