extern "C" {
    int multiply(int a, int b) {
        return a * b;
    }
    int power(int base, int exponent) {
        int result = 1;
        for (int i = 0; i < exponent; i++ ) {
            result *= base;
        }
        return result;
    }
}
