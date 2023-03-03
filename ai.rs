use crate::rai;
use rai::Action::*;

pub struct Ai();

impl rai::Ai for Ai {
    fn run<'a>(&self, e: Box<dyn rai::AiEnv + 'a>) -> rai::Action {
        if e.no_resources(0.0, 0.0) > 0.0 {
            return Create;
        }

        if e.no_enemies(0.0, 0.0) > 0.0 && e.no_enemies(0.0, 0.0) < e.no_friends(0.0, 0.0) {
            return Nothing;
        }
        
        let r = self.rnd(e);
        let r: u8 = (r*8.0) as u8;
        return match r % 8 {
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

    impl Ai {
        fn rnd<'a>(&self, mut e: Box<dyn rai::AiEnv + 'a>) -> f32 {
            let s1 = e.get_memory(0.0);
            let s2 = s1 + 11.34;
            let s3 = s2.rem_euclid(1.0);
            e.set_memory(0.0, s3);
            s3
        }
    }

}
