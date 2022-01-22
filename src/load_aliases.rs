pub mod lib;
use lib::{activate_aliases, load_json_file, unpack_alias_list};

fn main() {
    let file_name = "test.json";
    let alias_list = load_json_file(file_name).expect(format!("Unable to load {} as json", file_name).as_str());
    let alias_list = unpack_alias_list(&alias_list).expect("unable to unpack aliases");
    activate_aliases(&alias_list);
}
