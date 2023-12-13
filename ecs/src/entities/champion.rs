use sdl2::rect::{Point, Rect};

//is there a way to give a struct something without macros applying?
#[derive(Clone, Copy)]
pub struct Champion {
	pub sprite: Rect,
	pub position: Point,
	pub speed: i32,
	pub velocity: Point,
	pub target: Point,
}

//move this into a the input thing in systems that just handles inputs
impl Champion {
	pub fn move_player(&mut self) {
		//println!("x:{},y:{}", self.position.x,self.position.y);

		if self.target != self.position {
			let x_vel = (self.target.x - self.position.x) / self.speed;
			let y_vel = (self.target.y - self.position.y) / self.speed;
			self.velocity = Point::new(x_vel / 10, y_vel / 10);
			self.position = self.position + self.velocity;
		} else {
			self.velocity = Point::new(0, 0)
		};
	}
}
