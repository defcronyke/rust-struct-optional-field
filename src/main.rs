#[derive(Debug)]	// Make the object printable.
struct Person {
	name: Option<String>	// The name field is optional. It can be Some<String> or None, as per the Option enum.
}

impl Person {	// Define member functions for Person in here.
	fn new() -> Self {
		Person{			// Instantiate a new Person, and return them.
			name: None,	// A new person has no name.
		}
	}
}

fn main() {

	// Instantiate a new person. They have no name yet, but they are mutable (because of mut) so
	// we can give them a name during our annual naming ceremony.

	let mut parson = Person::new();
	let mut harry = Person::new();

	// It is time for Parson to realize their true name. Optional field values must be wrapped in
	// Some(), because a value of the Option<T> enum type must be either Some<T> or None.
	// Also note that Strings in Rust are different from str literals, like the "Parson" below.
	// We must convert the str literal into a full-fledged String object using the to_string()
	// function.

	parson.name = Some("Parson".to_string());

	// Print the debug representation of parson, so we can see its contents.

	println!("1. Hello {:?}", parson);		// 1

	// Print parson's name. Because the name String is wrapped in a Some<String> value, we have to
	// unwrap or pattern match it to get at the actual String. Unwrapping with the unwrap()
	// function is quick and easy, and good for prototyping and example code, but usually you'll
	// want to use the more powerful pattern matching instead in real programs, and use combinator
	// functions to reduce the amount of pattern matching that's needed.

	println!("2. Hello {}", parson.name.clone().unwrap());	// 2

	// The problem with unwrap()'ing everything, is that it purposely crashes your program on the
	// error condition, so if you want to handle the error instead, and not crash your program,
	// you'll need to use pattern matching and/or combinator functions to add your custom error
	// handling. If we uncomment the line below and run the program, we'll see that unwrapping a
	// person with no name causes the program to crash, even though we've specified that a name
	// should be optional (by giving the name field the Option<String> type).

	// println!("Hello {}", harry.name.clone().unwrap());

	// It crashes simply because that's what unwrap() does. So really we don't want to unwrap
	// optional types most of the time, because then the optional field doesn't seem so optional
	// anymore. The best and most concise thing to use in this case turns out to be a combinator
	// function, unwrap_or(). This function returns the data inside the Some value, or a message
	// that you want to show when the value is None.

	println!("3. Hello {}", harry.name.clone().unwrap_or("nobody".to_string()));	// 3

	// Sometimes you need more fine-grained control over what happens when a value of an enum type
	// is of a certain specific type in the enum. Combinator functions are preferred when they will
	// do what you need, but if you can't do it with combinators, we use the more verbose "pattern
	// matching", which allows us to handle each case however we want (similar to the switch
	// statement in other languages).

	let parson_name = match parson.name.clone() {
		Some(name) => name,
		None => "nobody".to_string()
	};

	println!("4. Hello {}", parson_name);	// 4

	// Finally, it is time for Harry to realize her true name.

	harry.name = Some("Harry".to_string());

	// Now we could simply print harry.name.unwrap(), but instead lets do something custom when
	// harry has a name, and also handle the None case, using pattern matching. When harry has a
	// name, we will prepend "Dj" to the front.

	let harry_name = match harry.name.clone() {
		Some(name) => {
			"Dj".to_string() + &name		// We could run any code in here.
		},
		None => "nobody".to_string()
	};

	println!("5. Hello {}", harry_name);	// 5

	// We could have done the same thing in a more concise way using a couple of combinator
	// functions. Comment out line 74, where we gave harry her name, and run the program. You'll
	// see we print "nobody" instead of crashing.

	println!("6. Hello {}", harry.name.map(|name| "Dj".to_string() + &name).unwrap_or("nobody".to_string()));
}
