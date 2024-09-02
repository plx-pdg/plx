#include <stdio.h>
#include <signal.h>
#include <unistd.h>

void handle_signal(int signal)
{
	if (signal == SIGTERM) {
		printf("Received SIGTERM, but ignoring it.\n");
	} else if (signal == SIGINT) {
		printf("Received SIGINT, but ignoring it.\n");
	}
}

int main()
{
	signal(SIGTERM, handle_signal);
	signal(SIGINT, handle_signal);
	while (1) {
		sleep(1);
	}
}
