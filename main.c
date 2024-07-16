#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

void criarGraficoDePontos(unsigned *valores, unsigned tamanho, const char *nomeGrafico, const char *nomeEixoX, const char *nomeEixoY) {
    if (valores == NULL) {
        printf("\n--- Dados nulos obtidos ---\n");
        return;
    }

    FILE *arquivo = fopen("dados.dat", "w");
    if (arquivo == NULL) {
        fprintf(stderr, "Erro ao abrir o arquivo.\n");
        return;
    }

    // Escrever os dados no arquivo
    for (unsigned i = 0; i < tamanho; i++) {
        fprintf(arquivo, "%u %u\n", i + 1, valores[i]);
    }

    // Fechar o arquivo
    fclose(arquivo);

    sleep(1);

    // Criar o gráfico de pontos usando gnuplot
    FILE *gnuplotPipe = fopen("gnuplot_commands", "w");
    if (gnuplotPipe == NULL) {
        fprintf(stderr, "Erro ao abrir o arquivo de comandos do gnuplot.\n");
        return;
    }

    fprintf(gnuplotPipe, "set title \"%s\"\n", nomeGrafico);
    fprintf(gnuplotPipe, "set xlabel \"%s\"\n", nomeEixoX);
    fprintf(gnuplotPipe, "set ylabel \"%s\"\n", nomeEixoY);
    fprintf(gnuplotPipe, "set size 1.1, 1.0\n");

    fprintf(gnuplotPipe, "plot 'dados.dat' with points pointtype 7 pointsize 1.5, \\\n");
    fprintf(gnuplotPipe, "'dados.dat' smooth csplines with lines linestyle 2 linewidth 2 dashtype 2 notitle\n");

    fclose(gnuplotPipe);

    system("gnuplot -persistent gnuplot_commands");

    system("rm dados.dat");
    system("rm gnuplot_commands");
}
int main() {
    char nome[64];
    char eixoX[64];
    char eixoY[64];
    unsigned size;
    unsigned *dados;

    fgets(nome, 64, stdin);
    nome[strcspn(nome, "\n")] = '\0'; 

    fgets(eixoX, 64, stdin);
    eixoX[strcspn(eixoX, "\n")] = '\0'; 

    fgets(eixoY, 64, stdin);
    eixoY[strcspn(eixoY, "\n")] = '\0'; 

    scanf("%u", &size);

    dados = (unsigned *) malloc(size * sizeof(unsigned));
    if (dados == NULL) {
        fprintf(stderr, "Erro ao alocar memória.\n");
        return 1;
    }

    for (unsigned i = 0; i < size; i++) {
        scanf("%u", &dados[i]);
    }

    criarGraficoDePontos(dados, size, nome, eixoX, eixoY);

    free(dados);
    return 0;
}
