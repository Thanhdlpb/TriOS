#[derive(Debug)]
pub struct Reader {
    chars: Vec<char>,
    index: usize,
    dong: usize,
    cot: usize,
}

impl Reader {
    pub fn moi(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            index: 0,
            dong: 1,
            cot: 1,
        }
    }

    pub fn eof(&self) -> bool {
        self.index >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    pub fn peek2(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    pub fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.index += 1;

        if ch == '\n' {
            self.dong += 1;
            self.cot = 1;
        } else {
            self.cot += 1;
        }

        Some(ch)
    }

    pub fn bo_qua_khoang_trang(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn doc_khi<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if f(c) {
                s.push(c);
                self.next();
            } else {
                break;
            }
        }

        s
    }

    pub fn dong(&self) -> usize {
        self.dong
    }

    pub fn cot(&self) -> usize {
        self.cot
    }
}
