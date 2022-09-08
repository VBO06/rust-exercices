use std::collections::HashMap;
use thiserror::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Record {
    id: i32,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records {
    inner : HashMap<i32, Record>,
}

impl Records {
    fn new(&mut self) -> Self {
        Self {
            inner : HashMap::new(),
        }        
    }
   
    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }
   
    fn into_vec(&mut self) -> Vec<Record> {
        let records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
        records.sorty_by_keys(|rec| rec.id);
        records
    }
   
    fn next_id(&self) -> i64 {
        let ids: Vec<_> = &self.inner.keys().collet();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }
}

#[derive(Error)]
enum ParseError {
    #[error("invalid id")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String)
}


fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(",").collect();
    // Check id
    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::InvalidId),
    };
    let name = match fields.get(1).map(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };
    let email = match fields.get(2).map(|email| email.to_string()).filter(|email| email != "");
    
        Ok(Record {id, name, email})
}


fn parse_records(buffer: String, verbose: bool) -> Records {
    let mut records = Records::new();
    for (num, record) in buffer.split("\n").enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(record) => records.add(record),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {},{} ",
                            num+1,
                            e,
                            record
                        );
                    }
                }
            }
        }
    }
}
   


fn load_records(file_name: PathBuf, verbose: bool) -> Result<Records> {
    let mut file = File::open(file_name);
    let mut buffer = String::new();
    file_name.read_to_string(&mut buffer)?;
    Ok(parse_records(buffer, verbose));
}

struct Ticket {
    holder: String,
    price: i64,
}

impl Tickets {
    fn new(&self) -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    fn add(&self, ticket: Ticket) {
        self.inner.insert(ticket);
    }
   
    fn get_all() -> Vec<Ticket> {
        return self.inner.values();
    }
}

struct Tickets {
    inner: HashMap<i32, Ticket>,
}

enum Ticket {
    BackStage(i32, String),
    Vip(i32, String),
    Standard(i32),
}


struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

struct Student {
    name: String,
    locker: Option<i32>,
}


enum Position {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Active,
    Terminated
}

struct Employee {
    position: Position,
    status: Status,
}

struct Item {
    name: String,
    number: i32,
}

fn is_in_stock(item: &Item) -> Result<(), String> {
    match item.number {
        0 => Err("out of stock".to_owned()),
        _ => Ok(()),
    }

}

fn try_access(employee: &Employee) -> Result<(), String> {
    
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }
    
    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Managers => Ok(()),
        _ => return Err("invalid position".to_owned())
    }

}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_access(employee)?;
    println!("access ok");
    Ok(())
}



fn main() {
   
    let tickets = Tickets::new();
    tickets.add(Ticket::BackStage(50.0, "backstage".to_owned()));
    tickets.add(Ticket::Vip(40.0, "vip".to_owned()));
    tickets.add(Ticket::Standard(10.0, "standard".to_owned()));
   
    for ticket in tickets.get_all() {
        match ticket {
            Ticket::BackStage(holder, price) => println!("{:?},{:?}", holder, price),
            Ticket::Vip(holder, price) => println!("{:?},{:?}", holder, price),
            Ticket::Standard(price) => println!("{:?}", price),
        }
    }

    // Another version 

    let tickets = vec![
        Ticket::BackStage(50.0, "Toto".to_owned()),
        Ticket::Vip(50.0, "Titi".to_owned()),
        Ticket::Standard(50.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::BackStage(price, holder)=> println!("BackStage : {:?}, {:?}", price, holder),
            Ticket::Vip(price, holder)=> println!("Vip : {:?}, {:?}", price, holder),
            Ticket::Standard(price)=> println!("Standard : {:?}", price),
        }
    }

    let response = Survey {
        q1:Some(12),
        q2:None,
        q3:Some("A".to_owned())
    };

    match response.q1 {
        Some(answer) => println!("q1 : {:?}", answer),
        None => println!("q1  no response"),
    }
    match response.q2 {
        Some(answer) => println!("q1 : {:?}", answer),
        None => println!("q1  no response"),
    }
    match response.q3 {
        Some(answer) => println!("q1 : {:?}", answer),
        None => println!("q1  no response"),
    }

    let toto = Student {
        name: "Toto".to_owned(),
        locker: Some(10),
    };
    println!("Student {:?}", toto.name);
    match toto.locker {
        Some(num) => println!("Locker number : {:?}", num),
        None => println!("No locker assign"),
    }


    let manager = Employee {
        position: Position::LineSupervisor,
        status: Status::Active,
    };

    match print_access(&manager) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }

    let chairs = Item {
        name: "Chair".to_owned(),
        number: 5,
    };
    let beds = Item {
        name: "Beds".to_owned(),
        number: 3,
    };
    let tables = Item {
        name: "Table".to_owned(),
        number: 2,
    };
    let couches = Item {
        name: "Couches".to_owned(),
        number: 0,
    };

    let mut stock = HashMap::new();
    stock.insert("1, &chairs");
    stock.insert("Test".to_owned, 5);
    stock.insert(2, &beds);
    stock.insert(3, &tables);
    stock.insert(4, &couches);
    
    for (k, item) in stock.iter() {
        match is_in_stock(item) {
            Err(e) => println!("{:?}, {:?}", item.name, e),
            _ => println!("in stock : {:?}", item.name),
        }
        
    }

    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    
    for (k, &item) in stock.iter() {
        if item == 0 {
            println!("out of stock");
        } else {
            println!(" qty : {:?} {:?}", k, item);
        }
        
    }

    for key in stock.keys() {
        println!("KEY : {:?}", key);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {:?}", y);

        if y == 1<<5 {break;} // Nom d'occurences de la loop 
    }

    /**
     * Différence entre 1..11 (s'arrête à 10 et 1..=11 s'arrête à 11)
     */
    for x in 1..=11 {
        if x == 3 { continue; }
        if x == 8 { break;}
        println!("{}", x);
    }

    for x in 1..11 {
        if x == 3 { continue; }
        println!("{}", x);
        if x == 8 { break;}
    }

    for (k, v) in (30..41).enumerate() {
        println!("Key : {:?}, Value : {:?}", k, v );
    }

    let country_code = 2;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("country : {:?}", country);

}
