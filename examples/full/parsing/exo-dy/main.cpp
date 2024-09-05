#include "exo.h"
/*#include "exo_parser.h"*/
#include <iostream>
#include <string>
using namespace std;

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  string raw = argv[1];
  cout << "Parsing '" << raw << "' result in:" << endl;
  /*Exo exo = ExoParser::parse(raw);*/ // TODO: fix build with #ifndef
  Exo exo = {.name = "What is the best OS?", .solution = "GNU/Linux"};
  cout << "title='" << exo.name << "' and solution='" << exo.solution << "'"
       << endl;
}
