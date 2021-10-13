gcc -fPIC -c foo.c
ld -Bshareable -o foo.so foo.o
