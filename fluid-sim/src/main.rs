mod engine;
mod solver;

use core::future;

use engine::Engine;
use futures::executor::block_on;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut engine = futures::executor::block_on(Engine::new(&window));

    tokio::task::block_in_place(|| {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;
            match event {
                winit::event::Event::NewEvents(_) => {}
                winit::event::Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    winit::event::WindowEvent::Resized(_) => {}
                    winit::event::WindowEvent::Moved(_) => {}
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                winit::event::Event::DeviceEvent {
                    device_id: _,
                    event: _,
                } => {}
                winit::event::Event::UserEvent(_) => {}
                winit::event::Event::Suspended => {}
                winit::event::Event::Resumed => {}
                winit::event::Event::MainEventsCleared => {}
                winit::event::Event::RedrawRequested(_) => {
                    engine.update();
                    engine.render();
                }
                winit::event::Event::RedrawEventsCleared => {
                    window.request_redraw();
                }
                winit::event::Event::LoopDestroyed => {}
            }
        });
    });
}
