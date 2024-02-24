trait Account {
    fn deposit(&mut self, amount: f32) -> Result<(), String>;
    fn withdraw(&mut self, amount: f32) -> Result<(), String>;
    fn balance(&self) -> f32;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f32,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f32) -> Result<(), String> {
        if amount >= 0.00001 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Dostum yüklediğin para işlem masrafına bile değmez.".to_string())
        }
    }
    fn withdraw(&mut self, amount: f32) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Yetersiz bakiye".to_string())
        }
    }
    fn balance(&self) -> f32 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 123456,
        holder_name: "Elon Musk".to_string(),
        balance: 100.0,
    };

    let mut account2 = BankAccount {
        account_number: 789456,
        holder_name: String::from("Kendall Roy"),
        balance: 250.0,
    };

    account1.deposit(275.0);
    if let Err(e) = account2.withdraw(350.0) {
        println!("{e}")
    };

    println!(
        "account number: {}, holder name: {}, balance: {}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "account number: {}, holder name: {}, balance: {}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
