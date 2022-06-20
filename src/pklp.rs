//region Public

use rayon::prelude::*;

#[allow(dead_code)]
pub fn str_to_json(data: &str) -> String {
    to_json(&parse_string(data), data).unwrap()
}

#[allow(dead_code)]
pub fn path_to_json(path: &str) -> String {
    let data = std::fs::read_to_string(path).unwrap();
    str_to_json(&data)
}

#[allow(dead_code)]
pub fn path_to_json_file(path: &str, output_path: &str) {
    let data = std::fs::read_to_string(path).unwrap();
    std::fs::write(output_path, &str_to_json(&data)).unwrap();
}

#[allow(dead_code)]
pub fn strs_to_json(data: &[&str]) -> String {
    let strings = data.par_iter()
        .map(|f| str_to_json(*f))
        .collect::<Vec<String>>();
    json_join(&strings)
}

#[allow(dead_code)]
pub fn paths_to_json(paths: &[&str]) -> String {
    let strings = paths.par_iter()
        .map(|f| path_to_json(*f))
        .collect::<Vec<String>>();
    json_join(&strings)
}

#[allow(dead_code)]
pub fn paths_to_json_file(paths: &[&str], output_path: &str) {
    std::fs::write(output_path, &paths_to_json(paths)).unwrap();
}

fn json_join(json: &[String]) -> String {
    // todo: Parallelize
    let cap = json.iter().map(|x| x.len()).sum();
    let mut out = String::with_capacity(cap);
    out.push('[');
    let mut first = true;
    for s in json { 
        if !first { out.push(','); } 
        out.push_str(&s[1..s.len()-1]); 
        first = false; 
    }
    out.push(']');
    out
}

//endregion

//region Lib

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    begin: u32,
    size: u16,
}

impl Span {
    fn new(begin: u32, size: u16) -> Self { Self{begin, size} }
    
    fn from_str_slice(part: &str, whole_buffer: &str) -> Self {
        let offset = part.as_ptr() as usize - whole_buffer.as_ptr() as usize;
        Self::new(offset as u32, part.len() as u16)
    }
}


impl std::ops::Index<Span> for str {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        <str as std::ops::Index<std::ops::Range<usize>>>::index(self, index.begin as usize..(index.begin + index.size as u32) as usize)
    }
}

impl<T> std::ops::Index<Span> for [T] {
    type Output = [T];
    fn index(&self, index: Span) -> &Self::Output {
        <[T] as std::ops::Index<std::ops::Range<usize>>>::index(self, index.begin as usize..(index.begin + index.size as u32) as usize)
    }
}


trait StringOps<'a> {
    fn str(&self) -> &'a str;
    #[inline] fn prefix(&self, end: char)                 -> &'a str { &self.str()[..self.str().find(end).unwrap()] }
    #[inline] fn rprefix(&self, end: char)                -> &'a str { &self.str()[..self.str().rfind(end).unwrap()] }
    #[inline] fn prefix_str(&self, end: &'a str)          -> &'a str { &self.str()[..self.str().find(end).unwrap()] }
    #[inline] fn rprefix_str(&self, end: &'a str)         -> &'a str { &self.str()[..self.str().rfind(end).unwrap()] }
    #[inline] fn suffix(&self, start: char)               -> &'a str { &self.str()[self.str().find(start).unwrap()+1..] }
    #[inline] fn rsuffix(&self, start: char)              -> &'a str { &self.str()[self.str().rfind(start).unwrap()+1..] }    
    #[inline] fn suffix_str(&self, start: &str)           -> &'a str { &self.str()[self.str().find(start).unwrap()+start.len()..] }    
    #[inline] fn rsuffix_str(&self, start: &str)          -> &'a str { &self.str()[self.str().rfind(start).unwrap()+start.len()..] }
    #[inline] fn between(&self, start: char, end: char)   -> &'a str { let b = self.str().find(start).unwrap()+1; &self.str()[b..b+self.str()[b..].find(end).unwrap()] }
    #[inline] fn rbetween(&self, start: char, end: char)  -> &'a str { let e = self.str().rfind(end).unwrap(); &self.str()[self.str()[..e].rfind(start).unwrap()+1..e] }
}

impl<'a> StringOps<'a> for &'a str {
    fn str(&self) -> &'a str { self }
}

//endregion

//region Structs

use stackvector::StackVec;

pub type Cards = StackVec<[Card; 5]>;
pub type PlayerVec = StackVec<[Player; 8]>;
pub type ActionVec = Vec<Action>;
pub type Float = f32;

#[derive(Debug, Clone, Copy)]
pub enum Pot { Main, Side }


#[derive(Clone, Copy)]
pub struct Card {pub n: u8, pub suit: Suit}


#[derive(Clone, Copy)]
pub struct Currency{pub symbol: char, pub amount: Float}


