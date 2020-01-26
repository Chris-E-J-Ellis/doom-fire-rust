mod doom_fire;

use device_query::{DeviceQuery, DeviceState, Keycode};
use doom_fire::console_fire_renderer as cfr;
use doom_fire::fire_engine as fe;
use doom_fire::sdl_fire_renderer as sfr;

const DEFAULT_SLEEP_DURATION: u64 = 10;
enum Renderer {
    Sdl,
    Console,
}

fn main() -> Result<(), String> {
    if std::env::args().len() < 3 {
        print_help();
        return Err(String::from("Insufficient arguments specified."));
    }

    let width: usize = get_arg_or_print_help(1, "Invalid WIDTH specified")?;
    let height: usize = get_arg_or_print_help(2, "Invalid HEIGHT specified")?;

    let sleep_in_milliseconds = match std::env::args().len() {
        4 => get_arg_or_print_help(3, "Invalid sleep duration specified")?,
        _ => DEFAULT_SLEEP_DURATION,
    };

    let render_type = if std::env::args().len() >= 5 && std::env::args().nth(4).unwrap() == "-c" {
        Renderer::Console
    } else {
        Renderer::Sdl
    };

    let buffer: Vec<i32> = vec![0; width * height];
    let mut fire_buffer = fe::FireBuffer {
        height: height,
        width: width,
        buffer: buffer,
    };

    let mut renderer: Box<dyn fe::FireRenderer> = match render_type {
        Renderer::Sdl => Box::new(sfr::SdlFireRenderer::new(width as u32, height as u32)),
        Renderer::Console => Box::new(cfr::ConsoleFireRenderer::new()),
    };

    let max_ignition_value = doom_fire::fire_palette::MAX_PALETTE_ENTRIES as i32 - 1;
    fire_buffer.initialise_buffer(max_ignition_value);

    let device_state = DeviceState::new();
    let mut exit_requested = false;
    loop {
        renderer.render(&fire_buffer);
        fire_buffer.step_fire();

        let keys = device_state.get_keys();
        if keys.contains(&Keycode::Escape) {
            exit_requested = true;
        }

        exit_requested |= renderer.poll_for_exit();

        if exit_requested {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(sleep_in_milliseconds));
    }

    return Ok(());
}

fn get_arg_or_print_help<T: std::str::FromStr>(position: usize, msg: &str) -> Result<T, String> {
    match std::env::args().nth(position).unwrap().parse::<T>() {
        Ok(value) => return Ok(value),
        Err(_) => {
            print_help();
            return Err(String::from(msg));
        }
    };
}

fn print_help() {
    println!("Usage: doom-fire WIDTH HEIGHT [SLEEP] [RENDERER]");
    println!("       SLEEP    - Render loop delay in ms");
    println!("       RENDERER - Enable console renderer with '-c'");
    println!();
}
