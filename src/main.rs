use ggez::{
	conf::WindowMode,event::{self,EventHandler},graphics::{self,Color,DrawMode,Mesh,Rect,Text},mint::Point2,Context,ContextBuilder,GameError,GameResult
};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use ggez::winit::{self, dpi};
pub use winit::event::{MouseButton, ScanCode};

const CELL_SIZE: (f32, f32) = (20.0, 20.0); // Zellgröße
const GRID_SIZE: (f32, f32) = (40.0, 40.0);	// Anzahl Zellen
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

const BG_COLOR: Color = Color::WHITE;
const CELL_COLOR: Color = Color::BLACK;
const LINE_WIDTH: f32 = 2.0;
const LINE_COLOR: Color = Color {
	r: 0.5,
	g: 0.5,
	b: 0.5,
	a: 1.0,
};
const TEXT_COLOR: Color = Color::BLACK;


struct State {
	grid: Vec<Vec<bool>>,
	fps: u32,
	running: bool,
	mouse_down: bool,
	mouse_up: bool,
}

impl State {
	pub fn new() -> Self {
		State {
			grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
			fps: 1,
			running: false,
			mouse_down: false,
			mouse_up: false,
		}
	}

	fn count_neighbours(&self,x:usize,y:usize) -> u16  {
		let mut count = 0;
		for i in x-1..x+1 {
			for j in y-1..y+1 {
				if self.grid[i as usize][j as usize] == true {
					if i == x && j == y {
							continue;
					}
					count += 1;
				}
			}
		}
		count
	}

	fn rules(&mut self){
		//Conways Game of Life
		/*Rules:
				1. live cell < 2 live neighbours dies
				2. live cell > 3 live neighbours dies
				3. live cell == 2 or 3 live neighbours lives
				4. dead cell == 3 live neighbours lives
		*/
		for (i,row) in self.grid.iter_mut().enumerate()  {
				for (j,col) in row.iter_mut().enumerate() {
						//count neighbours
						// let count_n = self.count_neighbours(i,j); //count neighbours for given cell
						// //evaluate rules
						// if *col == true {
						// 	if count_n < 2 {
						// 			//cell dies
						// 			self.grid[i][j] = false;
						// 	} else if count_n > 3 {
						// 			//cell dies
						// 			self.grid[i][j] = false;
						// 	} else if count_n == 2 || count_n == 3 {
						// 			//cell lives
						// 			self.grid[i][j] = true;
						// 	}
						// }else {
						// 	if count_n == 3 {
						// 			//cell lives
						// 			self.grid[i][j] = true;
						// 	}
						// }
				}
		}		
	}
}

impl EventHandler<GameError> for State{

	fn update(&mut self, ctx: &mut ggez::Context) -> Result<(),GameError> {
		self.rules();
		// self.grid[3][4] ^= true;
		// State::add_point(&mut self, 1, 2);
		Ok(())

	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, BG_COLOR);

		for i in 0..GRID_SIZE.0 as usize {
			for j in 0..GRID_SIZE.1 as usize {
				if self.grid[i][j] {
					let rect = Mesh::new_rectangle(
						ctx,
						DrawMode::fill(),
						Rect::new(						// CELL BLACK
							i as f32 * CELL_SIZE.0,
							j as f32 * CELL_SIZE.1,
							CELL_SIZE.0,
							CELL_SIZE.1,
						),
						CELL_COLOR,
					)?;
					graphics::draw(ctx, &rect, (Point2 { x: 0.0, y: 0.0 },))?;
				}

				if j == 0 {
					continue;
				}

				let line = Mesh::new_line(
					ctx,
					&vec![
						Point2 {
							x: 0.0,
							y: j as f32 * CELL_SIZE.1,
						},
						Point2 {
							x: WINDOW_SIZE.0,
							y: j as f32 * CELL_SIZE.1,
						},
					],
					LINE_WIDTH,
					LINE_COLOR,
				)?;
				graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
			}

			if i == 0 {
				continue;
			}

			let line = Mesh::new_line(
				ctx,
				&vec![
					Point2 {
						x: i as f32 * CELL_SIZE.0,
						y: 0.0,
					},
					Point2 {
						x: i as f32 * CELL_SIZE.0,
						y: WINDOW_SIZE.1,
					},
				],
				LINE_WIDTH,
				LINE_COLOR,
			)?;
			graphics::draw(ctx, &line, (Point2 { x: 0.0, y: 0.0 },))?;
		}

		/*let text = Text::new(self.fps.to_string());
		graphics::draw(ctx, &text, (Point2 { x: 0.0, y: 2.0 }, TEXT_COLOR))?;*/  //NUMBER

		graphics::present(ctx)?;

		Ok(())
	}

	fn mouse_motion_event(
		&mut self,
		_ctx: &mut Context,
		x: f32,
		y: f32,
		_dx: f32,
		_dy: f32
	) -> (){
		
	}
	
	fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> () { self.mouse_up = true;}
        

	fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> () {
		//self.mouse_down = true;
        //while self.mouse_up{
					self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize] ^= true;
	    //}
	}

	
	

}


fn main() -> GameResult {
	let mut state = State::new();
	state.grid[1][2] = true;
	
	let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	event::run(ctx, event_loop, state);
}