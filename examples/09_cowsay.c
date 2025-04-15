// Compile using: cc -c 09_cowsay.c -o libcowsay.a

#include <stdio.h>
#include <string.h>

void print_border(int len) {
  printf(" ");
  for (int i = 0; i < len + 2; i++)
    printf("-");
  printf("\n");
}

void print_cow() {
  printf("        \\   ^__^\n");
  printf("         \\  (oo)\\_______\n");
  printf("            (__)\\       )\\/\\\n");
  printf("                ||----w |\n");
  printf("                ||     ||\n");
}

void cowsay(char *message) {
  int len = strlen(message);
  print_border(len);
  printf("< %s >\n", message);
  print_border(len);
  print_cow();
}
