#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "node.h"

void printTree(node_t* node, const char* indent) {
  int packetVersion = node->packetVersion;
  if (node->type == LiteralValue) {
    int literalValue = node->literal.value;
    printf("%s(\n%sPacketVersion: %i,\n%sLiteralValue: %i\n%s)", indent, indent, packetVersion, indent, literalValue, indent);
  } else if (node->type == Operator) {
    int typeId = node->op.typeId;
    printf("%s{\n%sPacketVersion: %i,\n%sTypeId: %i,\n%sNodesSize: %i,\n%sNodes: [\n", indent, indent, packetVersion, indent, typeId, indent, node->op.nodes.len, indent);
    node_t** otherNodes = node->op.nodes.nodes;
    for (int i = 0; i < node->op.nodes.len; i++) {
      node_t* otherNode = otherNodes[i];
      char indent2[strlen(indent) + 2 + 1];
      sprintf(indent2, "%s  ", indent);
      if (otherNode == NULL) {
        printf("%snil\n", indent);
      } else {
        printTree(otherNode, indent2);
      }
    }
    printf("%s]\n}", indent);
  }
  printf("\n");
}

void freeTree(node_t* node) {
  if (node->type == Operator) {
    for (int i = 0; i < node->op.nodes.len; i++) {
      freeTree(node->op.nodes.nodes[i]);
    }
    free(node);
  }
}
