#include <stdio.h>
#include <string.h>

#define MAX_CHAR 100
char leetmake(char c){
    //TODO transformar letra
    char ALFABETO[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    //char LEET[]     = "46<∂3ƒ&#!¿κ1mn0¶9®$+µvw%Ψ246<∂3ƒ&#!¿κ1MN0¶9®$+µVW%Ψ2";
    char LEET[]     = "46<d3f&#!jk1mn0p9r$7uvw%y246<D3F&#!JK1MN0P9R$7uvw%y2";
    char *index = strchr(ALFABETO, c);

    if(index == NULL){
        return c;
    }

    return LEET[index - ALFABETO];
}

int main(int argc, char *argv[]){
    if(argc < 2){
        printf("erro: falta argumentos");
        return 1;
    }
    char leet[MAX_CHAR];
    for (int i=0; i< strlen(argv[1]); i++){
        leet[i] = leetmake(argv[1][i]);
    }
    
    printf("%s\n", leet);
    return 0;
}    
