#include "node.h"
#include "part1.h"

#include <stdio.h> // TODO: remove

int countPacketVersions(node_t* node) {
  int total = 0;
  
  printf("Node: %p\n", node);
  // printf("packetVersion: %i\n", node->packetVersion);
  
  total += node->packetVersion;
  if (node->type == Operator) {
    // printf("> nodes len = %i\n", node->op.nodes.len);
    for (int i = 0; i < node->op.nodes.len; i++) {
      total += countPacketVersions(node->op.nodes.nodes[i]);
    }
  }
  
  return total;
}
