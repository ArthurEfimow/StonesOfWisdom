use serde::ser::{Serialize, Serializer,SerializeSeq, SerializeMap, SerializeStruct};


#[derive(Serialize,Deserialize, Debug)]
pub struct Background {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Serialize,Deserialize,Debug,Copy, Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub s: f32,
    pub t: f32
}

impl Point {
    pub fn createPoint(x: f32,y:f32,r: f32,g:f32,b:f32,a:f32,s:f32,t:f32 ) -> Point {
	return Point {x,y,r,g,b,a,s,t};
    }
    pub fn get_x(&self) -> f32 {return self.x;}
    pub fn get_y(&self) -> f32 {return self.y;}
    pub fn move_x (&mut self,length : f32) { self.x += length;}
    pub fn move_y (&mut self,length : f32) { self.y += length;}
    pub fn move_xy (&mut self,length_x : f32,length_y : f32) {self.x += length_x;self.y += length_y;}

    pub fn getInfo (&self) -> Vec<f32> {return vec![self.x,self.y,self.r,self.g,self.b,self.a,self.s,self.t];}
    pub fn getInfoAdd (&self,add:Point) -> Vec<f32> {return vec![self.x+add.x,self.y+add.y,self.r+add.r,self.g+add.g,self.b+add.b,self.a+add.a,self.s+add.s,self.t+add.t];}

    pub fn getCopy (&self) -> &Point {return self;}  

    pub fn get_degree_to_Point (&self,x: f32,y :f32) -> f32 {
	let mx = self.x;
	let my = self.y;

	let distance = ((x - mx) * (x - mx) + (y - my) * (y - my)).sqrt();
	let alpha_sin = ((y - my)/distance).asin().to_degrees();
        let alpha_cos = ((x - mx)/distance).acos().to_degrees();
	if alpha_sin as i32 == alpha_cos as i32 {return 90.0 - alpha_sin;}
	else if -1*alpha_sin as i32 == alpha_cos as i32 {return 90.0 - alpha_sin;}
	else if alpha_sin <= 0.0 {return 90.0 + alpha_cos;}
	else {return 270.0 + alpha_sin;}
    }

    pub fn move_degree_around_Point (&mut self,deg: f32,x: f32,y :f32) {
	let mx = self.x;
	let my = self.y;

	let distance = ((mx - x) * (mx - x) + (my - y) * (my - y)).sqrt();
	let alpha_sin = ((my - y)/distance).asin().to_degrees();
        let alpha_cos = ((mx - x)/distance).acos().to_degrees();
	let mut org_deg = 0.0;

	if alpha_sin as i32 == alpha_cos as i32 {org_deg = 90.0 - alpha_sin;}
	else if -1*alpha_sin as i32 == alpha_cos as i32 {org_deg = 90.0 - alpha_sin;}
	else if alpha_sin <= 0.0 {org_deg = 90.0 + alpha_cos;}
	else {org_deg = 270.0 + alpha_sin;}

	org_deg += deg;
	loop {if org_deg < 360.0 {break;};org_deg -= 360.0;}
        loop {if org_deg >= 0.0 {break;};org_deg += 360.0;}

	self.x = x + org_deg.to_radians().sin() * distance;
	self.y = y + org_deg.to_radians().cos() * distance;
	
    }

}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
	return Point{x: self.x+_rhs.x,y: self.y+_rhs.y,r: self.r+_rhs.r,g:self.g+_rhs.g,b:self.b+_rhs.b,a:self.a+_rhs.a,s:self.s+_rhs.s,t:self.t+_rhs.t}
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Triangle {
    A: Point,
    B: Point,
    C: Point
}

impl Triangle {
    pub fn create(A : Point, B : Point, C : Point) -> Triangle {
	return Triangle {A,B,C};
    }
    pub fn move_x (&mut self,length : f32) { self.A.move_x(length);self.B.move_x(length);self.C.move_x(length);}
    pub fn move_y (&mut self,length : f32) { self.A.move_y(length);self.B.move_y(length);self.C.move_y(length);}

    pub fn getInfo (&self, Ret : &mut Vec<f32>) { 
	Ret.extend(self.A.getInfo().iter());
	Ret.extend(self.B.getInfo().iter());
	Ret.extend(self.C.getInfo().iter());
    }

    pub fn getInfoAdd (&self, Ret : &mut Vec<f32>, add: Point) { 
	Ret.extend(self.A.getInfoAdd(add).iter());
	Ret.extend(self.B.getInfoAdd(add).iter());
	Ret.extend(self.C.getInfoAdd(add).iter());
    }

