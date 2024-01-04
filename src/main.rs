use ggez::{
	conf::WindowMode,event::{self,EventHandler},graphics::{self,Color,DrawMode,Mesh,Rect,Text,MeshBuilder,DrawParam},mint::Point2,Context,ContextBuilder,GameError,GameResult
};
use rand::{Rng};
use std::time::{Duration, SystemTime};
use crate::event::{KeyCode,KeyMods};
use std::thread::sleep;
use ggez::winit::{self, dpi};
pub use winit::event::{MouseButton, ScanCode};

const CELL_SIZE: (f32, f32) = (20.0, 20.0); // Zellgröße
const GRID_SIZE: (f32, f32) = (43.0, 43.0);	// Anzahl Zellen
const MENU_SIZE: f32 = 5.0; // Höhe des Menüs
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * (GRID_SIZE.1 + MENU_SIZE)); // Fenstergröße

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

// Menu
const BUTTON_COLOR: Color = Color::new(0.527, 0.527, 0.527, 1.0);
const BUTTON_WIDTH: f32 = 5.0 * CELL_SIZE.0;
const BUTTON_HEIGHT: f32 = 2.0 * CELL_SIZE.1;
const MENU_START: f32 = WINDOW_SIZE.1 - MENU_SIZE * CELL_SIZE.1;
const START_X: f32 = WINDOW_SIZE.0/2.0 - 2.0*BUTTON_WIDTH;
const NEXT_X: f32 = WINDOW_SIZE.0/2.0;
const CLEAR_X: f32 = WINDOW_SIZE.0/2.0 + 2.0*BUTTON_WIDTH;


struct State {
	grid: Vec<Vec<bool>>,
	fps: u32, 
	running: bool,
	color: Vec<Vec<Color>>,
	drawn: bool, // true if static objects have been drawn
	mouse_down: bool,
	mouse_up: bool,
}

