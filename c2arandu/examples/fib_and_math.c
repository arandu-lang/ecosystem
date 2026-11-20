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

int sum_array(const int* arr, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += arr[i];
        i++;
    }
    return total;
}
