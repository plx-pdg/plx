#ifndef EXO_PARSER
#define EXO_PARSER
#include "exo.h"
#include <string>

class ExoParser {
public:
  static Exo parse(const std::string &raw);
};

#endif