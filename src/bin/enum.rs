#![allow(unused)]
#[derive(Debug)]

enum IpAddressVersion {
    V4, //they are unit variants of the IpAddressVersion enum.
    V5,
}
enum IpAddressInformation {
    V4(String), //they are unit variants of the IpAddressVersion enum.
    V6(String),
}
enum IpAddressInformationDifferent {
    V4(u8, u8, u8, u8), //they are unit variants of the IpAddressVersion enum.
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: u32,
        y: u32,
        z: u32,
        spatial_orientation: String,
    },
    Write(String),
    Change(u32, u32, u32, String),
}

impl Message {
    fn print_itself(&self) -> () {
        println!("{:#?}", self);
    }
}

enum MyOption<T> {
    None,
    Some(T),
}

fn main() {
    let my_ip_address_version = IpAddressVersion::V4;
    let my_ip_address_version2 = IpAddressVersion::V4;

    let her_ip_info: IpAddressInformation = IpAddressInformation::V4(String::from("127.0.0.1"));
    let her_ip_inf2: IpAddressInformation = IpAddressInformation::V6(String::from("::1"));

    let my_home_ip = IpAddressInformationDifferent::V4(192, 168, 18, 1);

    let m1: Message = Message::Move {
        x: 10,
        y: 20,
        z: 121,
        spatial_orientation: String::from("Right handed coordinated system"),
    };

    println!("{:#?}", m1);
    m1.print_itself();

    //to implement the concept of presence or absence of value rust implement Option enum
    let some_number = Some(5);
    let some_char = Some('e');

    let no_value: MyOption<u32> = MyOption::None;
    let mut no_value: Option<u32> = None; //already have one in prelude
    no_value = Some(4);

    let y: u32 = 12;
    let add = y + no_value.unwrap();
}