#[derive(Debug, Clone, Copy)]
pub struct Table{pub name: Span, pub max_players: u8, pub button: u8}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum BlindType {
    Small,
    Big,
    SmallAndBig,
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Event {
    Connect,
    Disconnect,
    Timeout,
    Leave,
    Sitout,
    NotShow,
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Play {
    Bet,
    Call,
    Check,
    Raise,
    Stand,
    Discard,
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StreetType {
    Flop,
    Turn,
    River,
    Showdown,
}


#[derive(Debug, Clone)]
pub enum Action {
    Event(Event, Span),
    Join(u8, Span),
    Play(Play, Span, Option<ActionData>),
    Blind(BlindType, Span, Currency),
    ShowHand(Span, Cards),
    Fold(Span, Option<Cards>),
    Say(Span, Span),
    CashOut(Span, Currency, Currency),
    UncalledBetReturned(Span, Currency),
    CollectedPot(Pot, Span, Currency),
}


#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond
}


#[derive(Debug, Clone, Copy)]
pub struct ActionData {
    pub bet: Currency,
    pub bet_to: Option<Currency>,
    pub all_in: bool,
}


#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub name: Span,
    pub seat: u8,
    pub chips: Currency,
    pub bounty: Option<Currency>,
}


#[derive(Debug)]
pub struct HandInfo {
    pub site: Span,
    pub hand: Span,
    pub id: Span,
    pub game_type: Span,
    pub buy_in_min: Currency,
    pub buy_in_max: Currency,
}


#[derive(Debug)]
pub struct Header {
    pub info: HandInfo,
    pub table: Table,
    pub players: PlayerVec,
    pub actions: ActionVec,
}


#[derive(Debug)]
pub struct HoleCards {
    pub dealt_to: Vec<(Span, Cards)>,
    pub actions: ActionVec,
}


#[derive(Debug)]
pub struct Street {
    pub t: StreetType,
    pub index: u8,
    pub cards: Option<Cards>,
    pub new_card: Option<Card>,
    pub actions: ActionVec,
}


#[derive(Debug)]
pub struct Summary {
    pub pot: Currency,
    pub main_pot: Currency,
    pub side_pot: Currency,
    pub rake: Option<Currency>,
    pub boards: Vec<Cards>,
}


#[derive(Debug)]
pub struct Hand {
    pub header: Header,
    pub hole_cards: HoleCards,
    pub streets: Vec<Street>,
    pub summary: Summary,
}


impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit_c = match self.suit {
            Suit::Heart => 'h',
            Suit::Club => 'c',
            Suit::Diamond => 'd',
            Suit::Spade => 's',
        };
        write!(f, "{}{}", self.n, suit_c)
    }
}

impl std::fmt::Debug for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.symbol, self.amount)
    }
}

//endregion

//region Parser

pub fn parse_string(s: &str) -> Vec<Hand> {
    let mut parser = Parser::new(s);
    parser.parse();
    parser.hands
}

enum ParseState {
    HandHeader,
    HoleCards,
    Street,
    Summary,
}

struct Parser<'a> {
    state: ParseState,
    lines: std::str::Split<'a, &'static str>,
    line: &'a str,
    data: &'a str,
    hands: Vec<Hand>,
    header: Option<Header>,
    hole_cards: Option<HoleCards>,
    streets: Vec<Street>,
}

impl<'a> Parser<'a> {
    fn eof(&self) -> bool { self.line == "@@@EOF@@@" }
    fn line(&self) -> &'a str { self.line }
    fn next(&mut self) -> &'a str { let l = self.line; self.advance(); l }
    fn advance(&mut self) { self.line = self.lines.next().unwrap_or("@@@EOF@@@"); }

    fn span(&self, s: &str) -> Span { Span::from_str_slice(s, self.data) }

    fn new(s: &'a str) -> Self { 
        let mut p = Self{
            state: ParseState::HandHeader,
            lines: s.split(if s.contains("\r\n") { "\r\n" } else { "\n" }),
            line: "",
            data: s,
            hands: Vec::new(),
            header: None, 
            hole_cards: None, 
            streets: Vec::new()
        };
        p.advance();
        p
    }

    fn parse(&mut self) {
        while !self.eof() {
            let line = self.line();
            if line.is_empty() || line == "\r" {
                self.state = ParseState::HandHeader;
                self.advance();
                continue;
            }
            else if line.starts_with("*** HOLE") {
                self.state = ParseState::HoleCards;
            }
            else if line.starts_with("*** SUMM") {
                self.state = ParseState::Summary;
            }
            else if line.starts_with("***") {
                self.state = ParseState::Street;
            }
            match self.state {
                ParseState::HandHeader => {
                    let header = parse_header(self);
                    self.header = Some(header);
                }
                ParseState::HoleCards => {
                    let hole_cards = parse_hole_cards(self);
                    self.hole_cards = Some(hole_cards);
                }
                ParseState::Street => {
                    let street = parse_street(self);
                    self.streets.push(street);
                }
                ParseState::Summary => {
                    let summary = parse_summary(self);

                    let mut header = None;
                    std::mem::swap(&mut self.header, &mut header);

                    let mut hole_cards = None;
                    std::mem::swap(&mut self.hole_cards, &mut hole_cards);

                    let mut streets = Vec::new();
                    std::mem::swap(&mut self.streets, &mut streets);

                    self.hands.push(Hand { header: header.unwrap(), hole_cards: hole_cards.unwrap(), streets, summary });
                }
            }
        }
    }
}

