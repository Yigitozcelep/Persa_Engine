use std::cmp::Ordering;
use std::str::SplitWhitespace;
use std::time::{Instant, Duration};
use crate::pieces::pieces_controller::{BoardStatus, MoveBitField, MoveList, BoardHistory};
use crate::debug::FenString;
use std::sync::{Arc, RwLock};
use crate::debug::perft_driver;
use crate::eveluation::find_best_move;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct UciInformation {
    pub find_move_signal  : Arc<RwLock<bool>>,
    pub is_searching      : Arc<RwLock<bool>>,
    pub quit_signal       : Arc<RwLock<bool>>,
    pub wtime             : usize,
    pub btime             : usize,
    pub winc              : usize,
    pub binc              : usize,
    pub depth_limit       : isize,
    pub board             : BoardStatus,

    pub moves_to_go       : Option<usize>,
    pub nodes_limit       : Option<usize>,
    pub time_limit        : Option<usize>,

    pub search_until_mate : bool,
    pub ponder_search     : bool,
    pub infinity_search   : bool,
    pub stop_signal       : Arc<RwLock<bool>>,

    pub start_time        : Instant,
    pub node_count        : usize,
    pub board_history     : BoardHistory,
}

impl UciInformation {
    pub fn new() -> Self {
        Self {
            is_searching      : Arc::new(RwLock::new(false)),
            find_move_signal  : Arc::new(RwLock::new(false)),
            quit_signal       : Arc::new(RwLock::new(false)),
            wtime             : usize::MAX,
            btime             : usize::MAX,
            winc              : 0,
            binc              : 0,
            moves_to_go       : None,
            depth_limit       : 1,
            nodes_limit       : None,
            search_until_mate : false,
            node_count        : 0,
            ponder_search     : false,
            infinity_search   : false,
            time_limit        : None,
            start_time        : Instant::now(),
            board             : BoardStatus::new(),
            board_history     : BoardHistory::new(),
            stop_signal       : Arc::new(RwLock::new(false)),
        }
    }
    
    pub fn copy(&self) -> Self {
        Self { 
            is_searching: self.is_searching.clone(),
            find_move_signal: self.find_move_signal.clone(), 
            quit_signal: self.quit_signal.clone(), 
            wtime: self.wtime, 
            btime: self.btime, 
            winc: self.winc, 
            binc: self.binc, 
            depth_limit: self.depth_limit, 
            board: self.board, 
            moves_to_go: self.moves_to_go, 
            nodes_limit: self.nodes_limit, 
            time_limit: self.time_limit, 
            search_until_mate: self.search_until_mate, 
            ponder_search: self.ponder_search, 
            infinity_search: self.infinity_search, 
            stop_signal: self.stop_signal.clone(), 
            start_time: self.start_time, 
            node_count: self.node_count, 
            board_history: self.board_history, 
        }
    }

    pub fn is_search_fnished(&self) -> bool {
        if *self.stop_signal.read().unwrap() || *self.quit_signal.read().unwrap() {return true;}
        if let Some(time)  = self.time_limit {
            let time_limit = Duration::from_millis(time as u64);
            if self.start_time.elapsed().cmp(&time_limit) == Ordering::Less {return true;}
        }
        if let Some(node_limit) = self.nodes_limit {
            if self.node_count >= node_limit { return true; }
        }
        return false;
    }

    pub fn set_wtime(mut self, wtime: usize) -> Self {
        self.wtime = wtime;
        self
    }
    pub fn set_btime(mut self, btime: usize) -> Self {
        self.btime = btime;
        self
    }

    pub fn set_winc(mut self, winc: usize) -> Self {
        self.winc = winc;
        self
    }
    pub fn set_binc(mut self, binc: usize) -> Self {
        self.binc = binc;
        self
    }
    pub fn set_moves_to_go(mut self, moves_to_go: usize) -> Self {
        self.moves_to_go = Some(moves_to_go);
        self
    }
    pub fn set_depth_limit(mut self, depth_limit: isize) -> Self {
        self.depth_limit = depth_limit;
        self
    }

    pub fn set_nodes_limit(mut self, nodes_limit: usize) -> Self {
        self.nodes_limit = Some(nodes_limit);
        self
    }
    pub fn set_search_until_mate(mut self) -> Self {
        self.search_until_mate = true;
        self
    }

    pub fn set_ponder(mut self) -> Self {
        self.ponder_search = true;
        self.depth_limit = isize::MAX;
        self
    }

