#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![forbid(non_ascii_idents)]
#![allow(clippy::uninlined_format_args)]

use sha2::{Digest, Sha224};

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

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

    Ok(())
}

// #include <QCryptographicHash>
// #include <QString>
// #include <iostream>
//
// int main(int argc, char *argv[]) {
//     QCryptographicHash hash(QCryptographicHash::Sha224);
//     hash.addData(QStringLiteral("mQcLvEg1HW8JuRXY3BawjSpe").toUtf8());//a salt
//     hash.addData(QStringLiteral("000000000000000000000000000nHj").toUtf8());
//     const QByteArray result=hash.result();
//
//     std::cout << "length: " << result.length() << std::endl;
//
//     for (int i = 0; i < result.length(); i++) {
//         std::cout << (unsigned int)(unsigned char)result[i] << std::endl;
//     }
//
//     std::cout << QString(result).data() << std::endl;;
//
//     if (result.at(0)==0x00 && result.at(1)==0x00) {
//         std::cout << "winner!" << std::endl;;
//     }
// }

struct Permutator<T> {
    range: T,
}

impl<T> Permutator<T>
where
    T: Iterator<Item = u8> + Clone,
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
