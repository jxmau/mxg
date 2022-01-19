use message::Kind;

pub mod message;
pub mod buffer;
#[cfg(feature= "info")]
#[unstable(feature=info, reason="The struct used in this feature is not integrated into the normal overall workflow. Changes can happen.")]
pub mod info;


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