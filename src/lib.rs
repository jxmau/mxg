use kind::Kind;

pub mod kind;
pub mod message;
#[cfg_attr(feature = "cargo-clippy", deny(clippy::drop_ref))]
#[cfg(feature= "buffer")]
pub mod buffer;

#[cfg(feature= "info")]
pub mod info;


pub trait Message {
    fn consume(&self);
    fn kind(&self) -> Kind;
}

//pub trait SMessage: Message + std::fmt::Debug {}
type Msg = Box<dyn Message>;


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


/// Trait to simplify Boxing a Message.
pub trait Boxable{

    /// Will return Self boxed.
    /// To not use on a method returning &mut Self or it will throw a compiler error.
    fn boxed( &mut self) -> Box<&mut Self> {
        Box::new(self)
    }

}