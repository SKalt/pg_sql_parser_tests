typedef struct {
    int32 length;
    char data[FLEXIBLE_ARRAY_MEMBER];
} text;
