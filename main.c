#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

typedef struct string_counter string_counter_t;

extern void hello();

extern string_counter_t *string_counter_new(void);

extern void string_counter_free(string_counter_t *);

extern void string_counter_insert(const string_counter_t *, const char *);

extern uint64_t string_counter_count(const string_counter_t *, const char *);

int main() {
  uint64_t cnt;
  hello();
  string_counter_t *sc = string_counter_new();
  cnt = string_counter_count(sc, "jabber");
  printf("%" PRId64 "\n", cnt);
  string_counter_insert(sc, "jabber");
  cnt = string_counter_count(sc, "jabber");
  printf("%" PRId64 "\n", cnt);
  string_counter_free(sc);
  return 0;
}
