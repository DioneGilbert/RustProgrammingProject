use std::borrow::BorrowMut;

use p32::bank::Bank;
use p32::bank::User;

fn main() {
    println!("Hello, world!");
    let user1 = User::new("Fred".to_string(), 2500, 1000000);
    let user2 = User::new("Moussa".to_string(), 800, 1000000);
    println!("user1 = {:?}", user1);
    println!("user2 = {:?}", user2);

    //let binding = [user1, user2];
    // let bank_users = [user1, user2];
    // let bank1: Bank = Bank::new(&bank_users, "ADCB".to_string(), 5000000000, 2000000);
    //println!("bank = {:?}", bank1);

    // let mut bank_users = [user1, user2];
    let mut bank_users = [user1.clone(), user2.clone()];
    let binding = bank_users.borrow_mut();
    //let users_2 = bank_users.clone_from_slice(&bank_users);
    let bank1: Bank = Bank::new(&binding, "ADCB".to_string(), 5000000000, 2000000);

    let cal_bal: (u64, u64) = bank1.calc_balance();
    let amount: u64 = 200000;
    println!("cal_bal = {:?}", cal_bal);
    let users_2: (User, User) = (user1, user2);
    let transfer_status = bank1.transfer_funds(users_2, amount);
    println!("transfer_status = {:?}", transfer_status);
}
