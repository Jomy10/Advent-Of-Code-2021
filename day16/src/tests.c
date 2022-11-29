#include <stdio.h>
#include <strings.h>
#include "parsing.h"
#include "part1.h"
#include "part2.h"

int testVersionTotal(int id, const char* hex, int expected);
void testVT(int testCode, const char* hex, int expected, int* failedTests, int* failedCount);

int testEvaluation(int id, const char* hex, int expected);
void testEval(int id, const char* hex, int expected, int* failedTests, int* failedCount);

int main() {
  int failedCount = 0;
  int failedTests[100];
  
  testVT(0, "8A004A801A8002F478", 16, failedTests, &failedCount);
  testVT(1, "620080001611562C8802118E34", 12, failedTests, &failedCount);
  testVT(2, "C0015000016115A2E0802F182340", 23, failedTests, &failedCount);
  testVT(3, "A0016C880162017C3686B18A3D4780", 31, failedTests, &failedCount);
  
  testEval(4, "C200B40A82", 3, failedTests, &failedCount);
  testEval(5, "04005AC33890", 54, failedTests, &failedCount);
  testEval(6, "880086C3E88112", 7, failedTests, &failedCount);
  testEval(7, "CE00C43D881120", 9, failedTests, &failedCount);
  testEval(8, "D8005AC2A8F0", 1, failedTests, &failedCount);
  testEval(9, "F600BC2D8F", 0, failedTests, &failedCount);
  testEval(10, "9C005AC2F8F0", 0, failedTests, &failedCount);
  testEval(11, "9C0141080250320F1802104A08", 1, failedTests, &failedCount);
  
  int totalTests = 12;
  
  printf("%i tests succeeded, %i failed\n", totalTests - failedCount, failedCount);
  
  fflush(stdout);
  
  return 0;
}

void testVT(int testCode, const char* hex, int expected, int* failedTests, int* failedCount) {
  if (!testVersionTotal(testCode, hex, expected)) {
    failedTests[*failedCount++] = testCode;
  }
}

int testVersionTotal(int id, const char* hex, int expected) {
  const char* bin = hexToBin(hex, strlen(hex));

  transm_t transm = {
    .bin = bin,
    .ptr = 0,
  };
  
  node_t* tree = parse(&transm);
  
  int packetVersion = countPacketVersions(tree);
  
  if (expected != packetVersion) {
    printf("\x1b[31m[FAIL %i]\x1b[0m Expected %i, got %i", id, expected, packetVersion);
    return 0;
  } else {
    printf("\x1b[32m[SUCCEDED %i]\x1b[0m\n", id);
  }
  
  freeTree(tree);
  
  return 1;
}

void testEval(int id, const char* hex, int expected, int* failedTests, int* failedCount) {
  if (!testEvaluation(id, hex, expected)) {
    failedTests[*failedCount++] = id;
  }
}

int testEvaluation(int id, const char* hex, int expected) {
  const char* bin = hexToBin(hex, strlen(hex));
  
  transm_t transm = {
    .bin = bin,
    .ptr = 0,
  };
  
  node_t* tree = parse(&transm);
  
  uint64_t result = evalExpression(tree);
  
  if (expected != result) {
    printf("\x1b[31m[FAIL %i]\x1b[0m Expected %i, got %llu\n", id, expected, result);
  } else {
    printf("\x1b[32m[SUCCEDED %i]\x1b[0m\n", id);
  }
  
  freeTree(tree);
  
  return 1;
}
