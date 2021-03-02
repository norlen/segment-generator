mod segment;

use segment::Segment;
use std::{fs::File, io::Read};

/// Converts a list of segment letters to a SCC.
///
///
/// The basic layout is that each indivial segment maps
/// to a certain nibble in an u16. These are then combined
/// into the final SCC.
///              0             1           2          3
///         15 14 13 12    11 10 9 8    7 6 5 4    3 2 1 0
/// SCC = {{ K, 0, 0, A}, {J, F, H,B}, {L,E,G,C}, {M,P,N,D}}
/// Output should be in a 16-bit unsigned byte. In the order
/// {3 2 1 0}. So the actual repr should be.
fn convert(segments: &[char]) -> u16 {
    let scc = segments
        .iter()
        .fold(0, |scc, &c| scc | Segment::from_ch(c).mask());

    // Swap nibble order from {0,1,2,3} to {3,2,1,0}.
    let mut out = 0;
    for i in 0..4 {
        // Get nibble i.
        let nibble = (scc >> i * 4) & 0b1111;

        // Put it at nibble position 4-i.
        let swapped = nibble << (12 - i * 4);

        out |= swapped;
    }
    out
}

/// Parse the input in the form (Character Segment[, Segment])
fn parse_input(input: &str) -> (Option<char>, Vec<char>) {
    let mut ch = None;
    let mut others = Vec::new();

    for c in input.chars() {
        if c.is_alphanumeric() {
            if ch.is_none() {
                ch = Some(c);
            } else {
                others.push(c);
            }
        }
    }

    (ch, others)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or("./input.txt".into());
    let contents = {
        let mut input = File::open(path)?;
        let mut contents = String::new();
        input.read_to_string(&mut contents)?;
        contents
    };

    for line in contents.lines() {
        let (letter, segments) = parse_input(line);

        // Only convert the segments if we got a valid letter back when parsing.
        if let Some(letter) = letter {
            let bytes = convert(segments.as_slice());
            println!("{}: {:#x}", letter, bytes);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "A A,B,C,E,F,G,L";
        assert_eq!(
            parse_input(input),
            (Some('A'), vec!['A', 'B', 'C', 'E', 'F', 'G', 'L'])
        );
    }

    #[test]
    fn it_works() {
        let input = "A A,B,C,E,F,G,L";
        let (_letter, segments) = parse_input(input);
        assert_eq!(convert(segments.as_slice()), 0x0F51)
    }
}
