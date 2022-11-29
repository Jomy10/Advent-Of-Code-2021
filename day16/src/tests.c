#include <stdio.h>
#include <strings.h>
#include "parsing.h"
#include "part1.h"

int testVersionTotal(int id, const char* hex, int expected);
void testVT(int testCode, const char* hex, int expected, int* failedTests, int* failedCount);

int main() {
  int failedCount = 0;
  int failedTests[100];
  
  testVT(0, "8A004A801A8002F478", 16, failedTests, &failedCount);
  testVT(1, "620080001611562C8802118E34", 12, failedTests, &failedCount);
  testVT(2, "C0015000016115A2E0802F182340", 23, failedTests, &failedCount);
  testVT(3, "A0016C880162017C3686B18A3D4780", 31, failedTests, &failedCount);
  
  int totalTests = 4;
  
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
    printf("[FAIL %i] Expected %i, got %i", id, expected, packetVersion);
    return 0;
  } else {
    printf("[SUCCEDED %i]\n", id);
  }
  
  freeTree(tree);
  
  return 1;
}
