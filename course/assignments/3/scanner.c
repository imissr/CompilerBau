#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
     STATE_BAD, 
     STATE_A, 
     STATE_B 
    } State;

int is_accepting(State s) {
    
     return s == STATE_A; 
    }

const char *input;

size_t input_len;

size_t pos;


int next_char(void)
{
    int c = pos < input_len ? (unsigned char)input[pos] : EOF;
    pos++;
    return c;
}

typedef struct 
{

    State state;
    size_t pos;
} Entry;

#define STACK_MAX 8

Entry stack[STACK_MAX];

int sp;


void clear_stack(void) 
{
    sp = 0; 
}

void push(State s, size_t p)
{
    if (sp == STACK_MAX) {
        fprintf(stderr, "internal error: rollback stack overflow\n");
        exit(EXIT_FAILURE);
    }

    stack[sp].state = s;
    stack[sp].pos = p;
    sp++;
}

Entry pop(void) 
{ 

    return stack[--sp]; 

}

long next_word(void)
{
    size_t start = pos;
    int c;
    Entry top;

    clear_stack();
    push(STATE_BAD, start);
    goto s_A;

s_A:
    clear_stack();
    push(STATE_A, pos);
    c = next_char();
    if (c == 'a')
        goto s_B;
    if (c == 'c')
        goto s_A;
    goto s_out;

s_B:
    push(STATE_B, pos);
    c = next_char();
    if (c == 'b')
        goto s_A;
    goto s_out;

s_out:
    top = pop();
    while (!is_accepting(top.state) && top.state != STATE_BAD)
        top = pop();
    pos = top.pos;
    if (is_accepting(top.state))
        return (long)(pos - start);
    return -1;
}

int matches(const char *w)
{
    input = w;
    input_len = strlen(w);
    pos = 0;
    return next_word() == (long)input_len;
}

void tokenise(const char *line)
{
    input = line;
    input_len = strlen(line);
    pos = 0;

    while (pos < input_len) {
        size_t start = pos;
        long len = next_word();

        if (len > 0) 
        {
            printf("word accepted \"%.*s\"\n", (int)len, line + start);
        } else {
            pos = start;
            printf("ERROR unexpected character '%c' at column %zu\n",
                   line[pos], pos + 1);
            pos++;
        }
    }
}

int main(int argc, char **argv)
{
    if (argc == 3 && strcmp(argv[1], "-m") == 0) 
    {
        printf("%d\n", matches(argv[2]));
        return 0;
    }

    char buf[4096];
    while (fgets(buf, sizeof buf, stdin)) 
    {
        buf[strcspn(buf, "\r\n")] = '\0';
        printf("input: \"%s\"\n", buf);
        tokenise(buf);
    }
    return 0;
}
