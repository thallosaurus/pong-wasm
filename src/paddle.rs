use crate::{ui::Rect, game::Player};

#[derive(Clone, Copy)]
pub struct Paddle {
    pub area: Rect
}

impl Paddle {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            area: Rect {
                x, y, w, h
            }
        }
    }
}

impl From<Player> for Paddle {
    fn from(p: Player) -> Self {
        match p {
            Player::One => Self {
                area: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 50.0,
                    h: 50.0
                }
            },
            Player::Two => Self {
                area: Rect {
                    x: 200.0,
                    y: 200.0,
                    w: 50.0,
                    h: 50.0
                },
            }
        }
    }
}