use raylib::prelude::*;
use crate::consts::KeyboardKey::*;

// For now, we will 'register' each experiment

fn main() {
    let experiments: Vec<String> = vec!["Test 1".to_string(), "Test 2".to_string()];

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Raylib vis Rust.")
        .build();

    while !rl.window_should_close() {
        {
            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLUE);
                d.draw_text("Experiments", 20, 20, 20, Color::BLACK);
                
                for (i, e) in experiments.iter().enumerate() {
                    // hopefully going to run out of numbers - todo: will need to swtich to letters.
                    let s = format!("{}. {:?}", i, e);
                    d.draw_text(&s, 20, 40 + 20 * i as i32, 20, Color::BLACK);
                }
            }

            {
                let kp = rl.get_key_pressed();

                if rl.is_key_down( KEY_ONE ){
                    println!( "Key 1 is down" );
                }
                
                if rl.is_key_down( KEY_TWO ) {
                    println!( "Key 2 is down" );
                }

            }

        }
    }
}
