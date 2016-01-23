package com.pseidel.fp.lambda;

public class Main {

    public static void main(String[] args) {
        MathOperator op = (a, b) -> a + b;
        double resultA = op.doOperation(10, 20);
        System.out.println(resultA);    // Prints '30.0'

        op = Main::subtract;
        double resultB = op.doOperation(10, 20);
        System.out.println(resultB);    // Prints '-10.0'
    }

    private static double subtract(double a, double b) {
        return a - b;
    }
}
