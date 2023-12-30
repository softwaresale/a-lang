use std::str::Chars;

pub struct InputReader<'input> {
    /// the original source
    source: &'input str,
    /// the buffer we are reading right now
    source_chars: Chars<'input>,
    /// used to track our index in the original string. Each time we advance, bump the cursor
    cursor: usize,
}

impl<'input> InputReader<'input> {
    pub fn new(ascii_source: &'input str) -> Self {
        Self {
            source: ascii_source,
            source_chars: ascii_source.chars(),
            cursor: 0
        }
    }

    /// read a single character and advance to the next one
    pub fn read_and_advance(&mut self) -> Option<char> {
        self.cursor += 1;
        self.source_chars.next()
    }

    /// read at most amount bytes
    pub fn read_and_advance_n(&mut self, amount: usize) -> Option<&'input str> {
        // bounds check
        if self.cursor >= self.source.len() {
            return None;
        }

        // don't read past end
        let read_count = if self.cursor + amount >= self.source.len() {
            self.source.len()
        } else {
            amount
        };

        // actually read
        let slice = &self.source[self.cursor..read_count];

        // update cursor and chars
        self.cursor += amount;
        self.source_chars.nth(amount);

        Some(slice)
    }

    pub fn rewind(&mut self, amount: usize) {
        self.cursor -= amount;
        self.source_chars = self.source.chars();
        for _ in 0..self.cursor {
            self.source_chars.next();
        }
    }

    pub fn advance_n(&mut self, amount: usize) {
        self.cursor += amount;
        for _ in 0..amount {
            self.source_chars.next();
        }
    }

    /// read until pred evals to false
    pub fn read_until<PredT: Fn(char) -> bool>(&mut self, pred: PredT) -> Option<&'input str> {
        self.read_until_callback(pred, |_| {})
    }

    /// read characters while pred evaluates to true. Returns a string subslice of the characters
    /// read by this operation
    pub fn read_until_callback<PredT: Fn(char) -> bool, CharReceiverT: FnMut(char) -> ()>(&mut self, pred: PredT, mut on_char_receiver: CharReceiverT) -> Option<&'input str> {
        let start = self.cursor;
        let end = loop {
            let next_char = self.source_chars.next();
            if next_char.is_some_and(|next_ch| pred(next_ch)) {
                // run the given callback on the next character
                on_char_receiver(next_char.unwrap());

                // this character is valid, so advance the cursor
                self.cursor += 1;
            } else {
                // either no more characters, or an invalid character
                break self.cursor;
            }
        };

        if start != end {
            Some(&self.source[start..end])
        } else {
            None
        }
    }

    pub fn peek_n(&self, amount: usize) -> Option<&'input str> {
        if self.cursor + amount >= self.source.len() {
            return None;
        }

        Some(&self.source[self.cursor..self.cursor + amount])
    }
}
