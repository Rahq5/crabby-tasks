mod IpAddressTypesEnums;
use IpAddressTypesEnums::IpAddressType;

#[derive(Debug)]
    struct IpAddr {
        kind: IpAddressType,
        address: String,
    }

fn main (){
    let home = IpAddr {
        kind: IpAddressType::IPv4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddressType::IPv6,
        address: String::from("::1"),
    };

    println!("{:?}",home);// why it doesnt accept that ??
    println!("{:?}",loopback);
}

/* 
output:

IpAddr { kind: IPv4, address: "127.0.0.1" }
IpAddr { kind: IPv6, address: "::1" }
*/