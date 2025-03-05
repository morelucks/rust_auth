use std::collections::HashMap;
fn create_network_list() -> HashMap<String, i32>{
    let mut name_list=HashMap::new();
    name_list.insert(String::from("Base"), 0);
    name_list.insert(String::from("solana"), 1);
    name_list.insert(String::from("lisk"), 3);

    name_list

}
fn looop(name_list:HashMap<String, i32>){
    for (name,sn) in name_list {
        println!("name :{} val {}", name, sn);
        
    }

}
fn main() {
    let name_list=create_network_list() ;
    looop(name_list);
}
