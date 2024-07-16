# Gráfico com Linha Curva Tracejada em C

Este projeto em C gera um gráfico a partir de um conjunto de dados fornecido pelo usuário. Uma linha curva tracejada é desenhada sobre as cordenadas em azul. O gnuplot é utilizado para a criação do gráfico.

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
    gcc -o grafico grafico.c
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

Digite os dados (x e y consecutivamente):
1 3 
2 5
3 7
4 9
5 11
