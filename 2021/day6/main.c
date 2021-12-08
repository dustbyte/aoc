#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long u64;

void            state_print(u64 *state) {
    size_t      i;

    for (i = 0; i < 9; i++) {
        printf("%u = %llu\n", (unsigned int)i, state[i]);
    }
    puts("");
}

int             state_init(u64 *state, char const *filename) {
    FILE        *file;
    char        buffer[784];
    size_t      bytes;
    size_t      i;

    file = fopen(filename, "r");
    if (!file) {
        fprintf(stderr, "Can't open file %s\n", filename);
        return -1;
    }

    bytes = fread(buffer, sizeof(*buffer), sizeof(buffer), file);
    if (bytes == 0) {
        fprintf(stderr, "Can't read file %s\n", filename);
        fclose(file);
        return -1;
    }
    buffer[bytes] = 0;
    fclose(file);

    for (i = 0; i < bytes; i++) {
        if (buffer[i] != ',') {
            state[buffer[i] - '0']++;
        }
    }

    return 0;
}

void            state_evolve(u64 *state) {
    u64         new_state[9];

    memset(new_state, 0, sizeof(new_state));

    new_state[8] = state[0];
    new_state[7] = state[8];
    new_state[6] = state[0] + state[7];
    new_state[5] = state[6];
    new_state[4] = state[5];
    new_state[3] = state[4];
    new_state[2] = state[3];
    new_state[1] = state[2];
    new_state[0] = state[1];

    memcpy(state, new_state, 9 * sizeof(*state));
}

u64          sum_state(u64 *state) {
    size_t      i;
    u64         sum;

    for (i = 0, sum=0; i < 9; i++) {
        sum += state[i];
    }

    return sum;
}

int             main(int argc, char **argv) {
    u64         state[9];
    size_t      days;

    memset(state, 0, sizeof(state));
    if (argc < 3) {
        fprintf(stderr, "usage: %s input_file days\n", argv[0]);
        return EXIT_FAILURE;
    }

    days = atoi(argv[2]);

    if (state_init(state, argv[1])) {
        return EXIT_FAILURE;
    }

    while (days--) {
        state_evolve(state);
    }

    printf("%llu\n", sum_state(state));
}
