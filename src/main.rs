mod engine;
mod api;

fn main() {
    let mut event_loop = engine::event_loop::EventLoop::new();
    event_loop.run_event_loop();
}
