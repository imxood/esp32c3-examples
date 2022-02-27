#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "freertos/semphr.h"
#include "esp_system.h"
#include "lvgl/lvgl.h"
#include "lvgl_helpers.h"
// #include "esp_heap_caps.h"

#define LV_TICK_PERIOD_MS 1

void guiTask(void *pvParameter);
void create_demo_application(void);
void lv_tick_task(void *arg);
void lv_example_get_started_2(void);