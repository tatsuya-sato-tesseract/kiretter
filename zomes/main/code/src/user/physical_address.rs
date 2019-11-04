use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

pub const PHYSICAL_ADDRESS_ENTRY: &str = "physical_address";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct PhysicalAddress {
    country: String,
    zip_code: String,
    street_number_name: String,
    building_name_floor_roomno: String,
    city_ward_town_village: String,
    prefecture: String,
}

impl PhysicalAddress {
    pub fn new(
        country: String,
        zip_code: String,
        street_number_name: String,
        building_name_floor_roomno: String,
        city_ward_town_village: String,
        prefecture: String,
    ) -> Self {
        Self {
            country,
            zip_code,
            street_number_name,
            building_name_floor_roomno,
            city_ward_town_village,
            prefecture,
        }
    }
}

pub fn handle_create_physical_address(
    country: String,
    zip_code: String,
    street_number_name: String,
    building_name_floor_roomno: String,
    city_ward_town_village: String,
    prefecture: String,
) -> ZomeApiResult<Address> {
    let physical_address_data = PhysicalAddress::new(
        country,
        zip_code,
        street_number_name,
        building_name_floor_roomno,
        city_ward_town_village,
        prefecture,
    );
    let entry = Entry::App(PHYSICAL_ADDRESS_ENTRY.into(), physical_address_data.into());
    let physical_address_address = hdk::commit_entry(&entry)?;
    Ok(physical_address_address)
}

pub fn physical_address_definition() -> ValidatingEntryType {
    entry!(
        name: PHYSICAL_ADDRESS_ENTRY,
        description: "This is the user's physical address",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<PhysicalAddress>| {
            Ok(())
        }
    )
}

#[cfg(test)]
pub mod test {

    #[test]
    fn physical_address_works() {
        let country = "test";
        let zip_code = "test";
        let street_number_name = "test";
        let building_name_floor_roomno = "test";
        let city_ward_town_village = "test";
        let prefecture = "test";

        let result = super::PhysicalAddress::new(
            country.into(),
            zip_code.into(),
            street_number_name.into(),
            building_name_floor_roomno.into(),
            city_ward_town_village.into(),
            prefecture.into(),
        );

        assert_eq!(
            result,
            super::PhysicalAddress {
                country: country.into(),
                zip_code: zip_code.into(),
                street_number_name: street_number_name.into(),
                building_name_floor_roomno: building_name_floor_roomno.into(),
                city_ward_town_village: city_ward_town_village.into(),
                prefecture: prefecture.into(),
            }
        );
    }

}
