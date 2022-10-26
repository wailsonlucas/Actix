
#[derive(Debug)]

enum PaymentMethod {
    Cash {amount: u32, name: String},
    DebitCard { bin: u32, name: String},
}


impl PaymentMethod {
    fn payment_validation(&self) -> String {
        match *self {
            PaymentMethod::Cash{..} => String::from("Paying with cash"),
            PaymentMethod::DebitCard{..} => String::from("Paying with Debit Card"),
            _ => String::from("Unsupported payment method")
        }
    }
}


fn main() {
    let method_2:PaymentMethod = PaymentMethod::DebitCard { bin: 1000, name: String::from("Wailson") };

    let method_1:PaymentMethod = PaymentMethod::Cash{amount: 1234, name: String::from("Yassine")};
    let checked:String = method_1.payment_validation();
    println!("{}", checked);
}