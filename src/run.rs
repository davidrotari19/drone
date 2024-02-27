use crate::cpu::Cpu;
use crate::screen::{ButtonName, CPU_FREQ, HEIGHT, Message, Screen, ScreenWriter, WIDTH};
//use crate::ppu::{Mirroring, Ppu, CPU_FREQ, HEIGHT, WIDTH};
//import mirroring
use crate::ppu::mirroring::Mirroring;
use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{env, thread};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::ppu::Ppu;

fn run_ppu(
    mirroring: Mirroring,
    cpu: &mut impl Cpu,
    writer: &mut ScreenWriter,
    max_cycles: Option<usize>,

) -> Result<(), Box<dyn Error>> {
    const ITER_PER_CYCLE: usize = 1000;
    let mut ppu = Ppu::new(mirroring);

    //connect the ppu to the cpu bus
    let mut busy_time = Duration::default();
    let mut cycles = 0;
    let mut last_tick = Instant::now();
    loop {
        for _ in 0..ITER_PER_CYCLE {
            if let ScreenWriter::Real {
                control_rx: buttons_rx,
                ..
            } = writer
            {   let mut controller;
                while let Ok(msg) = buttons_rx.try_recv() {
                    controller=0x00;
                    match msg {
                        Message::Button(name, pressed) => match name {
                            ButtonName::A => {
                                controller |= if pressed { 0x80} else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.a = pressed;
                            }
                            ButtonName::B => {
                                controller|= if pressed { 0x40 } else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.b = pressed;
                            }
                            ButtonName::Up => {
                                controller |= if pressed {0x08} else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.up = pressed;
                            }
                            ButtonName::Down => {
                                controller|= if pressed {0x04} else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.down = pressed;
                            }
                            ButtonName::Left => {
                                controller|= if pressed { 0x02 } else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.left = pressed;
                            }
                            ButtonName::Right => {
                                controller |= if pressed { 0x01 } else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.right = pressed;
                            }
                            ButtonName::Start => {
                                controller |= if pressed { 0x10 } else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.start = pressed;
                            }
                            ButtonName::Select => {
                                controller |= if pressed { 0x20 } else { 0x00 };
                                cpu.update_controller(controller);
                                ppu.buttons.select = pressed;
                            }
                        },
                        Message::Pause(true) => {
                            while let Message::Pause(true) =
                                buttons_rx.recv().expect("sender closed")
                            {
                            }
                            // skip over previous iterations
                            last_tick = Instant::now();
                        }
                        Message::Pause(false) => {}
                    }
                }
            }
            //cpu.update_controller(controller);
            if let Err(e) = cpu.tick(&mut ppu) {
                log::warn!("cpu stopped");
                return Err(e);
            }

            for _ in 0..3 {
                ppu.update(cpu, writer);
            }
        }

        cycles += ITER_PER_CYCLE;

        if let Some(max_cycles) = max_cycles {
            if cycles > max_cycles {
                break Ok(());
            }
        }

        let now = Instant::now();
        busy_time += now.duration_since(last_tick);

        let expected_time_spent = Duration::from_secs_f64((1.0 / CPU_FREQ) * cycles as f64);

        if expected_time_spent > busy_time {
            thread::sleep(expected_time_spent - busy_time);
        } else if cycles % 1000 == 0
            && (busy_time - expected_time_spent) > Duration::from_secs_f64(0.2)
        {
            println!(
                "emulation behind by {:?}. trying to catch up...",
                busy_time - expected_time_spent
            );
        }

        last_tick = now;
    }
}

/// Like [`run_cpu_headless`], but takes a cycle limit after which the function returns.
pub fn run_cpu_headless_for<CPU>(
    cpu: &mut CPU,
    mirroring: Mirroring,
    cycle_limit: usize,
) -> Result<(), Box<dyn Error>>
where
    CPU: Cpu + 'static,
{
    let (_, mut writer) = Screen::dummy();

    run_ppu(mirroring, cpu, &mut writer, Some(cycle_limit))
}

/// Runs the cpu as if connected to a PPU, but doesn't actually open
/// a window. This can be useful in tests.
pub fn run_cpu_headless<CPU>(cpu: &mut CPU, mirroring: Mirroring) -> Result<(), Box<dyn Error>>
where
    CPU: Cpu + 'static,
{
    let (_, mut writer) = Screen::dummy();

    run_ppu(mirroring, cpu, &mut writer, None)
}

/// Runs the cpu with the ppu. Takes ownership of the cpu, creates
/// a PPU instance, and runs the tick function at the correct rate.
///
/// This function *has to be called from the main thread*. This means it will not
/// work from unit tests. Use [`run_cpu_headless`] there.
///
/// # Panics
/// [`run_cpu`] can panic when the `cpu` returns an Error
pub fn run_cpu<CPU>(mut cpu: CPU, mirroring: Mirroring)
where
    CPU: Cpu + Send + 'static,
{
    env::set_var("WAYLAND_DISPLAY", "wayland-1");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("NES")
        .build(&event_loop)
        .expect("failed to create window");

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).expect("failed to create surface");

    let (mut screen, mut writer, control_tx) = Screen::new(pixels, window);

    let handle = Arc::new(Mutex::new(Some(thread::spawn(move || {
        match run_ppu(mirroring, &mut cpu, &mut writer, None) {
            Ok(_) => unreachable!(),
            Err(e) => {
                panic!("cpu implementation returned an error: {e}")
            }
        }
    }))));

    let mut last = Instant::now();
    let wait_time = Duration::from_secs_f64(1.0 / 60.0);

    event_loop.run(move |event, _, control_flow| {
        #[allow(clippy::single_match)]
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            Event::WindowEvent {
                event: WindowEvent::Focused(f),
                ..
            } => {
                control_tx.send(Message::Pause(!f)).expect("failed to send");
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(code) = input.virtual_keycode {
                    match code {
                        VirtualKeyCode::Left | VirtualKeyCode::A => {

                            control_tx
                                .send(Message::Button(
                                    ButtonName::Left,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::Up | VirtualKeyCode::W => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::Up,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::Right | VirtualKeyCode::D => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::Right,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::Down | VirtualKeyCode::S => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::Down,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::Return => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::Start,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::RShift | VirtualKeyCode::LShift => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::Select,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::Z => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::B,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        VirtualKeyCode::X => {
                            control_tx
                                .send(Message::Button(
                                    ButtonName::A,
                                    input.state == ElementState::Pressed,
                                ))
                                .expect("failed to send");
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        *control_flow = ControlFlow::WaitUntil(Instant::now() + wait_time);

        if handle.lock().unwrap().as_ref().unwrap().is_finished() {
            handle
                .lock()
                .unwrap()
                .take()
                .expect("cpu emulation exited unexpectedly");
            return;
        }

        if Instant::now().duration_since(last) > wait_time {
            screen.redraw();
            last = Instant::now();
        }
    });
}
