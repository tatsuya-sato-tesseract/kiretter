const path = require("path");
const tape = require("tape");

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require("@holochain/diorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/kiretter-test.dna.json");
const dna = Diorama.dna(dnaPath, "kiretter-test");

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require("tape")),
  middleware: backwardCompatibilityMiddleware
});

diorama.registerScenario("can create user", async (s, t, { alice }) => {
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const create_user_result = await alice.call("main", "create_user", {
    handle: "Tats"
    // first_name: "Tatsuya",
    // last_name: "Sato",
    // birth_year: 1996,
    // birth_month: 1,
    // birth_day: 1,
    // nationality: "Japanese",
    // country: "japan",
    // zip_code: "1400002",
    // street_number_name: "1-8-18",
    // building_name_floor_roomno: "201",
    // city_ward_town_village: "Shinagawa",
    // prefecture: "Tokyo",
    // mail_address: "tatsuya.g.sato@yumeville.com",
    // country_code: "+81",
    // phone_number: "9061570847"
  });
  // check for equality of the actual and expected result
  t.deepEqual(create_user_result.Ok.length, 46);

  const get_user_result = await alice.call("main", "get_user_entry", {
    user_address: create_user_result.Ok
  });
  console.log(get_user_result);
  t.deepEqual(get_user_result.Err, undefined);

  const create_birthdate_result = await alice.call(
    "main",
    "create_user_birthdate",
    {
      birth_year: 1996,
      birth_month: 1,
      birth_day: 1
    }
  );
  // check for equality of the actual and expected result
  t.deepEqual(create_birthdate_result.Ok.length, 46);

  const create_physical_address_result = await alice.call(
    "main",
    "create_user_physical_address",
    {
      country: "japan",
      zip_code: "1400002",
      street_number_name: "1-8-18",
      building_name_floor_roomno: "201",
      city_ward_town_village: "Shinagawa",
      prefecture: "Tokyo"
    }
  );
  // check for equality of the actual and expected result
  t.deepEqual(create_physical_address_result.Ok.length, 46);

  const create_nationality_result = await alice.call(
    "main",
    "create_user_nationality",
    { nationality: "japanese" }
  );
  // check for equality of the actual and expected result
  t.deepEqual(create_nationality_result.Ok.length, 46);

  const create_mail_address_result = await alice.call(
    "main",
    "create_user_mail_address",
    { mail_address: "tatsuya.g.sato@yumeville.com" }
  );
  // check for equality of the actual and expected result
  t.deepEqual(create_mail_address_result.Ok.length, 46);

  const create_phone_number_result = await alice.call(
    "main",
    "create_user_phone_number",
    { country_code: "+81", phone_number: "9061570847" }
  );
  // check for equality of the actual and expected result
  t.deepEqual(create_phone_number_result.Ok.length, 46);

  const create_user_name_result = await alice.call("main", "create_user_name", {
    first_name: "Tatsuya",
    last_name: "Sato"
  });
  // check for equality of the actual and expected result
  t.deepEqual(create_user_name_result.Ok.length, 46);
});

diorama.run();
