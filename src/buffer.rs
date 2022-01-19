use crate::{Message, message::Kind};

#[derive(Debug, Default, Clone)]
pub struct Buffer<T: Message>{
    error_ct : u8,
    warn_ct : u8,
    err_limit : u8,
    msgs: Vec<T>,
}

impl<T:  Message> Buffer<T> {

    /// Return a new Buffer with a limit of error of 20.
    pub fn new() -> Self {
        Buffer {error_ct: 0, warn_ct: 0, err_limit: 20, msgs: Vec::new()}
    }

    /// Change the limit of errors before early return
    pub fn limit(&mut self, l: u8) -> &mut Self {
        self.err_limit = l;
        self
    }

    /// Add the Message to the buffer.
    /// If adding the message causing the limit to be reached. 
    /// The buffer will self-drop, casuing an early return.
    pub fn link(&mut self, m: T) {
        match m.kind() {
            Kind::Error => self.error_ct += 1,
            Kind::Warning => self.warn_ct += 1,
            _ => {}
        };
        self.msgs.push(m);
        if self.error_ct.eq(&self.err_limit) {
            println!("{}", ansi_term::Colour::Red.paint("Early return caused by the limit of error messages having been reached."));
            drop(self);
        }
    }



}

impl<T: Message> Drop for Buffer<T>
{

    fn drop(&mut self) {
        use ansi_term::Colour::{Red, Green, Yellow};
        for m in &self.msgs {
            m.consume();
        }
        let colour = if self.error_ct != 0 { Red } else if self.warn_ct != 0 { Yellow } else { Green};
        let m = colour.paint(format!("Final Result | Error : {} , Warning : {}", self.error_ct, self.warn_ct));
        println!("{}\n", m);

    }
}