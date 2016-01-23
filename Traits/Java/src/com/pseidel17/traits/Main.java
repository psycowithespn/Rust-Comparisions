package com.pseidel17.traits;

public class Main {

    public static void main(String[] args) {
        Printable printable = new Name("Patrick");
        printable.print();
    }

    public interface Printable {
        void print();
    }

    public static class Name implements Printable {

        private final String name;

        public Name(String name) {
            this.name = name;
        }

        @Override
        public void print() {
            System.out.println(name);
        }
    }
}
