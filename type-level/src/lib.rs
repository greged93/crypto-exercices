use std::marker::PhantomData;

struct Attack;
struct Defend;

#[derive(PartialEq, Debug)]
pub enum Action {
    Attack = 0,
    Defend = 1,
}

pub struct Character<State> {
    action: Action,
    _state: PhantomData<State>,
}

impl Character<Attack> {
    // use self and not &self in order to move the Character after action is called
    fn action(self) -> (Character<Defend>, Action) {
        (Character::new(), self.action)
    }
}

impl Character<Defend> {
    fn new() -> Self {
        Character {
            action: Action::Defend,
            _state: PhantomData,
        }
    }

    // use self and not &self in order to move the Character after action is called
    fn action(self) -> (Character<Attack>, Action) {
        (
            Character {
                action: Action::Attack,
                _state: PhantomData,
            },
            self.action,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        let hero: Character<Defend> = Character::new();
        let (hero, action) = hero.action();
        assert_eq!(Action::Defend, action);
        let (_, action) = hero.action();
        assert_eq!(Action::Attack, action);
    }
}
