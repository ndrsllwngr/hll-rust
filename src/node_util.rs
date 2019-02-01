use super::node::*;
use prettytable::format;


pub fn print_current_node_state(node: &Node) {
    let mut pre_id_string = "None".to_string();
    let mut pre_ip_string = "None".to_string();

    if let Some(pre) = node.predecessor.clone() {
        pre_id_string = pre.get_id().to_string();
        pre_ip_string = pre.get_ip_addr().to_string();
    }
    let succ_id_string = node.get_successor().get_id().to_string();
    let succ_ip_string = node.get_successor().get_ip_addr().to_string();

    //Node Info
    let mut node_info_table = table!(["", "id", "ip"],
                    [Brb => "Self", &node.id.clone().to_string(), &node.ip_addr.clone().to_string()],
                    ["Predecessor", &pre_id_string, &pre_ip_string],
                    ["Successor",  &succ_id_string, &succ_ip_string]);
    node_info_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);


    // Successor List
    let mut successor_list_table = table!(["i", "node_id"]);
    for i in 0..node.successor_list.len() {
        let succ = &node.successor_list[i];
        successor_list_table.add_row(row![&i.to_string(), &succ.get_id().clone().to_string()]);
    }
    successor_list_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);


    // Finger Table
    let mut finger_table_table = table!(["i", "finger_id", "node_id"]);
    for i in 0..node.finger_table.entries.len() {
        let entry = &node.finger_table.entries[i];
        finger_table_table.add_row(row![&i.to_string(), &entry.id.clone().to_string(), &entry.node.get_id().to_string()]);
    }
    finger_table_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    // Storage
    let mut storage_table = table!(["key_id", "key", "value"]);
    for (key_id, dht_entry) in node.storage.data.iter() {
        storage_table.add_row(row![&key_id.clone().to_string(), &dht_entry.key, &dht_entry.value]);
    }
    storage_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);


    let mut state_table = table!(["Node Info", "Successor List", "Finger Table", "Storage"],
                    [node_info_table, successor_list_table, finger_table_table, storage_table]);
    state_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    info!("\n{}", state_table);
}