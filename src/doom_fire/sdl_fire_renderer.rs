use super::fire_engine::FireBuffer;
use super::fire_palette;
use sdl2::{
    render::{Canvas, TextureAccess},
    video::Window,
};

pub struct SdlFireRenderer {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    back_buffer: Vec<u8>,
    bytes_per_pixel: usize,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl super::fire_engine::FireRenderer for SdlFireRenderer {
    fn render(&mut self, buffer: &FireBuffer) {
        SdlFireRenderer::render(self, buffer)
    }
    fn poll_for_exit(&self) -> bool {
        SdlFireRenderer::poll_for_exit(self)
    }
}

impl SdlFireRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Doom Fire", width, height)
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();
        let bytes_per_pixel: usize = 4;
        let back_buffer = vec![0; (width * height) as usize * bytes_per_pixel];
        let texture_creator = canvas.texture_creator();

        SdlFireRenderer {
            sdl_context,
            canvas,
            back_buffer,
            bytes_per_pixel,
            texture_creator,
        }
    }

    pub fn render(&mut self, buffer: &FireBuffer) {
        // I'm going to move all this out to the SdlFireRenderer struct, for an exercise, but I think there's
        // some lifetime or RC stuff to read and understand before playing with that.
        let mut texture = self
            .texture_creator
            .create_texture(
                None,
                TextureAccess::Static,
                buffer.width as u32,
                buffer.height as u32,
            )
            .unwrap();

        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let pixel = buffer.buffer[x + (y * buffer.width)];
                let palette_index = (pixel * 3) as usize;

                let r = fire_palette::DOOM_FIRE_PALETTE[palette_index];
                let g = fire_palette::DOOM_FIRE_PALETTE[palette_index + 1];
                let b = fire_palette::DOOM_FIRE_PALETTE[palette_index + 2];

                let source_position = (x + (buffer.width * y)) * self.bytes_per_pixel;
                self.back_buffer[source_position] = b;
                self.back_buffer[source_position + 1] = g;
                self.back_buffer[source_position + 2] = r;
            }
        }
        texture
            .update(None, &self.back_buffer, buffer.width * 4)
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn poll_for_exit(&self) -> bool {
        let mut exit_requested = false;
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                exit_requested = true;
            }
            /*if let sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(width, height),
                ..
            } = event
            {}*/
        }
        return exit_requested;
    }
}
