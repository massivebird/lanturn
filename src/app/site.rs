use std::collections::VecDeque;

pub const MAX_STATUSES: usize = 50;

#[derive(Clone)]
pub struct Site {
    pub name: String,
    pub url: String,
    status_codes: VecDeque<Option<Result<u16, ()>>>,
}

impl Site {
    pub(super) fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            url: addr.to_string(),
            status_codes: vec![None; MAX_STATUSES].into(),
        }
    }

    pub fn push_status_code(&mut self, code: Option<Result<u16, ()>>) {
        if self.status_codes.len() == MAX_STATUSES {
            self.status_codes.pop_back();
        }

        self.status_codes.push_front(code);
    }

    pub fn get_status_codes(&self) -> VecDeque<Option<Result<u16, ()>>> {
        self.status_codes.clone()
    }
}

