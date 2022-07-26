#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rust_lib.h"

void callback(const char* string) {
    printf("5. Printed from C: %s\n", string);
}

int main() {
    // ==== Passing rust string to C ====

    // Option 1: Provide create and delete methods
    const char* rust_string = create_string();
    printf("1. Printed from C: %s\n", rust_string);
    free_string(rust_string);

    // Option 2: Pass the buffer
    size_t len = get_string_len();
    char *buffer = malloc(len);
    copy_string(buffer);
    printf("2. Printed from C: %s\n", buffer);
    free(buffer);

    // Option 3: Pass memory allocator to rust
    char* rust_string_3 = get_string_with_allocator(malloc);
    printf("3. Printed from C: %s\n", rust_string_3);
    free(rust_string_3);

    // Option 4: Call libc malloc in rust
    char* rust_string_4 = get_string_with_malloc();
    printf("4. Printed from C: %s\n", rust_string_4);
    free(rust_string_4);

    // Option 5: Borrow rust string
    get_string_in_callback(callback);

    // ==== Passing C string to rust ====
    char *test = (char*) malloc(13*sizeof(char));
    strcpy(test, "Hello from C");
    print_c_string(test);
    free(test);

    return 0;
}
