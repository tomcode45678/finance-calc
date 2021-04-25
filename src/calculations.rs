use crate::read_and_write::PRICE_VALUE_DELIMITER;

pub fn calculate_savings (monthly_income: f32, monthly_bills: f32) -> (f32, f32) {
    let remaining = (monthly_income - monthly_bills).floor();
    let savings_percentage = calculate_savings_percentage(remaining, monthly_income);

    return (remaining, savings_percentage);
}

fn calculate_savings_percentage(remaining: f32, monthly_salary: f32) -> f32 {
    ((remaining / monthly_salary) * 100.0).floor()
}

pub fn calculate_monthly_bills_total (monthly_bills: String) -> f32 {
    let mut monthly_bills_total: f32 = 0.0;
    let bills = monthly_bills.split("\n");

    for bill in bills {
        let value = bill.split(PRICE_VALUE_DELIMITER).last().unwrap();
        
        if !value.is_empty() {
            monthly_bills_total += value.parse::<f32>().unwrap();
        }
    }

    return monthly_bills_total;
}