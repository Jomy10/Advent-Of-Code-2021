#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include "parsing.h"

// Convert input as hexadecimal string to binary string
const char* hexToBin(const char* hex, int hexLen) {
  char* bin = malloc(hexLen *  4 + 1);
  
  for (int i = 0; i < hexLen; i++) {
    switch(hex[i]) {
      case '0': strcpy(bin + i*4, "0000"); break;
      case '1': strcpy(bin + i*4, "0001"); break;
      case '2': strcpy(bin + i*4, "0010"); break;
      case '3': strcpy(bin + i*4, "0011"); break;
      case '4': strcpy(bin + i*4, "0100"); break;
      case '5': strcpy(bin + i*4, "0101"); break;
      case '6': strcpy(bin + i*4, "0110"); break;
      case '7': strcpy(bin + i*4, "0111"); break;
      case '8': strcpy(bin + i*4, "1000"); break;
      case '9': strcpy(bin + i*4, "1001"); break;
      case 'A': strcpy(bin + i*4, "1010"); break;
      case 'B': strcpy(bin + i*4, "1011"); break;
      case 'C': strcpy(bin + i*4, "1100"); break;
      case 'D': strcpy(bin + i*4, "1101"); break;
      case 'E': strcpy(bin + i*4, "1110"); break;
      case 'F': strcpy(bin + i*4, "1111"); break;
      default:
        printf("Invalid hexadecimal character \"%c\"\n", hex[i]);
    }
  }
  
  return bin;
}

// Binary string to integer
uint64_t binToInt(const char* bin, int binLen) {
  int i = 0;
  uint64_t total = 0;
  while (i < binLen) {
    char bit = bin[binLen - i - 1];
    uint64_t bitAsInt = bit - '0';
    total += bitAsInt * pow(2, i);
    
    i++;
  }
  
  return total;
}

// Read in the packet number
int readPacketVersion(transm_t* transm) {
  uint64_t v = binToInt(transm->bin + transm->ptr, 3);
  transm->ptr += 3;
  return v;
}

// Read in the packet type id
int readPacketTypeId(transm_t* transm) {
  int t = binToInt(transm->bin + transm->ptr, 3);
  transm->ptr += 3;
  return t;
}

// Retuns wheter the next literal sequence should be read
bool readLiteralBits(transm_t* transm, char* buf) {
  int first = binToInt(transm->bin + transm->ptr, 1);
  transm->ptr += 1;
  for (int i = 0; i < 4; i++) {
    buf[i] = *(transm->bin + transm->ptr + i);
  }
  
  transm->ptr += 4;
  
  return first;
}

// Returns the value of the literal
uint64_t readLiteral(transm_t* transm) {
  int totalCap = 16;
  int totalPtr = 0;
  // is a string containing 1 or 0 (as ASCII)
  char* total = malloc(totalCap + 1);
  char buf[4];
  while (readLiteralBits(transm, buf)) {
    if (totalCap == totalPtr) {
      totalCap += 16;
      total = realloc(total, totalCap + 1);
    }
    for (int i = 0; i < 4; i++) {
      total[totalPtr++] = buf[i];
    }
  }
  if (totalCap == totalPtr) {
    totalCap += 16;
    total = realloc(total, totalCap + 1);
  }
  for (int i = 0; i < 4; i++) {
    total[totalPtr++] = buf[i];
  }
  
  uint64_t literal = binToInt(total, totalPtr);
  
  free(total);
  
  return literal;
}

int readLengthTypeId(transm_t* transm) {
  int lengthTypeId = binToInt(transm->bin + transm->ptr, 1);
  transm->ptr += 1;
  
  return lengthTypeId;
}

// Read length of sub packets in bits
int readLengthSubPackets(transm_t* transm) {
  int lengthOfSubPackets = binToInt(transm->bin + transm->ptr, 15);
  transm->ptr += 15;
  return lengthOfSubPackets;
}

// Read number of sub packets
int readNumSubPackets(transm_t* transm) {
  int num = binToInt(transm->bin + transm->ptr, 11);
  transm->ptr += 11;
  return num;
}

node_t* readSubPacket(transm_t* transm) {
  return parse(transm);
}

struct Nodes readOperatorValue(int code, transm_t* transm) {
  struct Nodes nodes;
  int nodesCap = 2;
  nodes.nodes = malloc(sizeof(void*) * nodesCap);
  nodes.len = 0;
  
  int lengthTypeId = readLengthTypeId(transm);
  if (lengthTypeId == 0) {
    int lengthSubPacketsBits = readLengthSubPackets(transm);
    int startPtr = transm->ptr;
    while(transm->ptr < startPtr + lengthSubPacketsBits) {
      // Bound check
      if (nodes.len == nodesCap) {
        nodesCap *= 2;
        nodes.nodes = realloc(nodes.nodes, sizeof(void*) * nodesCap);
      }
      // assign sub packet to nodes->nodes
      readNextSubPacket(&nodes, transm);
    }

    if (transm->ptr > startPtr + lengthSubPacketsBits) {
      printf("ERROR: ptr is %i, should be %i\n", transm->ptr, startPtr + lengthSubPacketsBits);
      exit(1);
    }
  } else {
    int numSubPackets = readNumSubPackets(transm);
    while (numSubPackets > 0) {
      // Bound check
      if (nodes.len == nodesCap) {
        nodesCap *= 2;
        nodes.nodes = realloc(nodes.nodes, sizeof(void*) * nodesCap);
      }
      // assign sub packet to nodes->nodes
      readNextSubPacket(&nodes, transm);
      numSubPackets--;
    }
  }
  
  return nodes;
}

void readNextSubPacket(struct Nodes* nodes, transm_t* transm) {
  node_t* packet = readSubPacket(transm);
  nodes->nodes[nodes->len] = packet;
  nodes->len += 1;
}

node_t* parse(transm_t* transm) {
  node_t* node = malloc(sizeof(node_t));
  
  int version = readPacketVersion(transm);
  int typeId = readPacketTypeId(transm);
  if (typeId == 4) {
    uint64_t literal = readLiteral(transm);
    node->type = LiteralValue;
    node->packetVersion = version;
    node->literal.value = literal;
  } else {
    node->type = Operator;
    node->packetVersion = version;
    node->op.typeId = typeId;
    node->op.nodes = readOperatorValue(typeId, transm);
  }
  
  return node;
}
