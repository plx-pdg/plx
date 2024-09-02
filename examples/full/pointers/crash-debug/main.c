#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  printf("Make it not crash !\n");

  // Creating an array of 3 int
  int *ptr = malloc(3);
  ptr[0] = 2;
  ptr[1] = 3;
  ptr[2] = 4;
  printf("ptr[0] = %d\n", ptr[0]);
  printf("ptr[1] = %d\n", ptr[1]);
  printf("ptr[2] = %d\n", ptr[2]);

  // Reusing ptr pointer for a single int
  free(ptr);
  ptr = NULL;
  *ptr = 3;
  printf("new ptr = %d\n", *ptr);
  free(ptr);
}
