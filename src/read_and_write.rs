use std::{ fs, fs::File, fs::remove_dir_all };
use std::io::{ BufReader, BufRead, BufWriter, Write, stdin };
use std::path::{ Path };

use crate::prompts::{
    COMPLETE_PROMPT,
    DELETE_PROMPT
};

use crate::print_utils::{
    print_bills,
    print_update_bills,
    print_deleted_all_data_deleted,
    print_cannot_delete_data
};

pub static MONTHLY_INCOME_PREFIX: &str = "monthly_income = ";

static DATA_DIR: &str = "data";

static PROFILE_FILE_NAME: &str = "profile.txt";
static BILLS_FILE_NAME: &str = "bills.txt";

pub static PRICE_VALUE_DELIMITER: char = '£';

lazy_static! {
    pub static ref PROFILE_PATH: String = format!("{}/{}", DATA_DIR, PROFILE_FILE_NAME);
    pub static ref BILLS_PATH: String = format!("{}/{}", DATA_DIR, BILLS_FILE_NAME);
}

// TODO remove and replace usages with get_file
pub fn read_file (path: &str) -> String {
    let file = fs::read_to_string(path).expect("Unable to open file");

    return file;
}

pub fn read_bills () -> String {
    read_file(&BILLS_PATH.to_owned())
}

pub fn file_exists (path: &str) -> bool {
    Path::new(path).is_file()
} 

pub fn profile_exists () -> bool {
    file_exists(&PROFILE_PATH.to_owned())
}

pub fn bills_file_exists () -> bool {
    file_exists(&BILLS_PATH.to_owned())
}

pub fn get_file (path: &str) -> BufReader<File> {
    let file = File::open(path.to_owned()).expect("Unable to open file");
    let buffer = BufReader::new(file);

    return buffer;
}

pub fn get_profile_file () -> BufReader<File> {
    get_file(&PROFILE_PATH.to_owned())
}

fn safe_create_directory (directory: &str) {
    if !Path::new(directory).exists() {
        fs::create_dir(directory).expect("Unable to create dir");
    }
}

pub fn set_monthly_income (profile_monthly_income: &String) {
    safe_create_directory(DATA_DIR);

    let file = File::create(PROFILE_PATH.to_owned()).expect("Unable to create file");
    let mut file = BufWriter::new(file);

    let formatted_profile_monthly_income = if profile_monthly_income.contains("£") {
        profile_monthly_income.to_string()
    } 
    else {
        format!("{}{}", PRICE_VALUE_DELIMITER, profile_monthly_income)
    };

    let profile_monthly_income_string = format!("{}{}", MONTHLY_INCOME_PREFIX, formatted_profile_monthly_income);

    file.write_all(profile_monthly_income_string.as_bytes()).expect("Unable to write data");
}

pub fn get_monthly_income_from_profile () -> f32 {
    let file = get_profile_file();

    let mut monthly_income = String::new();

    for line in file.lines() {
        let line = line.expect("Unable to read line");
        
        if line.starts_with(MONTHLY_INCOME_PREFIX) {
            monthly_income = line.trim().splitn(2, PRICE_VALUE_DELIMITER).last().unwrap_or("0").to_string();
        }
    }
    return monthly_income.parse().unwrap();
}

pub fn input_bills() {
    let mut input = String::new();
        
    stdin().read_line(&mut input).unwrap();

    let formatted_input = input.trim();

    let delete_input: String = format!("{} ", DELETE_PROMPT);

    if formatted_input.contains(&delete_input) {
        let bill_name = formatted_input.split(&delete_input).last().unwrap().to_string();

        delete_bill(&bill_name);
    }
    else if formatted_input != COMPLETE_PROMPT {
        set_bill(&input);
    }

    if formatted_input != COMPLETE_PROMPT || formatted_input == DELETE_PROMPT {
        print_bills(read_bills(), None);
        print_update_bills();
        
        input_bills();
    }
}

pub fn create_bills_file () {
    safe_create_directory(DATA_DIR);

    if !Path::new(&BILLS_PATH.to_owned()).is_file() {
        File::create(BILLS_PATH.to_owned()).expect("Unable to create file");
    }
}

fn set_bill (bill: &String) -> String {
    let buffer = get_file(&BILLS_PATH.to_owned());
    let mut bills = String::new();

    let bill_type = bill.trim().rsplitn(2, ' ').last().unwrap_or("");

    for line in buffer.lines() {
        let line = line.expect("Unable to read line");

        let line_type = line.trim().rsplitn(2, ' ').last().unwrap_or("");

        if line_type.to_lowercase() != bill_type.to_lowercase() {
            bills.push_str(&line);
            bills.push_str("\n");
        }
    }

    let formatted_bill = if bill.contains("£") {
        bill.to_string()
    }
    else {
        let bill_amount = bill.trim().rsplitn(2, ' ').next().unwrap();

        format!("{} {}{}", bill_type, PRICE_VALUE_DELIMITER, bill_amount)
    };

    bills.push_str(&formatted_bill);

    let bills_file = File::create(BILLS_PATH.to_owned()).expect("Unable to create file");
    let mut bills_file = BufWriter::new(bills_file);

    bills_file.write_all(bills.as_bytes()).expect("Unable to write data");

    return bills;
}

pub fn delete_bill (bill_name: &String) {
    let buffer = get_file(&BILLS_PATH.to_owned());

    let mut bills = String::new();

    for line in buffer.lines() {
        let line = line.expect("Unable to read line");

        let line_type = line.trim().rsplitn(2, ' ').last().unwrap_or("");

        if line_type.to_lowercase() != bill_name.to_lowercase() {
            bills.push_str(&line);
            bills.push_str("\n");
        }
    }

    let bills_file = File::create(BILLS_PATH.to_owned()).expect("Unable to create file");
    let mut bills_file = BufWriter::new(bills_file);

    bills_file.write_all(bills.as_bytes()).expect("Unable to write data");
}

pub fn delete_data_dir () {
    if Path::new(DATA_DIR).exists() {
        remove_dir_all(DATA_DIR).expect("Unable to remove directory");
        print_deleted_all_data_deleted();
    }
    else {
        print_cannot_delete_data();
    }
}