    pub fn hit (&self,x: f32,y :f32) -> bool {
	let alpha = self.A.get_degree_to_Point(x,y);
	let beta = self.A.get_degree_to_Point(self.B.get_x(),self.B.get_y());
	let gamma = self.A.get_degree_to_Point(self.C.get_x(),self.C.get_y());
	if alpha as i32 == beta as i32 {return self.hit2(x,y);}
	if alpha as i32 == gamma as i32 {return self.hit2(x,y);}
	if beta > gamma && beta - gamma > 180.0 {return (alpha > beta || alpha < gamma) && self.hit2(x,y);}
	if beta > gamma {return alpha > gamma && beta > alpha && self.hit2(x,y);}
	if gamma > beta && gamma - beta > 180.0 {return (alpha > gamma || alpha < beta) && self.hit2(x,y);}
	return alpha > beta && gamma > alpha && self.hit2(x,y);
    } 

    pub fn hit2 (&self,x: f32,y :f32) -> bool {
	let alpha = self.B.get_degree_to_Point(x,y);
	let beta = self.B.get_degree_to_Point(self.A.get_x(),self.A.get_y());
	let gamma = self.B.get_degree_to_Point(self.C.get_x(), self.C.get_y());
	if alpha as i32 == beta as i32 {return true;}
	if alpha as i32 == gamma as i32 {return true;}
	if beta > gamma && beta - gamma > 180.0  {return alpha > beta || alpha < gamma;}
	if beta > gamma {return alpha > gamma && beta > alpha;}
	if gamma > beta && gamma - beta > 180.0 {return alpha > gamma || alpha < beta;}
	return alpha > beta && gamma > alpha;
    }

    pub fn move_degree_around_Point (&mut self,deg: f32,x: f32,y :f32) {
	self.A.move_degree_around_Point(deg,x,y);
	self.B.move_degree_around_Point(deg,x,y);
	self.C.move_degree_around_Point(deg,x,y);
    } 

}

impl Drop for Triangle {
    fn drop(&mut self) {}
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Form {
    pub triangles_: Vec<Triangle>,
    pub x_fix : f32,
    pub y_fix : f32,
    pub radius : f32
}


impl Form {
    pub fn create(triangles_: Vec<Triangle>,x_fix: f32,y_fix:f32,radius:f32) -> Form {
	return Form {triangles_,x_fix,y_fix,radius};
    }


