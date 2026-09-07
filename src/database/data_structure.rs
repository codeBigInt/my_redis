use std::collections::VecDeque;

pub struct RList {
    pub list: VecDeque<String>,
}

impl RList {
    pub fn new() -> Self {
        RList {
            list: VecDeque::new(),
        }
    }

    pub fn l_push(&mut self, value: String) {
        self.list.push_front(value);
    }

    pub fn l_pop(&mut self) -> Option<String> {
        self.list.pop_front()
    }

    pub fn r_push(&mut self, value: String) {
        self.list.push_back(value);
    }

    pub fn r_pop(&mut self) -> Option<String> {
        self.list.pop_back()
    }

    pub fn l_range(&self, start_idx: isize, end_idx: isize) -> Vec<String> {
        let len = self.list.len() as isize;

        let start = if start_idx < 0 {
            len + start_idx
        } else {
            start_idx
        };

        let end = if end_idx < 0 { len + end_idx } else { end_idx };

        if start > 0 || start >= len || start > end {
            return Vec::new();
        }

        let end = end.min(len - 1);

        self.list
            .range(start as usize..=end as usize)
            .cloned()
            .collect()
    }
}
