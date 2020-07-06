#include <CoreFoundation/CoreFoundation.h>

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    const char *string;
    int32_t int_;
} ParamStruct;

typedef struct {
    CFStringRef string;
    int32_t int_;
} ReturnStruct;

int32_t add_values(int32_t value1, int32_t value2);

CFStringRef greet(const char *who);

void pass_struct(const ParamStruct *object);

void register_callback(void (*callback)(CFStringRef));

ReturnStruct return_struct(void);
