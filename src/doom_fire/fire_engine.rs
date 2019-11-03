use rand::random;

pub struct FireBuffer<'a> {
    pub height: usize,
    pub width: usize,
    pub buffer: &'a mut [i32],
}

pub trait FireRenderer {
    fn initialise(&mut self);
    fn render(&mut self, _buffer: &FireBuffer);
    fn cleanup(&self);
    fn poll_for_exit(&self) -> bool {
        return false;
    }
}

// I could implement all these methods on FireBuffer, but that seems weird?
// Should there be an empty "FireEngine" that contains these methods?
// How would they be better grouped/encapsulated? Maybe the module is fine.
pub fn initialise_buffer(buffer: &mut FireBuffer, ignition_value: i32) {
    let final_row_index = buffer.width * (buffer.height - 1);
    for i in final_row_index..buffer.buffer.len() {
        buffer.buffer[i] = ignition_value;
    }
}

pub fn step_fire(buffer: &mut FireBuffer) {
    for x in 0..buffer.width {
        for y in 1..buffer.height {
            let current_position = (y * buffer.width) + x;
            spread_fire(buffer, current_position);
        }
    }
}

// Is the checked sub an expected thing to do here? I could not use usize and just
// check to see if I'm a valid index.
fn spread_fire(buffer: &mut FireBuffer, source_position: usize) {
    let pixel = buffer.buffer[source_position];

    if pixel <= 0 {
        buffer.buffer[source_position - buffer.width] = 0;
    } else {
        let decay = random::<usize>() % 3; // This is usually and '&', but I prefer the reduced decay from mod.
        let destination_position = (source_position - buffer.width + 1).checked_sub(decay);
        if destination_position == None {
            return;
        }

        buffer.buffer[destination_position.unwrap()] = pixel - (decay as i32 & 1)
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
            buffer: &mut buffer,
        };
        let ignition_value = 5;
        fe::initialise_buffer(&mut fire_buffer, ignition_value);
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
