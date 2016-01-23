package com.psyco.exceptions;

public class ExceptionsExampleSimple {

    public static void main(String[] args) {
        String number = "42";
        int result = Integer.parseInt(number);
        System.out.println(number + " + 10 = " + (result + 10));

        String notANumber = "Hello World";
        result = Integer.parseInt(notANumber);
        System.out.println(number + " + 10 = " + (result + 10));
    }
}
