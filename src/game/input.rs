use stm32f4xx_hal::gpio::{Input, Pin};

#[derive(Debug)]
pub enum InputDirection {
    Stay,
    Up,
    Down,
}

pub enum LeftRightPosition {
    Left,
    Right,
}

struct FourButtonSettup<
    const P1: char,
    const P2: char,
    const P3: char,
    const P4: char,
    const N1: u8,
    const N2: u8,
    const N3: u8,
    const N4: u8,
> {
    left_up: Pin<P1, N1, Input>,
    left_down: Pin<P2, N2, Input>,
    right_up: Pin<P3, N3, Input>,
    right_down: Pin<P4, N4, Input>,
}

trait UserInteraction {
    fn get_user_input(&self, user_position: LeftRightPosition) -> InputDirection;
}

impl<
        const P1: char,
        const P2: char,
        const P3: char,
        const P4: char,
        const N1: u8,
        const N2: u8,
        const N3: u8,
        const N4: u8,
    > UserInteraction for FourButtonSettup<P1, P2, P3, P4, N1, N2, N3, N4>
{
    fn get_user_input(&self, user_position: LeftRightPosition) -> InputDirection {
        let up_button_pressed;
        let down_botton_pressed;
        match user_position {
            LeftRightPosition::Left => {
                up_button_pressed = self.left_up.is_high();
                down_botton_pressed = self.left_down.is_high();
            }
            LeftRightPosition::Right => {
                up_button_pressed = self.right_up.is_high();
                down_botton_pressed = self.right_down.is_high();
            }
        };
        match (up_button_pressed, down_botton_pressed) {
            (true, false) => InputDirection::Up,
            (false, true) => InputDirection::Down,
            (_, _) => InputDirection::Stay,
        }
    }
}
