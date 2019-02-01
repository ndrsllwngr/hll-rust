use super::node::*;
use prettytable::format;


pub fn print_current_node_state(node: &Node) {
    let node_string = node.id.clone().to_string();
    let predecessor_string = if let Some(pre) = node.predecessor.clone() {
        pre.get_id().to_string()
    } else {
        "None".to_string()
    };
    let successor_string = node.get_successor().get_id().to_string();

    //Node Info
    let mut node_info_table = table!(["I am Node #", &node_string],
                    ["Predecessor", &predecessor_string],
                    ["My Successor is",  &successor_string]);
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
    let mut storage_table = table!(["key", "value"]);
    for (key, value) in node.storage.data.iter() {
        storage_table.add_row(row![&key.clone().to_string(), value]);
    }
    storage_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);


    let mut state_table = table!(["Node Info", "Successor List", "Finger Table", "Storage"],
                    [node_info_table, successor_list_table, finger_table_table, storage_table]);
    state_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    state_table.printstd();
}