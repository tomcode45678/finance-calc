use crate::inputs::{ get_input };

use crate::print_utils::{
    print_create_bills,
    print_update_monthly_income,
    print_bills,
    print_update_bills,
    clear_terminal,
};

use crate::read_and_write::{
    profile_exists,
    bills_file_exists,
    get_monthly_income_from_profile,
    read_bills,
    create_bills_file,
    input_bills,
    set_monthly_income,
    delete_data_dir
};

pub fn get_monthly_income () -> f32 {
    if !profile_exists() {
        update_monthly_income();
    }

    return get_monthly_income_from_profile();
}

pub fn update_monthly_income () {
    print_update_monthly_income();

    let profile_monthly_income = get_input();

    set_monthly_income(&profile_monthly_income);
}

pub fn get_bills () -> String {
    if bills_file_exists() {
        return read_bills();
    }
    else {
        create_bills_file();

        print_create_bills();

        input_bills();

        return read_bills();
    }
}

pub fn update_bills () {
    clear_terminal();

    print_bills(read_bills(), None);

    print_update_bills();

    input_bills();
}

pub fn remove_data () {
    delete_data_dir();
}