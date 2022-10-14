
pub struct CartesianProduct {
    pub length: usize,
    pub counters: Vec<usize>,
    pub lengths: Vec<usize>,
    pub lists: Vec<Vec<String>>,
}

impl <'parse> CartesianProduct {
    pub fn new(input: &'parse Vec<Vec<String>>) -> Self {
        Self {
            length: input.len(),
            counters: vec![0; input.len()],
            lengths: vec![],
            lists: input.clone(),
        }
    }
    pub fn next(&'parse mut self, values: &'parse mut Vec<&'parse String>) -> bool {
        for i in 0 .. self.length {
            values[i] = &self.lists[i][self.counters[i]];
        }
        for i in self.length ..= 0 {
            if self.counters[i] < self.lengths[i] - 1 {
                self.counters[i] += 1;
                return true;
            } else {
                self.counters[i] = 0;
            }
        }
        return false;
    }
}