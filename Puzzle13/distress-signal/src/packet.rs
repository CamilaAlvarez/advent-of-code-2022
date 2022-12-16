use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketData {
    List(Vec<Box<PacketData>>),
    Number(u32),
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::List(left_packet_list) => match other {
                Self::List(right_packet_list) => compare_lists(left_packet_list, right_packet_list),
                Self::Number(right_number) => {
                    let right_list = vec![Box::new(Self::Number(*right_number))];
                    Self::List(left_packet_list.clone()).cmp(&Self::List(right_list))
                }
            },
            Self::Number(left_number) => match other {
                Self::List(right_packet_list) => {
                    let left_list = vec![Box::new(Self::Number(*left_number))];
                    Self::List(left_list).cmp(&Self::List(right_packet_list.clone()))
                }
                Self::Number(right_number) => {
                    return left_number.cmp(right_number);
                }
            },
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_lists(
    left_packet_list: &Vec<Box<PacketData>>,
    right_packet_list: &Vec<Box<PacketData>>,
) -> cmp::Ordering {
    for i in 0..cmp::max::<usize>(left_packet_list.len(), right_packet_list.len()) {
        // There are still elements left in the right array
        if i >= left_packet_list.len() && i < right_packet_list.len() {
            return cmp::Ordering::Less;
        }
        // There are still elements left in the left array
        else if i < left_packet_list.len() && i >= right_packet_list.len() {
            return cmp::Ordering::Greater;
        }
        match left_packet_list[i]
            .as_ref()
            .cmp(right_packet_list[i].as_ref())
        {
            cmp::Ordering::Less => {
                return cmp::Ordering::Less;
            }
            cmp::Ordering::Equal => {
                continue;
            }
            cmp::Ordering::Greater => {
                return cmp::Ordering::Greater;
            }
        }
    }
    // If we reached here we know the packets are equal
    cmp::Ordering::Equal
}