//endregion

//region Parse - Header

fn parse_header(p: &mut Parser) -> Header {
    let info = parse_header_info( p);
    let table = parse_header_table(p);

    let mut players = PlayerVec::new();
    while p.line().starts_with("Seat ") {
        players.push(parse_header_player(p));
    }

    let actions = parse_action_list(p);

    Header{info, table, players, actions}
}


fn parse_header_player(p: &mut Parser) -> Player {
    let line = p.next();
    let seat = str::parse::<u64>(&line["Seat ".len()..line.find(':').unwrap()]).unwrap() as u8;
    let chips_begin = line.rfind('(').unwrap() + 1;
    let name_end = chips_begin - 2;
    let chips_end = line.rfind(" in chips").unwrap();
    let chips = parse_currency(&line[chips_begin..chips_end]);
    let bounty = if let Some(bounty_end) = line.rfind(" bounty") {
        Some(parse_currency(&line[chips_end+2..bounty_end]))
    } else {
        None
    };
    Player{name: p.span(&line["Seat 0: ".len()..name_end]), seat, chips, bounty}
}


fn parse_header_table(p: &mut Parser) -> Table {
    let line = p.next();
    let name = line.between('\'', '\'');
    let seat_begin = line.rfind("#").unwrap() + 1;
    let count_begin = name.len() + "Table '' ".len();
    let max_players = parse_integer(&line[count_begin..]).unwrap().0 as u8;
    let button = parse_integer(&line[seat_begin..]).unwrap().0 as u8;
    Table { name: p.span(name), max_players, button }
}


fn parse_header_info(p: &mut Parser) -> HandInfo {
    let line = p.next();
    let id_begin = line.find('#').unwrap() + 1;
    let id_end = line.find(':').unwrap();
    let buy_in_begin = line.find('(').unwrap() + 1;

    let id = &line[id_begin..id_end];
    let game_type = line[id_end + 1..buy_in_begin-1].trim();
    
    let mut site_hand = line[0..id_begin - 1].trim().split(' ');
    let (site, hand) = (site_hand.next().unwrap(), site_hand.next().unwrap());
    
    let buy_in_part = &line[buy_in_begin..line.find(')').unwrap()];
    let buy_in_min_end = buy_in_part.find('/').unwrap();
    let buy_in_max_end = buy_in_min_end + buy_in_part[buy_in_min_end..].find(' ').unwrap_or(buy_in_part.len() - buy_in_min_end);
    let buy_in_min = parse_currency(&buy_in_part[..buy_in_min_end]);
    let buy_in_max = parse_currency(&buy_in_part[buy_in_min_end+1..buy_in_max_end]);

    HandInfo{site: p.span(site), hand: p.span(hand), id: p.span(id), game_type: p.span(game_type), buy_in_min, buy_in_max}
}

//endregion

//region Parse - HoleCards

fn parse_hole_cards(p: &mut Parser) -> HoleCards {
    p.advance();

    let mut dealt_to = Vec::new();
    while p.line().starts_with("Dealt to") {
        dealt_to.push(parse_hole_cards_dealt_to(p));
    }

    let actions = parse_action_list(p);

    HoleCards{dealt_to, actions}
}

fn parse_hole_cards_dealt_to(p: &mut Parser) -> (Span, Cards) {
    let line = p.next();
    let name_cards = line.rsuffix_str("Dealt to ");
    let cards_begin = name_cards.rfind('[').unwrap() + 1;
    let name = &name_cards[..cards_begin-2];
    let cards = parse_cards(&name_cards[cards_begin..name_cards.len()-1]);
    (p.span(name), cards)
}

//endregion

//region Parse - Street

fn parse_street(p: &mut Parser) -> Street {
    let header_line = p.next();
    let actions = parse_action_list(p);
    let (index, t) = parse_street_type(header_line);
    if let StreetType::Showdown = t {
        Street{t, index, cards: None, new_card: None, actions}
    } else {
        let (cards, new_card) = parse_street_cards(header_line);
        Street{t, index, cards: Some(cards), new_card, actions}
    }
}


