use clap::Parser;
use distress_signal::packet::PacketData;
use distress_signal::parser::{parse_packet, parse_packet_pair};
use std::cmp;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(args.input).unwrap();

    // Parse packets
    let mut packet_pairs: Vec<(PacketData, PacketData)> = vec![];
    let mut pair_data_vec = vec![];
    for line in content.lines() {
        if line.trim().is_empty() {
            packet_pairs.push(parse_packet_pair(pair_data_vec));
            pair_data_vec = vec![];
        } else {
            pair_data_vec.push(line.trim().to_string());
        }
    }
    if pair_data_vec.len() == 2 {
        packet_pairs.push(parse_packet_pair(pair_data_vec));
    }

    // Sum packet indexes in the right order
    let mut correct_indexes = vec![];
    let mut all_packets = vec![];
    for i in 0..packet_pairs.len() {
        let (packet_left, packet_right) = &packet_pairs[i];
        if packet_left.cmp(&packet_right) == cmp::Ordering::Less {
            correct_indexes.push(i + 1);
        }
        all_packets.push(packet_left);
        all_packets.push(packet_right);
    }
    let sum_indexes: usize = correct_indexes.iter().sum();
    println!("Sum indexes in right order: {}", sum_indexes);

    // Get the decoder key
    let divider_packet1 = parse_packet(&"[[2]]".to_string());
    let divider_packet2 = parse_packet(&"[[6]]".to_string());
    all_packets.push(&divider_packet1);
    all_packets.push(&divider_packet2);
    all_packets.sort();
    let mut decoder_key = 1;
    for (i, packet) in all_packets.iter().enumerate() {
        if packet.cmp(&&divider_packet1) == std::cmp::Ordering::Equal
            || packet.cmp(&&divider_packet2) == std::cmp::Ordering::Equal
        {
            let index = i + 1;
            decoder_key *= index;
        }
    }
    println!("Decoder key: {}", decoder_key);
}
