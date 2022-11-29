#include "node.h"
#include "part1.h"

#include <stdio.h> // TODO: remove

int countPacketVersions(const node_t* node) {
  int total = 0;
  
  total += node->packetVersion;
  if (node->type == Operator) {
    for (int i = 0; i < node->op.nodes.len; i++) {
      total += countPacketVersions(node->op.nodes.nodes[i]);
    }
  }
  
  return total;
}