fn parse_street_type<'a>(line: &'a str) -> (u8, StreetType) {
    let end = line.rfind("***").unwrap();
    let part = line[2..end].trim();
    let t = match &part[part.len()-2..] {
        "OP" => StreetType::Flop,
        "RN" => StreetType::Turn,
        "ER" => StreetType::River,
        "WN" => StreetType::Showdown,
        _ => unreachable!(),
    };
    let idx = if let Some((i, _)) = &["FIRST", "SECOND", "THIRD", "FOURTH"].iter().enumerate().find(|x| part.starts_with(*x.1)) {
        *i
    } else {
        0
    };
    (idx as u8, t)
}


fn parse_street_cards<'a>(line: &'a str) -> (Cards, Option<Card>) {
    let cards = parse_cards(line.between('[', ']'));
    if line.matches('[').count() == 2 {
        (cards, Some(parse_cards(line.rbetween('[', ']'))[0]))
    } else {
        (cards, None)
    }
}

//endregion

//region Parse - Summary

fn parse_summary(p: &mut Parser) -> Summary {
    
    p.advance();
    
    let mut boards = Vec::new();
    let (pot, main_pot, side_pot, rake) = parse_summary_pot(p.next());

    while p.line().starts_with("Hand was run twice") {
        p.advance();
    }

    while let Some(board_begin) = p.line().rfind("Board [") {
        boards.push(parse_cards(&p.line()[board_begin + "Board [".len()..p.line().len()-1]));
        p.advance();
    }

    while p.line().starts_with("Seat ") {
        p.advance();
        if p.eof() { break; }
    }

    Summary{pot, main_pot, side_pot, rake, boards}
}


fn parse_summary_pot<'a>(line: &'a str) -> (Currency, Currency, Currency, Option<Currency>) {
    let total_begin = line.find("Total pot ").unwrap();
    let total_pot = parse_currency_dynamic(&line[total_begin + "Total pot ".len()..]).unwrap().0;
    let main_pot = if let Some(begin) = line.find("Main pot ") {
        parse_currency_dynamic(&line[begin + "Main pot ".len()..]).unwrap().0
    } else {
        total_pot
    };
    let side_pot = if let Some(begin) = line.rfind("Side pot ") {
        parse_currency_dynamic(&line[begin + "Side pot ".len()..]).unwrap().0
    } else {
        Currency{symbol: total_pot.symbol, amount: 0.0}
    };
    let rake = if let Some(begin) = line.rfind("Rake ") {
        let rake_part = &line[begin + "Rake ".len()..];
        Some(if rake_part.contains(total_pot.symbol) {
            parse_currency_dynamic(rake_part).unwrap().0
        } else {
            let amount = str::parse::<Float>(rake_part).unwrap();
            Currency{symbol: total_pot.symbol, amount: amount}
        })
    } else {
        None
    };
    (total_pot, main_pot, side_pot, rake)
}

//endregion

//region Parse - Action

fn parse_action_list(p: &mut Parser) -> ActionVec {
    let mut actions = ActionVec::new();
    while let Some(a) = parse_action(p) {
        actions.push(a);
        p.advance();
        if p.eof() || p.line().starts_with("***") { break; }
    }
    actions
}


