#include <iostream>

using namespace std;

// Enum pour tous les jours de la semaine
typedef enum {
  Lundi = 1,
  Mardi,
  Mercredi,
  Jeudi,
  Vendredi,
  Samedi,
  Dimanche
} Day;

int main(int argc, char *argv[]) {
  string day;
  switch ((Day)atoi(argv[1])) {
  case Lundi:
    day = "Lundi";
    break;
  case Mardi:
    day = "Mardi";
    break;
  case Mercredi:
    day = "Mercredi";
    break;
  case Jeudi:
    day = "Jeudi";
    break;
  case Vendredi:
    day = "Vendredi";
    break;
  case Samedi:
    day = "Samedi";
    break;
  case Dimanche:
    day = "Dimanche";
    break;
  }

  cout << "Le jour " << argv[1] << " est " << day << endl;
}
