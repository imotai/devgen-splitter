#include <iostream>
#include <string>
#include <vector>

class Person {
private:
    std::string name;
    int age;

public:
    Person(const std::string& n, int a) : name(n), age(a) {}

    void introduce() const {
        std::cout << "Hello, my name is " << name << " and I'm " << age << " years old." << std::endl;
    }

    void haveBirthday() {
        age++;
        std::cout << name << " is now " << age << " years old." << std::endl;
    }
};

int main() {
    std::vector<Person> people;
    
    people.push_back(Person("Alice", 25));
    people.push_back(Person("Bob", 30));
    people.push_back(Person("Charlie", 22));

    std::cout << "Introductions:" << std::endl;
    for (const auto& person : people) {
        person.introduce();
    }

    std::cout << "\nCelebrating birthdays:" << std::endl;
    for (auto& person : people) {
        person.haveBirthday();
    }

    return 0;
}

