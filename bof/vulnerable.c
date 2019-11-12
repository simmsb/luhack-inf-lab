#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

void target() {
  srand(time(NULL));
  int port = (rand() % 5000) + 1000;
  if (fork() == 0) {
    int host_sock = socket(AF_INET, SOCK_STREAM, 0);

    struct sockaddr_in host_addr = {.sin_family = AF_INET,
                                    .sin_port = htons(port),
                                    .sin_addr.s_addr = INADDR_ANY};

    bind(host_sock, (struct sockaddr *)&host_addr, sizeof(host_addr));

    listen(host_sock, 0);

    int client_sock = accept(host_sock, NULL, NULL);

    dup2(client_sock, 0);
    dup2(client_sock, 1);
    dup2(client_sock, 2);

    execl("/bin/sh", "sh", (char *)NULL);
  } else {
    printf("Take a look at %d\n", port);
    exit(0);
  }
}

void inner() {
  char buf[512];
  printf("%p\n", (void *)buf);
  fgets(buf, 4096, stdin);
}

int main(void) { inner(); }
