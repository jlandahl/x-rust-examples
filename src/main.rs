fn main() {
    process_users_imperative();
}

// A structure to hold user details
struct User {
    name: String,
    email_address: String,
    age: i16,
}

// use imperative style to get the email addresses of users who are 18 or older
fn process_users_imperative() {
    // get the user records from some source like a remote server or a database
    let users = get_users();

    // create an empty container to hold the email addresses
    let mut email_addresses = Vec::new();

    // create a variable to index into the user list; 0 is the first user
    let mut i = 0;

    // loop while our index is less than the length of the user list
    while i <= users.len() - 1 {
        // test for age
        if users[i].age >= 18 {
            // add the email address to the list
            email_addresses.push(users[i].email_address.clone());
        }

        // increment the index variable
        i += 1;
    }

    // display the email addresses
    println!("{:#?}", email_addresses);
}

// use functional style to get the email addresses of users who are 18 or older
fn process_users_functional() {
    // get the user records from some source like a remote server or a database
    let users = get_users();

    // one simple step to do the same as above
    let email_addresses: Vec<String> = users
        .into_iter()
        // filter the user list to include only those 18 or older
        .filter(|user| user.age >= 18)
        // for each user that's 18 or older, return just their email address
        .map(|user| user.email_address)
        // collect the results into a new list
        .collect();

    // display the email addresses
    println!("{:#?}", email_addresses);
}

fn get_users() -> Vec<User> {
    // This is where we'd retrieve user details from a database or a remote service; since this is
    // just an example, return some fake data
    vec![
        User {
            name: "Person1".to_string(),
            email_address: "person1@gmail.com".to_string(),
            age: 17,
        },
        User {
            name: "Person2".to_string(),
            email_address: "person2@gmail.com".to_string(),
            age: 21,
        },
        User {
            name: "Person3".to_string(),
            email_address: "person3@gmail.com".to_string(),
            age: 42,
        },
    ]
}
