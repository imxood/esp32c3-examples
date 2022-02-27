#include "lvgl_init.h"

/*********************
 *      DEFINES
 *********************/
#define TAG "demo"

/**********************
 *   APPLICATION MAIN
 **********************/
void app_main()
{
    /* If you want to use a task to create the graphic, you NEED to create a Pinned task
     * Otherwise there can be problem such as memory corruption and so on.
     * NOTE: When not using Wi-Fi nor Bluetooth you can pin the guiTask to core 0 */
    xTaskCreatePinnedToCore(guiTask, "gui", 4096 * 2, NULL, 0, NULL, 1);
}