fn parse_action(p: &mut Parser) -> Option<Action> {
    let line = p.line();
    
    // toyochan: checks
    // toyochan: folds
    // toyochan: discards
    // toyochan: stands
    // toyochan: doesn't show hand
    // toyochan: mucks hand
    // toyochan: sits out
    // toyochan: is sitting out
    // toyochan is connected
    // toyochan is disconnected
    // toyochan leaves the table
    // toyochan has timed out
    // toyochan has timed out while disconnected
    // toyochan has timed out while being disconnected
    // toyochan was removed from the table for failing to post
    // toyochan will be allowed to play after the button
    
    // toyochan: bets $1.89
    // toyochan: calls $1.89
    // toyochan: raises $0.70 to $1.20
    // toyochan: raises $0.70 to $1.20 and is all in
    // toyochan: raises $0.70 to $1.20 and is all-in
    // toyochan: posts small blind $0.25
    // toyochan: posts big blind $0.50
    // toyochan: posts small & big blinds $0.50
    // todochan: shows [Ac 6s] (a pair of Deuces)

    // toyochan said, "ANYTHING"
    // toyochan collected $14.81 from pot
    // toyochan collected $14.81 from main pot
    // toyochan collected $14.81 from side pot
    // toyochan joins the table at seat #6
    // toyochan cashed out the hand for $1 | Cash Out Fee $2
    
    // Uncalled bet ($8.58) returned to toyochan

    let r = if line.ends_with(": checks")
    {
        Action::Play(Play::Check, p.span(line.prefix(':')), None)
    }
    else if line.ends_with(": folds")
    {
        Action::Fold(p.span(line.prefix(':')), None)
    }
    else if line.ends_with(": discards")
    {
        Action::Play(Play::Discard, p.span(line.prefix(':')), None)
    }
    else if line.ends_with(": stands")
    {
        Action::Play(Play::Stand, p.span(line.prefix(':')), None)
    }
    else if line.ends_with(": sits out") 
         || line.ends_with(": is sitting out") 
    {
        Action::Event(Event::Sitout, p.span(line.prefix(':')))
    }
    else if line.ends_with(": doesn't show hand") 
         || line.ends_with(": mucks hand") 
    {
        Action::Event(Event::NotShow, p.span(line.prefix(':')))
    }
    else if line.ends_with("leaves the table")
    {
        Action::Event(Event::Leave, p.span(line.rprefix_str(" leaves the table")))
    }
    else if line.ends_with("is connected")
    {
        Action::Event(Event::Connect, p.span(line.rprefix_str(" is connected")))
    }
    else if line.ends_with("is disconnected")
    {
        Action::Event(Event::Disconnect, p.span(line.rprefix_str(" is disconnected")))
    }
    else if line.ends_with("has timed out")
         || line.ends_with("has timed out while disconnected")
         || line.ends_with("has timed out while being disconnected")
    {
        Action::Event(Event::Timeout, p.span(line.rprefix_str(" has timed out")))
    }
    else if line.ends_with("was removed from the table for failing to post")
    {
        Action::Event(Event::Timeout, p.span(line.rprefix_str(" was removed from")))
    }
    else if line.ends_with("will be allowed to play after the button")
    {
        Action::Join(255, p.span(line.rprefix_str(" will be allowed")))
    }
    else if line.starts_with("Uncalled bet") 
    {
        let returned_to_begin = line.rfind(" returned to ").unwrap();
        let name = &line[returned_to_begin+" returned to ".len()..];
        Action::UncalledBetReturned(p.span(name), parse_currency(line.between('(', ')')))
    }
    else if line.contains(": folds [")
    {
        Action::Fold(p.span(line.prefix(':')), Some(parse_cards(line.rbetween('[', ']'))))
    }
    else if let Some(a) = parse_bet_call_raise(p, ": bets ", Play::Bet) {
        a
    }
    else if let Some(a) = parse_bet_call_raise(p, ": calls ", Play::Call) {
        a
    }
    else if let Some(a) = parse_bet_call_raise(p, ": raises ", Play::Raise) {
        a
    }
    else if line.contains(": posts") 
    {
        let name_end = line.rfind(": posts ").unwrap();
        let blind_begin = name_end + ": posts ".len();
        let blind = parse_blind(&line[blind_begin..]).unwrap().0;
        let currency = parse_currency(&line[blind_begin + blind.len() + 1..]);
        let bt = if blind.ends_with('s') {
            BlindType::SmallAndBig
        } else {
            if blind.starts_with('s') { BlindType::Small } else { BlindType::Big }
        };
        Action::Blind(bt, p.span(&line[0..name_end]), currency)
    }
    else if line.contains(": shows") 
    {
        Action::ShowHand(p.span(line.prefix(':')), parse_cards(line.rbetween('[', ']')))
    }
    else if line.contains(" said, \"") 
    {
        Action::Say(p.span(line.rprefix_str(" said,")), p.span(line.rbetween('\"', '\"')))
    }
    else if line.contains(" collected ") 
    {
        let name_end = line.rfind(" collected ").unwrap();
        let currency_begin = name_end + " collected ".len();
        let currency_end = line.rfind(" from ").unwrap();
        let pot_t_begin = currency_end + " from ".len();
        let name = &line[0..name_end];
        let pot_t = match line[pot_t_begin..].chars().next().unwrap() {
            'p' | 'm' => Pot::Main,
            's' => Pot::Side,
            _ => unreachable!()
        };
        let currency = parse_currency(&line[currency_begin..currency_end]);
        Action::CollectedPot(pot_t, p.span(name), currency)
    }
    else if line.contains(" cashed out the hand for ")
    {
        let name_end = line.rfind(" cashed out the hand for ").unwrap();
        let currency_begin = name_end + " cashed out the hand for ".len();
        if let Some(e) = &line[currency_begin..].find(" |") {
            let currency_end = currency_begin + e;
            let fee_begin = line.rfind("Cash Out Fee ").unwrap() + "Cash Out Fee ".len();
            let amount = parse_currency(&line[currency_begin..currency_end]);
            let fee = parse_currency(&line[fee_begin..]);
            Action::CashOut(p.span(&line[..name_end]), amount, fee)
        } else {
            let amount = parse_currency(&line[currency_begin..]);
            Action::CashOut(p.span(&line[..name_end]), amount, Currency{symbol: amount.symbol, amount: 0.0})
        }
    }
    else if line.contains(" joins the table at seat ")
    {
        let name_end = line.rfind(" joins the table at seat ").unwrap();
        let seat_begin = name_end + " joins the table at seat ".len() + 1;
        let seat = str::parse::<u64>(&line[seat_begin..]).unwrap() as u8;
        Action::Join(seat, p.span(&line[0..name_end]))
    }
    else {
        return None;
    };
    Some(r)
}


