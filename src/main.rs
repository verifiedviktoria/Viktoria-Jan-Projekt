use ggez::{
	conf::WindowMode,event::{self,EventHandler},graphics::{self,Color,DrawMode,Mesh,Rect,Text},mint::Point2,Context,ContextBuilder,GameError,GameResult
};
use rand::{Rng};
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use ggez::winit::{self, dpi};
pub use winit::event::{MouseButton, ScanCode};

const CELL_SIZE: (f32, f32) = (20.0, 20.0); // Zellgröße
const GRID_SIZE: (f32, f32) = (43.0, 43.0);	// Anzahl Zellen
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
		// Define the range of valid indices
    let x_start = x.saturating_sub(1);
    let x_end = (x + 1).min(self.grid.len() - 1);

    let y_start = y.saturating_sub(1);
    let y_end = (y + 1).min(self.grid[0].len() - 1);

    for i in x_start..=x_end {
        for j in y_start..=y_end {
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
		let mut new_grid = self.grid.clone();
		for (i,row) in self.grid.iter().enumerate()  {
				for (j,col) in row.iter().enumerate() {
						//count neighbours
						let count_n = self.count_neighbours(i,j); //count neighbours for given cell
						// let count_n = 2;
						
						//evaluate rules
						if *col == true {
							if count_n < 2 {
									//cell dies
									// *col = false;
									new_grid[i][j] = false;
							} else if count_n > 3 {
									//cell dies
									// *col = false;
									new_grid[i][j] = false;
							} else if count_n == 2 || count_n == 3 {
									//cell lives
									// *col = true;
									new_grid[i][j] = true;
							}
						}else {
							if count_n == 3 {
									//cell lives
									// *col = true;
									new_grid[i][j] = true;
							}
						}
				}
		}
		self.grid = new_grid;		
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
	preset3(&mut state);
	
	let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "mathletedev")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	event::run(ctx, event_loop, state);
}

fn preset1(state: &mut State) {
		state.grid[1][2] = true;
		state.grid[2][3] = true;
		state.grid[3][3] = true;
		state.grid[3][2] = true;
		state.grid[4][2] = true;
		state.grid[6][3] = true;
		state.grid[8][3] = true;
		state.grid[8][2] = true;
		state.grid[9][2] = true;
		state.grid[11][3] = true;
		state.grid[13][3] = true;
		state.grid[13][2] = true;
		state.grid[14][2] = true;
		state.grid[17][3] = true;
		state.grid[18][3] = true;
		state.grid[19][2] = true;

		state.grid[5][5] = true;
		state.grid[7][8] = true;
		state.grid[10][12] = true;
		state.grid[15][18] = true;
		state.grid[20][25] = true;

		state.grid[25][30] = true;
		state.grid[30][35] = true;
		state.grid[35][40] = true;
		state.grid[40][1] = true;
		state.grid[2][38] = true;
}

fn preset2(state: &mut State) {

	state.grid[1][2] = true;
	state.grid[2][3] = true;
	state.grid[3][3] = true;
	state.grid[3][2] = true;
	state.grid[4][2] = true;
	state.grid[6][3] = true;
	state.grid[8][3] = true;
	state.grid[8][2] = true;
	state.grid[9][2] = true;
	state.grid[11][3] = true;
	state.grid[13][3] = true;
	state.grid[13][2] = true;
	state.grid[14][2] = true;
	state.grid[17][3] = true;
	state.grid[18][3] = true;
	state.grid[19][2] = true;

	state.grid[5][5] = true;
	state.grid[7][8] = true;
	state.grid[10][12] = true;
	state.grid[15][18] = true;
	state.grid[20][25] = true;

	state.grid[25][30] = true;
	state.grid[30][35] = true;
	state.grid[35][40] = true;
	state.grid[40][1] = true;
	state.grid[2][38] = true;

	state.grid[3][10] = true;
	state.grid[5][15] = true;
	state.grid[10][20] = true;
	state.grid[12][28] = true;
	state.grid[18][32] = true;

	state.grid[20][5] = true;
	state.grid[22][10] = true;
	state.grid[28][15] = true;
	state.grid[32][20] = true;
	state.grid[38][25] = true;

	state.grid[1][30] = true;
	state.grid[5][35] = true;
	state.grid[10][38] = true;
	state.grid[15][3] = true;
	state.grid[20][12] = true;

}
fn preset3(state: &mut State) {
	for _ in 0..500 {
		let x = rand::thread_rng().gen_range(0..40);
		let y = rand::thread_rng().gen_range(0..40);
		state.grid[x][y] = true;
	}
}