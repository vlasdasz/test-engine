
#include <stdint.h>

enum TestEngineAction {
    None = 0,
    OpenKeyboard = 1,
    CloseKeyboard = 2
};

enum KeyEvent {
    Letter = 0,
    Backspace = 1,
};

void set_screen_size(unsigned int width, unsigned int height);

enum TestEngineAction update_screen(void);

void on_touch(unsigned long long id, float x, float y, int event);

void set_gyro(float pitch, float roll, float yaw);

void set_monitor(int ppi,
                float scale,
                int refresh_rate,
                int resolutionX,
                int resolutionY,
                float width,
                float height,
                float diagonal);

void add_key(char key, enum KeyEvent event);

void opengl_ready(void);
