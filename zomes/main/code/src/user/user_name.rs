use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

const USER_NAME_ENTRY: &str = "user_name";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct UserName {
    pub first_name: String,
    pub last_name: String,
    // pub nationality: String,
    // pub birthdate: BirthDate,
    // pub age: u32,
    // pub physical_address: PhysicalAddress,
    // pub mail_address: String,
    // pub account_level: u32,
    // pub phone_number: PhoneNumber,
}

impl UserName {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
        }
    }
}

pub fn handle_create_user_name(first_name: String, last_name: String) -> ZomeApiResult<Address> {
    let user_name_data = UserName::new(first_name, last_name);
    let entry = Entry::App(USER_NAME_ENTRY.into(), user_name_data.into());
    let user_name_address = hdk::commit_entry(&entry)?;
    Ok(user_name_address)
}

pub fn user_name_definition() -> ValidatingEntryType {
    entry!(
        name: USER_NAME_ENTRY,
        description: "This is the user's physical address",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<UserName>| {
            Ok(())
        }
    )
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
// pub struct BirthDate {
//     birth_year: i32,
//     birth_month: u32,
//     birth_day: u32,
// }

// impl BirthDate {
//     pub fn new(birth_year: i32, birth_month: u32, birth_day: u32) -> Self {
//         Self {
//             birth_year,
//             birth_month,
//             birth_day,
//         }
//     }

//     pub fn get_age(birth_year: i32, birth_month: u32, birth_day: u32) -> u32 {
//         let birthday_dt: Date<Local> =
//             Local.ymd(birth_year.clone(), birth_month.clone(), birth_day.clone());
//         let local_dt: Date<Local> = Local::today();
//         let year_diff: i32 = local_dt.year() - birthday_dt.year();
//         if birthday_dt.month() < local_dt.month() {
//             return year_diff as u32;
//         } else if birthday_dt.month() == local_dt.month() {
//             if birthday_dt.day() >= local_dt.day() {
//                 return year_diff as u32;
//             } else {
//                 return year_diff as u32;
//             }
//         } else {
//             return year_diff as u32;
//         }
//     }

//     //currently using a dummy function becaue using get_age() that uses the Chrono
//     //is causing the diorama to fail when other method of testing (unit test, insomnia) are working fine.
//     pub fn get_age_dummy(birth_year: i32, birth_month: u32, birth_day: u32) -> u32 {
//         // let birthday_dt: Date<Local> =
//         //     Local.ymd(birth_year.clone(), birth_month.clone(), birth_day.clone());
//         let birthday_dummy: (i32, u32, u32) = (birth_year, birth_month, birth_day);
//         let local_dummy: (i32, u32, u32) = (2019, 07, 31);
//         let year_diff: i32 = local_dummy.0 - birthday_dummy.0;
//         if birthday_dummy.1 < local_dummy.1 {
//             return year_diff as u32;
//         } else if birthday_dummy.1 == local_dummy.1 {
//             if birthday_dummy.2 >= birthday_dummy.2 {
//                 return year_diff as u32;
//             } else {
//                 return year_diff as u32;
//             }
//         } else {
//             return year_diff as u32;
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
// pub struct PhysicalAddress {
//     country: String,
//     zip_code: String,
//     street_number_name: String,
//     building_name_floor_roomno: String,
//     city_ward_town_village: String,
//     prefecture: String,
// }

// impl PhysicalAddress {
//     pub fn new(
//         country: String,
//         zip_code: String,
//         street_number_name: String,
//         building_name_floor_roomno: String,
//         city_ward_town_village: String,
//         prefecture: String,
//     ) -> Self {
//         Self {
//             country,
//             zip_code,
//             street_number_name,
//             building_name_floor_roomno,
//             city_ward_town_village,
//             prefecture,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
// pub struct PhoneNumber {
//     country_code: String,
//     phone_number: String,
// }

// impl PhoneNumber {
//     pub fn new(country_code: String, phone_number: String) -> Self {
//         Self {
//             country_code,
//             phone_number,
//         }
//     }
// }

#[cfg(test)]
pub mod test {
    use super::*;

    // //helper function for creating new birthdate
    // fn create_birthdate() -> BirthDate {
    //     let birth_year = 1996;
    //     let birth_month = 1;
    //     let birth_day = 1;
    //     BirthDate::new(birth_year, birth_month, birth_day)
    // }

    // #[test]
    // fn test_birthday_new() {
    //     assert_eq!(
    //         create_birthdate(),
    //         super::BirthDate {
    //             birth_year: 1996,
    //             birth_month: 1,
    //             birth_day: 1
    //         }
    //     );
    // }

    #[test]
    fn test_user_name_new() {
        let first_name = "Tatsuya";
        let last_name = "Sato";
        // let year = 1996;
        // let month = 1;
        // let day = 1;
        // let nationality = "Japanese";
        // let country = "Japan";
        // let zip_code = "1400002";
        // let street_number_name = "1-8-18";
        // let building_name_floor_roomno = "201";
        // let city_ward_town_village = "shinagawa";
        // let prefecture = "Tokyo";
        // let mail_address = "tatsuya.g.sato@yumeville.com";
        // let country_code = "+81";
        // let phone_number = "9061570847";
        // let account_level = 1;

        assert_eq!(
            UserName::new(
                first_name.into(),
                last_name.into(),
                // year,
                // month,
                // day,
                // nationality.into(),
                // country.into(),
                // zip_code.into(),
                // street_number_name.into(),
                // building_name_floor_roomno.into(),
                // city_ward_town_village.into(),
                // prefecture.into(),
                // mail_address.into(),
                // account_level,
                // country_code.into(),
                // phone_number.into(),
            ),
            super::UserName {
                first_name: "Tatsuya".to_string(),
                last_name: "Sato".to_string(),
                // birth_date: create_birthdate(),
                // age: 23,
                // nationality: "Japanese".to_string(),
                // physical_address: super::PhysicalAddress {
                //     country: country.to_string(),
                //     zip_code: zip_code.to_string(),
                //     street_number_name: "1-8-18".to_string(),
                //     building_name_floor_roomno: "201".to_string(),
                //     city_ward_town_village: "shinagawa".to_string(),
                //     prefecture: "Tokyo".to_string(),
                // },
                // mail_address: "tatsuya.g.sato@yumeville.com".to_string(),
                // account_level: 1,
                // phone_number: super::PhoneNumber {
                //     country_code: "+81".to_string(),
                //     phone_number: "9061570847".to_string(),
                // }
            }
        );
    }

    // #[test]
    // fn test_age_new() {
    //     let birth_year = 1996;
    //     let birth_month = 1;
    //     let birth_day = 1;
    //     assert_eq!(BirthDate::get_age(birth_year, birth_month, birth_day), 23);
    // }

}
