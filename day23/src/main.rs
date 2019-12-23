extern crate intcode;

use intcode::IntcodeProgram;
use std::collections::{HashMap, VecDeque};

const NIC_PROGRAM: &str = include_str!("../day23.txt");
const NAT_ADDR: i64 = 255;

fn main() {
    println!("Day 23-1: {}", run_network(1));
    println!("Day 23-2: {}", run_network(2));
}

fn run_network(part_no: u8) -> i64 {
    let mut nics = Vec::new();
    let mut packet_queues: HashMap<i64, VecDeque<Packet>> = HashMap::new();

    for i in 0..50 {
        let mut program = IntcodeProgram::init_from(NIC_PROGRAM);
        program.buffer_input(i);
        nics.push(program);
        packet_queues.insert(i, VecDeque::new());
    }

    let mut nat_packet: Option<Packet> = None;
    let mut last_received_from_nat_by_0: Option<Packet> = None;

    let mut idle;
    loop {
        idle = true;

        for (nic_id, nic) in nics.iter_mut().enumerate() {
            let nic_id = nic_id as i64;

            nic.run();

            if nic.is_awaiting_input() {
                let pq = packet_queues.get_mut(&nic_id).unwrap();

                match pq.pop_front() {
                    Some(packet) => {
                        idle = false; // receiving, network not idle

                        // NAT --> 0 receiving
                        if part_no == 2 && packet.src == NAT_ADDR {
                            if let Some(last_nat_packet) = last_received_from_nat_by_0 {
                                if packet.y == last_nat_packet.y {
                                    return packet.y;
                                }
                            }

                            last_received_from_nat_by_0 = Some(packet);
                        }

                        nic.buffer_input(packet.x);
                        nic.buffer_input(packet.y);
                    }
                    None => {
                        nic.buffer_input(-1);
                    }
                }
            }

            if let Some(dest_addr) = nic.consume_output() {
                idle = false; // sending, network not idle

                let send_packet = {
                    let x = nic.consume_output().unwrap();
                    let y = nic.consume_output().unwrap();
                    Packet { src: nic_id, x, y }
                };

                let pq = packet_queues.get_mut(&dest_addr);
                match pq {
                    Some(pq) => pq.push_back(send_packet),
                    None => {
                        // 0 --> NAT sending
                        if part_no == 1 && dest_addr == NAT_ADDR {
                            return send_packet.y;
                        } else {
                            nat_packet = Some(send_packet);
                        }
                    }
                }
            }
        }

        // NAT --> 0 sending
        if part_no == 2 && idle {
            if let Some(packet_sent_to_nat) = nat_packet {
                // rewrite the packet to send to 0
                let packet_for_0 = Packet {
                    src: NAT_ADDR,
                    x: packet_sent_to_nat.x,
                    y: packet_sent_to_nat.y,
                };
                packet_queues.get_mut(&0).unwrap().push_back(packet_for_0);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Packet {
    src: i64,
    x: i64,
    y: i64,
}

#[cfg(test)]
mod tests {
    use crate::run_network;

    #[test]
    fn test_part1() {
        assert_eq!(27846, run_network(1))
    }

    #[test]
    fn test_part2() {
        assert_eq!(19959, run_network(2))
    }
}
