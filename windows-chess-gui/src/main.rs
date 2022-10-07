/**
 * Chess GUI template.
 * Author: Olle Thomsen <olleth@kth.se>
 * Last updated: 2022-10-16
 */

use chess::{Game, Color, Piece, Board, BoardStatus, BitBoard, ChessMove};
use jblomlof_chess::{Game as ChessGame, GameState};

use ggez::{conf, event::{self, winit_event}, graphics, Context, ContextBuilder, GameError, GameResult, input};
use std::{collections::HashMap, path, str::FromStr, vec, time::{self, Duration, Instant}, thread};

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
const _CIRCLE_GRAY: graphics::Color =
    graphics::Color::new(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 0.8);
const BACKGROUND_COLOR: graphics::Color =
    graphics::Color::new(49.0 / 255.0, 46.0 / 255.0, 43.0 / 255.0, 1.0);
const MENU_COLOR: graphics::Color =
    graphics::Color::new(39.0 / 255.0, 37.0 / 255.0, 34.0 / 255.0, 1.0);    


/// GUI logic and event implementation structure.
#[derive(Clone)]
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

    saved_replay: Vec<Vec<Board>>,

    replay_boards: Vec<Board>,

    replay_turn: usize,



}

impl AppState {

    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            board:  Board::default(),
            status: BoardStatus::Checkmate,
            game: Game::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Valid FEN"),
            side_to_move: Color::White,
            pos_x: 355.0,
            pos_y: 355.0,
            piece: (None, None),
            saved_replay: vec![],
            replay_boards: vec![Board::default()],
            replay_turn: 999,
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
        

