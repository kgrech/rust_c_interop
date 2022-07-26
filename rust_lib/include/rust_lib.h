#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *(*Allocator)(uintptr_t);

typedef void (*Callback)(const char*);

const char *create_string(void);

/**
 * # Safety
 * The ptr should be a valid pointer to the string allocated by rust
 */
void free_string(const char *ptr);

uintptr_t get_string_len(void);

/**
 * # Safety
 * The ptr should be a valid pointer to the buffer of required size
 */
void copy_string(char *ptr);

/**
 * # Safety
 * The allocator function should return a pointer to a valid buffer
 */
char *get_string_with_allocator(Allocator allocator);

char *get_string_with_malloc(void);

void get_string_in_callback(Callback callback);

/**
 * # Safety
 * The ptr should be a pointer to valid String
 */
void print_c_string(const char *ptr);
