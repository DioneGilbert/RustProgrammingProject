use std::fmt;

// type Result<T> = std::result::Result<T, HitBalanceError>;

// #[derive(Debug, Clone)]
// pub struct HitBalanceError;

// impl fmt::Display for HitBalanceError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "You cannot transfer more than you current balance")
//     }
// }

// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wecome:")
    }
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

    // fn transfer_funds(users: (User, User), amount: u64) -> Result<()>  {
    //     let user0: User = users.0;
    //     let mut user1: User = users.1;
    //     if user0.balance < amount {
    //         Err(HitBalanceError)
    //         //panic!("You cannot transfer more than you current balance: balance = {} -- amount = {}",user0.balance,  amount);
    //     } else {
    //         user1.balance = user0.balance ;
    //         Ok(())
    //     }
    // }

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
    pub fn accrue_interest(&self, users: (User, User), amount: u64) -> Result<(), String> {
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
}

// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err(String::from("division by zero"))
//     } else {
//         Ok(a / b)
//     }
// }

// Ok(`, `)
