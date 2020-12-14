use structopt::StructOpt;
use shared::{ FileContentOpts };
use shared::regex;
use std::collections::HashMap;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let cmds = parse_input(&opts.file);

    let mut vals = HashMap::new();
    let mut mask = Mask::new_xs();
    for cmd in &cmds {
        match cmd {
            Cmd::Mask(m) => { mask = *m },
            Cmd::SetMem { n, val} => { vals.insert(*n, mask.apply_to_value(*val)); }
        }
    }
    println!("Star 1: {}", vals.values().sum::<u64>());

    let mut vals = HashMap::new();
    let mut mask = Mask::new_xs();
    for cmd in &cmds {
        match cmd {
            Cmd::Mask(m) => { mask = *m },
            Cmd::SetMem { n, val} => {
                for address in mask.apply_to_address(*n) {
                    vals.insert(address, *val);
                }
            }
        }
    }
    println!("Star 2: {}", vals.values().sum::<u64>());

    Ok(())
}

fn parse_input(s: &str) -> Vec<Cmd> {
    s.lines().filter_map(|line| {
        if let Some(caps) = regex!(r"mem\[([0-9]+)\] = ([0-9]+)").captures(line) {
            Some(Cmd::SetMem {
                n: caps.get(1)?.as_str().parse().ok()?,
                val: caps.get(2)?.as_str().parse().ok()?
            })
        } else if let Some(caps) = regex!(r"mask = ([X01]+)").captures(line) {
            Some(Cmd::Mask(
                Mask::from_str(caps.get(1)?.as_str())?
            ))
        } else {
            None
        }
    }).collect()
}

#[derive(Clone,Copy,Debug)]
enum Cmd {
    Mask(Mask),
    SetMem { n: u64, val: u64 }
}

#[derive(Clone,Copy,Debug)]
struct Mask {
    // stored from lowest bit to highest bit
    bits: MaskBits
}

impl Mask {
    fn new_xs() -> Mask {
        Mask { bits: [MaskBit::BitX; 36] }
    }
    fn from_str(s: &str) -> Option<Mask> {
        let mut bits = [MaskBit::BitX;36];
        for (idx, b) in s.as_bytes().iter().rev().enumerate() {
            match b {
                b'0' => { bits[idx] = MaskBit::Bit0 },
                b'1' => { bits[idx] = MaskBit::Bit1 },
                b'X' => { bits[idx] = MaskBit::BitX },
                _ => { return None }
            }
        }
        Some(Mask { bits })
    }
    fn apply_to_value(&self, mut val: u64) -> u64 {
        for (idx,mask_bit) in self.bits.iter().enumerate() {
            match mask_bit {
                MaskBit::Bit0 => { val &= !(1 << idx) },
                MaskBit::Bit1 => { val |= 1 << idx },
                _ => { /* do nout with X */ }
            }
        }
        val
    }
    fn apply_to_address(&self, val: u64) -> Vec<u64> {
        // The possible combinations represented here as mask bits:
        let mut combination = [MaskBit::Bit0; 36];
        for (idx,mask_bit) in self.bits.iter().enumerate() {
            let new_bit = match mask_bit {
                MaskBit::Bit0 => {
                    match (val >> idx) & 1 {
                        0 => MaskBit::Bit0,
                        _ => MaskBit::Bit1
                    }
                },
                other => *other
            };
            combination[idx] = new_bit;
        }

        // Expand X bits into 0s and 1s somewhat inefficiently:
        let x_locs = combination
            .iter()
            .enumerate()
            .filter(|(_,m)| m.is_x())
            .map(|(idx,_)| idx);
        let mut expanded_combos = vec![combination];
        for loc in x_locs {
            let mut new_expanded = Vec::new();
            for mut c in expanded_combos {
                c[loc] = MaskBit::Bit0;
                new_expanded.push(c);
                c[loc] = MaskBit::Bit1;
                new_expanded.push(c);
            }
            expanded_combos = new_expanded;
        }

        // Convert back to regular usizes to return:
        expanded_combos
            .into_iter()
            .map(|c| Mask { bits: c }.apply_to_value(0))
            .collect()
    }
}

#[derive(Clone,Copy,Debug)]
enum MaskBit {
    Bit0,
    Bit1,
    BitX,
}

impl MaskBit {
    fn is_x(&self) -> bool {
        match self {
            MaskBit::BitX => true,
            _ => false
        }
    }
}

type MaskBits = [MaskBit; 36];