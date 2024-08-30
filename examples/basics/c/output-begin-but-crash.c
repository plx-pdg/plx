#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  printf("PLX is amazing !\n");
  int *ptr = malloc(10);
  *ptr = 2;
  free(ptr);
  ptr = NULL;
  *ptr = 3; // this would crash and generate something like
  // "Segmentation fault (core dumped)"
}
