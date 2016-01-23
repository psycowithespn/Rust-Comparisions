package com.psyco.exceptions;

public class ExceptionsExampleCaught {

    public static void main(String[] args) {
        String number = "42";
        try {
            int result = Integer.parseInt(number);
            System.out.println(number + " + 10 = " + (result + 10));
        } catch (NumberFormatException ex) {
            System.out.println("Tried to decode a string without a number.");
        }

        String notANumber = "Hello World";
        try {
            int result2 = Integer.parseInt(notANumber);
            System.out.println(number + " + 10 = " + (result2 + 10));
        } catch (NumberFormatException ex) {
            System.out.println("Tried to decode a string without a number.");
        }
    }
}
