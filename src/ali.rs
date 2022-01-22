pub mod lib;
use lib::*;

fn main() 
{   
    // read the json file and parse it to a manageable format
    let command = std::env::args().nth(1).expect("no command given, need atleast one argument");
    let file_name = "test.json";
    let alias_list = load_json_file(&file_name).expect("Unable to read the json file");
    let mut alias_list = unpack_alias_list(&alias_list).expect("Json file format invalid");

    // Handle arguments and select function
    match String::as_str(&command) {
        "list" => {
            // list code goes here
            for (_, entry) in alias_list {
                println!("Description: {}", entry.description);
                println!("{} => {}\n\n", entry.name, entry.command);
            }
        },

        "search" => {
            // search code goes here
            let mut search_tags : Vec<String> = Vec::new();
            for argument in std::env::args().collect::<Vec<String>>().iter().skip(2) {
                search_tags.push(String::from(argument));
            } 
            for name in find_entries_by_tag(&alias_list, &search_tags) {
                if let Some(entry) = alias_list.get(name) {
                    println!("Description: {}", entry.description);
                    println!("{} => {}\n\n", entry.name, entry.command);
                } else {
                    panic!("Something has gone terribly wrong")
                }
            }
        },

        "add" => {
            // Read user input
            println!("Enter name for alias: ");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).expect("Error while reading name");
            name.pop(); // pop off the newline char

            println!("Enter command for alias: ");
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).expect("Error while reading command");
            command.pop();

            println!("Enter description of alias: ");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description).expect("Error while reading description");
            description.pop(); 

            println!("Enter tags for alias: ");
            let mut tags = String::new();
            std::io::stdin().read_line(&mut tags).expect("Error while reading tags");
            tags.pop();
            let tags = tags.split(' ');
            let tags = tags.collect::<Vec<&str>>();

            // push the new entry
            if alias_list.contains_key(name.as_str()) {
                println!("Cannot add alias. Already have an alias by that name");
            }
            else {
                alias_list.insert(
                    name.as_str(),
                    Entry{
                        name : name.as_str(), 
                        command : command.as_str(), 
                        description : description.as_str(), 
                        tags : tags
                    }
                );

                // save the new alias_list to file
                let alias_list = pack_alias_list(&alias_list);
                write_json_file(file_name, alias_list);
            }

        },

        "remove" => {
            let command_name = std::env::args().nth(2).expect("no command name given");
            if let Some(entry) = alias_list.get(command_name.as_str()) {
                println!("Description: {}", entry.description);
                println!("{} => {}", entry.name, entry.command);
                println!("Are you sure you want to delete {}? (y/n)", entry.name);
                let mut yes_or_no = String::new();
                yes_or_no.pop(); // remove trailing endline

                while !(yes_or_no == "y" || yes_or_no == "n") {
                    // Repeat taking input until we get valid response
                    yes_or_no = String::new();
                    std::io::stdin().read_line(&mut yes_or_no).expect("Error while reading input");
                    yes_or_no.pop(); // remove trailing endline
                } 

                // handle yes/no cases
                if yes_or_no == "y" {
                    println!("Confirmed. Deleting {}", entry.name);
                    alias_list.remove(entry.name);
                    let alias_list = pack_alias_list(&alias_list);
                    write_json_file(file_name, alias_list);
                } else if (yes_or_no == "n") {
                    println!("Not deleting {}", entry.name);
                }
            } else {
                println!("No entry by the name given");
            }
        },

        other => {
            println!("'{}' is not a valid command, try 'list', 'add', 'search' or 'remove'", other)
        }
    }
}


