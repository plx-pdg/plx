#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc < 3)
    printf("Error: missing argument firstname and legs number");
  else
    printf("The dog is %s and has %s legs\n", argv[1], argv[2]);
}
