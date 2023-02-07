// mod state;
// use state::PulsarState;

// use std::{
//         boxed::Box,
//         error::Error
// };

// use winit::{
//         event::*,
//         event_loop::{ ControlFlow, EventLoop },
//         window::{ self, WindowBuilder },
//         dpi::PhysicalSize,
// };

// pub struct Window {
//         event_loop: EventLoop<()>,
//         window: window::Window,
//         size: PhysicalSize<u32>,
//         state: PulsarState,
// }

// impl Window {
//         pub async fn new(title: &str) -> Result<Self, Box<dyn Error>> {
//                 let event_loop = EventLoop::new();
//                 let window = WindowBuilder::new().build(&event_loop)?;
//                 let size = PhysicalSize::new(1280, 720);
//                 window.set_inner_size(size);
//                 window.set_title(title);

//                 let state = PulsarState::new(&window, &size).await;

//                 Ok(Window { event_loop, window, size, state })
//         }

//         pub async fn from_defined(window: window::Window, event_loop: EventLoop<()>) -> Self {
//                 let size = window.inner_size();
//                 let state = PulsarState::new(&window, &size).await;

//                 Window { event_loop, window, size, state }
//         }

//         pub fn run(mut self) {
//                 self.event_loop.run(move |event, _, control_flow| match event {
//                         Event::WindowEvent {
//                                 ref event,
//                                 window_id        
//                         } if window_id == self.window.id() => match event {
//                                 WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
//                                         input: KeyboardInput {
//                                                 state: ElementState::Pressed,
//                                                 virtual_keycode: Some(VirtualKeyCode::Escape),
//                                                 ..
//                                         },
//                                         ..
//                                 } => *control_flow = ControlFlow::Exit,
//                                 WindowEvent::Resized(physical_size) => {
//                                         if physical_size.width > 0 && physical_size.height > 0 {
//                                                 self.size = *physical_size;
//                                                 self.state.resize(&self.size);
//                                         }
//                                 },
//                                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                                         if new_inner_size.width > 0 && new_inner_size.height > 0 {
//                                                 self.size = **new_inner_size;
//                                                 self.state.resize(&self.size);
//                                         }
//                                 },
//                                 _ => { },
//                         },
//                         _ => { },
//                 });
//         }

//         pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
//                 if new_size.width > 0 && new_size.height > 0 {
//                         self.size = new_size;
//                         self.state.resize(&self.size);
//                 }
//         }
// }