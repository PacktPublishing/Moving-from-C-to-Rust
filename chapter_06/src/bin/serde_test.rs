use serde::{Deserialize, Serialize};
use std::fs;

type AnyError = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize, Serialize)]
struct PetInfo {
    kind: String,
    age: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Pet {
    name: String,
    info: PetInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct PetFile {
    pets: Vec<Pet>,
}

fn read_pet_file(filename: &str) -> Result<PetFile, AnyError> {
    let data = std::fs::read_to_string(filename)?;
    let pets = serde_json::from_str::<PetFile>(&data)?;
    return Ok(pets);
}

fn print_pet_data(data: &PetFile) -> Result<(), AnyError> {
    let output = serde_json::to_string(data)?;
    println!("{}", output);
    Ok(())
}

fn print_pet_data_toml(data: &PetFile) -> Result<(), AnyError> {
    let output = toml::to_string(data)?;
    println!("{}", output);
    Ok(())
}

fn main() -> Result<(), AnyError> {
    let mut pets = read_pet_file("pets.json")?;

    for pet in &mut pets.pets {
        pet.info.age += 1;
    }

    println!("{:#?}", pets);

    println!("\nAs JSON:");
    print_pet_data(&pets)?;

    println!("\nAs TOML:");
    print_pet_data_toml(&pets)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct PetInfoOptional {
    kind: String,
    age: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PetInfoWithDefault {
    kind: String,
    #[serde(default)]
    age: usize,
}

#[derive(Debug, Deserialize)]
struct DataWithRename {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DataWithCase {
    test_field: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct DataStrict {
    test_field: String,
}
