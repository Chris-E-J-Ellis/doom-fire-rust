use rand::random;

pub struct FireBuffer {
    pub height: usize,
    pub width: usize,
    pub buffer: Vec<i32>,
}

pub trait FireRenderer {
    fn render(&mut self, buffer: &FireBuffer);
    fn poll_for_exit(&self) -> bool {
        return false;
    }
}

impl FireBuffer {
    pub fn initialise_buffer(&mut self, ignition_value: i32) {
        let final_row_index = self.width * (self.height - 1);
        for i in final_row_index..self.buffer.len() {
            self.buffer[i] = ignition_value;
        }
    }

    pub fn step_fire(&mut self) {
        for x in 0..self.width {
            for y in 1..self.height {
                let current_position = (y * self.width) + x;
                self.spread_fire(current_position);
            }
        }
    }

    fn spread_fire(&mut self, source_position: usize) {
        let pixel = self.buffer[source_position];

        if pixel <= 0 {
            self.buffer[source_position - self.width] = 0;
        } else {
            let decay = random::<usize>() % 3; // This is usually and '&', but I prefer the reduced decay from mod.

            if let Some(destination_position) =
                (source_position - self.width + 1).checked_sub(decay)
            {
                self.buffer[destination_position] = pixel - (decay as i32 & 1);
            }
        }
    }
}

// Just checking I'm onboard with where I'd put a test =D
#[cfg(test)]
mod fire_engine_tests {
    use crate::doom_fire::fire_engine as fe;
    #[test]
    fn initialising_a_buffer_fills_the_last_row_with_specified_value() {
        let width = 3;
        let height = 3;
        let mut buffer: Vec<i32> = vec![0; (width * height) as usize];
        let mut fire_buffer = fe::FireBuffer {
            height: height,
            width: width,
            buffer: buffer,
        };
        let ignition_value = 5;
        fire_buffer.initialise_buffer(ignition_value);
        let expected_buffer = [
            0,
            0,
            0,
            0,
            0,
            0,
            ignition_value,
            ignition_value,
            ignition_value,
        ];
        for i in 0..expected_buffer.len() {
            assert_eq!(
                expected_buffer[i], fire_buffer.buffer[i],
                "Failed at index {}",
                i
            );
        }
    }
}
