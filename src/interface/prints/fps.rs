use std::time::Duration;

pub fn fps(delta_time: Duration) {
    let dt = delta_time.as_secs_f32();
    println!("FPS: {:.1}", 1.0 / dt);
}
