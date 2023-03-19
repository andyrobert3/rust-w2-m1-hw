// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount {
   name: String,
   age: Option<u32>
}

// define a trait called Balance, and within, function get_balance returning integer of 10
trait Balance {
   fn get_balance(&self) -> u32 {
      10
   }
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount {}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>


fn increase_balance<T:Balance>(balance: &T, increase_amount: u32) -> Result<u32, String> {
   if increase_amount <= 10 {
      Ok(balance.get_balance() + increase_amount)
   } else {
      Err("Increase must be less than 10!".to_string())
   }

   // Experimental "exclusive_range_pattern" 
   // https://doc.rust-lang.org/beta/unstable-book/language-features/exclusive-range-pattern.html
   // match increase_amount {
   //    0..10 => Ok(balance_type.get_balance() + increase_amount),
   //    _ => Err("Increase must be less than 10!".to_string())
   // }
}

fn main() {
   // create user_account, and set his age as Option::None
   let user_account = UserAccount {
      name: "Matthew".to_string(),
      age: Option::None 
   };

   // You want to increase the user_account's balance by 11
   // use a match, if the result of increase_balance is
   // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
   // - Err: print the error message returned
   match increase_balance(&user_account, 11) {
      Ok(bal) => println!("UserAccount balance increased to {}", bal),
      Err(msg) => println!("{}", msg)
   }

   // use an if...let...else statement to print the UserAccount age if it is a Option::Some
   if let Some(val) = user_account.age {
      println!("User's age is {}", val);
   }  else {
      println!("User provided no age");
   }
}

