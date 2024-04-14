use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    frequency: u32,
    address: usize,
}

impl Variable {
    pub fn get_address(&self) -> String {
        format!("S{}", self.address)
    }
}

#[derive(Debug)]
pub struct Variables {
    vars: HashMap<String, Variable>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            vars: HashMap::new(),
        }
    }

    pub fn add(&mut self, var_name: String, frequency: u32) {
        let var = Variable {
            frequency,
            address: self.vars.len(),
        };
        self.vars.insert(var_name, var);
    }

    pub fn set(&mut self, var_name: String) {
        if let Some(var) = self.get_mut(&var_name) {
            var.frequency += 1;
        } else {
            let var = Variable {
                frequency: 1,
                address: self.vars.len(),
            };
            self.vars.insert(var_name, var);
        }
    }
    pub fn get(&self, var_name: &str) -> Option<&Variable> {
        self.vars.get(var_name)
    }
    pub fn get_mut(&mut self, var_name: &str) -> Option<&mut Variable> {
        self.vars.get_mut(var_name)
    }

    // function to calculate addresses of all variables, give with most frequency, smallest address
    pub fn calculate_addresses(&mut self) {
        let mut sorted_vars: Vec<&mut Variable> = self.vars.values_mut().collect();
        sorted_vars.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        for (i, var) in sorted_vars.iter_mut().enumerate() {
            var.address = i + 1;
        }
    }

    pub fn count(&self) -> i32 {
        self.vars.len() as i32
    }
}
