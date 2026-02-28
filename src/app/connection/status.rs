use super::Address;
use ratatui::style::Color;

type Code = Result<u16, String>;
type Timestamp = chrono::DateTime<chrono::Local>;

#[derive(Clone, Debug)]
pub struct Status {
    code: Code,
    msg: Option<String>,
    time: Timestamp,
    pub id: u32,
}

impl Status {
    pub fn new(code: Code, id: u32) -> Self {
        Self {
            code,
            time: chrono::Local::now(),
            msg: None,
            id,
        }
    }

    pub fn id(self, id: u32) -> Self {
        Self {
            id,
            msg: self.msg,
            code: self.code,
            time: self.time,
        }
    }

    pub fn set_msg(self, msg: String) -> Self {
        Self {
            msg: Some(msg),
            code: self.code,
            time: self.time,
            id: self.id,
        }
    }

    pub fn msg(&self) -> Option<String> {
        self.msg.clone()
    }

    pub const fn code(&self) -> &Code {
        &self.code
    }

    pub const fn timestamp(&self) -> Timestamp {
        self.time
    }

    pub fn is_recent(&self) -> bool {
        chrono::Local::now()
            .signed_duration_since(self.time)
            .num_milliseconds()
            < 275
    }

    pub const fn generate_color(&self, addr: &Address) -> Color {
        let Ok(code) = self.code() else {
            return Color::Red;
        };

        match addr {
            Address::Remote { .. } | Address::Json { .. } => match code {
                200 => Color::Green,
                400.. => Color::Red,
                _ => Color::Yellow,
            },

            Address::Local { .. } => Color::Green,
        }
    }
}

impl From<Code> for Status {
    fn from(value: Code) -> Self {
        Self::new(value, 0)
    }
}
