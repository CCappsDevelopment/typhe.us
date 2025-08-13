use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::ops::Index;

pub type Coord = (usize, usize);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Color {
    Black = 0,
    White = 1,
    Empty = 2,
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Empty => Color::Empty,
        }
    }
    pub fn is_stone(self) -> bool { matches!(self, Color::Black | Color::White) }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Stone { pub coord: Coord, pub color: Color }

#[derive(Clone, Serialize, Deserialize)]
pub struct Goban {
    pub size: usize,
    pub tab: Vec<Color>,
}

impl Goban {
    pub fn new(size: usize) -> Self { Goban { size, tab: vec![Color::Empty; size * size] } }
    fn idx(&self, coord: Coord) -> usize { coord.0 * self.size + coord.1 }
    pub fn coord_valid(&self, coord: &Coord) -> bool { coord.0 < self.size && coord.1 < self.size }

    pub fn push(&mut self, coord: Coord, color: Color) -> Result<(), &'static str> {
        if !self.coord_valid(&coord) { return Err("Out of bounds"); }
        let pos = self.idx(coord);
        if self.tab[pos] != Color::Empty { return Err("Occupied"); }
        self.tab[pos] = color; Ok(())
    }

    pub fn set(&mut self, coord: Coord, color: Color) {
        if self.coord_valid(&coord) { let idx = self.idx(coord); self.tab[idx] = color; }
    }

    pub fn push_many<I: IntoIterator<Item = Coord>>(&mut self, coords: I, color: Color) {
        for coord in coords { self.set(coord, color); }
    }

    pub fn get_neighbors(&self, coord: &Coord) -> impl Iterator<Item = Stone> + '_ {
        neighbors_coords(coord)
            .into_iter()
            .filter(move |c| self.coord_valid(c))
            .map(move |c| Stone { coord: c, color: self[c] })
    }

    pub fn dfs(&self, start: &Stone) -> HashSet<Stone> {
        let mut explored: HashSet<Stone> = HashSet::new();
        explored.insert(*start);
        let mut stack: Vec<Stone> = self.get_neighbors(&start.coord)
            .filter(|n| n.color == start.color)
            .collect();
        while let Some(stone) = stack.pop() {
            if explored.insert(stone) {
                for n in self.get_neighbors(&stone.coord) {
                    if n.color == start.color && !explored.contains(&n) { stack.push(n); }
                }
            }
        }
        explored
    }

    pub fn has_liberty(&self, stone: &Stone) -> bool {
        self.get_neighbors(&stone.coord).any(|n| n.color == Color::Empty)
    }

    pub fn group_liberties(&self, stones: &HashSet<Stone>) -> HashSet<Coord> {
        let mut libs: HashSet<Coord> = HashSet::new();
        for s in stones {
            for n in self.get_neighbors(&s.coord) {
                if n.color == Color::Empty { libs.insert(n.coord); }
            }
        }
        libs
    }

    pub fn is_string_dead(&self, stones: &HashSet<Stone>) -> bool { self.group_liberties(stones).is_empty() }

    pub fn get_groups_of_stones_color_without_liberties(&self, color: Color) -> Vec<HashSet<Stone>> {
        let mut visited: HashSet<Stone> = HashSet::new();
        let mut out = Vec::new();
        for (i, &c) in self.tab.iter().enumerate() {
            if c == color {
                let coord = (i / self.size, i % self.size);
                let stone = Stone { coord, color };
                if visited.contains(&stone) { continue; }
                let group = self.dfs(&stone);
                for s in &group { visited.insert(*s); }
                if self.is_string_dead(&group) { out.push(group); }
            }
        }
        out
    }
}

impl Index<Coord> for Goban {
    type Output = Color;
    fn index(&self, i: Coord) -> &Self::Output { &self.tab[self.idx(i)] }
}

pub fn neighbors_coords(coord: &Coord) -> Vec<Coord> {
    let mut n = Vec::with_capacity(4);
    // Raw neighbor coordinates; callers should filter with coord_valid
    if coord.0 > 0 { n.push((coord.0 - 1, coord.1)); }
    n.push((coord.0 + 1, coord.1));
    if coord.1 > 0 { n.push((coord.0, coord.1 - 1)); }
    n.push((coord.0, coord.1 + 1));
    n
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player { Black, White }
impl std::ops::Not for Player { type Output = Player; fn not(self) -> Player { match self { Player::Black => Player::White, Player::White => Player::Black } } }
impl Player { pub fn color(self) -> Color { match self { Player::Black => Color::Black, Player::White => Color::White } } }

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum EndGame { WinnerByPoints(Player, f32), WinnerByResign(Player), Draw }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Move { Play(usize, usize), Pass, Resign }

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    goban: Goban,
    turn: Player,
    passes: u8,
    plays: Vec<u64>, // position hashes for Ko/repetition
    resigned: Option<Player>,
    outcome: Option<EndGame>,
    zobrist_table: Vec<u64>,
    black_captures: u32,
    white_captures: u32,
    history: Vec<Snapshot>,
    future: Vec<Snapshot>,
}