        if input::keyboard::is_key_pressed(_ctx, input::keyboard::KeyCode::B)  {
            println!("x:{} y:{} -Up", self.pos_x, self.pos_y);
            println!("{:?}", self.piece);

        }

        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background Color
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let side_to_move_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?} to move...", self.side_to_move))
                .scale(graphics::PxScale { x: 25.0, y: 25.0 }),
        );

        // get size of text
        let text_dimensions = side_to_move_text.dimensions(ctx);
        
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
                40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                20.0,
                340.0,
                8.0 * GRID_CELL_SIZE.0 as f32,
            ),
            5.0,
            MENU_COLOR,
        )?;
    
        // draw Menu
        graphics::draw(ctx, &menu, graphics::DrawParam::default())
            .expect("Failed to draw menu.");

        
        let side = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                20.0,
                340.0,
                60.0,
            ),
            5.0,
            graphics::Color { r: (1.0), g: (1.0), b: (1.0), a: (1.0) },
        )?;
    
        // draw Menu
        graphics::draw(ctx, &side, graphics::DrawParam::default())
            .expect("Failed to draw menu.");


        
        //Start button and replay button
        if self.status == BoardStatus::Checkmate {
            let pos = input::mouse::position(ctx);
            
            // create text representation
            let start_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Start Game"))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
            );
            
            let start_button = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                    100.0,
                    340.0,
                    60.0,
                ),
                graphics::Color { r: (1.0), g: (1.0), b: (1.0), a: (1.0) },
            )?;
        
            // draw Menu
            graphics::draw(ctx, &start_button, graphics::DrawParam::default())
                .expect("Failed to draw menu.");

            //draw text with dark gray Coloring and center position
            graphics::draw(
            ctx,
            &start_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x:  120.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) as f32,
                    y: 120.0,
                }),
            )
            .expect("Failed to draw text.");
            
            // create text representation
            let replay_text = graphics::Text::new(
                graphics::TextFragment::from(format!("Replays"))
                    .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
                );


            let replay_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                160.0,
                340.0,
                60.0,
                ),
                graphics::Color { r: (1.0), g: (1.0), b: (1.0), a: (1.0) },
            )?;
        
            // draw Menu
            graphics::draw(ctx, &replay_button, graphics::DrawParam::default())
                .expect("Failed to draw menu.");

            //draw text with dark gray Coloring and center position
            graphics::draw(
                ctx,
                &replay_text,
                graphics::DrawParam::default()
                    .color([0.0, 0.0, 0.0, 1.0].into())
                    .dest(ggez::mint::Point2 {
                        x: 140.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) as f32,
                        y: 160.0,
                    }),
                )
                .expect("Failed to draw text.");

                if (pos.x >= 40.0 + GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 && pos.x <= 40.0 + GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 + 340.0) && (pos.y >= 160.0 && pos.y <= 220.0) {
                    let replay_options = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32),
                            220.0,
                            340.0,
                            30.0 * self.saved_replay.len() as f32,
                        ),
                        graphics::Color { r: (1.0), g: (1.0), b: (1.0), a: (1.0) },
                    )?;
                
                    // draw Menu
                    graphics::draw(ctx, &replay_options, graphics::DrawParam::default())
                        .expect("Failed to draw menu.");

                    // create text representation
                    for i in 0..self.saved_replay.len() {
                        let replays = graphics::Text::new(
                        graphics::TextFragment::from(format!("{}: Game", i))
                            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
                        );
                        //draw text with dark gray Coloring and center position
                        graphics::draw(
                            ctx,
                            &replays,
                            graphics::DrawParam::default()
                                .color([0.0, 0.0, 0.0, 1.0].into())
                                .dest(ggez::mint::Point2 {
                                    x: 140.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) as f32,
                                    y: 160.0 + 10.0 * i as f32,
                                }),
                            )
                            .expect("Failed to draw text.");
                    }

                    while self.status == BoardStatus::Ongoing {
                        
                    }
        
                } 
        }

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

        //draw text with dark gray Coloring and center position
        graphics::draw(
            ctx,
            &side_to_move_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x:  100.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) as f32,
                    y: 35.0,
                }),
        )
        .expect("Failed to draw text.");


        //draw text with dark gray Coloring and center position
        graphics::draw(
            ctx,
            &side_to_move_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x:  100.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) as f32,
                    y: 35.0,
                }),
        )
        .expect("Failed to draw text.");

            
            if input::mouse::cursor_grabbed(ctx) == true && self.status != BoardStatus::Checkmate {

                let pos = input::mouse::position(ctx);

                let sq = chess::Square::make_square(chess::Rank::from_index(7-self.pos_y as usize), chess::File::from_index(self.pos_x as usize));
                self.piece = (self.board.color_on(sq), self.board.piece_on(sq));

                if self.piece != (None, None) && self.piece.0 == Some(self.side_to_move)  { 


                    let mut kingside = chess::CastleRights::kingside_squares(&self.board.castle_rights(self.side_to_move), self.side_to_move) & !*self.board.combined();
                    let mut queenside = chess::CastleRights::queenside_squares(&self.board.castle_rights(self.side_to_move), self.side_to_move) & !*self.board.combined();
                    
                    match self.side_to_move {
                        chess::Color::White => queenside = queenside & BitBoard::set(chess::Rank::First, chess::File::B),
                        chess::Color::Black => queenside = queenside & BitBoard::set(chess::Rank::Eighth, chess::File::B),
                    }

                    match self.side_to_move {
                        chess::Color::White => if self.board.piece_on(chess::Square::make_square(chess::Rank::First, chess::File::F)) != None { kingside = kingside & BitBoard::set(chess::Rank::First, chess::File::F) },
                        chess::Color::Black => if self.board.piece_on(chess::Square::make_square(chess::Rank::Eighth, chess::File::F)) != None   { kingside = kingside & BitBoard::set(chess::Rank::Eighth, chess::File::F) },
                    }
                    
                    let mut bb = chess::BitBoard(0);
                    match self.piece.1 {
                        Some(Piece::Pawn) => bb = chess::get_pawn_moves(sq, self.piece.0.unwrap(), *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Rook) =>  bb = chess::get_rook_moves(sq, *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Knight) =>  bb = chess::get_knight_moves(sq) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Bishop) =>  bb =chess::get_bishop_moves(sq, *self.board.combined()) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::Queen) =>  bb = (chess::get_rook_moves(sq, *self.board.combined()) | chess::get_bishop_moves(sq, *self.board.combined())) & !*self.board.color_combined(self.side_to_move),
                         Some(Piece::King) =>  bb = chess::get_king_moves(sq) & !*self.board.color_combined(self.side_to_move) | kingside | queenside,
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

                        if self.board.en_passant() != None && (sq.right() == self.board.en_passant() || sq.left() == self.board.en_passant()) {
                            let en_sq = self.board.en_passant().unwrap().uup();
                            let er = 7-en_sq.get_rank().to_index();
                            let ef = en_sq.get_file().to_index();
                            let rectangle = graphics::Mesh::new_rectangle(
                                ctx,
                                graphics::DrawMode::fill(),
                                graphics::Rect::new_i32(
                                    ef as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    er as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    GRID_CELL_SIZE.0 as i32,
                                    GRID_CELL_SIZE.1 as i32,
                                ),
                                match (ef as i32) % 2 {
                                    0 => {
                                        if  (er as i32) % 2 == 0 {
                                            graphics::Color::new(233.0 / 255.0, 61.0 / 255.0, 77.0 / 255.0, 1.0) //White cell
                                        } else {
                                            graphics::Color::new(177.0 / 255.0, 38.0 / 255.0, 49.0 / 255.0, 1.0)
                                        }
                                    }
                                    _ => {
                                        if (er as i32) % 2 == 0 {
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

                        if self.board.en_passant() != None && (sq.right() == self.board.en_passant() || sq.left() == self.board.en_passant()) {
                            let en_sq = self.board.en_passant().unwrap().uup();
                            let er = 7-en_sq.get_rank().to_index();
                            let ef = en_sq.get_file().to_index();
                            let rectangle = graphics::Mesh::new_rectangle(
                                ctx,
                                graphics::DrawMode::fill(),
                                graphics::Rect::new_i32(
                                    ef as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    er as i32 * GRID_CELL_SIZE.0 as i32 + 20,
                                    GRID_CELL_SIZE.0 as i32,
                                    GRID_CELL_SIZE.1 as i32,
                                ),
                                match (ef as i32) % 2 {
                                    0 => {
                                        if  (er as i32) % 2 == 0 {
                                            graphics::Color::new(233.0 / 255.0, 61.0 / 255.0, 77.0 / 255.0, 1.0) //White cell
                                        } else {
                                            graphics::Color::new(177.0 / 255.0, 38.0 / 255.0, 49.0 / 255.0, 1.0)
                                        }
                                    }
                                    _ => {
                                        if (er as i32) % 2 == 0 {
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

                        // draw all the piecess
                        let pieces = (self.board.color_on(x), self.board.piece_on(x));
                        if pieces.1 != None {
                            let pieces = (self.board.color_on(x).unwrap(), self.board.piece_on(x).unwrap());
                            graphics::draw(
                                ctx,
                                self.sprites.get(&pieces).unwrap(),
                                graphics::DrawParam::default()
                                    .scale([0.78125, 0.78125]) // Tile size is 110 pixels, while image sizes are 440 pixels.
                                    .dest([
                                        f as f32 * GRID_CELL_SIZE.0 as f32 + 25.0,
                                        r as f32 * GRID_CELL_SIZE.1 as f32 + 25.0,
                                    ]),
                            )
                            .expect("Failed to draw piece.");
                    }

                    }

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

            if input::mouse::cursor_grabbed(ctx) == false && self.piece != (None, None) && self.piece.0 == Some(self.side_to_move) && self.status != BoardStatus::Checkmate {

                let pos = input::mouse::position(ctx);

                let from_sq = chess::Square::make_square(chess::Rank::from_index(7-self.pos_y as usize), chess::File::from_index(self.pos_x as usize));
                let to_sq = chess::Square::make_square(chess::Rank::from_index(7-((pos.y-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize), chess::File::from_index(((pos.x-20.0)/GRID_CELL_SIZE.0 as f32).floor() as usize));

                let mut promotion = None;
                if (to_sq.get_rank() == chess::Rank::First || to_sq.get_rank() == chess::Rank::Eighth) && self.piece.1 == Some(Piece::Pawn) {
                    promotion = Some(Piece::Queen);
                }
                let mv = chess::ChessMove::new(from_sq, to_sq, promotion);

                
                    
                if self.game.make_move(mv) == true {
                    self.board = self.game.current_position();
                    self.status = self.board.status();

                    self.replay_boards.push(self.board);

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

                    println!("{:?} move: {}\nboard: {}\nStatus: {:?}", self.side_to_move, mv, self.board, self.status);
                    
                    if self.status == BoardStatus::Checkmate {
                        match self.side_to_move {
                            Color::White => println!("White Won by Checkmate!"),
                            Color::Black => println!("Black Won by Checkmate!"),
                        }

                        //Saves the moves to the replay vector.
                        self.saved_replay.push(self.replay_boards.clone());
                        
                       
                    } else { self.side_to_move = !self.side_to_move; }

                }

                self.piece = (None, None);

            }

            if self.replay_turn < 777 && self.status == BoardStatus::Checkmate {

                if self.replay_turn < self.saved_replay[0].len() {
                    self.board = self.saved_replay[0][self.replay_turn];         
                }
            }
    
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

            if ( 20.0 < x && x < GRID_CELL_SIZE.0 as f32 * 8.0 + 20.0) && ( 20.0 < y && y < GRID_CELL_SIZE.0 as f32 * 8.0 + 20.0) {
                self.pos_x = (((x-20.0)/GRID_CELL_SIZE.0 as f32)).floor();
                self.pos_y = (((y-20.0)/GRID_CELL_SIZE.0 as f32)).floor();

                input::mouse::set_cursor_grabbed(ctx, true).ok(); 
            }

            if self.status == BoardStatus::Checkmate && (x >= 40.0 + GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 && x <= 40.0 + GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 + 340.0) && (y >= 100.0 && y <= 160.0) {
                self.board = Board::default();
                self.status = BoardStatus::Ongoing;
                self.game = Game::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Valid FEN");
                self.side_to_move = Color::White;
                self.piece = (None, None);
                self.replay_boards.clear();
                self.replay_boards.push(Board::default());
                self.replay_turn = 999;
            }

            if self.status == BoardStatus::Checkmate && (x >= 40.0 + (GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32) && x <= 40.0 + GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32 + 340.0) && (y >= 160.0 && y <= 220.0) {
                self.replay_turn = 0;
            }
            
            

       
        } 
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: event::KeyCode,
            _keymods: event::KeyMods,
            _repeat: bool,
        ) {
        if keycode == event::KeyCode::D && self.replay_turn >= self.replay_boards.len() { self.replay_turn += 1; }
        if keycode == event::KeyCode::A && self.replay_turn >= 1 { self.replay_turn -= 1; }
    }

}


pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources/pieces-png");

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