package com.pseidel.structs;

public class Person {

    int age;
    String name;

    public void print() {
        System.out.println(this.name + ": " + age + " years old");
    }


    public static void main(String[] args) {
        Person person = new Person();
        person.age = 20;
        person.name = "Patrick";

        person.print();
    }
}