fn parse_bet_call_raise<'a>(p: &mut Parser<'a>, part: &str, t: Play) -> Option<Action> {
    let line = p.line();
    if let Some(name_end) = line.find(part) {
        let bet_begin = name_end + part.len();
        let all_in = line.ends_with("and is all in") || line.ends_with("and is all-in");
        let bet_end = if all_in { line.len() - " and is all in".len() } else { line.len() };
        let (bet, bet_to) = match t {
            Play::Raise => {
                let bet_to_begin = line.rfind(" to ").unwrap();
                let bet = parse_currency(&line[bet_begin..bet_to_begin]);
                (bet, Some(parse_currency(&line[bet_to_begin + " to ".len()..bet_end])))
            }
            _ => {
                (parse_currency(&line[bet_begin..bet_end]), None)
            }
        };
        Some(Action::Play(t, p.span(&line[0..name_end]), Some(ActionData{bet, bet_to, all_in})))
    } else {
        None
    }
}

//endregion

//region Parse - General

fn extract_integer(input: &str) -> Option<usize> {
    let mut offset = 0;
    for v in input.chars() {
        if !v.is_numeric() { break; }
        offset += v.len_utf8();
    }
    if offset > 0 { Some(offset) } else { None }
}


fn parse_integer(input: &str) -> Option<(u64, usize)> {
    if let Some(i) = extract_integer(input) {
        if let Ok(v) = str::parse::<u64>(&input[..i]) {
            return Some((v, i))
        }
    }
    None
}


fn extract_float(input: &str) -> Option<usize> {
    if let Some(i) = extract_integer(input) {
        Some(if input[i..].starts_with('.') {
            extract_integer(&input[i+1..]).unwrap_or(i)
        }
        else {
            i
        })
    } else {
        None
    }
}


fn parse_float(input: &str) -> Option<(Float, usize)> {
    if let Some(i) = extract_float(input) {
        if let Ok(v) = str::parse::<Float>(&input[..i]) {
            return Some((v, i))
        }
    }
    None
}


fn parse_currency_char(input: &str) -> Option<(char, usize)> {
    //                                 euro      indian rupee
    for c in &['$', '£', '¥', '\u{20ac}', '\u{20b9}'] {
        if input.starts_with(*c) {
            return Some((*c, c.len_utf8()));
        }
    }
    None
}


fn parse_currency_dynamic(input: &str) -> Option<(Currency, usize)> {
    if let Some((c, i)) = parse_currency_char(input) {
        if let Some((v, end)) = parse_float(&input[i..]) {
            return Some((Currency{symbol: c, amount: v}, end));
        }
    }
    None
}


fn parse_currency<'a>(part: &'a str) -> Currency {
    let (symbol, num_start) = parse_currency_char(part).unwrap();
    if let Ok(v) = str::parse::<Float>(&part[num_start..]) {
        Currency { symbol: symbol, amount: v }
    } 
    else if let Ok(v) = str::parse::<u64>(&part[num_start..]) {
        Currency { symbol: symbol, amount: v as Float }
    } 
    else {
        panic!("Failed to parse float in string: '{}'", &part[1..]);
    }
}


fn parse_cards<'a>(part: &'a str) -> Cards {
    let mut cards = Cards::new();
    for p in part.split(' ') {
        let mut chars = p.chars();
        let (nc, sc) = (chars.next().unwrap(), chars.next().unwrap());
        let n = match nc {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            x => x as u8 - b'0'
        };
        let suit = match sc.to_lowercase().next().unwrap() {
            's' => Suit::Spade,
            'h' => Suit::Heart,
            'd' => Suit::Diamond,
            'c' => Suit::Club,
            s => unreachable!("Invalid suit: {} in '{}'", s, part),
        };
        cards.push(Card{n, suit});
    }
    cards
}


fn parse_blind(input: &str) -> Option<(&str, usize)> {
    for t in &["button", "small blind", "big blind", "button blind", "small & big blinds"] {
        if input.starts_with(t) {
            return Some((&input[0..t.len()], t.len()));
        }
    }
    None
}

//endregion

//region JSON

use std::fmt::Write;
type JsonResult = std::fmt::Result;

pub trait Json {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult;
}

pub fn to_json<T: Json>(v: &T, data: &str) -> Result<String, std::fmt::Error> {
    let mut f = JsonFormatter{out: String::with_capacity(100000), data};
    v.serialize(&mut f)?;
    Ok(f.out)
}

pub struct JsonFormatter<'a> {
    out: String,
    data: &'a str,
}

pub struct JsonArrayFormatter<'a, 'b> {
    f: &'b mut JsonFormatter<'a>,
    is_first: bool,
}

pub struct JsonObjectFormatter<'a, 'b> {
    f: &'b mut JsonFormatter<'a>,
    is_first: bool,
}

