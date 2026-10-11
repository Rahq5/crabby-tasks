
fn main (){
    let ip1 = IpType::IPv6;
    process(ip1);
}


fn process (ip:IpType){
    println!("Received: {:#?}", ip);
    ip.call();
    
}


// output expected:
// 
#[derive(Debug)]

enum IpType{
    IPv4,
    IPv6
}

impl IpType{
    fn call(&self){
        match self {
            IpType::IPv4 => println!("it's ipv4 type"),
            IpType::IPv6 => println!("it's ipv6 type")
        }
    }
 }

#[derive(Debug)]
struct test{
    name:String,
    age: i32,
    gender: bool
}

#[derive(Debug)]
struct device{

    name: String,
    ip: IpType,
    something:String


}

#[derive(Debug)]
enum size{
	big,
	med,
	small,
	none
}



/* 
output:

IpAddr { kind: IPv4, address: "127.0.0.1" }
IpAddr { kind: IPv6, address: "::1" }
*/