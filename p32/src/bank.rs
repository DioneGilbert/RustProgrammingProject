#[derive(Debug, Clone)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct Bank<'a> {
    users: &'a [User],
    name: String,
    credit_interest: u64,
    debit_interest: i64,
}

// Traits for User
pub trait UserName {
    fn get_user_name(&self) -> String;
}

impl UserName for User {
    fn get_user_name(&self) -> String {
        self.name.clone()
    }
}

pub trait UserCreditLine {
    fn get_user_credit_line(&self) -> u64;
}

impl UserCreditLine for User {
    fn get_user_credit_line(&self) -> u64 {
        self.credit_line
    }
}

pub trait UserBalance {
    fn get_user_balance(&self) -> i64;
}

impl UserBalance for User {
    fn get_user_balance(&self) -> i64 {
        self.credit_line.try_into().unwrap()
    }
}

// Traits for Bank

pub trait BankUsers {
    fn get_bank_users(&self) -> &[User];
}

impl BankUsers for Bank<'_> {
    fn get_bank_users(&self) -> &[User] {
        self.users
    }
}

pub trait BankName {
    fn get_bank_name(&self) -> String;
}

impl BankName for Bank<'_> {
    fn get_bank_name(&self) -> String {
        self.name.clone()
    }
}

pub trait BankCreditInterest {
    fn get_bank_credit_interest(&self) -> u64;
}

impl BankCreditInterest for Bank<'_> {
    fn get_bank_credit_interest(&self) -> u64 {
        self.credit_interest
    }
}

pub trait BankDebitInterest {
    fn get_bank_debit_interest(&self) -> i64;
}

impl BankDebitInterest for Bank<'_> {
    fn get_bank_debit_interest(&self) -> i64 {
        self.debit_interest
    }
}

// Constructors
impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
}

impl<'a> Bank<'a> {
    pub fn new(users: &'a [User], name: String, credit_interest: u64, debit_interest: i64) -> Self {
        Self {
            users,
            name,
            credit_interest,
            debit_interest,
        }
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let mut bank_liabilities: u64 = 0;
        let mut bank_assets: u64 = 0;
        let bank_users = self.get_bank_users();
        let mut i = 0;
        while i < bank_users.len() {
            match bank_users[i].credit_line > 0 {
                true => bank_liabilities += bank_users[i].credit_line,
                false => bank_assets += bank_users[i].credit_line,
            }

            i = i + 1;
        }
        (bank_liabilities, bank_assets)
    }

    pub fn transfer_funds(&self, users: (User, User), amount: u64) -> Result<(), String> {
        let mut user0: User = users.0;
        let mut user1: User = users.1;
        match user0.balance > 0 {
            false => Err(String::from("Your balance is not sufficient")),
            true => match user0.balance as u64 > amount {
                false => Err(String::from("Your balance is not sufficient")),
                true => {
                    user1.balance = user0.balance;
                    user0.balance = user0.balance - (amount as i64);
                    Ok(())
                }
            },
        }
    }
    pub fn accrue_interest(&self, user: User) {
        let mut user: User = user;
        let balance_interest: i64;
        let credit_interest: i64;
        match user.balance > 0 {
            false => {
                credit_interest = (self.credit_interest as i64) * user.balance;
                user.balance = user.balance + credit_interest
            }
            true => {
                balance_interest = (self.debit_interest as i64) * user.balance;
                user.balance = user.balance + balance_interest
            }
        }
    }
    pub fn merge_bank(&mut self, bank_to_be_merged: &mut Bank) {
        println!("{:?}", bank_to_be_merged);
        todo!()
    }
}