impl<'a> JsonFormatter<'a> {
    fn array<'b>(&'b mut self) -> JsonArrayFormatter<'a, 'b> {
        self.write_char('[').unwrap();
        JsonArrayFormatter { f: self, is_first: true }
    }

    fn object<'b>(&'b mut self) -> JsonObjectFormatter<'a, 'b> {
        self.write_char('{').unwrap();
        JsonObjectFormatter { f: self, is_first: true }
    }
}

impl<'a, 'b> JsonArrayFormatter<'a, 'b> {
    fn entry<T: Json>(&mut self, value: &T) -> &mut Self {
        if !self.is_first { write!(self.f, ",").unwrap(); }
        self.is_first = false;
        value.serialize(self.f).unwrap();
        self
    }

    
    fn entries<T: Json>(&mut self, values: &[T]) -> &mut Self {
        for v in values { self.entry(v); }
        self
    }

    fn finish(&mut self) -> JsonResult {
        write!(self.f, "]")
    }
}

impl<'a, 'b> JsonObjectFormatter<'a, 'b> {
    fn entry<T: Json>(&mut self, name: &str, value: &T) -> &mut Self {
        if !self.is_first { self.f.write_char(',').unwrap() ;}
        self.is_first = false;
        write!(self.f, "\"{}\"", name).unwrap();
        self.f.write_char(':').unwrap();
        value.serialize(self.f).unwrap();
        self
    }

    fn finish(&mut self) -> JsonResult {
        self.f.write_char('}')
    }
}

impl<'a> std::fmt::Write for JsonFormatter<'a> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.out.push(c);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.out.push_str(s);
        Ok(())
    }
}

impl Json for bool {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_str(if *self { "true" } else { "false" })
    }
}

impl Json for char {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_char('"')?;
        f.write_char(*self)?;
        f.write_char('"')
    }
}

impl Json for u8 {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        write!(f, "{}", self)
    }
}

impl Json for u64 {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        write!(f, "{}", self)
    }
}

impl Json for Float {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        write!(f, "{}", self)
    }
}

impl Json for str {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        write!(f, "\"{}\"", self)
    }
}

impl Json for &str {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        write!(f, "\"{}\"", self)
    }
}

impl Json for Span {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        let s = &f.data[*self];
        s.serialize(f)
    }
}

impl<T: Json> Json for Option<T> {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        match self {
            Some(v) => v.serialize(f),
            None => write!(f, "null"),
        }
    }
}

impl<T: Json> Json for [T] {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.array().entries(self).finish()
    }
}

impl<T: Json, const N: usize> Json for [T; N] {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        self.as_slice().serialize(f)
    }
}

impl<T: Json> Json for Vec<T> {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        self.as_slice().serialize(f)
    }
}

macro_rules! impl_stack_vec_serialize {
    () => {};
    ($n:literal $( $rest:literal) *) => {
        impl<T: Json + std::fmt::Debug + Clone> Json for StackVec<[T; $n]> {
            fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
                self.as_slice().serialize(f)
            }
        }
        impl_stack_vec_serialize!($($rest)*);
    };
}

impl_stack_vec_serialize!(5 8 20);



impl<T1: Json, T2: Json> Json for (T1, T2) {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_char('[')?;
        self.0.serialize(f)?;
        f.write_char(',')?;
        self.1.serialize(f)?;
        f.write_char(']')
    }
}

//endregion

//region JSON - Structs

const JSON_KEY_NAME   : &'static str = "name";
const JSON_KEY_TYPE   : &'static str = "type";
const JSON_KEY_AMOUNT : &'static str = "amount";
const JSON_KEY_ACTIONS: &'static str = "actions";


impl<'a> Json for Hand {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry("header", &self.header)
            .entry("preflop", &self.hole_cards)
            .entry("streets", &self.streets)
            .entry("summary", &self.summary)
            .finish()
    }
}

impl<'a> Json for Header {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry("info", &self.info)
            .entry("table", &self.table)
            .entry("players", &self.players)
            .entry(JSON_KEY_ACTIONS, &self.actions)
            .finish()
    }
}

impl<'a> Json for HandInfo {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry("site", &self.site)
            .entry("hand", &self.hand)
            .entry("id", &self.id)
            .entry("game_type", &self.game_type)
            .entry("buy_in_min", &self.buy_in_min)
            .entry("buy_in_max", &self.buy_in_max)
            .finish()
    }
}

impl<'a> Json for Table {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry(JSON_KEY_NAME, &self.name)
            .entry("max_seats", &self.max_players)
            .entry("button", &self.button)
            .finish()
    }
}

impl<'a> Json for Player {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry(JSON_KEY_NAME, &self.name)
            .entry("seat", &self.seat)
            .entry("chips", &self.chips)
            .entry("bounty", &self.bounty)
            .finish()
    }
}

impl<'a> Json for HoleCards {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry("dealt_to", &self.dealt_to)
            .entry(JSON_KEY_ACTIONS, &self.actions)
            .finish()
    }
}

