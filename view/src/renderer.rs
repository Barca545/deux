use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas,Texture};
use ecs::entities::champion::Champion;
//At some stage I want to save the champs vector in a better place than randomly in main
//also really each champ needs to be stored with a unique texture instead of all sharing the same one


pub fn render(
  canvas:&mut WindowCanvas,
  champs:Vec<&Champion>,
  textures:&Texture,
)-> Result<(), String> {
  //do I really need to clear and present at the start of rendering?
  canvas.clear();
  let area = Rect::new(0,0,1920,1080);
  let color =  Color::RGB(2, 2, 2);
  canvas.set_draw_color(color);
  canvas.fill_rect(area)?;
  
  draw_champs(champs,canvas,textures)?;
  
  canvas.present();
  Ok(())
}

fn draw_champs (
  champs:Vec<&Champion>,
  canvas:&mut WindowCanvas,
  texture:&Texture)
-> Result<(), String>{ 
  
  for champ in champs {
    let screen_rect = Rect::from_center(champ.position, champ.sprite.width(), champ.sprite.height());
    canvas.copy(texture, champ.sprite, screen_rect)?;
    canvas.present();
  }
  
  Ok(())
}
