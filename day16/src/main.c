#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "input.h"
#include "node.h"
#include "transmission.h"
#include "parsing.h"
#include "part1.h"
#include "part2.h"

int main() {
  const char* hex = readInput("input.txt");
  const char* bin = hexToBin(hex, strlen(hex));

  transm_t transm = {
    .bin = bin,
    .ptr = 0,
  };
  
  node_t* tree = parse(&transm);
  
  printf("PacketVersionsTotal: %i\n", countPacketVersions(tree));
  
  uint64_t result = evalExpression(tree);
  
  printf("Expression evaluated to: %llu\n", result);
  
  freeTree(tree);
  free((void*) bin);
  return 0;
}