    pub fn set_infinite(mut self) -> Self {
        self.infinity_search = true;
        self.depth_limit = isize::MAX;
        self
    }

    pub fn set_board(mut self, board: BoardStatus) -> Self {
        self.board = board;
        self
    }
}

pub fn get_move(uci_info: &UciInformation, move_name: String) -> MoveBitField {
    let moves = MoveList::new(uci_info);
    let mov = moves.iterate_moves().find(|mov| mov.get_move_name() == move_name).unwrap();
    mov
}

pub fn position(mut data: SplitWhitespace, uci_info: &mut UciInformation) {
    match data.next() {
        Some("startpos") => uci_info.board = FenString::new(START_POS.to_string()).convert_to_board(),
        Some("fen")      => {
            uci_info.board = FenString::new(data.clone().collect::<Vec<&str>>().join(" ")).convert_to_board();
            data.next();
        }
        _ => println!("unkown arguments"),
    }

    if let Some("moves") = data.next() {
        for mov in data { uci_info.board.make_move(get_move(&uci_info, mov.to_string())); }
    }
}

pub fn go(mut data: SplitWhitespace, uci_info: &mut UciInformation) {
    loop {
        match data.next() {
            Some("wtime")     => uci_info.wtime           = data.next().unwrap().parse().unwrap(),
            Some("btime")     => uci_info.btime           = data.next().unwrap().parse().unwrap(),
            Some("winc")      => uci_info.winc            = data.next().unwrap().parse().unwrap(),
            Some("binc")      => uci_info.binc            = data.next().unwrap().parse().unwrap(),
            Some("depth")     => uci_info.depth_limit     = data.next().unwrap().parse().unwrap(),
            Some("movestogo") => uci_info.moves_to_go     = Some(data.next().unwrap().parse().unwrap()),
            Some("nodes")     => uci_info.nodes_limit     = Some(data.next().unwrap().parse().unwrap()),
            Some("movetime")  => uci_info.time_limit      = Some(data.next().unwrap().parse().unwrap()),
            Some("mate")      => uci_info.moves_to_go     = Some(data.next().unwrap().parse().unwrap()),
            Some("ponder")    => uci_info.ponder_search   = true,
            Some("infinite")  => uci_info.infinity_search = true,
            Some("perft")     => {
                perft_driver(&uci_info.copy().set_depth_limit(data.next().unwrap().parse().unwrap()));
                return;
            },
            _ => break,
        }
    }
    *uci_info.find_move_signal.write().unwrap() = true;
}


pub fn get_best_move(mut uci_info: UciInformation) {
    uci_info.node_count = 0;
    uci_info.start_time = Instant::now();
    uci_info.board_history = BoardHistory::new();
    *uci_info.stop_signal.write().unwrap() = false;
    *uci_info.is_searching.write().unwrap() = true;
    find_best_move(&mut uci_info);
    *uci_info.stop_signal.write().unwrap() = false;
    *uci_info.find_move_signal.write().unwrap() = false;
    *uci_info.is_searching.write().unwrap() = false;
    println!("bestmove {}", uci_info.board_history.found_best_move.get_move_name());
}

pub fn uci_loop() {
    let mut input = String::new();
    let mut uci_info = UciInformation::new();
    while *uci_info.quit_signal.read().unwrap() == false {
        input.clear();
        let _ = std::io::stdin().read_line(&mut input);

        let mut data = input.split_whitespace();
        match data.next() {
            Some("uci")         => if !*uci_info.find_move_signal.read().unwrap() {println!("id name Persa\nid author Yigit\nuciok")},
            Some("isready")     => if !*uci_info.find_move_signal.read().unwrap() {println!("readyok")},
            Some("go")          => if !*uci_info.find_move_signal.read().unwrap() { go(data, &mut uci_info) },
            Some("position")    => if !*uci_info.find_move_signal.read().unwrap() {position(data, &mut uci_info)} ,
            Some("stop")        => *uci_info.stop_signal.write().unwrap() = true,
            Some("ucinewgame")  => uci_info = UciInformation::new(),
            Some("quit")        => *uci_info.quit_signal.write().unwrap() = true,
            _                   => println!("unkown argument"),
        }
        let copy_uci = uci_info.copy();

        if *uci_info.find_move_signal.read().unwrap() && *uci_info.is_searching.read().unwrap() == false {
            std::thread::spawn(move || get_best_move(copy_uci) );
        }
    }
}