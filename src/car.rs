use std::{fmt::{Display,Debug,Formatter,Result as FmtResult}, ops::Add, borrow::{Borrow, BorrowMut}};
use serde::{Deserialize, Serialize};
use serde_json::{Result as SerdeResult};
use std::cell::RefCell;

const UNDEFINED: &'static str = "undefined";

#[derive(Debug,Serialize, Deserialize)]
pub enum CarTransmission {
    Manual,
    Automatic,
    SemiAutomatic
}

impl Display for CarTransmission {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Clone for CarTransmission {
    fn clone(&self) -> CarTransmission {
        match self {
            CarTransmission::Automatic => CarTransmission::Automatic,
            CarTransmission::Manual => CarTransmission::Manual,
            CarTransmission::SemiAutomatic => CarTransmission::SemiAutomatic,
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub enum CarLine {
    Mid,
    Lux
}

impl Display for CarLine {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Clone for CarLine {
    fn clone(&self) -> CarLine {
        match self {
            CarLine::Lux => CarLine::Lux,
            CarLine::Mid => CarLine::Mid,
        }
    }
}

#[derive(Debug,Clone,Serialize, Deserialize,)]
pub struct Car {
    make: String, 
    model: String,
    line: CarLine,
    transmission: CarTransmission
}


// Pre-req for Error trait
impl Display for Car {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "make:{}\nmodel:{}\ntransmission: {}", self.make, self.model, self.transmission)
    }
}

impl Car {
    pub fn new() -> Car {
        Car{
            make: UNDEFINED.to_string(),
            model: UNDEFINED.to_string(),
            line: CarLine::Mid,
            transmission: CarTransmission::Automatic
        }        
    }

    pub fn make<'a>(&'a mut self, m: &str) -> &'a mut Car {
        self.make = String::from(m);
        self
    }

    pub fn model<'a>(&'a mut self, m: &str) -> &'a mut Car {
        self.model = String::from(m);
        self
    }

    pub fn transmission<'a>(&'a mut self, t: CarTransmission) -> &'a mut Car {
        self.transmission = t;
        self
    }
    
    pub fn line<'a>(&'a mut self, t: CarLine) -> &'a mut Car {
        self.line = t;
        self
    }

    pub fn build(&self) -> Car{
        self.clone()
    }

    pub fn print(&self){
        println!("{}", self);
    }

    pub fn printj(&self) ->  SerdeResult<()>{
       let j =  serde_json::to_string(self)?;
       println!("{}", j);
       Ok(())
    }
}


// See: https://doc.rust-lang.org/std/cell/index.html
#[derive(Debug,Serialize, Deserialize)]
pub struct CarLot {
    location: String,
    cars: RefCell<Vec<Car>>,
}

impl CarLot{
    pub fn new(location: &str) -> CarLot {
        CarLot{
            location: String::from(location),
            cars: RefCell::new(Vec::new()) 
        }
    }

    pub fn add(&self, c: Car) -> &Self {
        self.cars.borrow_mut().push(c);
        self
    }

    pub fn print(&self) {
        self.printj().unwrap();
    }

    pub fn printj(&self) ->  SerdeResult<()>{
        let j =  serde_json::to_string(self)?;
        println!("{}", j);
        Ok(())
     }

    pub fn to_string(&self) -> String {
        let j =  serde_json::to_string(self).unwrap();
        j
    }
}
