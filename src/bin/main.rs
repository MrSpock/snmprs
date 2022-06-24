use snmp::{Client, ObjectIdentifier, SNMP_PORT, Version};
use std::net::{SocketAddrV4, UdpSocket};
use std::net::Ipv4Addr;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not open socket");
    let addr: Ipv4Addr = env::args().collect::<Vec<_>>()[1].parse().expect("Not an IP Addr");
    socket.connect(SocketAddrV4::new(addr, SNMP_PORT)).expect("Failed to connect");
    let mut c = Client::new(Version::V2C, &mut socket);
    let soid = args[2].parse::<String>().expect("OID not a string");
    let void: Vec<u32> = soid.split(".").map(|v| v.parse::<u32>().unwrap()).collect();
    let mut oids = vec![ObjectIdentifier::new(void).unwrap()];
    //let mut oids = vec![oid! [1,3,6,1,2,1,1,9,1,2]];
    let first_oid = &oids.clone()[oids.len()-1];
    let first_oid_len = first_oid.len();
    //println!("Initial OID: {:?} LENGHT:{}", first_oid,first_oid_len);
    let mut vars = c.get_next(&oids).expect("No data returned");
    //println!("First next value: {:?}", vars.last().unwrap().name);
    let mut result_set = Vec::new();
    //while &vars.last().unwrap().name[0..first_oid_len] == &first_oid[..] {
    loop {
        result_set.push(vars[0].clone());
        //println!("{}", vars[0]);
        oids = vars.into_iter().map(|v| v.name).collect();
        //println!("Current OID: {:?} -> Value:{}",oids,value);
        //last_oid = &oids[oids.len()-1];
        vars = c.get_next(&oids).expect("No data returned");
        if &vars.last().unwrap().name.len() < &first_oid_len {
            break;
        }
        if &vars.last().unwrap().name[0..first_oid_len] != &first_oid[..] {
            break;
        }

    }
    for r in result_set {
        println!("{}", r);
    }
}
//fn build_oid(oid_base: Vec<u32>, oid_end: Vec<u8>) -> Vec<u32> {
//    [oid_base, oid_end.iter().map(|x| *x as u32).collect()].concat()
//}

