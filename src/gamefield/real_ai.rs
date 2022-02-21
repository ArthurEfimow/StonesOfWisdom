use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use bigint::uint::U256;

pub struct RealAI {
    moves_made : Vec<(U256,u16)>,
    conn : Connection,
}

impl RealAI {
    pub fn create() -> RealAI {

    	let conn = Connection::open("ai.db").unwrap();

    	conn.execute(
            "create table if not exists ai_value_table2 (turn  integer NOT NULL,gamefield integer NOT NULL,score integer,PRIMARY KEY (turn,gamefield));",
        NO_PARAMS,
    	);

    	conn.execute(
            "ALTER TABLE ai_value_table2 ORDER BY turn ASC;",
        NO_PARAMS,
    	);
	return RealAI {moves_made : vec![],conn}
 
    }

    pub fn get_value(& self,field:U256,turn : u16) -> i32 {

    	let mut stmt = self.conn.prepare(
            &format!("SELECT score from ai_value_table2 where turn = {turn} and gamefield = {field} LIMIT 1;",turn=&turn,field = &field)
    	).unwrap();
	let mut output = 0;
    	stmt.query_map(NO_PARAMS, |row| {Ok(output = row.get(0).unwrap_or(0))});
	output

    }

    pub fn set(&mut self,field:U256,turn : u16) {
	self.moves_made.push((field,turn))
    }

    pub fn reward(&mut self,reward:i32) {
	
	for (f,t) in &self.moves_made{
	    let c = self.conn.execute(
            	&format!("update ai_value_table2 set score = score + {} where turn = {} and gamefield = {};",reward,&t,&f),
            	NO_PARAMS,
    	    );
	    match c {
		Ok(0) => {println!("new");self.conn.execute(&format!("INSERT INTO ai_value_table2 (turn,gamefield,score) values ({},{},0)",&t,&f),NO_PARAMS)}
		_ => Ok(0),
	    };
	    println!("{}",&f);
	    println!("{}",&c.unwrap());
	}
    }
}
