use pcap::{Device, Capture};
use etherparse::{PacketHeaders, SlicedPacket};


fn main() {
    println!("Hello, world!");
    // let device_list = Device::list().unwrap();
    // println!("{:#?}", device_list);
    let main_device = Device::lookup().unwrap().unwrap();
    println!("{:#?}", main_device);

    let mut cap = Capture::from_device(main_device).unwrap()
                .promisc(true)
                .snaplen(5000)
                .open()
                .unwrap();
    while let Ok(packet) = cap.next_packet(){
        //println!("Received packet: {:?}", packet);
        match SlicedPacket::from_ethernet(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("Value of ip: {:?}", value.ip)
            }
        }
    }
}
