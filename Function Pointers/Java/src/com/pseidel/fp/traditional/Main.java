package com.pseidel.fp.traditional;

import com.pseidel.fp.lambda.MathOperator;

public class Main {

    public static void main(String[] args) {
        MathOperator op = new AddOperator();
        double resultA = op.doOperation(10, 20);
        System.out.println(resultA);    // Prints '30.0'

        op = new SubtractOperator();
        double resultB = op.doOperation(10, 20);
        System.out.println(resultB);    // Prints '-10.0'
    }
}
