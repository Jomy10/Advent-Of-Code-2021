#ifndef AOC16_PARSING_H
#define AOC16_PARSING_H

#include <stdint.h>
#include <stdbool.h>

#include "transmission.h"
#include "node.h"

const char* hexToBin(const char* hex, int hexLen);
uint64_t binToInt(const char* bin, int binLen);
int readPacketVersion(transm_t* transm);
int readPacketTypeId(transm_t* transm);
bool readLiteralBits(transm_t* transm, char buf[4]);
uint64_t readLiteral(transm_t* transm);
int readLengthTypeId(transm_t* transm);
int readLengthSubPackets(transm_t* transm);
int readNumSubPackets(transm_t* transm);
node_t* readSubPacket(transm_t* transm);
struct Nodes readOperatorValue(int code, transm_t* transm);
node_t* parse(transm_t* transm);
void readNextSubPacket(struct Nodes* nodes, transm_t* transm);

#endif
