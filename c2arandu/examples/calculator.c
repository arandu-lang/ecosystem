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
