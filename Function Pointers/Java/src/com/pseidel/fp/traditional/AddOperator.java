package com.pseidel.fp.traditional;

import com.pseidel.fp.lambda.MathOperator;

public class AddOperator implements MathOperator {

    @Override
    public double doOperation(double a, double b) {
        return a + b;
    }
}
