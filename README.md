# Agrupamento de dados - DBSCAN e Kmeans

*Vinícius Calixto e Wellington Cesar*

Este repositório contém o trabalho final da disciplina de mineração de dados. O objetivo é a implementação e análise de resultados relacionados a mineração de dados utilizando algumas bases de dados.

No contexto foram abordadas duas técnicas de **agrupamento(clustering)**, **DBSCAN** e **Kmeans**, estes foram implementados sem uso de bibliotecas terceiras na linguagem **Rust**, por ser uma linguagem bastante eficiente se comparado a outras linguagens usadas para data mining.

Foram gerados binários para o **Windows e Fedora 38**, que podem ser executados sem a necessidade da instalção da linguagem Rust, para isso seguir as etapas na seção posterior.

## Início rápido

Para **execução** do projeto no sistema operacional windows ou linux/Fedora 30-36, executar o respectivo binário compilado na pasta bin **pelo terminal na pasta raiz do projeto**, caso for executado na pasta /bin será retornado problemas de diretório.
  - A execução irá realizar todas as etapas, pré-processamento, data mining e pós processamento salvando resultados na pasta postprocessing presente na pasta do executável.
  - Ex: Acessar no terminal a pasta root e escrever o comando:
  - - Para Fedora: `./bin/fedora`
  - - Para Windows: `.\bin\windows.exe`
  - **Atenção**: O pacote do fedora "fontconfig-devel" pode ser necessário para a compilação nesse sistema operacional.
  - Para visualizar a criação de arquivos na etapa de pós-processamento apagar os arquivos dentro da pasta postprocessing e executar o binário.

Para manipulações ao **código fonte** utilizando ferramentas do rust:
  - Instale Rust e Cargo(Ferramenta de gerenciamento do Rust) em sua maquina
  - Clone este repositório
  - Na pasta do clone execute no terminal o comando `$cargo run` para compilação e execução do projeto

### Dependencias

* rust 1.70.0
* csv = "1.2.2"
* plotters = "0.3.3"

### Alguns resultados

<img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Db_Circular_Generated_xXy.png" width="350" height="350" > <img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Km_Circular_Generated_xXy.png" width="350" height="350">

<img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Db_Clustering_gmm_WeightXHeight.png" width="350" height="350"> <img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Km_Clustering_gmm_WeightXHeight.png" width="350" height="350">
