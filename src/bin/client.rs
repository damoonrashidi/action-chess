use action_chess::{
    client::utils::get_vec2_for_coord,
    state::{coordinate::Coord, gamestate::GameState},
};
use comfy::{hecs::Bundle, *};

simple_game!("Action chess", setup, update);

#[derive(Debug, Bundle)]
struct ClientState {
    game: GameState,
}

fn setup(_c: &mut EngineContext) {
    set_main_camera_zoom(12.);
    let mut camera = main_camera_mut();
    camera.center = vec2(4., 4.);
    game_config_mut().bloom_enabled = true;

    let mut world = world_mut();
    world.spawn(GameState::new());
}

fn update(_c: &mut EngineContext) {
    if is_mouse_button_down(MouseButton::Left) {
        println!("{}", mouse_world());
    }
    for y in 0..8 {
        for x in 0..8 {
            let coord = Coord(y, x);
            let pos = get_vec2_for_coord(&coord);
            let square_color = if (x + y) % 2 == 0 {
                Color::rgb(0.035, 0.055, 0.349)
            } else {
                Color::rgb(0.522, 0.553, 0.969)
            };
            draw_rect(pos, vec2(1.0, 1.0), square_color, 1);

            for (_, state) in world().query::<&mut ClientState>().iter().collect_vec() {
                println!("{:?}", state);
            }
        }
    }
}
