#include <pthread.h>
#include <stdio.h>
int main()
{
  pthread_attr_t a;
  size_t s;
  pthread_attr_init(&a);
  pthread_attr_getstacksize(&a, &s);
  printf("pthread default stack (macOS): %zu = %.0f KiB\n", s, s / 1024.0);
}
