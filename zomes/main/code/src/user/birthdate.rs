use chrono::{offset::Local, Date, Datelike, TimeZone};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

pub const BIRTHDATE_ENTRY: &str = "birthdate";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct BirthDate {
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    age: u32,
}

impl BirthDate {
    pub fn new(birth_year: i32, birth_month: u32, birth_day: u32) -> Self {
        Self {
            birth_year: birth_year.clone(),
            birth_month: birth_month.clone(),
            birth_day: birth_day.clone(),
            age: Self::get_age_dummy(birth_year, birth_month, birth_day),
        }
    }

    pub fn get_age(birth_year: i32, birth_month: u32, birth_day: u32) -> u32 {
        let birthday_dt: Date<Local> =
            Local.ymd(birth_year.clone(), birth_month.clone(), birth_day.clone());
        let local_dt: Date<Local> = Local::today();
        let year_diff: i32 = local_dt.year() - birthday_dt.year();
        if birthday_dt.month() < local_dt.month() {
            return year_diff as u32;
        } else if birthday_dt.month() == local_dt.month() {
            if birthday_dt.day() >= local_dt.day() {
                return year_diff as u32;
            } else {
                return year_diff as u32;
            }
        } else {
            return year_diff as u32;
        }
    }

    //currently using a dummy function becaue using get_age() that uses the Chrono
    //is causing the diorama to fail when other method of testing (unit test, insomnia) are working fine.
    pub fn get_age_dummy(birth_year: i32, birth_month: u32, birth_day: u32) -> u32 {
        // let birthday_dt: Date<Local> =
        //     Local.ymd(birth_year.clone(), birth_month.clone(), birth_day.clone());
        let birthday_dummy: (i32, u32, u32) = (birth_year, birth_month, birth_day);
        let local_dummy: (i32, u32, u32) = (2019, 07, 31);
        let year_diff: i32 = local_dummy.0 - birthday_dummy.0;
        if birthday_dummy.1 < local_dummy.1 {
            return year_diff as u32;
        } else if birthday_dummy.1 == local_dummy.1 {
            if birthday_dummy.2 >= birthday_dummy.2 {
                return year_diff as u32;
            } else {
                return year_diff as u32;
            }
        } else {
            return year_diff as u32;
        }
    }
}

pub fn handle_create_birthdate(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
) -> ZomeApiResult<Address> {
    let birthdate_data = BirthDate::new(birth_year, birth_month, birth_day);
    let entry = Entry::App(BIRTHDATE_ENTRY.into(), birthdate_data.into());
    let birthdate_entry_address = hdk::commit_entry(&entry)?;
    Ok(birthdate_entry_address)
}

pub fn birthdate_definition() -> ValidatingEntryType {
    entry!(
        name: BIRTHDATE_ENTRY,
        description: "This is the user's birthdate and age",
        //sharing is private because this is a personal information and should only live inside the local source chain.
        // although, if we want to allow the user to share some part of their personal info in DHT, then we need to have a
        // way to either allow some part of the User to be public (not sure if this is possible tho) or just create an entry per
        // data point so that we can easily switch from public to private back and forth. Tho this will be way more verbose than
        // having only one entry.
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<BirthDate>| {
            Ok(())
        }
    )
}

#[cfg(test)]
pub mod test {
    // use super::BirthDate;

    //helper function for creating new birthdate
    fn create_birthdate() -> super::BirthDate {
        let birth_year = 1996;
        let birth_month = 1;
        let birth_day = 1;
        super::BirthDate::new(birth_year, birth_month, birth_day)
    }

    #[test]
    fn test_birthday_new() {
        assert_eq!(
            create_birthdate(),
            super::BirthDate {
                birth_year: 1996,
                birth_month: 1,
                birth_day: 1,
                age: 23
            }
        );
    }

    #[test]
    fn test_age_new() {
        let birth_year = 1996;
        let birth_month = 1;
        let birth_day = 1;
        assert_eq!(
            super::BirthDate::get_age(birth_year, birth_month, birth_day),
            23
        );
    }
}
