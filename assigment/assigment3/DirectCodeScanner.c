#include <stdio.h>

typedef enum {
    STATE_X,   // start (non-final)
    STATE_Y,   // final
    STATE_Z    // dead
} State;

int matches(const char *input) {
    State state = STATE_X;

    for (const char *p = input; *p != '\0'; ++p) {
        char ch = *p;

        switch (state) {
            case STATE_X:
                if (ch == 'a') {
                    state = STATE_Y;
                } else if (ch == 'b') {
                    state = STATE_Z;
                } else if (ch == 'c') {
                    state = STATE_X;
                } else {
                    state = STATE_Z;
                }
                break;

            case STATE_Y:
                if (ch == 'a') {
                    state = STATE_Z;
                } else if (ch == 'b') {
                    state = STATE_X;
                } else if (ch == 'c') {
                    state = STATE_Z;
                } else {
                    state = STATE_Z;
                }
                break;

            case STATE_Z:
                state = STATE_Z;
                break;
        }
    }

    return state == STATE_Y;
}

int main(void) {
    char buffer[1024];

    printf("Enter a word over {a,b,c}: ");
    if (scanf("%1023s", buffer) != 1) {
        return 1;
    }

    if (matches(buffer)) {
        printf("ACCEPT\n");
    } else {
        printf("REJECT\n");
    }

    return 0;
}