impl State {
	pub fn new() -> Self {
		State {
			grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
			fps: 0,
			running: false,
			color: vec![vec![Color::BLACK; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
			drawn: false,
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
				1. live cell < 2 live neighbours then lc dies
				2. live cell > 3 live neighbours then lc dies
				3. live cell == 2 or 3 live neighbours then lc lives
				4. dead cell == 3 live neighbours then dc lives
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

	fn start(&mut self) {
		println!("Start/Pause button pressed");
		self.running ^= true;
	}

	fn next(&mut self) {
			println!("Next button pressed");
			self.rules();
	}

	fn clear(&mut self) {
			println!("Clear button pressed");
			self.grid = vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];
	}
}

impl EventHandler<GameError> for State{

	fn update(&mut self, ctx: &mut ggez::Context) -> Result<(),GameError> {
		if self.running {
			if self.fps !=0 {
				sleep(Duration::from_millis(1000/self.fps as u64));
			}			
			self.rules();
		}
		// self.grid[3][4] ^= true;
		// State::add_point(&mut self, 1, 2);
		Ok(())

	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		if !self.drawn {graphics::clear(ctx, BG_COLOR);}

		// Create a mesh builder to build the filled cells
		let mut mesh_builder = MeshBuilder::new();

		// Build the filled cells
		for i in 0..GRID_SIZE.0 as usize {
				for j in 0..GRID_SIZE.1 as usize {
						if self.grid[i][j] {
								mesh_builder.rectangle(
										DrawMode::fill(),
										Rect::new(
												i as f32 * CELL_SIZE.0,
												j as f32 * CELL_SIZE.1,
												CELL_SIZE.0,
												CELL_SIZE.1,
										),
										self.color[i][j],
								).expect("Error building mesh");
						}
				}
		}
		// Build the mesh for filled cells
		let filled_cells_mesh = mesh_builder.build(ctx)?;

		// Draw the filled cells
		graphics::draw(ctx, &filled_cells_mesh, DrawParam::default())?;

		
		if !self.drawn {
			// Create a mesh builder to build the grid
			let mut mesh_builder = MeshBuilder::new();

			// Build horizontal lines
			for j in 1..(GRID_SIZE.1+1.0) as usize {
					let y = j as f32 * CELL_SIZE.1;
					mesh_builder.line(
							&[
									Point2 { x: 0.0, y },
									Point2 { x: WINDOW_SIZE.0, y },
							],
							LINE_WIDTH,
							LINE_COLOR,
					)?;
			}

			// Build vertical lines
			for i in 1..GRID_SIZE.0 as usize {
					let x = i as f32 * CELL_SIZE.0;
					mesh_builder.line(
							&[
									Point2 { x, y: 0.0 },
									Point2 { x, y: MENU_START },
							],
							LINE_WIDTH,
							LINE_COLOR,
					)?;
			}

			// Build the mesh
			let grid_mesh = mesh_builder.build(ctx)?;

			// Draw the grid
			graphics::draw(ctx, &grid_mesh, DrawParam::default())?;


			// Draw the menu section with buttons
			// Create text for buttons
			let start_button_text = Text::new("Start/Pause"); 
			let next_button_text = Text::new("Next");  
			let clear_button_text = Text::new("Clear");

			let button_start = Mesh::new_rectangle(
				ctx,
				DrawMode::fill(),
				Rect::new(START_X, MENU_START, BUTTON_WIDTH, BUTTON_HEIGHT),
				BUTTON_COLOR,
			)?;
			
			let button_next = Mesh::new_rectangle(
					ctx,
					DrawMode::fill(),
					Rect::new(NEXT_X, MENU_START, BUTTON_WIDTH, BUTTON_HEIGHT),
					BUTTON_COLOR,
			)?;
			
			let button_clear = Mesh::new_rectangle(
					ctx,
					DrawMode::fill(),
					Rect::new(CLEAR_X, MENU_START, BUTTON_WIDTH, BUTTON_HEIGHT),
					BUTTON_COLOR,
			)?;

			// Draw buttons
			graphics::draw(ctx, &button_start, (Point2 { x: 0.0, y: 0.0 },))?;
			graphics::draw(ctx, &start_button_text, (Point2 { x:START_X + 0.05*BUTTON_WIDTH, y: MENU_START + 0.5 * CELL_SIZE.1 },))?;

			graphics::draw(ctx, &button_next, (Point2 { x: 0.0, y: 0.0 },))?;
			graphics::draw(ctx, &next_button_text, (Point2 { x: NEXT_X + 0.3 * BUTTON_WIDTH, y: MENU_START + 0.5 * CELL_SIZE.1 },))?;

			graphics::draw(ctx, &button_clear, (Point2 { x: 0.0, y: 0.0 },))?;
			graphics::draw(ctx, &clear_button_text, (Point2 { x: CLEAR_X + 0.3 * BUTTON_WIDTH, y: MENU_START + 0.5 * CELL_SIZE.1 },))?;
			self.drawn = false
		}

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
					// check if mouse is in grid
					if y/CELL_SIZE.1 < GRID_SIZE.1 {
						self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize] ^= true;
					}else {
						// check if mouse is in menu
						if y >= MENU_START && y <= MENU_START + BUTTON_HEIGHT {
							// check if mouse is in start button
							if x >= START_X && x <= START_X + BUTTON_WIDTH {
								self.start();
							}
							// check if mouse is in next button
							if x >= NEXT_X && x <= NEXT_X + BUTTON_WIDTH {
								self.next();
							}
							// check if mouse is in reset button
							if x >= CLEAR_X && x <= CLEAR_X + BUTTON_WIDTH {
								self.clear();
							}
						}
					} 
	    //}
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
		match keycode {
				KeyCode::Space => self.start(),
				KeyCode::N => self.next(),
				KeyCode::Escape => std::process::exit(0),
				KeyCode::C => self.clear(),
				KeyCode::Key1 => preset1(self),
				KeyCode::Key2 => preset2(self),
				KeyCode::Key3 => preset3(self),
				KeyCode::Key4 => self.fps = 0,
				KeyCode::Key5 => self.fps = 1,
				KeyCode::Key6 => self.fps = 4,
				KeyCode::Key7 => change_color(self,"black"),
				KeyCode::Key8 => change_color(self,"custom1"),
				KeyCode::Key9 => change_color(self, "red"),
				_ => (),
		}
	}
}


fn main() -> GameResult {
	let mut state = State::new();
	preset3(&mut state);
	println!("Welcome to Conway's Game of Life!");
	println!("Press Space to start/pause, N for next generation, R to reset, C to clear and Esc to exit.");
	println!("Keys 1-3 set Preset, 4-6 set FPS.");
	println!("Click on a cell to toggle it's state.");
	let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "VikiUndJan")
		.window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
		.build()?;

	event::run(ctx, event_loop, state);
}

fn change_color(state: &mut State, c: &str) {
	let mut color = Color::BLACK;
	println!("Color changed");
	if c == "red" {
		color = Color::RED;
		for i in 0..GRID_SIZE.0 as usize {
			for j in 0..GRID_SIZE.1 as usize {
				state.color[i][j] = color;
			}
		}
	}else if c == "custom1" {
		for i in 0..GRID_SIZE.0 as usize {
			for j in 0..GRID_SIZE.1 as usize {
				state.color[i][j] = Color::new(i as f32/GRID_SIZE.0,j as f32/GRID_SIZE.1,(j+i) as f32 /(GRID_SIZE.0+GRID_SIZE.1),1.0); // CELL_COLOR,
			}
		}
	}else {
		for i in 0..GRID_SIZE.0 as usize {
			for j in 0..GRID_SIZE.1 as usize {
				state.color[i][j] = CELL_COLOR;
			}
		}
	}
}

fn preset3(state: &mut State) {
	println!("Preset 3");
	state.clear();
	for _ in 0..500 {
		let x = rand::thread_rng().gen_range(0..GRID_SIZE.0 as usize);
		let y = rand::thread_rng().gen_range(0..GRID_SIZE.1 as usize);
		state.grid[x][y] = true;
	}
}

fn preset1(state: &mut State) {
		println!("Preset 1");
		state.clear();
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
	println!("Preset 2");
	state.clear();
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
