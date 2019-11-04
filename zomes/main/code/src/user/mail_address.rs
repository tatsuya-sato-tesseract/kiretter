use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

pub const MAIL_ADDRESS_ENTRY: &str = "mail_address";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct MailAddress {
    mail_address: String,
}

impl MailAddress {
    pub fn new(mail_address: String) -> Self {
        Self { mail_address }
    }
}

pub fn handle_create_mail_address(mail_address: String) -> ZomeApiResult<Address> {
    let mail_address_data = MailAddress::new(mail_address);
    let entry = Entry::App(MAIL_ADDRESS_ENTRY.into(), mail_address_data.into());
    let mail_address_address = hdk::commit_entry(&entry)?;
    Ok(mail_address_address)
}

pub fn mail_address_definition() -> ValidatingEntryType {
    entry!(
        name: MAIL_ADDRESS_ENTRY,
        description: "This is the user's email address",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MailAddress>| {
            Ok(())
        }
    )
}

#[cfg(test)]
pub mod test {

    #[test]
    fn email_works() {
        let email = "tatsuya.g.sato@yumeville.com";

        let result = super::MailAddress::new(email.into());
        assert_eq!(
            result,
            super::MailAddress {
                mail_address: "tatsuya.g.sato@yumeville.com".into()
            }
        );
    }
}
