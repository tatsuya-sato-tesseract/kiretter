use chrono::{offset::Local, Datelike};
use hdk::{
    api::AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
    EntryValidationData,
};

pub mod birthdate;
pub mod mail_address;
pub mod nationality;
pub mod phone_number;
pub mod physical_address;
pub mod user_name;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
//defines the User struct which is the entry data for user entry.
pub struct User {
    pub agent: Address,
    pub handle: String,
    pub account_level: u32, //TODO: need to have a method on civil proofing to increase the account_level.
    pub joined_year: JoinedYear,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct JoinedYear {
    year: i32,
    month: u32,
    day: u32,
}

impl JoinedYear {
    pub fn date_today() -> Self {
        Self {
            year: Local::today().year(),
            month: Local::today().month(),
            day: Local::today().day(),
        }
    }

    //same case with get_age() function.
    pub fn date_today_dummy() -> Self {
        Self {
            year: 2019,
            month: 8,
            day: 1,
        }
    }
}

pub fn handle_create_user(handle: String) -> ZomeApiResult<Address> {
    let user_data = User {
        agent: AGENT_ADDRESS.clone(),
        handle,
        account_level: 1,
        joined_year: JoinedYear::date_today_dummy(),
    };
    let entry = Entry::App("user".into(), user_data.into()); //create an entry
    let user_entry_address = hdk::commit_entry(&entry)?; //commit the created entry.
    Ok(user_entry_address)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "this contains personal information about the user",
        //sharing is private because this is a personal information and should only live inside the local source chain.
        // although, if we want to allow the user to share some part of their personal info in DHT, then we need to have a
        // way to either allow some part of the User to be public (not sure if this is possible tho) or just create an entry per
        // data point so that we can easily switch from public to private back and forth. Tho this will be way more verbose than
        // having only one entry.
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<User>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data} => {
                    let user = User::from(entry);
                    //make sure that the agent_address of the User struct is the same with the agent_address that signed the entry.
                    if validation_data.sources().contains(&user.agent) {
                        return Ok(())
                    } else {
                        Err("You cannot create an account on behalf of another user".into())
                    }
                },
                _ => {
                    Err("User info cannot be modified or deleted".into()) //temporary only accepts create entry
                }
            }
        }
    )
}
