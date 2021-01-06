use picopb::PbReader;

fn main() {
    let raw = hex::decode("0a028ebd220857cd9c13d8719af6409982bdb4ed2e5aae01081f12a9010a31747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e54726967676572536d617274436f6e747261637412740a154146a23e25df9a0f6c18729dda9ad1af3b6a1311601215414402a4b64bcccaf59ed0eee49eaeb5d530abce372244a9059cbb00000000000000000000000008eae6b38f64b1fbf53727bb70592533d64f1cdc000000000000000000000000000000000000000000000000000000000000271070ddb3b9b4ed2e9001c0843d").unwrap();

    let mut rd = PbReader::new(&raw);

    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", hex::encode(rd.next_bytes().unwrap()));
    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", hex::encode(rd.next_bytes().unwrap()));
    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", rd.next_varint());
    println!("=> {:?}", rd.next_key());
    // println!("=> {:?}", hex::encode(rd.next_bytes().unwrap()));

    let mut cntr = rd.next_embedded_message().unwrap();
    println!("==> {:?}", cntr.next_key());
    println!("==> {:?}", cntr.next_varint());
    println!("==> {:?}", cntr.next_key());
    // println!("==> {:?}", hex::encode(cntr.next_bytes().unwrap()));

    let mut param = cntr.next_embedded_message().unwrap();
    println!("===> {:?}", param.next_key());
    println!("===> {:?}", param.next_string());
    println!("===> {:?}", param.next_key());
    // println!("=> {:?}", hex::encode(param.next_bytes().unwrap()));

    let mut value = param.next_embedded_message().unwrap();
    println!("====> {:?}", value.next_key());
    println!("====> {:?}", hex::encode(value.next_bytes().unwrap()));
    println!("====> {:?}", value.next_key());
    println!("====> {:?}", hex::encode(value.next_bytes().unwrap()));
    println!("====> {:?}", value.next_key());
    println!("====> {:?}", hex::encode(value.next_bytes().unwrap()));
    //println!("====> {:?}", value.next_key());

    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", rd.next_varint());
    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", rd.next_varint());
    println!("=> {:?}", rd.next_key());
    println!("=> {:?}", rd.next_varint());
    println!("=> {:?}", rd.next_key());
}
