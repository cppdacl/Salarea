use eframe::egui::{self, Color32, RichText, Stroke, Vec2};
use crate::compute::*;
use crate::constants::*;
use crate::style::*;
use crate::widgets::*;

#[derive(Default, PartialEq, Clone, Copy)]
enum Screen {
    #[default]
    Input,
    Result,
}

#[derive(Default)]
pub struct SalareaApp {
    last_name: String,
    first_name: String,
    middle_name: String,
    sex_str: String,
    department: String,
    position: String,
    rate_str: String,
    weekly_hours: [String; 4],
    is_manager: bool,
    dependents_str: String,
    screen: Screen,
    result: Option<PayResult>,
    error: Option<String>,
}

impl SalareaApp {
    fn try_compute(&mut self) {
        self.error = None;

        let sex: char = match self.sex_str.trim().chars().next().map(|c| c.to_ascii_uppercase()) {
            Some(c) if c == 'M' || c == 'F' => c,
            _ => { self.error = Some("Sex must be M or F.".into()); return; }
        };
        let rate: f64 = match self.rate_str.trim().parse::<f64>() {
            Ok(v) if v > 0.0 => v,
            _ => { self.error = Some("Invalid rate per hour.".into()); return; }
        };

        let dependents: u32 = match self.dependents_str.trim().parse() {
            Ok(v) => v,
            _ => { self.error = Some("Invalid dependents count.".into()); return; }
        };

        let mut hours = [0.0f64; 4];
        for i in 0..4 {
            match self.weekly_hours[i].trim().parse::<f64>() {
                Ok(v) if v >= 0.0 => hours[i] = v,
                _ => { self.error = Some(format!("Invalid hours for week {}.", i + 1)); return; }
            }
        }

        let total_hours: f64 = hours.iter().sum();
        let mut total_regular = 0.0f64;
        let mut total_overtime = 0.0f64;
        for h in &hours {
            total_regular += h.min(REGULAR_HOURS_PER_WEEK);
            total_overtime += (h - REGULAR_HOURS_PER_WEEK).max(0.0);
        }

        let gp = compute_gp(rate, total_regular, total_overtime);
        let transport = if self.is_manager { TRANSPORT_PER_WEEK * WEEKS_PER_MONTH } else { 0.0 };
        let (sss, pagiibig, philhealth, tax, dep_ded, total_ded) =
            compute_ded(gp, total_hours, dependents);
        let np = compute_np(gp, total_ded);

        self.result = Some(PayResult {
            last_name: self.last_name.clone(),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            sex,
            department: self.department.clone(), 
            position: self.position.clone(),
            rate,
            weekly_hours: hours,
            total_regular,
            total_overtime,
            gross_pay: gp + transport,
            sss, pagiibig, philhealth, tax, transport, dep_ded,
            total_ded,
            net_pay: np + transport,
        });
        self.screen = Screen::Result;
    }

    fn reset(&mut self) {
        *self = SalareaApp::default();
    }

    fn show_input(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("[ EMPLOYEE INFORMATION ]").font(mono(13.5)).color(gold()));
        ui.add_space(4.0);

