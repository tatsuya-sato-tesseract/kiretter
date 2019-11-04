use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

pub const PHONE_NUMBER_ENTRY: &str = "phonenumber";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct PhoneNumber {
    country_code: String,
    phone_number: String,
}

impl PhoneNumber {
    pub fn new(country_code: String, phone_number: String) -> Self {
        Self {
            country_code,
            phone_number,
        }
    }
}

pub fn handle_create_phone_number(
    country_code: String,
    phone_number: String,
) -> ZomeApiResult<Address> {
    let phone_number_data = PhoneNumber::new(country_code, phone_number);
    let entry = Entry::App(PHONE_NUMBER_ENTRY.into(), phone_number_data.into());
    let physical_address_address = hdk::commit_entry(&entry)?;
    Ok(physical_address_address)
}

pub fn physical_address_definition() -> ValidatingEntryType {
    entry!(
        name: PHONE_NUMBER_ENTRY,
        description: "This is the user's phone number",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<PhoneNumber>| {
            Ok(())
        }
    )
}

#[cfg(test)]
pub mod test {

    #[test]
    fn phone_number_works() {
        let country_code = "+81";
        let phone_number = "9061570847";

        let result = super::PhoneNumber::new(country_code.into(), phone_number.into());

        assert_eq!(
            result,
            super::PhoneNumber {
                country_code: country_code.into(),
                phone_number: phone_number.into(),
            }
        );
    }

}
