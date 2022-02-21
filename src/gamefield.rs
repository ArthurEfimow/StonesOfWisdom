#![allow(non_snake_case)]


use rand::thread_rng;
use rand::seq::SliceRandom;
use fp_rust::fp;

use bigint::uint::U256;

pub mod real_ai;

pub struct Field {
    pub id : usize,
    pub neighbors : Vec<usize>,
    pub content: i32,
    pub status : i32
}

impl Clone for Field {
    fn clone(&self) -> Field {
        Field{id:self.id.clone(),neighbors:self.neighbors.clone(),content:self.content.clone(),status:0}
    }
}

pub struct GameField {
    field : Vec<Field>,
    active_player : Vec<i32>,
    player_ai : Vec<i32>,
    game_end: bool,
    tokens: Vec<i32>,
    log: Vec<usize>,
    real : real_ai::RealAI,
}

impl GameField {
    pub fn create(player1:i32,player2:i32) -> GameField {
	let mut field = vec![];
	let mut j = 0;
	while j < 8 {
	    let mut i = 0;
	    while i < 8 {
		let index = j * 8 + i;
		let mut neighbors : Vec<usize> = vec![];
		if j != 0 && i != 0 {neighbors.push(index-(8+1));} else {neighbors.push(64);}
		if j != 0 	    {neighbors.push(index -8);} else {neighbors.push(64);}
		if j != 0 && i != 7 {neighbors.push(index-(8-1));} else {neighbors.push(64);}
		if i != 0 	    {neighbors.push(index-1);} else {neighbors.push(64);}
		if i != 7	    {neighbors.push(index+1);} else {neighbors.push(64);}
		if j != 7 && i != 0 {neighbors.push(index +8-1);} else {neighbors.push(64);}
		if j != 7 	    {neighbors.push(index+8);} else {neighbors.push(64);}
		if j != 7 && i != 7 {neighbors.push(index+8+1);} else {neighbors.push(64);}
		let mut content = -1;
		if vec![27,36].contains(&index) {content = 1;}
		if vec![28,35].contains(&index) {content = 2;}
		if vec![18,19,20,21,29,37,45,44,43,42,34,26].contains(&index) {content = 0;}
		field.push(Field{id:index,neighbors,content,status:0});
		i+=1;		
	   }
	   j+=1;
        }
	return GameField {player_ai:vec![0,player1,player2],field,active_player:vec![1,2],game_end:false, tokens:vec![60,2,2],log:vec![],real : real_ai::RealAI::create()}
    }

    pub fn copy(&mut self) -> GameField {
	return GameField {player_ai:self.player_ai.clone(),field:self.field.clone(),active_player:self.active_player.clone(),game_end:self.game_end.clone(), tokens:self.tokens.clone(),log:vec![],real : real_ai::RealAI::create()}
    }

    pub fn set(&mut self,index : usize) -> bool{
	if index >= self.field.len() {return false;}
	if self.field[index].content != 0 {return false;}
	self.field[index].content = self.active_player[0];
	self.tokens[0] -= 1;
	self.tokens[self.active_player[0]as usize] +=1;
	self.make_setable(index);
	if self.active_player[0] >= 1 {self.log.push(index);}
	self.active_player.swap(0,1);	
	true
    }

    pub fn print_log(&mut self) {
	print!("Log:");
	for l in &self.log {print!(",{}",(*l/8)*10+*l%8);}
	println!("");
    }

    pub fn make_setable(&mut self,index : usize) {
	let mut direction : usize = 0;
	for i in self.field[index].neighbors.clone() {
 	    if i >= 64 {direction+=1;continue;}
	    let c = self.field[i].content.clone();
 	    if c < 0 {self.field[i].content = 0;}
	    self.change_token(i,direction);
	    direction+=1;
	}
    }

    pub fn change_token(&mut self,index :usize, direction : usize) -> bool{
	if index >= 64 {return false;}
	let c =  self.field[index].content;
	if c <= 0 {return false;}
	if c ==  self.active_player[0] {return true;}
	if self.change_token(self.field[index].neighbors[direction],direction) {
	    self.field[index].content = self.active_player[0];
	    self.tokens[self.active_player[0]as usize] +=1;
	    self.tokens[self.active_player[1]as usize] -=1;
	    return true;
	}
	false
    }

    pub fn get_content(&self,index:usize) -> i32{
	self.field[index].content
    }

    pub fn get_tokens(&self) -> Vec<i32>{
	self.tokens.clone()
    }

