#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
  if (argc < 3)
    cout << "Error: missing argument firstname and legs number";
  else
    cout << "The dog is " << argv[1] << " and has " << argv[2] << " legs\n";
}
