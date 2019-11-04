#![feature(try_from, proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType, error::ZomeApiResult, holochain_core_types::entry::Entry,
};

// use hdk::holochain_json_api::{error::JsonError, json::JsonString};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

mod user;

// see https://developer.holochain.org/api/0.0.18-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[zome]
pub mod my_zome {

    #[genesis]
    pub fn genesis() {
        Ok(())
    }

    #[entry_def]
    pub fn user_entry_def() -> ValidatingEntryType {
        //defines the user entry
        user::definition()
    }

    #[entry_def]
    pub fn birthdate_entry_def() -> ValidatingEntryType {
        user::birthdate::birthdate_definition()
    }

    #[entry_def]
    pub fn physical_address_entry_def() -> ValidatingEntryType {
        user::physical_address::physical_address_definition()
    }

    #[entry_def]
    pub fn mail_address_entry_def() -> ValidatingEntryType {
        user::mail_address::mail_address_definition()
    }

    #[entry_def]
    pub fn nationality_entry_def() -> ValidatingEntryType {
        user::nationality::nationality_definition()
    }

    #[entry_def]
    pub fn phone_number_entry_def() -> ValidatingEntryType {
        user::phone_number::physical_address_definition()
    }

    #[entry_def]
    pub fn user_name_entry_def() -> ValidatingEntryType {
        user::user_name::user_name_definition()
    }

    #[zome_fn("hc_public")]
    pub fn create_user(handle: String) -> ZomeApiResult<Address> {
        user::handle_create_user(handle)
    }

    #[zome_fn("hc_public")]
    pub fn create_user_birthdate(
        birth_year: i32,
        birth_month: u32,
        birth_day: u32,
    ) -> ZomeApiResult<Address> {
        user::birthdate::handle_create_birthdate(birth_year, birth_month, birth_day)
    }

    #[zome_fn("hc_public")]
    pub fn create_user_physical_address(
        country: String,
        zip_code: String,
        street_number_name: String,
        building_name_floor_roomno: String,
        city_ward_town_village: String,
        prefecture: String,
    ) -> ZomeApiResult<Address> {
        user::physical_address::handle_create_physical_address(
            country,
            zip_code,
            street_number_name,
            building_name_floor_roomno,
            city_ward_town_village,
            prefecture,
        )
    }

    #[zome_fn("hc_public")]
    pub fn create_user_mail_address(mail_address: String) -> ZomeApiResult<Address> {
        user::mail_address::handle_create_mail_address(mail_address)
    }

    #[zome_fn("hc_public")]
    pub fn create_user_nationality(nationality: String) -> ZomeApiResult<Address> {
        user::nationality::handle_create_nationality(nationality)
    }

    #[zome_fn("hc_public")]
    pub fn create_user_phone_number(
        country_code: String,
        phone_number: String,
    ) -> ZomeApiResult<Address> {
        user::phone_number::handle_create_phone_number(country_code, phone_number)
    }

    #[zome_fn("hc_public")]
    pub fn create_user_name(first_name: String, last_name: String) -> ZomeApiResult<Address> {
        user::user_name::handle_create_user_name(first_name, last_name)
    }

    #[zome_fn("hc_public")]
    pub fn get_user_entry(user_address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&user_address)
    }

}
