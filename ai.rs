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
        
        let cases = vec![
            (0.0, 1.0, MoveN),
            (1.0, 1.0, MoveNE),
            (1.0, 0.0, MoveE),
            (1.0, -1.0, MoveSE),
            (0.0, -1.0, MoveS),
            (-1.0, -1.0, MoveSW),
            (-1.0, 0.0, MoveW),
            (-1.0, 1.0, MoveNW)
        ];
        for (dx, dy, a) in cases.into_iter() {
            let p = { self.rnd(e)  };
            if e.no_resources(dx, dy) > 0.0 && e.no_enemies(dx, dy) < 1.0 && e.no_friends(dx, dy) < 1.0 && p < 0.5 {
                return a;
            }   
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

}


impl Ai {
    fn rnd<'a>(&self, e: &mut Box<dyn rai::AiEnv + 'a>) -> f32 {
        let s1 = e.get_memory(0.0);
        let s2 = s1 + 11.34;
        let s3 = s2.rem_euclid(1.0);
        e.set_memory(0.0, s3);
        s3
    }
}
