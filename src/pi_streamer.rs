use std::collections::VecDeque;

/// A live digit streamer that yields characters of Pi with position tracking
pub struct PiStreamer {
    pub digits_buffer: VecDeque<char>,
    pub current_digit_index: usize,
    pub is_streaming: bool,
    pub stream_speed: usize, // digits per tick
    pub search_target: Option<String>,
    pub match_found_at: Option<usize>,
    pub recent_digits: String,
}

impl Default for PiStreamer {
    fn default() -> Self {
        let pi_slice: Vec<char> = crate::pi_search::FIRST_10000_DIGITS.chars().collect();
        Self {
            digits_buffer: VecDeque::from(pi_slice),
            current_digit_index: 0,
            is_streaming: true,
            stream_speed: 15,
            search_target: None,
            match_found_at: None,
            recent_digits: String::new(),
        }
    }
}

impl PiStreamer {
    pub fn reset(&mut self, target: Option<String>) {
        let pi_slice: Vec<char> = crate::pi_search::FIRST_10000_DIGITS.chars().collect();
        self.digits_buffer = VecDeque::from(pi_slice);
        self.current_digit_index = 0;
        self.is_streaming = true;
        self.search_target = target;
        self.match_found_at = None;
        self.recent_digits.clear();
    }

    pub fn tick(&mut self) {
        if !self.is_streaming {
            return;
        }

        for _ in 0..self.stream_speed {
            if let Some(c) = self.digits_buffer.pop_front() {
                self.current_digit_index += 1;
                self.recent_digits.push(c);

                // Check target match
                if let Some(ref target) = self.search_target {
                    if self.recent_digits.ends_with(target) {
                        self.match_found_at = Some(self.current_digit_index - target.len() + 1);
                        self.is_streaming = false;
                        break;
                    }
                }

                // Keep recent buffer limited for display performance (last 500 characters)
                if self.recent_digits.len() > 600 {
                    let drain_count = self.recent_digits.len() - 500;
                    self.recent_digits.drain(..drain_count);
                }
            } else {
                // If buffer exhausted, cycle more pseudo-digits or stop
                self.is_streaming = false;
                break;
            }
        }
    }
}
