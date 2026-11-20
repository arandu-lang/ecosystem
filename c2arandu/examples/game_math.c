#include <stdint.h>
#include <stdbool.h>

typedef struct Vec2 {
    float x;
    float y;
} Vec2;

typedef struct Player {
    int id;
    uint32_t health;
    Vec2 position;
    struct Player* next;
} Player;

typedef enum GameState {
    STATE_MENU = 0,
    STATE_PLAYING = 1,
    STATE_PAUSED = 2,
    STATE_GAMEOVER = 3
} GameState;

void spawn_player(Player* p, float x, float y);
float distance(const Vec2* a, const Vec2* b);
bool is_alive(const Player* p);
