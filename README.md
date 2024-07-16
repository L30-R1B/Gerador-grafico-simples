# Gráfico de Pontos com Linha Curva Tracejada em C

Este projeto em C gera um gráfico de pontos a partir de um conjunto de dados fornecido pelo usuário. Os pontos são plotados em azul e uma linha curva tracejada é desenhada sobre eles em vermelho. O gnuplot é utilizado para a criação do gráfico.

## Requisitos

- GCC (ou outro compilador C)
- Gnuplot

## Compilação e Execução

1. Clone o repositório:

    ```sh
    git clone https://github.com/L30-R1B/Gerador-grafico-simples.git
    cd <seu respectivo repositório>
    ```

2. Compile o código:

    ```sh
    gcc -o grafico main.c
    ```

3. Execute o programa:

    ```sh
    ./grafico
    ```

## Uso

Ao executar o programa, você será solicitado a inserir o nome do gráfico, o nome dos eixos X e Y, o tamanho do conjunto de dados e os valores dos dados. O programa então criará um arquivo de comandos para o gnuplot e gerará o gráfico.

### Exemplo de Entrada

```plaintext
Digite o nome do gráfico:
Número de Mortes por Ano

Digite o nome do eixo X:
Ano

Digite o nome do eixo Y:
Número de Mortes

Digite o tamanho dos dados:
5

Digite os dados:
100
150
130
180
160
