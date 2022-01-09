use rand::prelude::*;

type Piece = Vec<(i32, i32)>;

fn centerize_piece(piece: &Piece) -> Piece {
    let min_x: i32 = piece.iter().map(|(x, y)| *x).min().unwrap();
    let min_y: i32 = piece.iter().map(|(x, y)| *y).min().unwrap();

    piece.iter().map(|(x, y)| (x - min_x, y - min_y)).collect()
}

fn get_pieces() -> Vec<Piece> {
    let original: Vec<Piece> = vec![
        // #
        vec![(0, 0)],
        // ##
        vec![(0, 0), (0, 1)],
        // ###
        vec![(0, 0), (0, 1), (0, 2)],
        // ####
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // #####
        vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
        // ##
        //  #
        vec![(0, 0), (1, 1), (0, 1)],
        // ###
        //   #
        vec![(0, 0), (0, 1), (0, 2), (1, 2)],
        // ###
        //  #
        vec![(0, 0), (0, 1), (0, 2), (1, 1)],
        // ##
        //  ##
        vec![(0, 0), (0, 1), (1, 2), (1, 1)],
        // # #
        // ###
        vec![(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)],
        // ##
        //  ##
        //   #
        vec![(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
        // ###
        //   #
        //   #
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        // ###
        //  #
        //  #
        vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)],
        // ##
        // ##
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        // ###
        // ###
        // ###
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
    ];

    let mut all: Vec<Piece> = vec![];

    for piece in original {
        // Get flipped piece
        let flipped: Piece = piece.iter().map(|(a, b)| (*b, *a)).collect();
        // Mirror the original and flipped by y axis
        let piece_m = centerize_piece(
            &piece.iter().map(|(a, b)| (-*a, *b)).collect()
        );
        let flipped_m = centerize_piece(
            &flipped.iter().map(|(a, b)| (-*a, *b)).collect()
        );

        // Mirror original, flipped, and both mirrored pieces by an x axis
        all.push(centerize_piece(&piece.iter().map(|(a, b)| (*a, -*b)).collect()));
        all.push(centerize_piece(&flipped.iter().map(|(a, b)| (*a, -*b)).collect()));
        all.push(centerize_piece(&piece_m.iter().map(|(a, b)| (*a, -*b)).collect()));
        all.push(centerize_piece(&flipped_m.iter().map(|(a, b)| (*a, -*b)).collect()));

        all.push(piece);
        all.push(flipped);
        all.push(piece_m);
        all.push(flipped_m);
    }

    return all;
}

#[derive(Debug)]
struct PiecePlay {
    piece_index: usize,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct MoveCalculations {
    total_count: i32,
    best_score: i32,
    played_pieces: Vec<PiecePlay>,
}

#[derive(Copy, Clone)]
struct Board {
    pieces: [bool; 9 * 9]
}

impl Board {
    fn new() -> Board {
        Board {
            pieces: [false; 9 * 9]
        }
    }

