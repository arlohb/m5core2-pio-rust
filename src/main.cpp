//
// Arduino main stub file. Calls into Rust.
//

#include <M5Core2.h>

extern "C" void arduino_setup();
extern "C" void arduino_loop();

extern "C" void lcd_println(char text[])
{
    M5.Lcd.println(text);
}

void setup()
{
    M5.begin();
    M5.Lcd.println("Hello World");
    arduino_setup();
}

void loop()
{
    arduino_loop();
}
