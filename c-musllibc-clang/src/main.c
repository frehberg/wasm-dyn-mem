#include <stdlib.h>
#include <string.h>

#define CHUNK_SIZE (64*1024)
int
main(int argc, char **argv) {
  char* chunk = malloc(CHUNK_SIZE);
  strncpy(chunk, argv[0], CHUNK_SIZE-1);

  // do-something with the chunk, to prevent it is optimized out
   if (!strcmp("foobar", chunk)) {
    return EXIT_FAILURE;
  } else {
   return EXIT_SUCCESS;
  }
}
