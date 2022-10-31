use stm32f4xx_hal::{
    adc::{config::SampleTime, Adc},
    gpio::{Analog, Pin},
    hal::adc::Channel,
    pac::ADC1,
};

pub enum LeftRightPosition {
    Left,
    Right,
}

pub enum InpuDirection {
    Up,
    Down,
    Stay,
}

pub struct TwoUserInputs<const PL: char, const PR: char, const NL: u8, const NR: u8> {
    pub left_user: Pin<PL, NL, Analog>,
    pub right_user: Pin<PR, NR, Analog>,
    pub adc1: Adc<ADC1>,
}

pub trait UserInteraction {
    fn get_input_direction(&mut self, user_position: LeftRightPosition) -> InpuDirection;
}

impl<const PL: char, const PR: char, const NL: u8, const NR: u8> UserInteraction
    for TwoUserInputs<PL, PR, NL, NR>
where
    Pin<PL, NL, Analog>: Channel<ADC1, ID = u8>, // Pins must be capable on analog read by ADC1.
    Pin<PR, NR, Analog>: Channel<ADC1, ID = u8>,
{
    fn get_input_direction(&mut self, user_position: LeftRightPosition) -> InpuDirection {
        let input_percentage = self.get_input_percentage(user_position);
        match input_percentage {
            0..=39 => InpuDirection::Up,
            40..=59 => InpuDirection::Stay,
            60..=100 => InpuDirection::Down,
            _ => {
                panic!(
                    "Error, input percentage: {}% not valid. Check connections.",
                    input_percentage
                )
            }
        }
    }
}

impl<const PL: char, const PR: char, const NL: u8, const NR: u8> TwoUserInputs<PL, PR, NL, NR>
where
    Pin<PL, NL, Analog>: Channel<ADC1, ID = u8>, // Pins must be capable on analog read by ADC1.
    Pin<PR, NR, Analog>: Channel<ADC1, ID = u8>,
{
    fn get_input_percentage(&mut self, user_position: LeftRightPosition) -> u8 {
        let sample_time = SampleTime::Cycles_480;
        let sample = match user_position {
            LeftRightPosition::Left => self.adc1.convert(&self.left_user, sample_time),
            LeftRightPosition::Right => self.adc1.convert(&self.right_user, sample_time),
        };
        let milivolts: u32 = self.adc1.sample_to_millivolts(sample).into();
        let reference_voltage = self.adc1.reference_voltage();
        let percentage = milivolts * 100 / reference_voltage;

        assert!(percentage <= 100, "Error reading analog input values");
        percentage as u8
    }
}
