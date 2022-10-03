/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Edited: Isak Larsson <isaklar@kth.se>
 * Last updated: 2022-09-28
 */

use chess::{Game, Color, Piece, Board, BoardBuilder, BoardStatus};

use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult, input, };
use std::{collections::{HashMap, HashSet}, path, str::FromStr};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (110, 110);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 + 400.0,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32 + 40.0,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(93.0 / 255.0, 50.0 / 255.0, 49.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(121.0 / 255.0, 71.0 / 255.0, 56.0 / 255.0, 1.0);
const CIRCLE_GRAY: graphics::Color =
    graphics::Color::new(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 0.8);
const BACKGROUND_COLOR: graphics::Color =
    graphics::Color::new(49.0 / 255.0, 46.0 / 255.0, 43.0 / 255.0, 1.0);
const MENU_COLOR: graphics::Color =
    graphics::Color::new(39.0 / 255.0, 37.0 / 255.0, 34.0 / 255.0, 1.0);    


/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<(Color, Piece), graphics::Image>,
    // Example board representation.
    board: Board,
    // Imported game representation.
    status: BoardStatus,

    game: Game,

    side_to_move: Color,

    pos_x: f32,
    
    pos_y: f32,
    
    piece: (Option<Color>, Option<Piece>),
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            board:  Board::default(),
            status: BoardStatus::Ongoing,
            game: Game::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Valid FEN"),
            side_to_move: Color::White,
            pos_x: 355.0,
            pos_y: 355.0,
            piece: (None, None),
        };

        Ok(state)
    }
    #[rustfmt::skip] // Skips formatting on this function (not recommended)
    /// Loads chess piese images into hashmap, for ease of use.
    fn load_sprites(ctx: &mut Context) -> HashMap<(Color, Piece), graphics::Image> {
        [
            ((Color::Black, Piece::King), "/black-king.png".to_string()),
            ((Color::Black, Piece::Queen), "/black-queen.png".to_string()),
            ((Color::Black, Piece::Rook), "/black-rook.png".to_string()),
            ((Color::Black, Piece::Pawn), "/black-pawn.png".to_string()),
            ((Color::Black, Piece::Bishop), "/black-bishop.png".to_string()),
            ((Color::Black, Piece::Knight), "/black-knight.png".to_string()),
            ((Color::White, Piece::King), "/white-king.png".to_string()),
            ((Color::White, Piece::Queen), "/white-queen.png".to_string()),
            ((Color::White, Piece::Rook), "/white-rook.png".to_string()),
            ((Color::White, Piece::Pawn), "/white-pawn.png".to_string()),
            ((Color::White, Piece::Bishop), "/white-bishop.png".to_string()),
            ((Color::White, Piece::Knight), "/white-knight.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| {
                (*piece, graphics::Image::new(ctx, path).unwrap())
            })
            .collect::<HashMap<(Color, Piece), graphics::Image>>()
    }
}

