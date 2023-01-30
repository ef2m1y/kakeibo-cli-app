use std::io;
use kakeibo_cli_app::services;

fn main() {
    let mut service_type = String::new();
    println!("[実行] 0:登録 or 1:集計");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値で入力して下さい");
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        println!("登録機能");
    } else if service_type == 1 {
        println!("集計機能");
    }
}
