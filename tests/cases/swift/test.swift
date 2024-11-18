import Foundation

// Define a class named `Person`
class Person {
    var firstName: String
    var lastName: String
    var age: Int

    // Initializer for the class
    init(firstName: String, lastName: String, age: Int) {
        self.firstName = firstName
        self.lastName = lastName
        self.age = age
    }

    // Method to get the full name of the person
    func fullName() -> String {
        return "\(firstName) \(lastName)"
    }

    // Method to get a description of the person
    func description() -> String {
        return "Name: \(fullName()), Age: \(age)"
    }
}

// Create an instance of the `Person` class
let person = Person(firstName: "John", lastName: "Doe", age: 30)

// Print the full name and description of the person
print(person.fullName()) // Output: John Doe
print(person.description()) // Output: Name: John Doe, Age: 30