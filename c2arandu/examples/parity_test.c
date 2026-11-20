int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    int a = 0;
    int b = 1;
    int i = 2;
    while (i <= n) {
        int next = a + b;
        a = b;
        b = next;
        i++;
    }
    return b;
}

int calculate(int op, int a, int b) {
    int res = 0;
    switch (op) {
        case 1:
            res = a + b;
            break;
        case 2:
            res = a - b;
            break;
        case 3:
            res = a * b;
            break;
        default:
            res = 0;
            break;
    }
    return res;
}

int count_down(int start, int verbose) {
    int count = 0;
    if (verbose) {
        count = 1;
    }
    while (start > 0) {
        start--;
        count++;
    }
    return count;
}

int main(void) {
    int f10 = fibonacci(10);
    int c_add = calculate(1, 15, 27);
    int c_mul = calculate(3, 7, 8);
    int cd = count_down(50, 1);
    return f10 + c_add + c_mul + cd;
}
