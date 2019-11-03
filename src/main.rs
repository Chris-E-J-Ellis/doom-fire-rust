mod doom_fire;

fn main() -> Result<(), String> {
    use device_query::{DeviceQuery, DeviceState, Keycode};
    use doom_fire::console_fire_renderer as cfr;
    use doom_fire::fire_engine as fe; // This might be a crazy rename?
    use doom_fire::sdl_fire_renderer as sfr;

    // Is this stuff a crazy way to handle arguments and errors?
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return print_help_with_error("Insufficient arguments specified.");
    }

    let width: usize = match (&args[1]).parse() {
        Ok(val) => val,
        Err(_) => return print_help_with_error("Invalid WIDTH specified"),
    };

    let height: usize = match (&args[2]).parse() {
        Ok(val) => val,
        Err(_) => return print_help_with_error("Invalid HEIGHT specified"),
    };

    let sleep_in_milliseconds = match args.len() >= 4 {
        true => match (&args[3]).parse::<u64>() {
            Ok(val) => val,
            Err(_) => return print_help_with_error("Invalid sleep duration specified"),
        },
        false => 16, // default_sleep
    };

    let render_type: Renderer = if (args.len() >= 5) && (&args[4] == "-c") {
        Renderer::Console
    } else {
        Renderer::Sdl
    };

    // I could make the FireBuffer take a Vec, and not bother with slices, is that more idiomatic?
    // Sort of playing with lifetimes.
    let mut buffer: Vec<i32> = vec![0; (width * height) as usize];
    let mut fire_buffer = fe::FireBuffer {
        height: height,
        width: width,
        buffer: &mut buffer,
    };
    // Is this a nuts way of using traits? sort of in an interface headspace currently.
    // Could I make these live on the stack? I haven't really eaten docs around this yet =D 
    let mut renderer: Box<dyn fe::FireRenderer> = match render_type {
        Renderer::Sdl => Box::new(sfr::SdlFireRenderer::new(width as u32, height as u32)),
        Renderer::Console => Box::new(cfr::ConsoleFireRenderer {}),
    };
    renderer.initialise();

    // Here I'm playing with different ways of calling methods on things
    let max_ignition_value = doom_fire::fire_palette::MAX_PALETTE_ENTRIES as i32 - 1;
    fe::initialise_buffer(&mut fire_buffer, max_ignition_value);

    let device_state = DeviceState::new();
    let mut exit_requested = false;
    loop {
        renderer.render(&fire_buffer);
        fe::step_fire(&mut fire_buffer);

        let keys = device_state.get_keys();
        if keys.contains(&Keycode::Escape) {
            exit_requested = true;
        }

        exit_requested |= renderer.poll_for_exit();

        if exit_requested {
            renderer.cleanup();
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(sleep_in_milliseconds));
    }

    return Ok(());
}

enum Renderer {
    Sdl,
    Console,
}

fn print_help_with_error(message: &str) -> Result<(), String> {
    println!("Usage: doom-fire WIDTH HEIGHT [SLEEP] [RENDERER]");
    println!("       SLEEP    - Render loop delay in ms");
    println!("       RENDERER - Enable console renderer with '-c'");
    println!();
    Err(String::from(message))
}
