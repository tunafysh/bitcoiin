#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <malloc.h>
#include <ogcsys.h>
#include <gccore.h>
#include <wiiuse/wpad.h>
#include <network.h>

static u32 *xfb;
static GXRModeObj *rmode;


void Init() {
  
	VIDEO_Init();
	PAD_Init();
 
	rmode = VIDEO_GetPreferredMode(NULL);

	xfb = MEM_K0_TO_K1(SYS_AllocateFramebuffer(rmode));
	console_init(xfb,20,20,rmode->fbWidth,rmode->xfbHeight,rmode->fbWidth*VI_DISPLAY_PIX_SZ);
	
	VIDEO_Configure(rmode);
	VIDEO_SetNextFramebuffer(xfb);
	VIDEO_SetBlack(FALSE);
	VIDEO_Flush();
	VIDEO_WaitVSync();
	if(rmode->viTVMode&VI_NON_INTERLACE) VIDEO_WaitVSync();
}

u32 init_network() {
    int res;
    char ip[16] = {0};
	
    printf("Initializing network...\n");
    res = if_config(ip, NULL, NULL, true, 1);
    if (res < 0) {
        printf("Network initialization failed!\n");
        exit(1);
    }
    printf("Network initialized! IP address: %s\n", ip);
    return inet_addr(ip);
}

void create_socket(in_addr_t ip, int port) {
    int sockfd;
    struct sockaddr_in server_addr;

    sockfd = net_socket(AF_INET, SOCK_STREAM, IPPROTO_IP);
    if (sockfd < 0) {
        printf("Socket creation failed!\n");
        exit(1);
    }

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    server_addr.sin_addr.s_addr = ip;

    if (net_connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        printf("Socket connection failed!\n");
        net_close(sockfd);
        exit(1);
    }

    printf("Socket created at %s and port %d and connected successfully!\n", inet_ntoa(server_addr.sin_addr), port);
    net_close(sockfd);
}


int main() {
 
	Init();
	in_addr_t addr = init_network();
	create_socket(addr, 8307);

    printf("Press HOME to exit\n");

	for(;;){
        WPAD_ScanPads();
        u32 pressed = WPAD_ButtonsDown(0);
        if (pressed & WPAD_BUTTON_HOME) {
            break;
        }
		VIDEO_WaitVSync();
	}
	
	return 0;
}
