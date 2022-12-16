use super::packet::PacketData;

const BASE: u32 = 10;

pub fn parse_packet_pair(packet_pair: Vec<String>) -> (PacketData, PacketData) {
    assert!(packet_pair.len() == 2, "Invalid pair");
    let packet_left = parse_packet(&packet_pair[0]);
    let packet_right = parse_packet(&packet_pair[1]);
    (packet_left, packet_right)
}

pub fn parse_packet(packet_line: &String) -> PacketData {
    assert!(
        packet_line.starts_with('[') && packet_line.ends_with(']'),
        "Invalid list"
    );

    let mut chars_list = packet_line.chars();
    chars_list.next();
    let list = parse_list(&mut chars_list);
    *list
}

fn parse_list(chars_list: &mut std::str::Chars) -> Box<PacketData> {
    let mut list = vec![];
    'outer: while let Some(next_char) = chars_list.next() {
        match next_char {
            '[' => {
                list.push(parse_list(chars_list));
            }
            ']' => break,
            char => {
                if char == ',' {
                    continue;
                } else if char.is_numeric() {
                    let mut number_stack = vec![char];
                    while let Some(number_char) = chars_list.next() {
                        if number_char == ',' {
                            break;
                        } else if number_char == ']' {
                            let final_number = get_number_from_chars(&number_stack);
                            list.push(Box::new(PacketData::Number(final_number)));
                            break 'outer;
                        }
                        number_stack.push(number_char);
                    }
                    let final_number = get_number_from_chars(&number_stack);
                    list.push(Box::new(PacketData::Number(final_number)));
                }
            }
        }
    }
    Box::new(PacketData::List(list))
}

fn get_number_from_chars(number_chars: &Vec<char>) -> u32 {
    let mut final_number = 0;
    for (i, char_number) in number_chars.iter().enumerate() {
        let inverse_index = number_chars.len() - 1 - i;
        let parsed_char = char_number.to_digit(BASE);
        assert!(parsed_char.is_some(), "Invalid char");
        if let Some(parsed_char) = parsed_char {
            final_number += parsed_char * BASE.pow(inverse_index as u32);
        }
    }
    final_number
}
