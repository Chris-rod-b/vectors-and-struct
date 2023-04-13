struct Name {
    first_name: String,
    last_name: String
}

struct Account {
    name: Name,
    balance: f64,
}

impl Account {
    fn withdraw(&mut self, value:f64 ) {
        self.balance -= value;
    }
}

fn account() {
    let name: Name = Name { 
        first_name: String::from("Christian"), 
        last_name: String::from("Batista") 
    };

    let mut my_account: Account = Account { 
        name, 
        balance: (10000.00),
    };

    println!(
        "My Account: Name = {} {}, Balance = {}", 
        my_account.name.first_name, 
        my_account.name.last_name , 
        my_account.balance
    );

    my_account.withdraw(3675.21);

    println!(
        "My Account new balance: Balance = {}",
        my_account.balance
    );
}

fn main() {
    opcional_content();
    vectors();
    account();
}

fn opcional_content() {
    let content_file = read_file(String::from("/path/file.txt"));

    match &content_file {
        Some(value) => println!("{}", value),
        None => println!("The file doesn't exist!")
    }

    println!("{:?}", content_file);

    if let Some(value) = content_file {
        println!("Now, There is no doubt that the file exist: {}", value);
    } else {
        println!("No files here :(");
    }
}

fn read_file(file_path: String) -> Option<String> {
    println!("{}", file_path);
    Some(String::from("Some content here!"))
    //None
}

fn vectors() {
    //let mut grades:Vec<f32> = Vec::new();
        /*
        grades.push(10.0);
        grades.push(8.8);
        grades.push(6.5);
        */
    //With MACRO
        /* 
        let mut grades:Vec<f32> = vec![10.0, 8.8, 6.5]; 
        */

    //With Capacity
    let mut grades: Vec<f32> = Vec::with_capacity(5);

    grades.push(10.0);
    grades.push(8.8);
    grades.push(6.5);

    println!("{:?}", grades);

    grades.push(7.1);
    grades.push(9.2);

    println!("{:?}", grades);

    println!("Nota 1 = {}", grades[0]);

    println!("Nota 6 = {}", match grades.get(7) {
        Some(n) => *n, //removing the ref with *
        None => 0.0
    });

    /* 
    while let Some(grade) = grades.pop() {
        println!("Removed value = {}", grade);
    }
    */

    for grade in &grades {
        println!("Grades = {}", grade);
    }

    println!("{:?}", grades);

}
