
use ansi_term::Colour::{Red, Green, Yellow, White};

use crate::Add;

#[derive(Debug, Clone)]
pub enum Kind  {
    Info,
    Warning, 
    Error,
    Valid
}

impl Kind {

    pub fn get_title(&self, msg: &str) -> String {
        let s = match self {
            Kind::Info => White.paint(msg),
            Kind::Warning => Yellow.paint(msg),
            Kind::Error => Red.paint(msg),
            Kind::Valid => Green.paint(msg),
        };
        s.to_string()
    }

}

pub struct DebugMessage{
    pub kind: Kind,
    pub title: String,
    pub line: String,
    subline: String,
    pub line_nb: u8,
    pub explain: String,
}

impl DebugMessage {


    /// Return a new, empty Debug Message struct.
    pub fn new(kind: Kind, title: &str, line_nb: u8) -> Self {
        let line = format!("l.{} | ", line_nb);
        let mut subline = String::new();
        subline.add_patt(" ", line.len());
        DebugMessage {kind: kind.clone(), title: kind.get_title(title).to_string(), line, line_nb, explain: String::new(), subline }
    }

    /// Will appnd the line.
    /// This will create the field where the query debugged is displayed.
    pub fn append(&mut self, line: &str) -> &mut Self{
        self.line.push_str(line);
        self.subline.add_patt(" ", line.len());
        self
    }

    /// Will append a String to the query displayed where the user should consider appending something. 
    pub fn add(&mut self, add: &str) -> &mut Self{
        self.line.push_str(&Green.paint(add).to_string());
        self.subline.add_patt(&Green.paint("+").to_string(), add.len());
        self
    }

    /// Will append a String to the query displayed where the user shouldconsider  removing something
    pub fn remove(&mut self, remove: &str)-> &mut Self{
        self.line.push_str(&Red.paint(remove).to_string());
        self.subline.add_patt(&Red.paint("-").to_string(), remove.len());
        self
    }

    /// Append an explanation displayed after the query.
    pub fn explain(&mut self, explain: &str) -> &mut Self{
        self.explain = explain.into();
        self
    }

    /// Will print the message.
    pub fn consume(&self) {
        println!("\n{}\n\n    {}\n    {}\n\n   {}\n", self.title, self.line, self.subline, self.explain);
    } 

}

impl crate::Message for DebugMessage {
    
    /// Will print the message.
    fn consume(&self) {
        println!("{}\n\n    {}\n    {}\n\n   {}\n", self.title, self.line, self.subline, self.explain);
    } 

    /// Return the kind of the Message.
    fn kind(&self) -> Kind {
        self.kind.clone()
    }

}


