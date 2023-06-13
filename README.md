# Agrupamento de dados - DBSCAN e Kmeans

Este repositório contém o trabalho final da disciplina de mineração de dados. O objetivo é a implementação e análise de resultados relacionados a mineração de dados utilizando algumas bases de dados.

No contexto foram abordadas duas técnicas de **agrupamento(clustering)**, **DBSCAN** e **Kmeans**, estes foram implementados sem uso de bibliotecas terceiras na linguagem **Rust**, por ser uma linguagem bastante eficiente se comparado a outras linguagens usadas para data mining.

## Início rápido

Para **execução** do projeto no sistema operacional windows ou linux/Fedora 30-36, executar o respectivo binário rust compilado na pasta bin.
  - A execução irá realizar todas as etapas, pré-processamento, data mining e pós processamento slavando resultados na pasta postprocessing

Para manipulações ao **código fonte** para compilação e execução utilizando ferramentas do rust:
  - Instale Rust em sua maquina
  - Clone este repositório
  - Na pasta do clone execute no terminal o comando `$cargo run` para compilação e execução do projeto

### Dependencias

* `rust` 1.70.0
* csv = "1.2.2"
* plotters = "0.3.3"

### Alguns resultados