    pub fn get_weight_for_player(&self,player:i32) -> i32{
	let mut weight = 0;
	for f in  &self.field {
	    if f.content == player {
		if vec![0,7,56,63].contains(&f.id) {weight += 4;}
		else if vec![1,2,3,4,5,6,8,15,16,23,24,31,32,39,40,47,48,57,58,59,60,61,62].contains(&f.id) {weight += 2;}
		else {weight += 1;}
	    }
	}
	weight
    }

    pub fn get_choice(&self) -> Vec<usize>{
	self.field.clone().into_iter().filter(|f| f.content == 0).map(|f| f.id).collect()
    }

    pub fn game_is_finished(&self) -> bool{
	self.game_end.clone()
    }

    pub fn punish_ai(&mut self){
	self.real.reward(-1)
    }

    pub fn reward_ai(&mut self){
	self.real.reward(1)
    }

    pub fn ai_action(&mut self) -> bool {
	if self.game_end {return false;}
	let ai_level = self.player_ai[self.active_player[0]as usize];
	if ai_level <= 0 {return false;}
	let mut choice = self.get_choice();
	if choice.len() == 0 {self.game_end = true;return false;}
	if ai_level == 2 { self.greedy_ai(&mut choice,false);}
	if ai_level == 3 { self.minmax_ai(&mut choice,false);}
	if ai_level == 4 { self.greedy_ai(&mut choice,true);}
	if ai_level == 5 { self.minmax_ai(&mut choice,true);}
	if ai_level == 6 { self.real_ai(&mut choice);}
	choice.shuffle(&mut thread_rng());
	//if ai_level == 6 { let meta = self.transform_field_into_string(); self.real.set(meta + &self.active_player[0].to_string()+&choice[0].to_string());}
	if ai_level > 1 { let meta = self.transform_field_into_u256(); self.real.set(meta,(self.active_player[0]*100) as u16 +choice[0] as u16);}
	self.set(choice[0]);
	true
    }

    pub fn greedy_ai(&mut self, choice : &mut Vec<usize>,weight : bool){
	let mut tokens_after_set = vec![];
	let mut max_tokens = 0;
	for c in choice.clone() {
	    let mut n_field = self.copy();
	    n_field.set(c);
	    let token_count = if weight {n_field.get_weight_for_player(self.active_player[0])} else {n_field.get_tokens()[self.active_player[0]as usize]};
	    if token_count > max_tokens {max_tokens = token_count;}
	    tokens_after_set.push(token_count);
	}
	*choice = (0..choice.len()).filter(|i| tokens_after_set[*i] == max_tokens).map(|i| choice[i]).collect();
    }

    pub fn minmax_ai(&mut self, choice : &mut Vec<usize>,weight : bool){
	let mut tokens_after_set = vec![];
	let mut min_tokens = 200;
	for c in choice.clone() {
	    let mut n_field = self.copy();
	    n_field.set(c);
	    let mut max_tokens = 0;
	    for i in n_field.get_choice() {
		let mut n2_field = n_field.copy();
		n2_field.set(i);
		let token_count = if weight {n2_field.get_weight_for_player(self.active_player[1])} else {n2_field.get_tokens()[self.active_player[1]as usize]};
		if token_count > max_tokens {max_tokens = token_count;}
		if max_tokens > min_tokens {break;}
	    }
	    if max_tokens < min_tokens {min_tokens = max_tokens;}
	    tokens_after_set.push(max_tokens);
	}
	*choice = (0..choice.len()).filter(|i| tokens_after_set[*i] <= min_tokens).map(|i| choice[i]).collect();
    }

    pub fn real_ai(&mut self, choice : &mut Vec<usize>){
	let mut values = vec![];
	let code = self.transform_field_into_string() + &self.active_player[0].to_string();
	let mut max_value = 0;
	for c in choice.clone() {
	    let c_value = self.real.get_value(self.transform_field_into_u256(),(self.active_player[0]*100)as u16 +c as u16);
	    if c_value > max_value {max_value = c_value;}
	    values.push(c_value);
	}
	*choice = (0..choice.len()).filter(|i| values[*i] == max_value).map(|i| choice[i]).collect();
    }

    fn transform_field_into_string(&mut self) -> String{
	self.field.iter().fold("".to_string(), |mut sum, val| {sum.push_str(&val.content.to_string());sum})
	
    }

    fn transform_field_into_integer(&mut self) -> u64{
	//let output : u64 = fp_rust::fp::foldl(|sum, val| {val.content as u64 + sum*10},0,self.field);
	0
    }

    fn transform_field_into_u256(&self) -> U256{
	U256::from_dec_str(&self.field.iter().fold("".to_string(), |mut sum, val| {sum.push_str(&(val.content+1).to_string());sum})).unwrap()
    }

}
