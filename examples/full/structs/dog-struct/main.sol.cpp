#include <iostream>
using namespace std;

int main(int argc, char *argv[]) {
  if (argc < 3)
    cout << "Erreur: arguments manquants" << endl;
  else
    cout << "Le chien est " << argv[1] << " et a " << argv[2] << " pattes"
         << endl;
}
