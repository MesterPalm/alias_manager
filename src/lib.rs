extern crate json;

use std::fs; //file system operations
use json::*;
use std::collections::HashMap;
use std::process::Command;

pub struct Entry<'a> 
{
    pub name : &'a str,
    pub command : &'a str,
    pub description : &'a str,
    pub tags : Vec<&'a str>
}

// ~~~ Packing and Unpacking from json to typed ~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn unpack_entry (entry : &JsonValue) -> std::result::Result<Entry, &'static str> 
{
    //Unpack the fields of the entry
    if let (Some(name), 
            Some(command), 
            Some(description),
            JsonValue::Array(packed_tags)) 
            = 
            (entry["name"].as_str(), 
             entry["command"].as_str(), 
             entry["description"].as_str(),
             &entry["tags"]) 
    {

        // unpack all the tags
        let mut tags : Vec<&str> = Vec::new();
        for elem in packed_tags {
            match elem.as_str() {
                Some(tag) => tags.push(tag),
                None => return Err("Unable to unpack entry, a tag is not string")
            } 
        }
        // Return Ok if all fields matched the entry formulation
        return Ok(Entry{name, command, description, tags})
    } else {
        // Some field did not follow the formulation
        return Err("Unable to unpack entry");
    }
}

pub fn pack_entry(entry : &Entry) -> JsonValue 
{
    /// Convert an Entry to
    /// the equivalent JsonValue
    let mut tags = JsonValue::new_array();
    let mut packed_entry = JsonValue::new_object();
    // pack the tags
    for tag in &entry.tags {
        tags.push(JsonValue::String(tag.to_string()));;
    }
    // set the fields
    packed_entry["name"] = JsonValue::String(entry.name.to_string());
    packed_entry["command"] = JsonValue::String(entry.command.to_string());
    packed_entry["description"] = JsonValue::String(entry.description.to_string());
    packed_entry["tags"] = tags;
    return packed_entry;
}

pub fn unpack_alias_list (alias_list : &JsonValue) -> std::result::Result<HashMap<&str, Entry>, &'static str> 
{
    /// Convert the alias_list from the JsonValue format to
    /// the equivalent vector of entry format
    if let JsonValue::Array(alias_list) = alias_list {
        let mut entries : HashMap<&str, Entry> = HashMap::new();
        for entry in alias_list {
            let entry = unpack_entry(entry)?;
            entries.insert(
                entry.name,
                entry
            );
        }
        return Ok(entries);
    } else {
        return Err("Unable to unpack alias_list because it is not a JsonValue::Array");
    }
}

pub fn pack_alias_list (alias_list : &HashMap<&str, Entry>) -> JsonValue 
{
    /// convert the alias_list as a vector of Entry
    /// to the equivalent JsonValue form
    let mut wrapped_alist = JsonValue::new_array();
    for (_, entry) in alias_list {
        wrapped_alist.push(pack_entry(entry));
    }
    return wrapped_alist;
}

// ~~~ Manipulating vectors of entries ~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn find_entries_by_tag<'a>  (
    alias_list : &HashMap<&'a str, Entry>, 
    search_tags : &Vec<String>
) -> Vec<&'a str>
{
    /// returns the index of all alias entries in 
    /// the alias_list that contain a tag that matches 
    /// one of the provided tags
    
    let mut matched_entries = Vec::<&str>::new();
    for (name, entry) in alias_list {
        for search_tag in search_tags {
            for entry_tag in &entry.tags {
                if entry_tag == search_tag {
                    matched_entries.push(name);
                }
            }
        } 
    }
    return matched_entries;
}

pub fn add_entry (
    alias_list : &mut JsonValue, 
    name : String,
    cmd : String, 
    description : String, 
    tags : Vec<String>
)
{
    /// Add an entry to the alias_list
    /// that contains the provided tags, definitions and tags

    // setup
    let mut new_entry = JsonValue::new_object();
    let tags = 
        tags.iter()
        .map(|val| { JsonValue::String(val.to_string()) })
        .collect::<Vec<_>>(); 

    // Create the new alias_list entry
    new_entry.insert(
        "name",
        JsonValue::String(name)
    );
    new_entry.insert(
        "command",
        JsonValue::String(cmd)
    );
    new_entry.insert(
        "description",
        JsonValue::String(description)
    );
    
    new_entry.insert(
        "tags",
        JsonValue::Array(tags)
    );

    // Insert the new entry
    alias_list.push(new_entry);
}

// ~~~ IO-functionality ~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~

pub fn load_json_file(file_name : &str) ->  std::result::Result<JsonValue, Box<dyn std::error::Error>>
{ 
    /// Tries to load the file with the name given
    /// by file_name and returns error if it failed
    /// or the parsed JsonValue
    let file_contents =  fs::read_to_string(file_name)?;
    return Ok(json::parse(&file_contents)?);
}

pub fn write_json_file (file_name : &str, json_object : JsonValue) -> std::io::Result<()>
{
    /// Writes the JsonValue in json_object as a json to file named file_name
    /// returns error if write operation failed
    let content = stringify_pretty(json_object, 4);
    fs::write(file_name, content)?;
    return Ok(());
}

pub fn write_alias_file (
    file_name : &str, 
    alias_list : &HashMap<&str, Entry>
) -> std::io::Result<()>
{
    let mut content : String = String::new();
    for (_, entry) in alias_list {
        content.push_str(
            format!("alias {}=\"{}\"\n", entry.name, entry.command).as_str()
            );
    }
    fs::write(file_name, content)?;
    return Ok(());
}
