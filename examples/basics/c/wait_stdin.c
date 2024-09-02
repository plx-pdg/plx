#include <stdio.h>
#include <unistd.h>

int main(void)
{
	char line[50];
	while (read(STDIN_FILENO, &line, sizeof(line) - 1) > 0) {
		line[49] = 0;
		printf("%s\n", line);
	}
}
