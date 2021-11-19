use crate::read_and_write::{
    PRICE_VALUE_DELIMITER
};

use crate::prompts::{
    SAVINGS_PROMPT,
    COMPLETE_PROMPT,
    UPDATE_PROMPT,
    MENU_PROMPT,
    QUIT_PROMPT,
    DELETE_PROMPT,
    PROFILE_PROMPT,
    BILLS_PROMPT,
    REMOVE_DATA_PROMPT
};

static COMMANDS_CHAR_GAP: usize = 12;
static RECEIPT_LENGTH: usize = 26;

fn get_char_gap (offset: &str, gap: usize) -> String {
    let total_spaces = gap - offset.chars().count();

    let mut spaces = String::new();

    for _ in 1 .. total_spaces {
        spaces.push_str(" ");
    }

    return spaces;
}

pub fn print_cmd_options () {
    println!("{} {}- see per calendar month savings", SAVINGS_PROMPT, get_char_gap(SAVINGS_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- update details (income, bills, ...)", UPDATE_PROMPT, get_char_gap(UPDATE_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- clear screen and back to menu", MENU_PROMPT, get_char_gap(MENU_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- close application", QUIT_PROMPT, get_char_gap(QUIT_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- delete stored data (profile, bills, ...)", REMOVE_DATA_PROMPT, get_char_gap(REMOVE_DATA_PROMPT, COMMANDS_CHAR_GAP));
}

pub fn print_update_cmd_options () {
    clear_terminal();
    println!("{} {}- update your profile options (income, ...) ", PROFILE_PROMPT, get_char_gap(PROFILE_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- update your bills (create, update and delete)", BILLS_PROMPT, get_char_gap(BILLS_PROMPT, COMMANDS_CHAR_GAP));
    println!("{} {}- back to menu", MENU_PROMPT, get_char_gap(MENU_PROMPT, COMMANDS_CHAR_GAP));
}

pub fn command_not_found () {
    clear_terminal();
    println!("Sorry that command doesn't exist.");
    println!("Try one of the below:");
    println!("");
}

pub fn print_bills (bills: String, monthly_bills: Option<f32>) {
    clear_terminal();

    let amount_offset = 15;
    let separator = "-".repeat(RECEIPT_LENGTH + amount_offset);

    println!("BILLS");
    println!("{}", separator);

    for bill in bills.split("\n") {
        if !bill.is_empty() {
            let mut bill_segments = bill.trim().split(PRICE_VALUE_DELIMITER);
            let bill_type = bill_segments.next().unwrap_or("");
            let bill_amount = bill_segments.next().unwrap_or("");

            println!(
                "{}{}{}{}",
                bill_type,
                get_char_gap(bill_type, RECEIPT_LENGTH),
                PRICE_VALUE_DELIMITER,
                bill_amount
            );
        }
    }

    println!("{}", separator);

    if monthly_bills != None {
        let monthly_bills_formatted = format!("{:.2}", monthly_bills.unwrap());
        let total_copy = "Total bills";

        println!(
            "{}{}{}{}",
            total_copy,
            get_char_gap(total_copy, RECEIPT_LENGTH),
            PRICE_VALUE_DELIMITER,
            monthly_bills_formatted
        );
    }    
}

pub fn clear_terminal () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print_create_bills () {
    clear_terminal();
    
    println!("Please enter your monthly bills");
    println!("e.g. 'Water {}45'", PRICE_VALUE_DELIMITER);
    print_complete_prompt();
}

pub fn print_complete_prompt () {
    println!("");
    println!("When you are finished enter '{}'", COMPLETE_PROMPT);
}

pub fn print_update_bills () {
    println!("You can replace an existing bill by using the same name with a new value");
    println!("or remove by adding '{}' infront of the bill name", DELETE_PROMPT);
    println!("When you are finished enter '{}'", COMPLETE_PROMPT);
}

pub fn print_update_monthly_income () {
    println!("Enter your monthly income:");
}

pub fn print_income (income: f32) {
    let total_income_copy = "Income";

    println!(
        "{}{}{}{}",
        total_income_copy,
        get_char_gap(total_income_copy, RECEIPT_LENGTH),
        PRICE_VALUE_DELIMITER,
        income
    );
}

pub fn print_savings_after_expenses (remaining: f32, savings_percentage: f32, emoji: &char) {
    let savings_copy = "Savings";
    println!(
        "{}{}{}{} ({}%) {}",
        savings_copy,
        get_char_gap(savings_copy, RECEIPT_LENGTH),
        PRICE_VALUE_DELIMITER,
        remaining,
        savings_percentage,
        emoji
    );
}

pub fn print_deleted_all_data_deleted () {
    clear_terminal();
    println!("All data (profile, bills, ...) deleted");
    println!("--------------------------------------");
}

pub fn print_cannot_delete_data () {
    clear_terminal();
    println!("There is no data (profile, bills, ...) to delete");
    println!("------------------------------------------------");
}