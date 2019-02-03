use super::node::*;
use prettytable::format;
use colored::*;


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
    let mut node_info_table = table!(["Descr.".italic().yellow(), "ID".italic().yellow(), "SocketAddr".italic().yellow()],["", "", ""],
                    ["Predecessor", &pre_id_string, &pre_ip_string],
                    ["Self".green(), &node.id.clone().to_string().green(), &node.ip_addr.to_string().green()],
                    ["Successor",  &succ_id_string, &succ_ip_string]);
    // node_info_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    node_info_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);


    // Successor List
    let mut successor_list_table = table!(["#".italic().yellow(), "ID".italic().yellow(), "SocketAddr".italic().yellow()],["", "", ""]);
    for i in 0..node.successor_list.len() {
        let succ = &node.successor_list[i];
        successor_list_table.add_row(row![r -> &i.to_string(), &succ.get_id().clone().to_string(), &succ.get_ip_addr().clone().to_string()]);
    }
    // successor_list_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    successor_list_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);


    // Finger Table
    let mut finger_table_table = table!(["#".italic().yellow(), "Finger".italic().yellow(), "Node".italic().yellow()],["", "", ""]);
    for i in 0..node.finger_table.entries.len() {
        let entry = &node.finger_table.entries[i];
        finger_table_table.add_row(row![r -> &i.to_string(), &entry.id.clone().to_string(), &entry.node.get_id().to_string()]);
    }
    // finger_table_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    finger_table_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);

    // Storage
    let mut storage_table = table!(
        ["ID ".italic().yellow(), "Key".italic().yellow(), "Value".italic().yellow()],
        ["", "", ""]
        );
    for (key_id, dht_entry) in node.storage.data.iter() {
        storage_table.add_row(row![&key_id.clone().to_string(), &dht_entry.key, &dht_entry.value]);
    }
    storage_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);


    let mut state_table = table!(
                    ["> Node information ".black().on_white(), "> Successor list ".black().on_white(), "> Finger table ".black().on_white(), "> Storage ".black().on_white()],
                    ["", "", "", ""],
                    [node_info_table, successor_list_table, finger_table_table, storage_table]);
    state_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    // state_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let mut storage_logs_table = table!(
                    ["> Storage logs ".black().on_white()],
                    [""],
                    [node.storage.get_last_log_entry()]);
    storage_logs_table.set_format(*format::consts::FORMAT_BORDERS_ONLY);

    info!("\n\n{}{}", state_table, storage_logs_table);
}