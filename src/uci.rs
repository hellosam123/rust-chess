use std::{
    io::{self, BufRead},
    time::Duration,
};

use crate::{
    board::{Board, Color},
    search::{Search, SearchLimits},
};

pub struct Uci {
    board: Board,
    search: Search,
}

struct SearchParams {
    wtime: Option<u64>,
    btime: Option<u64>,
    winc: Option<u64>,
    binc: Option<u64>,
    depth: Option<u8>,
    infinite: bool,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            depth: None,
            infinite: false,
        }
    }
}

impl SearchParams {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Uci {
    fn default() -> Self {
        Self {
            board: Board::new(),
            search: Search::new(64),
        }
    }
}

impl Uci {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };

            self.handle_command(line.trim());
        }
    }

    fn handle_command(&mut self, command: &str) {
        let args: Vec<&str> = command.split_whitespace().collect();

        match args.as_slice() {
            ["uci"] => {
                println!("id name rust-chess 1.0");
                println!("id author isfsam");
                println!("uciok");
            }

            ["isready"] => {
                println!("readyok");
            }

            ["position", args @ ..] => {
                self.handle_position(args);
            }

            ["go", args @ ..] => {
                self.handle_go(args);
            }

            ["print"] => {
                self.board.print_board();
            }

            ["quit"] => {
                std::process::exit(0);
            }

            _ => {}
        }
    }

    fn handle_position(&mut self, mut args: &[&str]) {
        while !args.is_empty() {
            match args {
                ["startpos", rest @ ..] => {
                    if let Err(e) = self
                        .board
                        .parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                    {
                        println!("info string Error: {:?}", e);
                        return;
                    }

                    args = rest;
                }

                ["fen", rest @ ..] => {
                    if rest.len() < 6 {
                        println!("info string Error: incomplete FEN");
                        return;
                    }

                    let fen_str = rest[0..6].join(" ");

                    if let Err(e) = self.board.parse_fen(&fen_str) {
                        println!("info string Error: {:?}", e);
                        return;
                    }

                    args = &rest[6..];
                }

                ["moves", rest @ ..] => {
                    for &move_str in rest {
                        let Some(mv) = self.board.str_to_move(move_str) else {
                            println!("info string Error: invalid move {}", move_str);
                            return;
                        };

                        self.board.make_move(mv);
                    }

                    break;
                }
                _ => args = &args[1..],
            }
        }
    }

    fn handle_go(&mut self, mut args: &[&str]) {
        let mut search_params = SearchParams::new();
        while !args.is_empty() {
            match args {
                ["wtime", rest @ ..] => {
                    let Ok(value) = rest[0].parse::<u64>() else {
                        println!("info string Error: invalid wtime");
                        return;
                    };

                    search_params.wtime = Some(value);
                    args = &rest[1..];
                }

                ["btime", rest @ ..] => {
                    let Ok(value) = rest[0].parse::<u64>() else {
                        println!("info string Error: invalid btime");
                        return;
                    };

                    search_params.btime = Some(value);
                    args = &rest[1..];
                }

                ["winc", rest @ ..] => {
                    let Ok(value) = rest[0].parse::<u64>() else {
                        println!("info string Error: invalid winc");
                        return;
                    };

                    search_params.winc = Some(value);
                    args = &rest[1..];
                }

                ["binc", rest @ ..] => {
                    let Ok(value) = rest[0].parse::<u64>() else {
                        println!("info string Error: invalid binc");
                        return;
                    };

                    search_params.binc = Some(value);
                    args = &rest[1..];
                }

                ["depth", rest @ ..] => {
                    let Ok(value) = rest[0].parse::<u8>() else {
                        println!("info string Error: invalid depth");
                        return;
                    };

                    search_params.depth = Some(value);
                    args = &rest[1..];
                }

                ["infinite", rest @ ..] => {
                    search_params.infinite = true;
                    args = rest;
                }

                _ => args = &args[1..],
            }
        }

        let mut move_time: Option<Duration> = None;

        if self.board.active_color == Color::White
            && (search_params.wtime.is_some() || search_params.winc.is_some())
        {
            let time = search_params.wtime.unwrap_or(0);
            let inc = search_params.winc.unwrap_or(0);

            move_time = Some(Duration::from_millis(time / 20 + inc / 2));
        }

        if self.board.active_color == Color::Black
            && (search_params.btime.is_some() || search_params.binc.is_some())
        {
            let time = search_params.btime.unwrap_or(0);
            let inc = search_params.binc.unwrap_or(0);

            move_time = Some(Duration::from_millis(time / 20 + inc / 2));
        }

        if search_params.infinite {
            move_time = Some(Duration::MAX);
        }

        let limits = SearchLimits {
            max_depth: search_params.depth,
            max_move_time: move_time,
        };

        self.search.clear();
        self.search.root_search(&mut self.board, limits);
    }
}
