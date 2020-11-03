use futures::executor::block_on;
use learn_wgpu::state::{State, NUM_INSTANCES};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const CUBE_BACKFACE_OPTIMIZATION: bool = true;
const NUM_CUBE_INDICES: usize = if CUBE_BACKFACE_OPTIMIZATION {
    3 * 3 * 2
} else {
    3 * 6 * 2
};
const NUM_CUBE_VERTICES: usize = 8;

fn generate_index_buffer_data(num_cubes: usize) -> Vec<u32> {
    #[rustfmt::skip]
    let cube_indices = [
        0u32, 2, 1, 2, 3, 1,
        5, 4, 1, 1, 4, 0,
        0, 4, 6, 0, 6, 2,
        6, 5, 7, 6, 4, 5,
        2, 6, 3, 6, 7, 3,
        7, 1, 3, 7, 5, 1,
    ];

    let num_indices = num_cubes * NUM_CUBE_INDICES;

    (0..num_indices)
        .map(|i| {
            let cube = i / NUM_CUBE_INDICES;
            let cube_local = i % NUM_CUBE_INDICES;
            cube as u32 * NUM_CUBE_VERTICES as u32 + cube_indices[cube_local]
        })
        .collect()
}

fn main() {
    learn_wgpu::tracing::setup_global_subscriber();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let indices = generate_index_buffer_data(NUM_INSTANCES as usize);
    println!(
        "Num indices: {}, num instances: {} {}",
        indices.len(),
        indices.len() / NUM_CUBE_INDICES,
        NUM_INSTANCES,
    );
    let mut state = block_on(State::new(&window, &indices));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                state.render();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
