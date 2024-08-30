#include <stdio.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
  // 500ms duration loop in 5 steps
  for (int i = 0; i < 5; i++) {
    printf("Step %d\n", i);
    usleep(100000); // every 100 ms
  }
}