impl<'a> Json for Street {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry(JSON_KEY_TYPE, &self.t)
            .entry("index", &self.index)
            .entry("cards", &self.cards)
            .entry("new_card", &self.new_card)
            .entry(JSON_KEY_ACTIONS, &self.actions)
            .finish()
    }
}

impl Json for Summary {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.object()
            .entry("pot", &self.pot)
            .entry("main_pot", &self.main_pot)
            .entry("side_pot", &self.side_pot)
            .entry("rake", &self.rake)
            .entry("boards", &self.boards)
            .finish()
    }
}

impl Json for StreetType {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_str(match self {
            StreetType::Flop => "\"flop\"",
            StreetType::River => "\"river\"",
            StreetType::Turn => "\"turn\"",
            StreetType::Showdown => "\"showdown\"",
        })
    }
}

impl Json for BlindType {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_str(match self {
            BlindType::Small => "\"small\"",
            BlindType::Big => "\"big\"",
            BlindType::SmallAndBig => "\"both\"",
        })
    }
}

impl Json for Suit {
    #[inline] fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_char('"')?;
        f.write_char(match self {
            Suit::Club => 'c',
            Suit::Diamond => 'd',
            Suit::Spade => 's',
            Suit::Heart => 'h',
        })?;
        f.write_char('"')
    }
}

impl Json for Pot {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        f.write_str(match self {
            Pot::Main => "\"main\"",
            Pot::Side => "\"side\""
        })
    }
}

impl Json for Action {
    fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        let mut o = f.object();
        match self {
            Action::Event(event, name) => {
                o.entry(JSON_KEY_TYPE, &match event {
                    Event::Connect => "connect",
                    Event::Disconnect => "disconnect",
                    Event::Timeout => "timeout",
                    Event::Leave => "leave",
                    Event::Sitout => "sitout",
                    Event::NotShow => "not_show",
                });
                o.entry(JSON_KEY_NAME, name);
            }
            Action::Join(seat, name) => {
                o.entry(JSON_KEY_TYPE, &"join");
                o.entry(JSON_KEY_NAME, name);
                o.entry("seat", seat);
            }
            Action::Play(play, name, data) => {
                o.entry(JSON_KEY_TYPE, &match play {
                    Play::Bet => "bet",
                    Play::Call => "call",
                    Play::Check => "check",
                    Play::Raise => "raise",
                    Play::Stand => "stand",
                    Play::Discard => "discard",
                });
                o.entry(JSON_KEY_NAME, name);
                if let Some(v) = data {
                    o.entry("bet", &v.bet);
                    o.entry("bet_to", &v.bet_to);
                    o.entry("all_in", &v.all_in);
                }
            }
            Action::Blind(blind, name, amount) => {
                o.entry(JSON_KEY_TYPE, &"blind");
                o.entry("blind", blind);
                o.entry(JSON_KEY_NAME, name);
                o.entry(JSON_KEY_AMOUNT, amount);
            }
            Action::ShowHand(name, cards) => {
                o.entry(JSON_KEY_TYPE, &"show");
                o.entry(JSON_KEY_NAME, name);
                o.entry("cards", cards);

            }
            Action::Fold(name, cards) => {
                o.entry(JSON_KEY_TYPE, &"fold");
                o.entry(JSON_KEY_NAME, name);
                o.entry("cards", cards);
            }
            Action::Say(name, _msg) => {
                o.entry(JSON_KEY_TYPE, &"say");
                o.entry(JSON_KEY_NAME, name);
                // let escaped = msg.escape_default().to_string();
                // o.entry("msg", &escaped.as_ref());
            }
            Action::CashOut(name, amount, fee) => {
                o.entry(JSON_KEY_TYPE, &"cash_out");
                o.entry(JSON_KEY_NAME, name);
                o.entry(JSON_KEY_AMOUNT, amount);
                o.entry("fee", fee);

            }
            Action::UncalledBetReturned(name, amount) => {
                o.entry(JSON_KEY_TYPE, &"return_bet");
                o.entry(JSON_KEY_NAME, name);
                o.entry(JSON_KEY_AMOUNT, amount);
            }
            Action::CollectedPot(pot, name, amount) => {
                o.entry(JSON_KEY_TYPE, &"collect_pot");
                o.entry(JSON_KEY_NAME, name);
                o.entry(JSON_KEY_AMOUNT, amount);
                o.entry("pot", pot);
            }
        }
        o.finish()
    }
}

impl Json for Currency {
    #[inline] fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        (self.symbol, self.amount).serialize(f)
    }
}

impl Json for Card {
    #[inline] fn serialize(&self, f: &mut JsonFormatter) -> JsonResult {
        (self.n, self.suit).serialize(f)
    }
}

//endregion


#[cfg(test)]
mod tests {
    use super::*;

}