use std::time::Duration;

use Statistic::*;

pub enum Statistic {
    Average(u8),
    Best,
    Worst,
}

// TODO: Easy efficiency improvements:
// - Computation can easily be O(1) amortized, it's two elementary operations assuming you've
// stored the length and previous computation
//
// avg = (prev_avg * (prev_n) + new_time) / new_n
// worst should be saved and updated
// best should be saved and updated

impl Statistic {
    pub fn label(&self) -> String {
        match self {
            Average(n) => format!("avg{}", n),
            Best => String::from("best"),
            Worst => String::from("worst"),
        }
    }

    pub fn compute(&self, times: &[Duration]) -> Option<Duration> {
        match self {
            Average(n) => {
                if times.len() < (*n as usize) {
                    None
                } else {
                    let (sum, max, min) = times
                        .iter()
                        .rev()
                        .take(*n as usize)
                        .map(|x| x.as_secs_f64())
                        .fold((0.0, f64::MIN, f64::MAX),
                        |(s, ma, mi), t| (s + t, f64::max(ma, t), f64::min(mi, t)));

                    let average = (sum - max - min) / ((*n - 2) as f64);
                    Some(Duration::from_secs_f64(average))
                }
            }
            Worst => times.iter().cloned().max(),
            Best => times.iter().cloned().min(),
        }
    }
}
