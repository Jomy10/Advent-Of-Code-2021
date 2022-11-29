#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>
#include "parsing.h"

// Convert input as hexadecimal string to binary string
const char* hexToBin(const char* hex, int hexLen) {
  char* bin = malloc(hexLen *  4);
  
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
        printf("Invalid hexadecimal character %c\n", hex[i]);
    }
  }
  
  return bin;
}

// Binary string to integer
int binToInt(const char* bin, int binLen) {
  int i = 0;
  int total = 0;
  while (i < binLen) {
    char bit = bin[binLen - i - 1];
    int bitAsInt = bit - '0';
    total += bitAsInt * pow(2, i);
    
    i++;
  }
  
  return total;
}

// Read in the packet number
int readPacketVersion(transm_t* transm) {
  int v = binToInt(transm->bin + transm->ptr, 3);
  transm->ptr += 3;
  // printf("Version = %i\n", v);
  return v;
}

// Read in the packet type id
int readPacketTypeId(transm_t* transm) {
  int t = binToInt(transm->bin + transm->ptr, 3);
  transm->ptr += 3;
  // printf("TypeId = %i\n", t);
  return t;
}

// Retuns wheter the next literal sequence should be read
bool readLiteralBits(transm_t* transm, char* buf) {
  int first = binToInt(transm->bin + transm->ptr, 1);
  transm->ptr += 1;
  // char* next = malloc(sizeof(char) * 4);
  for (int i = 0; i < 4; i++) {
    printf("Adding (idx %i)\n", i);
    buf[i] = *(transm->bin + transm->ptr + i);
    printf("Added (idx %i)\n", i);
  }
  
  printf("Increasing ptr %i %p\n", transm->ptr, transm);
  
  fflush(stdout);
  
  transm->ptr += 4;
  
  // struct LiteralBit lb = {
  //   .readNext = first,
  //   .val = next,
  // };
  
  printf("Returning (first %i)\n", first);
  fflush(stdout);
  
  return first;
}

// Returns the value of the literal
int readLiteral(transm_t* transm) {
  int totalCap = 16;
  int totalPtr = 0;
  printf("Alocating\n");
  fflush(stdout);
  char* total = malloc(totalCap);
  printf("Alloced total: %p\n", total);
  char buf[4];
  while (readLiteralBits(transm, buf)) {
    if (totalCap == totalPtr) {
      totalCap += 16;
      total = realloc(total, totalCap);
    }
    for (int i = 0; i < 4; i++) {
      total[totalPtr++] = buf[i];
    }
  }
  printf("Read all literals\n");
  fflush(stdout);
  if (totalCap == totalPtr) {
    totalCap += 16;
    total = realloc(total, sizeof(char) * totalCap);
  }
  for (int i = 0; i < 4; i++) {
    total[totalPtr++] = buf[i];
  }
  
  int literal = binToInt(total, totalPtr);
  
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
  // printf("Length in bits of sub packets = %i\n", lengthOfSubPackets);
  return lengthOfSubPackets;
}

// Read number of sub packets
int readNumSubPackets(transm_t* transm) {
  int num = binToInt(transm->bin + transm->ptr, 11);
  transm->ptr += 11;
  // printf("Number of sub packets = %i\n", num);
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
  // printf("== lengthTypeId = %i ==\n", lengthTypeId);
  if (lengthTypeId == 0) {
    int lengthSubPacketsBits = readLengthSubPackets(transm);
    int startPtr = transm->ptr;
    // printf("- bit len = %i\n", lengthSubPacketsBits);
    while(transm->ptr < startPtr + lengthSubPacketsBits) {
      // printf("> Reading packet (bit %i)...\n", nodes.len);
      if (nodes.len == nodesCap) {
        nodesCap *= 2;
        nodes.nodes = realloc(nodes.nodes, sizeof(void*) * nodesCap);
      }
      
      // nodes.nodes[nodes.len * sizeof(struct Node)] = readSubPacket(transm);
      // nodes.len += 1;
      readNextSubPacket(&nodes, transm);
      
      // printf("Got node %p\n", nodes.nodes[(nodes.len - 1)]);
    }

    if (transm->ptr > startPtr + lengthSubPacketsBits) {
      printf("ERROR: ptr is %i, should be %i\n", transm->ptr, startPtr + lengthSubPacketsBits);
      exit(1);
    }
    
    // printf("Ptr = %i, start + len = %i\n", transm->ptr, startPtr + lengthSubPacketsBits);

    // printf("> Done (bit).\n");
  } else {
    int numSubPackets = readNumSubPackets(transm);
    // printf("- num packets = %i\n", numSubPackets);
    while (numSubPackets > 0) {
      // printf("> Reading packet (num)...\n");
      // nodes.nodes[nodes.len * sizeof(struct Node)] = readSubPacket(transm);
      // nodes.len += 1;
      readNextSubPacket(&nodes, transm);
      // printf("Got node %p\n", nodes.nodes[(nodes.len - 1)]);
      numSubPackets--;
    }
    // printf("> Done.\n");
  }
  
  return nodes;
}

void readNextSubPacket(struct Nodes* nodes, transm_t* transm) {
  node_t* packet = readSubPacket(transm);
  // printf("Packet: %p\n", packet);
  nodes->nodes[nodes->len] = packet;
  // printf("Packet2: %p\n", nodes->nodes[nodes->len]);
  nodes->len += 1;
}

node_t* parse(transm_t* transm) {
  node_t* node = malloc(sizeof(struct Node));
  
  if (node == NULL) {
    printf("Allocation error\n");
    exit(1);
  }
  
  int version = readPacketVersion(transm);
  int typeId = readPacketTypeId(transm);
  if (typeId == 4) {
    int literal = readLiteral(transm);
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
