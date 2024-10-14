// Interface definition
interface User {
    id: number;
    name: string;
    email: string;
}

// Function definition
function greetUser(user: User): string {
    return `Hello, ${user.name}! Your email is ${user.email}.`;
}

// Class with a method
class UserManager {
    private users: User[];

    constructor() {
        this.users = [];
    }
































    
    // Method definition
    addUser(user: User): void {
        this.users.push(user);
        console.log(`User ${user.name} added successfully.`);
    }
}

// Test the function and method
const testUser: User = {
    id: 1,
    name: "John Doe",
    email: "john@example.com"
};

console.log(greetUser(testUser));

const userManager = new UserManager();
userManager.addUser(testUser);

