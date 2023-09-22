use ecs::{World,ScreenHeight,ScreenWidth,entities::champion::Champion};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton::Right;
use std::time::Duration;

//goals
//add sprite and get it + camera moving properly with mice clicks
//make it so each tick the sprite offsets both components of it's position by the speed*cosX/sinX it can recalc the angle X each frame

fn main() -> Result<(), String>  {
  let mut world = World::new();
  
  world.add_resource(ScreenHeight(1080));
  world.add_resource(ScreenWidth(1920));
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem.window("deux",world.immut_get_resource::<ScreenWidth>().unwrap().0 , world.immut_get_resource::<ScreenHeight>().unwrap().0)
    .position_centered()
    .build()
    .expect("could not initialize video subsystem");

  let mut canvas = window.into_canvas().build()
    .expect("could not make a canvas");

  let texture_creator = canvas.texture_creator();
  let texture = texture_creator.load_texture("assets/bardo.png")?;
  
  let mut player = Champion{
    sprite: Rect::new(0, 0, 26, 36),
    position: Point::new(0,0),
    speed:2,
    velocity: Point::new(0,0),
    target:Point::new(0,0),
  };

  let mut event_pump = sdl_context.event_pump()?;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running;
        },
        Event::MouseButtonDown {mouse_btn:Right, x, y,..}=>{
          player.target = Point::new(x,y)
        },
        _ => {}
      }
    }
    //Update
    player.move_player();
    
    //Render
    render(&mut canvas,vec![&player],&texture)?;
      
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }

  Ok(())
}
