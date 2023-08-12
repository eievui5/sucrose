mod res {
    include!(concat!(env!("OUT_DIR"), "/res.rs"));
}

fn main() {
    println!("res::npcs::CAT = {:#?}", res::npcs::CAT);
    println!("res::npcs::DOG = {:#?}", res::npcs::DOG);
    println!("res::items::APPLE = {:#?}", res::items::APPLE);
    println!("res::items::PEAR = {:#?}", res::items::PEAR);
}
