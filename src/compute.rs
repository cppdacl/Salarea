use crate::constants::*;

pub struct PayResult {
    pub last_name: String,
    pub first_name: String,
    pub middle_name: String,
    pub sex: char,
    pub department: String,
    pub position: String,
    pub rate: f64,
    pub weekly_hours: [f64; 4],
    pub total_regular: f64,
    pub total_overtime: f64,
    pub gross_pay: f64,
    pub sss: f64,
    pub pagiibig: f64,
    pub philhealth: f64,
    pub tax: f64,
    pub transport: f64,
    pub dep_ded: f64,
    pub total_ded: f64,
    pub net_pay: f64,
}

pub fn compute_gp(rate: f64, reg_hours: f64, ot_hours: f64) -> f64 {
    rate * reg_hours + (rate * OVERTIME_RATE_MULTIPLIER) * ot_hours
}

pub fn compute_sss(gp: f64) -> f64 {
    if gp <= 5000.0 { 105.0 }
    else if gp <= 10000.0 { gp * 0.05 }
    else if gp <= 15000.0 { gp * 0.08 + 75.0 }
    else { gp * 0.12 + 110.0 }
}

pub fn compute_pagiibig(gp: f64) -> f64 {
    if gp < PAGIIBIG_LOW_THRESHOLD { PAGIIBIG_LOW_FIXED }
    else { gp * PAGIIBIG_HIGH_RATE }
}

pub fn compute_philhealth(total_hours: f64) -> f64 {
    if total_hours < PHILHEALTH_MIN_HOURS { 0.0 } else { PHILHEALTH }
}

pub fn compute_tax(gp: f64) -> f64 {
    if gp <= 10000.0 { gp * 0.03 }
    else if gp <= 25000.0 { gp * 0.08 }
    else if gp <= 40000.0 { gp * 0.11 }
    else { gp * 0.135 }
}

pub fn compute_ded(
    gp: f64,
    total_hours: f64,
    dependents: u32,
) -> (f64, f64, f64, f64, f64, f64) {
    let sss = compute_sss(gp);
    let pagiibig = compute_pagiibig(gp);
    let philhealth = compute_philhealth(total_hours);
    let tax = compute_tax(gp);
    let dep_ded = dependents as f64 * DEPENDENT_DEDUCTION;
    let total = sss + pagiibig + philhealth + tax + dep_ded;
    (sss, pagiibig, philhealth, tax, dep_ded, total)
}

pub fn compute_np(gp: f64, total_ded: f64) -> f64 {
    gp - total_ded
}