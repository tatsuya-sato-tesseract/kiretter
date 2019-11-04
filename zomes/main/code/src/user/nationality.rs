use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

pub const NATIONALITY_ENTRY: &str = "nationality";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct Nationality {
    nationality: String,
}

impl Nationality {
    pub fn new(nationality: String) -> Self {
        Self { nationality }
    }
}

pub fn handle_create_nationality(nationality: String) -> ZomeApiResult<Address> {
    let nationality_data = Nationality::new(nationality);
    let entry = Entry::App(NATIONALITY_ENTRY.into(), nationality_data.into());
    let nationality_address_address = hdk::commit_entry(&entry)?;
    Ok(nationality_address_address)
}

pub fn nationality_definition() -> ValidatingEntryType {
    entry!(
        name: NATIONALITY_ENTRY,
        description: "This is the user's nationality",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Nationality>| {
            Ok(())
        }
    )
}

#[cfg(test)]
pub mod test {

    #[test]
    fn nationality_works() {
        let nationality = "japanese";

        let result = super::Nationality::new(nationality.into());
        assert_eq!(
            result,
            super::Nationality {
                nationality: "japanese".into()
            }
        );
    }
}
