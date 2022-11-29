#ifndef AOT16_PART2_H
#define AOT16_PART2_H

#include <stdint.h>
#include "node.h"

enum OperatorType {
  Op_Sum = 0,
  Op_Prod = 1,
  Op_Min = 2,
  Op_Max = 3,
  Op_Gt = 5,
  Op_Lt = 6,
  Op_Eq = 7,
};

uint64_t evalExpression(const node_t* node);

#endif
