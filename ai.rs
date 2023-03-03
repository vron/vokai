use crate::rai;
use rai::Action::*;

pub struct Ai();

impl rai::Ai for Ai {
    fn run<'a>(&self, e: Box<dyn rai::AiEnv + 'a>) -> rai::Action {
        let r: u8 = 3;
        return match r % 9 {
            0 => MoveN,
            1 => MoveNE,
            2 => MoveE,
            3 => MoveSE,
            4 => MoveS,
            5 => MoveSW,
            6 => MoveW,
            7 => MoveNW,
            8 => Create,
            _ => Nothing,
        };
    }
}
