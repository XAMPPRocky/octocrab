```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_OUTPUT_SIZE 1024

struct OctocrabConfig {
    char *yew_path;
    int compile_flags;
};

int init_octocrab(struct OctocrabConfig **config) {
    struct OctocrabConfig *cfg = NULL;
    
    if (config == NULL) { return -1; }
    
    cfg = (struct OctocrabConfig*)malloc(sizeof(struct OctocrabConfig));
    if (cfg == NULL) { return -2; }
    
    cfg->yew_path = NULL;
    cfg->compile_flags = 0;
    
    *config = cfg;
    return 0;
}

int setup_yew_path(struct OctocrabConfig *config, const char *path) {
    size_t path_len;
    
    if (config == NULL || path == NULL) { return -1; }
    
    path_len = strlen(path) + 1;
    config->yew_path = (char*)malloc(path_len);
    if (config->yew_path == NULL) { return -2; }
    
    strncpy(config->yew_path, path, path_len);
    config->yew_path[path_len - 1] = '\0';
    
    return 0;
}

void cleanup_octocrab(struct OctocrabConfig *config) {
    if (config == NULL) { return; }
    
    if (config->yew_path != NULL) {
        free(config->yew_path);
    }
    
    free(config);
}

int main(void) {
    struct OctocrabConfig *cfg = NULL;
    int ret;
    
    ret = init_octocrab(&cfg);
    if (ret != 0) { goto cleanup; }
    
    ret = setup_yew_path(cfg, "/path/to/yew");
    if (ret != 0) { goto cleanup; }
    
    printf("Configured Yew path: %s\n", cfg->yew_path);

cleanup:
    cleanup_octocrab(cfg);
    return ret;
}
```