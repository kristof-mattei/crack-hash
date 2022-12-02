#![cfg_attr(not(debug_assertions), deny(warnings))]

use sha2::Digest;
use sha2::Sha224;

fn main() {
    let start = "mQcLvEg1HW8JuRXY3BawjSpe".as_bytes().to_vec();

    let numbers = 48u8..=57;
    let uppercase = 65u8..=90;
    let lowercase = 97u8..=122;

    let permutator = Permutator {
        range: (numbers.chain(uppercase)).chain(lowercase),
    };

    for max in 0..=100 {
        if let Some(v) = permutator.permutate(&start, 0, max) {
            let m = String::from_utf8_lossy(&v);
            println!(
                "Match found: {}",
                m.chars().skip(start.len()).collect::<String>()
            );
            break;
        }
    }
}

struct Permutator<T> {
    range: T,
}

impl<T> Permutator<T>
where
    T: Iterator<Item = u8>,
    T: Clone,
{
    fn permutate(&self, v: &[u8], current: usize, max: usize) -> Option<Vec<u8>> {
        let r = hash(v);

        if test(&r) {
            return Some(v.to_vec());
        }

        if current == max {
            return None;
        }

        for i in self.range.clone() {
            let mut c = v.to_vec();

            c.push(i);

            if let Some(v) = self.permutate(&c, current + 1, max) {
                return Some(v);
            }
        }

        None
    }
}

fn hash(v: &[u8]) -> Vec<u8> {
    let mut hasher = Sha224::new();

    hasher.update(v);

    hasher.finalize().to_vec()
}

fn test(s: &[u8]) -> bool {
    s[0] == 0x00u8 && s[1] == 0x00u8
}

#[cfg(test)]
mod tests {}
