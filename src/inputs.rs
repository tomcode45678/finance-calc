use std::io::stdin;
use std::process;

use crate::actions::{
    get_monthly_income,
    get_bills,
    update_monthly_income,
    update_bills,
    remove_data
};

use crate::read_and_write::{ read_bills };

use crate::print_utils::{
    print_cmd_options,
    command_not_found,
    print_bills,
    print_savings_after_expenses,
    print_income,
    clear_terminal,
    print_update_cmd_options
};

use crate::prompts::{
    SAVINGS_PROMPT,
    UPDATE_PROMPT,
    PROFILE_PROMPT,
    BILLS_PROMPT,
    MENU_PROMPT,
    QUIT_PROMPT,
    REMOVE_DATA_PROMPT
};

use crate::emojis::{ get_emoji };
use crate::calculations::{ calculate_monthly_bills_total, calculate_savings };

pub fn run () {
    clear_terminal();
    soft_run();
}

pub fn get_input () -> String {
    let mut input = String::new();
    
    stdin().read_line(&mut input).unwrap();

    return input;
}

fn soft_run () {
    print_cmd_options();

    capture_input();
}

fn capture_input () {
    let input = get_input();

    match input.trim() {
        x if x == SAVINGS_PROMPT => get_savings(),
        x if x == UPDATE_PROMPT => update(),
        x if x == MENU_PROMPT => run(),
        x if x == QUIT_PROMPT => exit(),
        x if x == REMOVE_DATA_PROMPT => remove_data(),
        _ => capture_error(),
    };

    soft_run();
}

fn exit () {
    process::exit(1);
}

fn capture_error () {
    command_not_found();
    soft_run();
}

fn update () {
    print_update_cmd_options();

    let input = get_input();

    match input.trim() {
        x if x == PROFILE_PROMPT => update_monthly_income(),
        x if x == BILLS_PROMPT => update_bills(),
        x if x == MENU_PROMPT => run(),
        _ => capture_error(),
    };

    update();
}

fn get_savings () {
    let monthly_income = get_monthly_income();
    let monthly_bills_text = get_bills();

    let monthly_bills = calculate_monthly_bills_total(monthly_bills_text);
    let (remaining, savings_percentage) = calculate_savings(monthly_income, monthly_bills);

    let emoji = get_emoji(savings_percentage);

    print_bills(read_bills(), Some(monthly_bills));
    print_income(monthly_income);
    print_savings_after_expenses(remaining, savings_percentage, emoji);
    println!("");
}