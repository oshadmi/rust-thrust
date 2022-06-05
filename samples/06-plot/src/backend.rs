use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Ord)]
pub struct Rep {
    _name: String,
    pub deals: u32,
}

// impl Ord for Rep {
//     fn cmp(&self, other: &Self) -> Ordering {
//         (self._name, self.deals).cmp(&(other._name, other.deals))
//     }
// }

impl PartialOrd for Rep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.deals.cmp(&other.deals))
    }
}

impl PartialEq for Rep {
    fn eq(&self, other: &Self) -> bool {
        self.deals == other.deals
    }
}

impl Eq for Rep {}

impl Rep {
    pub fn read_data(path: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut rep_lookup = HashMap::<String, i32>::new();
        // Read the JSON contents of the file as an array of objects.
        let data = serde_json::from_reader(reader)?;
        if let Value::Array(arr) = data {
            for d in arr {
                if let Some(obj) = d.as_object() {
                    let name = obj.get("name").unwrap().as_str().unwrap();
                    match rep_lookup.get_mut(name) {
                        Some(v) => *v += 1,
                        None => {
                            rep_lookup.insert(String::from(name), 1);
                            ()
                        }
                    }
                }
            }
        } else {
            return Err(Box::<dyn Error>::from("expecting an array of objects"));
        }
        let res = rep_lookup
            .drain()
            .map(|(k, v)| Self {
                _name: String::from(k.as_str()),
                deals: v as u32,
            })
            .collect();
        Ok(res)
    }

    pub fn get_histogram_data(data: Vec<Self>) -> Vec<u32> {
        let mut res = Vec::<u32>::new();
        let mut id = 0;
        for d in data {
            for _ in 0..d.deals {
                res.push(id)
            }
            id += 1;
        }
        res
    }
}
