#include <stdio.h>
#ifdef _WINDOWS
#include <windows.h>
#else
#include <unistd.h>
#define Sleep(x) usleep((x)*1000)
#endif

int main(void)
{
	int i = 0;
	while (1) {
		++i;
		printf("Hello %d\n", i);
		//Make sure we sleep so we don't spam too much
		Sleep(1000);
	}
}