    pub fn createHex(x : f32, y:f32, l:f32, h:f32,radius:f32) -> Form {
	let triangles_ = vec![Triangle::create(Point::createPoint((x-l/2.0)+l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.27,0.03), // A
					       Point::createPoint((x+l/2.0)-l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.72,0.03), // B
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x+l/2.0)-l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.72,0.03), // B
					       Point::createPoint(x+l/2.0,y,0.0,0.0,0.0,1.0,0.97,0.5), // C
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint(x+l/2.0,y,0.0,0.0,0.0,1.0,0.97,0.5), // C
					       Point::createPoint((x+l/2.0)-l/4.0,y-h/2.0,0.0,0.0,0.0,1.0,0.72,0.97), // D
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x+l/2.0)-l/4.0,y-h/2.0,0.0,0.0,0.0,0.97,0.72,0.97), // D
					       Point::createPoint((x-l/2.0)+l/4.0,y-h/2.0,0.0,0.0,0.0,0.97,0.27,0.97), // E
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x-l/2.0)+l/4.0,y-h/2.0,0.0,0.0,0.0,1.0,0.27,0.97), // E
					       Point::createPoint(x-l/2.0,y,0.0,0.0,0.0,1.0,0.03,0.5), // F
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint(x-l/2.0,y,0.0,0.0,0.0,1.0,0.03,0.5), // F
					       Point::createPoint((x-l/2.0)+l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.27,0.03), // A
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
			];
	return Form {triangles_,x_fix:x,y_fix:y,radius};
    }

    pub fn recreateHex(&mut self,l:f32, h:f32,radius:f32) {
	let x = self.x_fix;
        let y = self.y_fix;
        self.radius = radius;
	self.triangles_ = vec![Triangle::create(Point::createPoint((x-l/2.0)+l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.27,0.03), // A
					       Point::createPoint((x+l/2.0)-l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.72,0.03), // B
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x+l/2.0)-l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.72,0.03), // B
					       Point::createPoint(x+l/2.0,y,0.0,0.0,0.0,1.0,0.97,0.5), // C
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint(x+l/2.0,y,0.0,0.0,0.0,1.0,0.97,0.5), // C
					       Point::createPoint((x+l/2.0)-l/4.0,y-h/2.0,0.0,0.0,0.0,1.0,0.72,0.97), // D
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x+l/2.0)-l/4.0,y-h/2.0,0.0,0.0,0.0,0.97,0.72,0.97), // D
					       Point::createPoint((x-l/2.0)+l/4.0,y-h/2.0,0.0,0.0,0.0,0.97,0.27,0.97), // E
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint((x-l/2.0)+l/4.0,y-h/2.0,0.0,0.0,0.0,1.0,0.27,0.97), // E
					       Point::createPoint(x-l/2.0,y,0.0,0.0,0.0,1.0,0.03,0.5), // F
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
		  	      Triangle::create(Point::createPoint(x-l/2.0,y,0.0,0.0,0.0,1.0,0.03,0.5), // F
					       Point::createPoint((x-l/2.0)+l/4.0,y+h/2.0,0.0,0.0,0.0,1.0,0.27,0.03), // A
					       Point::createPoint(x,y,0.0,0.0,0.0,1.0,0.5,0.5)), // MID
			];
    }

    pub fn createQua(x : f32, y:f32, l:f32, h:f32,radius:f32) -> Form {
	let triangles_ = vec![Triangle::create(Point::createPoint(x-l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,0.0,0.0),
					       Point::createPoint(x+l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x-l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,0.0,1.0)),
		  	      Triangle::create(Point::createPoint(x+l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x-l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,0.0,1.0),
					       Point::createPoint(x+l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,1.0,1.0))];
	return Form {triangles_,x_fix:x,y_fix:y,radius};
    }

    pub fn recreateQua(&mut self,l:f32, h:f32,radius:f32){
       let x = self.x_fix;
       let y = self.y_fix;
       self.radius = radius;
       self.triangles_ = vec![Triangle::create(Point::createPoint(x-l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,0.0,0.0),
					       Point::createPoint(x+l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x-l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,0.0,1.0)),
		  	      Triangle::create(Point::createPoint(x+l/2.0,y+h/2.0,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x-l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,0.0,1.0),
					       Point::createPoint(x+l/2.0,y-h/2.0,0.0,0.0,0.0,1.0,1.0,1.0))];
    }

    pub fn recreateQuaPos(&mut self,x1 : f32, y1:f32, x2:f32, y2:f32){
       self.triangles_ = vec![Triangle::create(Point::createPoint(x1,y1+0.02,0.0,0.0,0.0,1.0,0.0,0.0),
					       Point::createPoint(x2,y2+0.02,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x1,y1-0.02,0.0,0.0,0.0,1.0,0.0,1.0)),
		  	      Triangle::create(Point::createPoint(x2,y2+0.02,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::createPoint(x1,y1-0.02,0.0,0.0,0.0,1.0,0.0,1.0),
					       Point::createPoint(x2,y2-0.02,0.0,0.0,0.0,1.0,1.0,1.0))];
    }

    pub fn get_info (& self) -> Vec<f32> {
	let mut vertices : Vec<f32> = vec![];
	for trian in self.triangles_.iter() {trian.getInfo(&mut vertices);}
	return vertices;
    }
    pub fn hit (&mut self,x: f32,y :f32) -> bool {
	if self.radius < distance(self.x_fix,self.y_fix,x,y) {return false;} 
	for trian in self.triangles_.iter() {if trian.hit(x,y) {return true;}}
	return false;
    }

    pub fn move_x (&mut self,length : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_x(length);
            index += 1;
        }
	self.x_fix += length;
    }

    pub fn move_y (&mut self,length : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_y(length);
            index += 1;
        }
	self.y_fix += length;
    }

    pub fn move_xy (&mut self,length_x : f32,length_y : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_x(length_x);
	    self.triangles_[index].move_y(length_y);
            index += 1;
        }
	self.x_fix += length_x;
	self.y_fix += length_y;
    }

    pub fn move_degree (&mut self,deg: f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_degree_around_Point(deg,self.x_fix,self.y_fix);
            index += 1;
        }
    }


    pub fn move_degree_around_Point (&mut self,deg: f32,x: f32,y :f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_degree_around_Point(deg,x,y);
            index += 1;
        }
    }

}

impl Drop for Form {
    fn drop(&mut self) {}
}

pub fn distance (x: f32,y :f32,mx: f32,my :f32) -> f32 {
	return ((mx - x) * (mx - x) + (my - y) * (my - y)).sqrt();
}

pub fn direction_s (x1: f32,y1 :f32,x2: f32,y2 :f32) -> i32 {
	let y = y2 - y1;
	let x = x2 - x1;

	if x < 0.0 && y < 0.0   {return 9;}
	if x == 0.0 && y < 0.0  {return 8;}
        if x > 0.0 && y < 0.0   {return 7;}
	if x < 0.0 && y == 0.0  {return 6;}
	if x == 0.0 && y == 0.0 {return 5;}
	if x > 0.0 && y == 0.0  {return 4;}	
	if x == 0.0 && y > 0.0  {return 3;}
	if x < 0.0 && y > 0.0   {return 2;}
	if x > 0.0 && y > 0.0   {return 1;}	
	return 0;

}
