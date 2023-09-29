use std::collections::hash_map::Iter;
use std::collections::{hash_map, HashMap};

use serde::{Deserialize, Serialize};

use crate::Job;

#[derive(Debug, PartialEq, Serialize, Default, Deserialize)]
pub struct Jobs(HashMap<String, Job>);

/// Implement HashMap methods for Jobs
impl Jobs {
    pub fn insert(&mut self, name: String, job: Job) -> Option<Job> {
        self.0.insert(name, job)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, String, Job> {
        self.0.iter()
    }

    pub fn get(&self, name: &str) -> Option<&Job> {
        self.0.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Job> {
        self.0.get_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Job> {
        self.0.remove(name)
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn keys(&self) -> hash_map::Keys<'_, String, Job> {
        self.0.keys()
    }

    pub fn values(&self) -> hash_map::Values<'_, String, Job> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, String, Job> {
        self.0.values_mut()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn get_or_insert(&mut self, name: String, job: Job) -> &mut Job {
        self.0.entry(name).or_insert(job)
    }
}

#[macro_export]
macro_rules! jobs {
    () => {
        $crate::Jobs::default()
    };
    ($($name:ident: $job:expr),* $(,)?) => {
        {
            let mut jobs = $crate::Jobs::default();
            $(
                jobs.insert(stringify!($name).to_string(), $job);
            )*
            jobs
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Job;

    #[test]
    fn jobs() {
        let jobs = jobs! {
            build: Job::default(),
            test: Job::default(),
        };
        assert_eq!(jobs.0.len(), 2);
    }
}
