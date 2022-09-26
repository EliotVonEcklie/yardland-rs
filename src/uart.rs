use crate::cp437::convert_byte;

#[derive(Debug)]
pub struct UART {
    buf: Vec<String>,
    current_string: usize
}

impl UART {
    pub fn new() -> Self {
        let mut buf = Vec::new();
        buf.push(String::new());

        Self {
            buf,
            current_string: 0
        }
    }

    pub fn push_char(&mut self, c: u8) {
        if c == 0 {
            self.current_string += 1;
            self.buf.push(String::new());
        }
        else {
            self.buf.get_mut(self.current_string).unwrap().push(convert_byte(c));
        }

        return;
    }

    pub fn get_str_buf(&self) -> String {
        let mut result = String::new();

        for s in &self.buf {
            result.push_str(s);
        }

        result
    }
}
