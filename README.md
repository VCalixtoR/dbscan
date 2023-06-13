# Agrupamento de dados - DBSCAN e Kmeans

Este repositório contém o trabalho final da disciplina de mineração de dados. O objetivo é a implementação e análise de resultados relacionados a mineração de dados utilizando algumas bases de dados.

No contexto foram abordadas duas técnicas de **agrupamento(clustering)**, **DBSCAN** e **Kmeans**, estes foram implementados sem uso de bibliotecas terceiras na linguagem **Rust**, por ser uma linguagem bastante eficiente se comparado a outras linguagens usadas para data mining.

## Início rápido

Para **execução** do projeto no sistema operacional windows ou linux/Fedora 30-36, executar o respectivo binário compilado na pasta bin **pelo terminal**, necessário a descompactação.
  - A execução irá realizar todas as etapas, pré-processamento, data mining e pós processamento salvando resultados na pasta postprocessing dpresente na pasta do executável.
  - Ex: Descompactar windows-release.zip ou fedora-release.zip seguido pela execução do arquivo executável nomeado dbscan.
  - **Atenção**: Repare que antes de processar resultados da postprocessing não estarão presentes e após serão criadas.

Para manipulações ao **código fonte** para compilação e execução utilizando ferramentas do rust:
  - Instale Rust em sua maquina
  - Clone este repositório
  - Na pasta do clone execute no terminal o comando `$cargo run` para compilação e execução do projeto

### Dependencias

* rust 1.70.0
* csv = "1.2.2"
* plotters = "0.3.3"

### Alguns resultados

<img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Db_Circular_Generated_xXy.png" width="350" height="350" > <img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Km_Circular_Generated_xXy.png" width="350" height="350">

<img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Db_Clustering_gmm_WeightXHeight.png" width="350" height="350"> <img src="https://github.com/VCalixtoR/dbscan/blob/main/postprocessing/Km_Clustering_gmm_WeightXHeight.png" width="350" height="350">