        let mut row = |lbl: &str, val: &mut String| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{:<22}", lbl)).font(mono(12.5)).color(gold_dim()));
                ui.add(
                    egui::TextEdit::singleline(val)
                        .font(mono(12.5))
                        .desired_width(220.0)
                        .text_color(gold()),
                );
            });
        };

        row("Last Name       >", &mut self.last_name);
        row("First Name      >", &mut self.first_name);
        row("Middle Name     >", &mut self.middle_name);
        row("Sex (M/F)       >", &mut self.sex_str);
        row("Department      >", &mut self.department);
        row("Position        >", &mut self.position);
        row("Rate Per Hour   >", &mut self.rate_str);

        ui.add_space(10.0);
        ui.label(RichText::new("[ WEEKLY HOURS WORKED ]").font(mono(13.5)).color(gold()));
        ui.add_space(4.0);

        for i in 0..4 {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("Week {} Hours      >", i + 1))
                        .font(mono(12.5))
                        .color(gold_dim()),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.weekly_hours[i])
                        .font(mono(12.5))
                        .desired_width(100.0)
                        .text_color(gold()),
                );
            });
        }

        ui.add_space(10.0);
        ui.label(RichText::new("[ OPTIONS ]").font(mono(13.5)).color(gold()));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{:<22}", "Is Manager?     >")).font(mono(12.5)).color(gold_dim()));
            ui.checkbox(&mut self.is_manager, "");
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{:<22}", "No. of Dependents >")).font(mono(12.5)).color(gold_dim()));
            ui.add(
                egui::TextEdit::singleline(&mut self.dependents_str)
                    .font(mono(12.5))
                    .desired_width(80.0)
                    .text_color(gold()),
            );
        });

        ui.add_space(12.0);

        if let Some(err) = self.error.clone() {
            ui.label(RichText::new(format!("  ERROR > {}", err)).font(mono(12.5)).color(red()));
            ui.add_space(6.0);
        }

        if ui.add(
            egui::Button::new(RichText::new("  [ COMPUTE SALARY ]  ").font(mono(13.5)).color(bg()))
                .fill(gold())
                .min_size(Vec2::new(220.0, 34.0)),
        ).clicked() {
            self.try_compute();
        }
    }

    fn show_result(&mut self, ui: &mut egui::Ui) {
        let r = match &self.result {
            Some(r) => r,
            None => return,
        };

        ui.label(RichText::new("======= PAY SLIP =======").font(mono(14.0)).color(gold()));
        ui.add_space(6.0);

        ui.label(RichText::new("[ EMPLOYEE ]").font(mono(13.0)).color(gold()));
        lv(ui, "Name:", &format!("{}, {} {}", r.last_name, r.first_name, r.middle_name), gold());
        lv(ui, "Sex:", &r.sex.to_string(), gold());
        lv(ui, "Department:", &r.department, gold());
        lv(ui, "Position:", &r.position, gold());
        lv(ui, "Rate/Hour:", &format!("P {:.2}", r.rate), gold());

        sep(ui);

        ui.label(RichText::new("[ HOURS ]").font(mono(13.0)).color(gold()));
        let hours = r.weekly_hours;
        for i in 0..4 {
            let h = hours[i];
            let reg = h.min(REGULAR_HOURS_PER_WEEK);
            let ot = (h - REGULAR_HOURS_PER_WEEK).max(0.0);
            ui.label(
                RichText::new(format!("  Week {}: {:5.1} hrs  (reg: {:.1}  ot: {:.1})", i + 1, h, reg, ot))
                    .font(mono(12.0))
                    .color(gold_dim()),
            );
        }
        lv(ui, "Total Regular:", &format!("{:.1} hrs", r.total_regular), gold());
        lv(ui, "Total Overtime:", &format!("{:.1} hrs", r.total_overtime), gold());

        sep(ui);

        ui.label(RichText::new("[ SALARY ]").font(mono(13.0)).color(gold()));
        mrow(ui, "Regular Pay:", r.rate * r.total_regular, gold_dim());
        mrow(ui, "Overtime Pay:", (r.rate * OVERTIME_RATE_MULTIPLIER) * r.total_overtime, gold_dim());
        if r.transport > 0.0 {
            mrow(ui, "Transport Allowance:", r.transport, green());
        }
        sep(ui);
        mrow(ui, "GROSS PAY:", r.gross_pay, gold());

        sep(ui);

        ui.label(RichText::new("[ DEDUCTIONS ]").font(mono(13.0)).color(gold()));
        mrow(ui, "SSS:", r.sss, red());
        mrow(ui, "Pag-IBIG:", r.pagiibig, red());
        mrow(ui, "PhilHealth:", r.philhealth, red());
        mrow(ui, "Tax:", r.tax, red());
        if r.dep_ded > 0.0 {
            mrow(ui, "Dependents:", r.dep_ded, red());
        }
        sep(ui);
        mrow(ui, "TOTAL DEDUCTIONS:", r.total_ded, red());

        sep(ui);
        mrow(ui, "NET PAY:", r.net_pay, green());

        ui.add_space(14.0);
        ui.horizontal(|ui| {
            if ui.add(
                egui::Button::new(RichText::new("[ NEW EMPLOYEE ]").font(mono(13.0)).color(bg()))
                    .fill(gold())
                    .min_size(Vec2::new(160.0, 30.0)),
            ).clicked() {
                self.reset();
            }
            ui.add_space(10.0);
            if ui.add(
                egui::Button::new(RichText::new("[ EXIT ]").font(mono(13.0)).color(bg()))
                    .fill(red())
                    .min_size(Vec2::new(100.0, 30.0)),
            ).clicked() {
                std::process::exit(0);
            }
        });
    }
}

impl eframe::App for SalareaApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        {
            let vis = &mut ui.style_mut().visuals;
            vis.panel_fill = bg();
            vis.window_fill = bg();
            vis.extreme_bg_color = Color32::from_rgb(5, 4, 2);
            vis.widgets.inactive.bg_fill = Color32::from_rgb(25, 20, 8);
            vis.widgets.inactive.fg_stroke = Stroke::new(1.0, gold_dim());
            vis.widgets.hovered.fg_stroke = Stroke::new(1.0, gold());
            vis.widgets.active.fg_stroke = Stroke::new(1.5, gold());
            vis.selection.bg_fill = Color32::from_rgb(70, 48, 0);
            vis.selection.stroke = Stroke::new(1.0, gold());
            vis.widgets.noninteractive.fg_stroke = Stroke::new(1.0, gold_dim());
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(bg()).inner_margin(20.0))
            .show_inside(ui, |ui| {
                ui.label(
                    RichText::new("S A L A R E A  //  PAYROLL MANAGEMENT")
                        .font(mono(15.0))
                        .color(gold()),
                );
                ui.label(
                    RichText::new("SOLARIS INTERGALACTIC REAL ESTATE INC.")
                        .font(mono(10.5))
                        .color(gold_dim()),
                );
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.screen {
                        Screen::Input => self.show_input(ui),
                        Screen::Result => self.show_result(ui),
                    }
                });
            });
    }
}