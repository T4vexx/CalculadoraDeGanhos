slint::include_modules!();

const TAXPER: f64 = 0.3;
const OWNER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    
    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXPER;
        let result: String = format!("Taxas: {:.2}\nDone: {:.2}\nLucro: {:.2}\nOperação: {:.2}",{tax},{owner},{profit},{opex});
        ui.set_results(result.into());
    });

    ui.run()
}
