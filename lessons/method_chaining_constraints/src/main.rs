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
        println!("{}'s balance is ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        println!("Deposit ${} to {}'s account", amount, self.owner);
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdraw ${} from {}'s account", amount, self.owner);
        } else {
            println!("Insufficient funds for wuthdrawl {}'s account", self.owner);
        }
        self
    }

    fn view_owner(&self) -> &Self {
        println!("Account owner {}", self.owner);
        self
    }
}

// method chaining: depends on how each method recieves and return back self

fn main() {
    let mut account = BankAccount::new(String::from("Micheal"), 4_000);
    //1. methods that does not return anything
    //methods returning nothing connot be chaind further to grow the chain
    account.check_balance();

    //2. methods that return a &mut Self
    //&mut Self -> chained with methods requiring &mut Self or &Self
    account.deposit(100).withdraw(50).view_owner();

    //3. methods that return &Self
    // &Self -> chained with methods requiring &Self
    account.view_owner().check_balance();

    //4. methods that return an owned form of Self
    // Self -> chained with methods accepting any of the three forms of self
    account
        .change_owner(String::from("new_owner"))
        .change_owner(String::from("another_owner"))
        .deposit(100);
    // println!("account {:?}", account);
    // A() . B() (The output for method A () conforms to the input of B())
}
