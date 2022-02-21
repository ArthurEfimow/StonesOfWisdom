#![allow(non_snake_case)]
#[macro_use]

pub mod gamefield;

fn main() {
    let mut index = 0;
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    while index < 10000 {
    	let mut GF : gamefield::GameField = gamefield::GameField::create(1,4);
    	while !GF.game_is_finished(){
	    GF.ai_action();
    	}
    	let output = GF.get_tokens();
	println!("Game:{}",index+1);
        if output[1] >= output[2] {p1_wins+=1;GF.punish_ai()} else {p2_wins+=1;GF.reward_ai()}
    	index +=1;
	
    }
    println!("P1:{},P2:{}",p1_wins,p2_wins);	
}

