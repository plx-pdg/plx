#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
  cout << "PLX is amazing !" << endl;
  int *ptr = NULL;
  *ptr = 3; // this would crash and generate something like
  // "Segmentation fault (core dumped)"
}