    fn print(&self) {
        for y in 0..9 {
            for x in 0..9 {
                if self.pieces[x + y * 9] == false {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }

    fn set(&mut self, x: i32, y: i32, state: bool) {
        self.pieces[(x + y * 9) as usize] = state;
    }

    fn get(&self, x: i32, y: i32) -> bool {
        return self.pieces[(x + y * 9) as usize];
    }

    fn placement_out_of_bounds(&self, piece: &Piece, x: i32, y: i32) -> bool {
        for (px, py) in piece.iter() {
            let nx = px + x;
            let ny = py + y;
            if nx >= 9 || ny >= 9 {
                return true;
            }
        }
        return false;
    }

    fn can_be_placed(&self, piece: &Piece, x: i32, y: i32) -> bool {
        for (px, py) in piece.iter() {
            let nx = px + x;
            let ny = py + y;
            if nx >= 9 || ny >= 9 || self.get(nx, ny) == true {
                return false;
            }
        }
        return true;
    }

    fn place(&self, piece: &Piece, x: i32, y: i32) -> Board {
        let mut new_board = self.clone();
        for (px, py) in piece.iter() {
            new_board.set(px + x, py + y, true)
        }

        // Clear horizontal line
        for x in 0..9 {
            let mut removable = true;
            for y in 0..9 {
                if new_board.get(x, y) == false {
                    removable = false;
                    break;
                }
            }
            if removable {
                for y in 0..9 {
                    new_board.set(x, y, false)
                }
            }
        }

        // Clear vertical line
        for y in 0..9 {
            let mut removable = true;
            for x in 0..9 {
                if new_board.get(x, y) == false {
                    removable = false;
                    break;
                }
            }
            if removable {
                for x in 0..9 {
                    new_board.set(x, y, false)
                }
            }
        }

        new_board
    }

    fn get_position_score(&self, pieces: &Vec<Piece>) -> i32 {
        let mut score = 0;

        for piece in pieces {
            let mut piece_has_place = false;
            for x in 0..9 {
                for y in 0..9 {
                    if self.can_be_placed(piece, x, y) {
                        piece_has_place = true;
                        score += 1;
                    }
                }
            }
            if !piece_has_place {
                score -= 100;
            }
        }

        // let mut rng = rand::thread_rng();
        // for i in 0..500 {
        //     let piece_index = rng.gen_range(0..(pieces.len()));
        //     let x = rng.gen_range(0..8);
        //     let y = rng.gen_range(0..8);
        //     if self.can_be_placed(&pieces[piece_index], x, y) {
        //         score += 1
        //     }
        // }

        score
    }

    fn figure_out_moves(&self, pieces: &Vec<Piece>, piece_indexes: &Vec<usize>) -> MoveCalculations {
        if piece_indexes.len() == 0 {
            return MoveCalculations {
                total_count: 1,
                best_score: self.get_position_score(pieces),
                played_pieces: vec![],
            };
        }
        let mut move_calculations = MoveCalculations {
            total_count: 0,
            best_score: 0,
            played_pieces: vec![],
        };
        let mut sum = 0;

        for i in 0..piece_indexes.len() {
            let playable_piece = &pieces[piece_indexes[i]];

            let mut new_piece_indexes: Vec<usize> = vec![];
            for j in 0..piece_indexes.len() {
                if j != i {
                    new_piece_indexes.push(piece_indexes[j])
                }
            }

            for x in 0..9 {
                for y in 0..9 {
                    if self.can_be_placed(playable_piece, x, y) {
                        let new_board = self.place(playable_piece, x, y);
                        let new_move_calculations = new_board.figure_out_moves(
                            pieces, &new_piece_indexes,
                        );

                        move_calculations.total_count += new_move_calculations.total_count;
                        if new_move_calculations.best_score > move_calculations.best_score {
                            move_calculations.best_score = new_move_calculations.best_score;
                            move_calculations.played_pieces = new_move_calculations.played_pieces;
                            move_calculations.played_pieces.push(PiecePlay {
                                x,
                                y,
                                piece_index: piece_indexes[i],
                            });
                        }
                    }
                }
            }
        }
        move_calculations
    }
}

fn get_3_random_piece_indexes(pieces: &Vec<Piece>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut piece_indexes: Vec<usize> = vec![];

    for i in 0..3 {
        let random_value = rng.gen_range(0..(pieces.len()));
        piece_indexes.push(random_value);
    }
    piece_indexes
}

fn play_a_game() -> i32 {
    let pieces = get_pieces();
    let mut board = Board::new();

    let mut piece_count = 0;
    loop {
        let random_piece_indexes = get_3_random_piece_indexes(&pieces);
        let move_ = board.figure_out_moves(&pieces, &random_piece_indexes);

        if move_.total_count == 0 {
            break;
        }

        for piece_play in move_.played_pieces {
            board = board.place(&pieces[piece_play.piece_index], piece_play.x, piece_play.y);
            println!("---------");
            board.print();
            println!("---------");
            piece_count += 1;
        }
    }

    println!("Piece count: {}", piece_count);

    piece_count
}

fn play_a_game_monte_carlo() -> i32 {
    let pieces = get_pieces();
    let mut piece_count = 0;
    let mut board = Board::new();

    let mut rng = rand::thread_rng();
    loop {
        let random_value = rng.gen_range(0..(pieces.len()));
        let random_piece = &pieces[random_value];

        let best_values = (0, 0, 0);

        for x in 0..9 {
            for y in 0..9 {
                if board.can_be_placed(random_piece, x, y) {}
            }
        }
    }

    piece_count
}

fn main() {
    // for j in 0..5 {
    //     let mut avg = 0;
    //     for i in 0..10 {
    //         avg += play_a_game()
    //     }
    //     println!("Avg is {}", avg)
    // }
    play_a_game();
}
