#ifndef AOC16_NODE_H
#define AOC16_NODE_H

enum NodeType {
  LiteralValue,
  Operator,
};

struct Nodes {
  int len;
  struct Node** nodes;
};

typedef struct Node {
  enum NodeType type;
  int packetVersion;
  union {
    struct {
      int value;
    } literal;

    struct {
      int typeId;
      struct Nodes nodes;
    } op;
  };
} node_t;

void printTree(node_t* node, const char* indent);
void freeTree(node_t* node);

#endif