// This is where we implement the functions that ggez requires to function
impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    /// It won't be necessary to touch this unless you are implementing something that's not triggered by the user, like a clock
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        if input::keyboard::is_key_pressed(_ctx, input::keyboard::KeyCode::B) {
            println!("x:{} y:{} -Up", self.pos_x, self.pos_y);
            println!("{:?}", self.piece);

        }
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background Color
        //graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?}.", self.side_to_move))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // create text representation
        let rank_text = graphics::Text::new(
            graphics::TextFragment::from(format!("8 7 6 5 4 3 2 1"))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0 as f32,
                0.0 as f32,
                SCREEN_SIZE.0 as f32,
                SCREEN_SIZE.1 as f32,
            ),
            BACKGROUND_COLOR,
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");

            let menu = graphics::Mesh::new_rounded_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    20.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                    20.0,
                    350.0,
                    8.0 * GRID_CELL_SIZE.0 as f32,
                ),
                5.0,
                MENU_COLOR,
            )?;
    
            // draw Menu
            graphics::draw(ctx, &menu, graphics::DrawParam::default())
                .expect("Failed to draw menu.");


        // draw grid
        for row in 0..8 {
            for col in 0..8 {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        col * GRID_CELL_SIZE.0 as i32 + 20,
                        row * GRID_CELL_SIZE.1 as i32 + 20,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match col % 2 {
                        0 => {
                            if row % 2 == 0 {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if row % 2 == 0 {
                                BLACK
                            } else {
                                WHITE
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                
                // draw all the piecess
                let sq = chess::Square::make_square(chess::Rank::from_index(7-row as usize), chess::File::from_index(col as usize));
                let piece = (self.board.color_on(sq), self.board.piece_on(sq));
                if piece.1 != None {
                    let pieces = (self.board.color_on(sq).unwrap(), self.board.piece_on(sq).unwrap());
                    graphics::draw(
                        ctx,
                        self.sprites.get(&pieces).unwrap(),
                        graphics::DrawParam::default()
                            .scale([0.78125, 0.78125]) // Tile size is 110 pixels, while image sizes are 440 pixels.
                            .dest([
                                col as f32 * GRID_CELL_SIZE.0 as f32 + 25.0,
                                row as f32 * GRID_CELL_SIZE.1 as f32 + 25.0,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }
            }
        }

        /*/ draw text with dark gray Coloring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");*/

            
            if input::mouse::cursor_grabbed(ctx) == true {


                let pos = input::mouse::position(ctx);

                let sq = chess::Square::make_square(chess::Rank::from_index(7-self.pos_y as usize), chess::File::from_index(self.pos_x as usize));
                self.piece = (self.board.color_on(sq), self.board.piece_on(sq));

                let mut bb = chess::BitBoard(0);
                    match self.piece.1 {
                        Some(Piece::Pawn) => bb = chess::get_pawn_moves(sq, self.piece.0.unwrap(), *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Rook) =>  bb = chess::get_rook_moves(sq, *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Knight) =>  bb = chess::get_knight_moves(sq) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Bishop) =>  bb =chess::get_bishop_moves(sq, *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Queen) =>  bb = (chess::get_rook_moves(sq, *self.board.combined()) | chess::get_bishop_moves(sq, *self.board.combined())) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::King) =>  bb = chess::get_king_moves(sq) & !*self.board.color_combined(self.side_to_move),
                         _ => bb = chess::BitBoard(0)
                    };
         
                    for x in bb  {
                        let r = 7-x.get_rank().to_index();
                        let f = x.get_file().to_index();
                        
                            let rectangle = graphics::Mesh::new_rectangle(
                                ctx,
                                graphics::DrawMode::fill(),
                                graphics::Rect::new_i32(
                                    f as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    r as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    GRID_CELL_SIZE.0 as i32,
                                    GRID_CELL_SIZE.1 as i32,
                                ),
                                match (f as i32) % 2 {
                                    0 => {
                                        if  (r as i32) % 2 == 0 {
                                            graphics::Color::new(233.0 / 255.0, 61.0 / 255.0, 77.0 / 255.0, 1.0) //White cell
                                        } else {
                                            graphics::Color::new(177.0 / 255.0, 38.0 / 255.0, 49.0 / 255.0, 1.0)
                                        }
                                    }
                                    _ => {
                                        if (r as i32) % 2 == 0 {
                                            graphics::Color::new(177.0 / 255.0, 38.0 / 255.0, 49.0 / 255.0, 1.0)
                                        } else {
                                            graphics::Color::new(233.0 / 255.0, 61.0 / 255.0, 77.0 / 255.0, 1.0) 
                                        }
                                    }
                                },
                            ).expect("Failed to create tile.");
                            graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                                .expect("Failed to draw tiles.");
                    }
                
                if self.piece != (None, None) && self.piece.0 == Some(self.side_to_move)  { 

                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            self.pos_x as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                            self.pos_y as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                            GRID_CELL_SIZE.0 as i32,
                            GRID_CELL_SIZE.1 as i32,
                        ),
                        graphics::Color::new(245.0 / 255.0, 175.0 / 255.0, 78.0 / 255.0, 1.0),
                    
                    ).expect("Failed to create tile.");
                    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                        .expect("Failed to draw tiles.");

                    let pieces = (self.board.color_on(sq).unwrap(), self.board.piece_on(sq).unwrap());
                    graphics::draw(
                        ctx,
                        self.sprites.get(&pieces).unwrap(),
                        graphics::DrawParam::default()
                            .scale([0.78125, 0.78125]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                pos.x-55.0,
                                pos.y-55.0,
                            ]),
                    ).expect("Failed to draw piece.");

                    

                    
                    }
                }

            if input::mouse::cursor_grabbed(ctx) == false && self.piece != (None, None) && self.piece.0 == Some(self.side_to_move) {

                let pos = input::mouse::position(ctx);

                let from_sq = chess::Square::make_square(chess::Rank::from_index(7-self.pos_y as usize), chess::File::from_index(self.pos_x as usize));
                let to_sq = chess::Square::make_square(chess::Rank::from_index(7-((pos.y-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize), chess::File::from_index(((pos.x-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize));


                let mv = chess::ChessMove::new(from_sq, to_sq, None);
                    
                if self.game.make_move(mv) == true {
                    self.board = self.game.current_position();
                    self.status = self.board.status();
                    println!("{}\nStatus: {:?}", self.board, self.status);
                    
                    if self.status == BoardStatus::Checkmate {
                        match self.side_to_move {
                            Color::White => println!("White Won by Checkmate!"),
                            Color::Black => println!("Black Won by Checkmate!"),
                        }
                        
                    } else { self.side_to_move = !self.side_to_move; }

                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            self.pos_x as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                            self.pos_y as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                            GRID_CELL_SIZE.0 as i32, 
                            GRID_CELL_SIZE.1 as i32,
                        ),
                        match (self.pos_x as i32) % 2 {
                            0 => {
                                if  (self.pos_y as i32) % 2 == 0 {
                                    WHITE
                                } else {
                                    BLACK
                                }
                            }
                            _ => {
                                if (self.pos_y as i32) % 2 == 0 {
                                    BLACK
                                } else {
                                    WHITE
                                }
                            }
                        },
                    ).expect("Failed to create tile.");
                    graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                        .expect("Failed to draw tiles.");


                }

                self.piece = (None, None);

            
                
            }

                //let circle_mesh = graphics::MeshBuilder::new()
                //.circle(graphics::DrawMode::fill(), Point2::new(3.0, 3.0), 20.0, 10.0, GRAY,)?.build(ctx).ok().unwrap();
                //let circles = graphics::MeshBatch::new(circle_mesh).ok().unwrap();
                //graphics::MeshBatch::draw(&mut circles, ctx, graphics::DrawParam::default());

                /*if self.piece != (None, None)  {

                    let to_sq = chess::Square::make_square(chess::Rank::from_index((pos.y/90.0).floor() as usize), chess::File::from_index((pos.x/90.0).floor() as usize));

                    self.board = self.board.set_piece(self.piece.1.unwrap(), self.piece.0.unwrap(), to_sq).expect("Valid Position");
                    let p = (self.piece.0.unwrap(), self.piece.1.unwrap());
                    self.board = self.board.clear_square(sq).expect("Valid Position");
                    graphics::draw(
                        ctx,
                        self.sprites.get(&p).unwrap(),
                        graphics::DrawParam::default()
                            .scale([0.21, 0.21]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                self.pos_x * GRID_CELL_SIZE.0 as f32 + 5.0,
                                self.pos_y as f32 * GRID_CELL_SIZE.1 as f32 + 5.0,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }*/
                
        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");
        
        
        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event (
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
        ) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
            input::mouse::set_cursor_grabbed(ctx, false).ok();
           
           
        }
    }

    fn mouse_button_down_event (
            &mut self,
            ctx: &mut Context,
            button: event::MouseButton,
            x: f32,
            y: f32,
        )  { 
        if button == event::MouseButton::Left  {

            self.pos_x = (((x-20.0)/GRID_CELL_SIZE.0 as f32)).floor();
            self.pos_y = (((y-20.0)/GRID_CELL_SIZE.0 as f32)).floor();

            input::mouse::set_cursor_grabbed(ctx, true).ok(); 
            let sq = chess::Square::make_square(chess::Rank::from_index(7-((y-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize), chess::File::from_index(((x-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize));
            println!("rank:{:?} file:{:?} -Down", sq.get_rank(), sq.get_file());
            println!("y:{:?} x:{:?} -Down", self.pos_y, self.pos_x);
            

            
      } 
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: event::KeyCode,
            _keymods: event::KeyMods,
            _repeat: bool,
        ) {
        if keycode == event::KeyCode::A { println!("{}", self.board);
        }
    }
}


pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "olle")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Schack") // Set window title "Schack"
                .icon("/icon.png"), // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        );
    let (mut contex, mut _event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, _event_loop, state) // Run window event loop
}