use ansi_term::Colour::{White, Red, Green, Yellow};

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
