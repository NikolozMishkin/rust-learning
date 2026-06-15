#[derive(Debug)]
struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    fn new(owner: String, initial_balance: i32) -> Self {
        println!("Account opened successfully");
        Self {
            balance: initial_balance,
            owner,
        }
    }

    fn change_owner(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }

    fn check_balance(&self) {
        println!("{}'s balance: ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        println!("Deposited ${} to {}'s account", amount, self.owner);
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
            println!("withdrew ${} from {}'s account", amount, self.owner);
        } else {
            println!(
                "Insufficient funds for withdrawl in {}'s account",
                self.owner
            );
        }
        self
    }

    fn view_owner(&self) -> &Self {
        println!("Account owner: {}", self.owner);
        self
    }
}

/*
---------------------------------------------------------------
Self type   | Methods & Notes
----------- | -------------------------------------------------
self        | change_owner(mut self, .. ) -> Self

&self       | check_balance(&self)
            | view_owner(&self) -> &Self

&mut self   | deposit(&mut self, .. ) -> &mut Self
            | withdraw(&mut self, ..) -> &mut Self

No self     |  new(String, i32) -> Self
---------------------------------------------------------------
*/

// Method Chaining: depends on how each method receives and return back self.
fn main() {
    let mut account = BankAccount::new(String::from("Micheal"), 4_000);
    // 1. Methods that does not return anything
    // Methods returning nothing cannot be chained further to grow the chain.
    account.check_balance();

    // 2. Methods that return a &mut Self
    // &mut Self -> chained with methods requiring &mut self or &self
    account.deposit(100).withdraw(50).view_owner();

    // 3. Methods that return &Self
    // &Self -> chained with methods requiring &self
    account.view_owner().check_balance();

    // 4. Method that retrun an owned form of Self
    // Self -> chained with methods accepting any of the three forms of self
    account
        .change_owner(String::from("new_owner"))
        .change_owner(String::from("another_owner"))
        .deposit(100)
        .view_owner();

    //println!("account: {:?}", account);
    // Method chaining works:
    // A() . B() (The output from method A() conforms to the input of B())
}

/*
----------------------------------------------------------------------------------------------
Form of Self         | Can Chain To methods   | Can Chain to  methods  | Can Chain To methods
                     | requiring self         | requiring &self        | requiring &mut self
-------------------- | ---------------------  | ---------------------- |----------------------
Self                 | ✅ Yes                 | ✅ Yes                | ✅ Yes
&Self                | ❌ No                  | ✅ Yes                | ❌ No
&mut Self            | ❌ No                  | ✅ Yes                | ✅ Yes
----------------------------------------------------------------------------------------------
 */
