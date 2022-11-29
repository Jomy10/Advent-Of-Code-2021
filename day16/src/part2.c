#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include "part2.h"
#include "node.h"

uint64_t evalExpression(const node_t* node) {
  if (node->type == LiteralValue) {
    return node->literal.value;
  } else if (node->type == Operator) {
    // If only switches in C weren't so bad
    if (node->op.typeId == Op_Sum) {
      uint64_t total = 0;
      for (int i = 0; i < node->op.nodes.len; i++) {
        total += evalExpression(node->op.nodes.nodes[i]);
      }
      return total;
    } else if (node->op.typeId == Op_Prod) {
      uint64_t total = 0;
      for (int i = 0; i < node->op.nodes.len; i++) {
        if (i == 0) {
          total = evalExpression(node->op.nodes.nodes[i]);
        } else {
          total *= evalExpression(node->op.nodes.nodes[i]);
        }
      }
      return total;
    } else if (node->op.typeId == Op_Min) {
      uint64_t total = 18446744073709551615u;
      if (node->op.nodes.len == 0) return 0;
      for (int i = 0; i < node->op.nodes.len; i++) {
        uint64_t val = evalExpression(node->op.nodes.nodes[i]);
        if (val < total) {
          total = val;
        }
      }
      return total;
    } else if (node->op.typeId == Op_Max) {
      uint64_t total = 0;
      for (int i = 0; i < node->op.nodes.len; i++) {
        uint64_t val = evalExpression(node->op.nodes.nodes[i]);
        if (val > total) {
          total = val;
        }
      }
      return total;
    } else if (node->op.typeId == Op_Gt) {
      return evalExpression(node->op.nodes.nodes[0]) > evalExpression(node->op.nodes.nodes[1]);
    } else if (node->op.typeId == Op_Lt) {
      return evalExpression(node->op.nodes.nodes[0]) < evalExpression(node->op.nodes.nodes[1]);
    } else if (node->op.typeId == Op_Eq) {
      return evalExpression(node->op.nodes.nodes[0]) == evalExpression(node->op.nodes.nodes[1]);
    }
  }
  
  printf("Invalid code path for node type %i (operator %i)\n", node->type, node->op.typeId);
  exit(1);
}