#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub ok: bool,
    pub reason: Option<String>,
    pub board: Vec<u8>,
    pub turn: Player,
    pub captures: (u32, u32),
    pub ko: Option<(usize, usize)>,
    pub is_over: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Snapshot {
    goban_tab: Vec<Color>,
    turn: Player,
    passes: u8,
    plays: Vec<u64>,
    black_captures: u32,
    white_captures: u32,
    resigned: Option<Player>,
    outcome: Option<EndGame>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Game {
    // Better panic messages in browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
        let zobrist_table = generate_zobrist_table(size);
        Game {
            goban: Goban::new(size),
            turn: Player::Black,
            passes: 0,
            plays: Vec::new(),
            resigned: None,
            outcome: None,
            zobrist_table,
            black_captures: 0,
            white_captures: 0,
            history: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn size(&self) -> usize { self.goban.size }

    pub fn get_board(&self) -> JsValue {
        // expose as u8s: 0=Black,1=White,2=Empty
        let v: Vec<u8> = self.goban.tab.iter().map(|c| *c as u8).collect();
        JsValue::from_serde(&v).unwrap()
    }

    pub fn get_turn(&self) -> JsValue { JsValue::from_serde(&self.turn).unwrap() }
    pub fn turn_num(&self) -> u8 { match self.turn { Player::Black => 0, Player::White => 1 } }

    pub fn play_move(&mut self, row: usize, col: usize) -> JsValue {
        let color = self.turn.color();
        let coord = (row, col);
        let mut reason: Option<String> = None;
        let mut ok = true;
        let before = self.snapshot();

        // place stone
        if let Err(e) = self.goban.push(coord, color) {
            ok = false; reason = Some(e.to_string());
        } else {
            // capture opponent first, then self (TT rule 7)
            let captured_opp = self.remove_captured_stones_for(!self.turn);
            let captured_self = self.remove_captured_stones_for(self.turn);
            if captured_self > 0 {
                ok = false; reason = Some("Suicide move".to_string());
                self.restore(before);
            } else {
                // Ko: disallow repeated positions
                let hash = self.goban_hash();
                if self.plays.contains(&hash) {
                    ok = false; reason = Some("Ko: repeated position".into());
                    self.restore(before);
                } else {
                    // Save snapshot to history, clear future, then commit
                    self.history.push(before);
                    self.future.clear();
                    self.plays.push(hash);
                    // update capture counts
                    if self.turn == Player::Black { self.black_captures += captured_opp as u32; } else { self.white_captures += captured_opp as u32; }
                    self.turn = !self.turn;
                    self.passes = 0;
                    // If there are no more legal plays for both sides, end by points
                    if self.no_more_legal_plays() {
                        self.compute_points_outcome();
                    }
                }
            }
        }

        let res = MoveResult {
            ok,
            reason,
            board: self.goban.tab.iter().map(|c| *c as u8).collect(),
            turn: self.turn,
            captures: (self.black_captures, self.white_captures),
            ko: None,
        is_over: self.is_over(),
        };
    JsValue::from_serde(&res).unwrap()
    }

    pub fn pass(&mut self) -> JsValue {
    self.history.push(self.snapshot());
    self.future.clear();
    self.passes += 1; self.turn = !self.turn;
    if self.passes >= 2 { self.compute_points_outcome(); }
    let res = MoveResult { ok: true, reason: None, board: self.goban.tab.iter().map(|c| *c as u8).collect(), turn: self.turn, captures: (self.black_captures, self.white_captures), ko: None, is_over: self.is_over() };
    JsValue::from_serde(&res).unwrap()
    }

    pub fn resign(&mut self) -> JsValue {
        self.history.push(self.snapshot());
        self.future.clear();
        self.resigned = Some(self.turn);
        self.outcome = Some(EndGame::WinnerByResign(!self.turn));
    let res = MoveResult { ok: true, reason: None, board: self.goban.tab.iter().map(|c| *c as u8).collect(), turn: self.turn, captures: (self.black_captures, self.white_captures), ko: None, is_over: true };
    JsValue::from_serde(&res).unwrap()
    }

    pub fn undo(&mut self) -> JsValue {
        if let Some(prev) = self.history.pop() {
            let cur = self.snapshot();
            self.future.push(cur);
            self.restore(prev);
        }
    let res = MoveResult { ok: true, reason: None, board: self.goban.tab.iter().map(|c| *c as u8).collect(), turn: self.turn, captures: (self.black_captures, self.white_captures), ko: None, is_over: self.is_over() };
    JsValue::from_serde(&res).unwrap()
    }

    pub fn redo(&mut self) -> JsValue {
        if let Some(next) = self.future.pop() {
            let cur = self.snapshot();
            self.history.push(cur);
            self.restore(next);
        }
    let res = MoveResult { ok: true, reason: None, board: self.goban.tab.iter().map(|c| *c as u8).collect(), turn: self.turn, captures: (self.black_captures, self.white_captures), ko: None, is_over: self.is_over() };
    JsValue::from_serde(&res).unwrap()
    }

    pub fn get_capture_counts(&self) -> JsValue {
        JsValue::from_serde(&(self.black_captures, self.white_captures)).unwrap()
    }

    pub fn get_group_and_liberties(&self, row: usize, col: usize) -> JsValue {
        let color = self.goban[(row, col)];
        if color == Color::Empty { return JsValue::NULL; }
        let stone = Stone { coord: (row, col), color };
        let group = self.goban.dfs(&stone);
        let libs = self.goban.group_liberties(&group);
        let group_vec: Vec<(usize,usize)> = group.iter().map(|s| s.coord).collect();
        let libs_vec: Vec<(usize,usize)> = libs.into_iter().collect();
    JsValue::from_serde(&(color, group_vec, libs_vec)).unwrap()
    }

    pub fn get_scores(&self) -> JsValue {
        let (b, w) = self.count_points();
    JsValue::from_serde(&(b, w)).unwrap()
    }

    pub fn is_over(&self) -> bool { self.passes >= 2 || self.resigned.is_some() || self.no_more_legal_plays() }

    pub fn get_outcome(&self) -> JsValue { JsValue::from_serde(&self.outcome).unwrap() }

    pub fn export_state(&self) -> JsValue {
        #[derive(Serialize)]
        struct State<'a> {
            size: usize,
            board: &'a [Color],
            turn: Player,
            passes: u8,
            plays: &'a [u64],
            black_captures: u32,
            white_captures: u32,
            resigned: Option<Player>,
            outcome: &'a Option<EndGame>,
        }
        let st = State {
            size: self.goban.size,
            board: &self.goban.tab,
            turn: self.turn,
            passes: self.passes,
            plays: &self.plays,
            black_captures: self.black_captures,
            white_captures: self.white_captures,
            resigned: self.resigned,
            outcome: &self.outcome,
        };
    JsValue::from_serde(&st).unwrap()
    }

    pub fn import_state(&mut self, v: JsValue) -> Result<(), JsValue> {
        #[derive(Deserialize)]
        struct State {
            size: usize,
            board: Vec<Color>,
            turn: Player,
            passes: u8,
            plays: Vec<u64>,
            black_captures: u32,
            white_captures: u32,
            resigned: Option<Player>,
            outcome: Option<EndGame>,
        }
        let st: State = serde_wasm_bindgen::from_value(v).map_err(|e| JsValue::from_str(&format!("{e}")))?;
        if st.size != self.goban.size { return Err(JsValue::from_str("Size mismatch")); }
        self.goban.tab = st.board;
        self.turn = st.turn;
        self.passes = st.passes;
        self.plays = st.plays;
        self.black_captures = st.black_captures;
        self.white_captures = st.white_captures;
        self.resigned = st.resigned;
        self.outcome = st.outcome;
        Ok(())
    }

    fn remove_captured_stones_for(&mut self, who: Player) -> usize {
        // remove captured groups of the specified player's color
        let groups = self.goban.get_groups_of_stones_color_without_liberties(who.color());
        let mut count = 0;
        for g in groups { count += g.len(); self.goban.push_many(g.iter().map(|s| s.coord), Color::Empty); }
        count
    }

    fn count_points(&self) -> (usize, usize) {
        // Tromp-Taylor scoring (area scoring): stones + territory that reaches only one color
        let size = self.goban.size;
        let mut empty_points: Vec<Stone> = Vec::new();
        for r in 0..size { for c in 0..size { if self.goban[(r,c)] == Color::Empty { empty_points.push(Stone{coord:(r,c), color: Color::Empty}); } } }
        let empty_groups = self.get_strings_from_stones(empty_points.into_iter());
        let mut black_score = 0usize; let mut white_score = 0usize;

        for group in empty_groups {
            let mut reaches_black = false; let mut reaches_white = false;
            for point in &group {
                for neighbor in self.goban.get_neighbors(&point.coord) {
                    if neighbor.color == Color::Black { reaches_black = true; }
                    else if neighbor.color == Color::White { reaches_white = true; }
                }
            }
            if reaches_black && !reaches_white { black_score += group.len(); }
            else if !reaches_black && reaches_white { white_score += group.len(); }
        }
        for &color in &self.goban.tab {
            match color { Color::Black => black_score += 1, Color::White => white_score += 1, _ => {} }
        }
        (black_score, white_score)
    }

    fn get_strings_from_stones<I>(&self, stones: I) -> Vec<HashSet<Stone>>
        where I: IntoIterator<Item = Stone>
    {
        let mut stones_set: HashSet<Stone> = stones.into_iter().collect();
        let mut groups: Vec<HashSet<Stone>> = Vec::new();
        while let Some(stone) = stones_set.iter().next().cloned() {
            let group = self.goban.dfs(&stone);
            for s in &group { stones_set.remove(s); }
            groups.push(group);
        }
        groups
    }

    fn goban_hash(&self) -> u64 { zobrist_hash_state(&self.goban, &self.zobrist_table) }

    fn compute_points_outcome(&mut self) {
        let (b, w) = self.count_points();
        if b > w { self.outcome = Some(EndGame::WinnerByPoints(Player::Black, (b - w) as f32)); }
        else if w > b { self.outcome = Some(EndGame::WinnerByPoints(Player::White, (w - b) as f32)); }
        else { self.outcome = Some(EndGame::Draw); }
    }

    fn no_more_legal_plays(&mut self) -> bool {
        !self.has_any_legal_play_for(Player::Black) && !self.has_any_legal_play_for(Player::White)
    }

    fn has_any_legal_play_for(&mut self, who: Player) -> bool {
        let size = self.goban.size;
        for r in 0..size { for c in 0..size {
            if self.goban[(r,c)] != Color::Empty { continue; }
            if self.is_legal_move_at(r, c, who) { return true; }
        }}
        false
    }

    fn is_legal_move_at(&mut self, row: usize, col: usize, who: Player) -> bool {
        if self.goban[(row, col)] != Color::Empty { return false; }
        // Use snapshot/restore to simulate without committing
        let snap = self.snapshot();
        let color = who.color();
        let coord = (row, col);
        let mut legal = true;
        if self.goban.push(coord, color).is_err() { legal = false; }
        else {
            // capture opponent then self
            let _ = self.remove_captured_stones_for(!who);
            let captured_self = self.remove_captured_stones_for(who);
            if captured_self > 0 { legal = false; }
            else {
                let hash = self.goban_hash();
                if self.plays.contains(&hash) { legal = false; }
            }
        }
        self.restore(snap);
        legal
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            goban_tab: self.goban.tab.clone(),
            turn: self.turn,
            passes: self.passes,
            plays: self.plays.clone(),
            black_captures: self.black_captures,
            white_captures: self.white_captures,
            resigned: self.resigned,
            outcome: self.outcome.clone(),
        }
    }

    fn restore(&mut self, snap: Snapshot) {
        self.goban.tab = snap.goban_tab;
        self.turn = snap.turn;
        self.passes = snap.passes;
        self.plays = snap.plays;
        self.black_captures = snap.black_captures;
        self.white_captures = snap.white_captures;
        self.resigned = snap.resigned;
        self.outcome = snap.outcome;
    }
}

// Zobrist hashing
fn generate_zobrist_table(size: usize) -> Vec<u64> {
    // Deterministic splitmix64 PRNG to avoid std/rand deps in wasm
    fn splitmix64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    let mut seed = 0x1234_5678_9ABC_DEF0u64 ^ (size as u64);
    let mut out = Vec::with_capacity(size * size * 3);
    for _ in 0..(size * size * 3) {
        seed = splitmix64(seed);
        out.push(seed);
    }
    out
}

fn zobrist_hash_state(goban: &Goban, zobrist_table: &[u64]) -> u64 {
    let mut hash = 0u64;
    for (i, &color) in goban.tab.iter().enumerate() { hash ^= zobrist_table[i * 3 + (color as usize)]; }
    hash
}
