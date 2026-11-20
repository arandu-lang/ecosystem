typedef struct Point {
    float x;
    float y;
} Point;

float get_distance_squared(const Point* p1, const Point* p2) {
    float dx = p1->x - p2->x;
    float dy = p1->y - p2->y;
    return dx * dx + dy * dy;
}

void move_point(Point* p, float dx, float dy) {
    p->x += dx;
    p->y += dy;
}
