## Dog struct
Petit programme qui affiche le nom d'un chien et son nombre de pattes. Affiche une erreur si des arguments manquants.

Example d'éxecution:
```sh
> ./dog Joe 5
Le chien est Joe et a 5 pattes
> ./dog 
Erreur: arguments manquants
```

Code de départ
```cpp
#include <iostream>
using namespace std;

int main(int argc, char *argv[]) {
  // "Erreur: arguments manquants" << endl;
}
```

<details>
<summary>Solution</summary>

```cpp
#include <iostream>
using namespace std;

int main(int argc, char *argv[]) {
  if (argc < 3)
    cout << "Erreur: arguments manquants" << endl;
  else
    cout << "Le chien est " << argv[1] << " et a " << argv[2] << " pattes"
         << endl;
}
```

