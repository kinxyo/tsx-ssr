interface Person {
    name: string;
    age: number;
    profession: string;
}

function greet(name: string): void {
    console.log(`Hello, ${name}!`);
}

const person: Person = {
    name: "John",
    age: 30,
    profession: "Developer"
};

greet(person.name);
console.log(person.age);
console.log(person.profession);