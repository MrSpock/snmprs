extern crate env_logger;
use log::{debug, error, info, log_enabled, Level};

use snmp::{Client, ObjectIdentifier, Version, SNMP_PORT};
use std::env;
use std::net::Ipv4Addr;
use std::net::{SocketAddrV4, UdpSocket};

fn main() {
    env_logger::init();
    // take args list
    let args = env::args().collect::<Vec<_>>();
    // create udp socket
    let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not open socket");
    // parse 1st argument as IP address & make connection to host
    let addr: Ipv4Addr = env::args().collect::<Vec<_>>()[1]
        .parse()
        .expect("Not an IP Addr");
    socket
        .connect(SocketAddrV4::new(addr, SNMP_PORT))
        .expect("Failed to connect");
    let mut c = Client::new(Version::V2C, &mut socket);
    // take OID string ("1.3.6") from args
    let soid = args[2].parse::<String>().expect("OID not a string");
    // convert OID -> [1,3,6]
    let void: Vec<u32> = soid.split(".").map(|v| v.parse::<u32>().unwrap()).collect();
    // make ObjectIdentifier
    let mut oids = vec![ObjectIdentifier::new(void).unwrap()];
    // save initial OID value for checking when snmpwalk will "leave" given branch
    let first_oid = &oids.clone()[oids.len() - 1];
    let first_oid_len = first_oid.len();
    // make first snmp_get
    let mut vars = c.get_next(&oids).expect("No data returned");
    // store results here
    let mut result_set = Vec::new();
    // iterate until last octet returnec by getnext is either "higher" (shorter) or deeper/branched
    // which means we shoud finish snmpwalk
    loop {
        debug!("{:?} = {:?}", vars[0].name, vars[0].value);
        // put value to result vec
        result_set.push(vars.clone());
        oids = vars.into_iter().map(|v| v.name).collect();
        vars = c.get_next(&oids).expect("No data returned");
        if &vars.last().unwrap().name.len() < &first_oid_len {
            break;
        }
        let tmp_c = &vars.clone().to_vec();
        let c_oid = &tmp_c.last().unwrap().name;
        info!("Compare {:?} vs {:?}", c_oid, &first_oid[..]);
        if &vars.last().unwrap().name[0..first_oid_len] != &first_oid[..] {
            break;
        }
    }
    // print results
    for r in result_set {
        println!("{:?}", r);
    }
}
