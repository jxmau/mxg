use message::Kind;

pub mod message;
pub mod buffer;

pub trait Message {
    fn consume(&self);
    fn kind(&self) -> Kind;
}

pub(crate) trait Add {
    fn add_patt(&mut self, c: &str, nb: usize);
}

impl Add for String {
    fn add_patt(&mut self, c: &str, nb: usize) {
        for _ in 0..nb {
            self.push_str(c);
        }
    }
}