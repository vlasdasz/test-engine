
#include <stdint.h>

void create_screen(void);

void set_screen_size(float width, float height);

void update_screen(void);

void on_touch(unsigned long long id, float x, float y, int event);