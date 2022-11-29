#include <stdlib.h>
#include <stdio.h>

char* readInput(char* fileName) {
  FILE* ptr;
  char ch;
  
  int strCap = 1024;
  int strLen = 0;
  char* str = (char*) malloc(1024);
  
  ptr = fopen("input.txt", "r");
  
  if (ptr == NULL) {
    printf("Input file can't be opened\n");
  }
  
  do {
    ch = fgetc(ptr);
    str[strLen] = ch;

    if (++strLen == strCap) {
      strCap += 1024;
      str = (char*) realloc(str, strCap);
    }
  } while(ch != EOF);
  
  fclose(ptr);
  
  return str;
}
