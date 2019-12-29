#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::cpu;
use crate::drawing::debugger;

pub fn mos_6502_emulator(program: &str) -> Result<(), Error> {
    // CPU Initialisation
    let mut cpu = cpu::CPU::new();
    cpu.setup();
    let prog_array: Vec<_> = program.split_whitespace().collect();
    for (index, prog_n) in prog_array.iter().enumerate() {
        cpu.bus.ram[0x8000 + index] = u8::from_str_radix(prog_n, 16).unwrap();
    }
    cpu.bus.ram[0xFFFC] = 0x00;
    cpu.bus.ram[0xFFFD] = 0x80;
    let mut code_map = cpu.dissasemble(0x0000, 0xFFFF);
    cpu.reset();

    // Window Initialisation
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(crate::globals::WIDTH as f64, crate::globals::HEIGHT as f64);
        WindowBuilder::new()
            .with_title("6502 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut hidpi_factor = window.hidpi_factor();
    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture =
            SurfaceTexture::new(crate::globals::WIDTH, crate::globals::HEIGHT, surface);
        Pixels::new(
            crate::globals::WIDTH,
            crate::globals::HEIGHT,
            surface_texture,
        )?
    };

    // First render on start
    debugger::draw_debug(pixels.get_frame(), &mut cpu, &mut code_map);

    // Event Loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            pixels.render();
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Space) {
                loop {
                    cpu.clock();
                    if cpu.complete() {
                        debugger::draw_debug(pixels.get_frame(), &mut cpu, &mut code_map);
                        break;
                    }
                }
            }

            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.hidpi_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                let size = size.to_physical(hidpi_factor);
                let width = size.width.round() as u32;
                let height = size.height.round() as u32;

                pixels.resize(width, height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
