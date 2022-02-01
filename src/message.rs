
use std::fmt::Debug;

use ansi_term::Colour::{Red, Green, Yellow};
use crate::kind::Kind;

use crate::Add;

#[derive(Debug)]
pub struct DebugMessage{
    kind: Kind,
    title: String,
    line: String,
    subline: String,
    explain: String,
}

impl DebugMessage {


    /// Return a new, empty Debug Message struct.
    /// This will create the core of the message. To specify the location of the error, use the locate method.
    pub fn new(kind: Kind, title: &str, line_nb: u8) -> Self {
        let line = format!("l.{} | ", line_nb);
        let mut subline = String::new();
        subline.add_patt(" ", line.len());
        DebugMessage {kind: kind.clone(), title: kind.get_title(title), line, explain: String::new(), subline }
    }

    /// Will create a prefix for the title with the name of the file, the line and the column where the error is located and/or starts.
    pub fn locate(&mut self, name: &str, line: u8, col: u8) -> &mut Self {
        let mut prefix = format!("{} {} - {} | ", name, line, col);
        prefix.push_str(&self.title);
        self.title = self.kind.get_title(&prefix);  
        self
    }

    /// Will append the line.
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

    /// To be used when pointing to a slice of a query when no suggestion are needed. 
    pub fn highlight(&mut self, hl: &str) -> &mut Self {
        self.line.push_str(&Yellow.paint(hl).to_string());
        self.subline.add_patt(&Yellow.paint("^").to_string(), hl.len());
        self
    }

    /// Append an explanation displayed after the query.
    pub fn explain(&mut self, explain: &str) -> &mut Self{
        self.explain.push_str(&format!("{}\n", explain));
        self
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

// impl crate::SMessage for DebugMessage {}


#[cfg(feature= "buffless")]
impl Drop for DebugMessage {
    
    fn drop(&mut self) {
        println!("{}\n\n    {}\n    {}\n\n   {}\n", self.title, self.line, self.subline, self.explain);
    }
}

impl super::Boxable for DebugMessage {}
