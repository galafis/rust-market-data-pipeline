# rust-market-data-pipeline

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Tokio](https://img.shields.io/badge/tokio-async-green.svg?style=for-the-badge)
![Polars](https://img.shields.io/badge/polars-data-orange.svg?style=for-the-badge)

**A robust Rust-based data pipeline for ingesting, processing, and storing real-time financial market data.**

---

## 🇧🇷 Descrição em Português

`rust-market-data-pipeline` é um sistema de código aberto para a ingestão, processamento e armazenamento de dados do mercado financeiro. Construído em Rust, este projeto foi projetado para ser escalável e de alto desempenho, com suporte para múltiplas fontes de dados e formatos de armazenamento.

Este é o segundo de uma série de cinco repositórios focados em trading, mercado financeiro e IA, criados para demonstrar a força do Rust em aplicações financeiras que exigem baixa latência e alta confiabilidade.

### Funcionalidades

- **Conectores de Dados:** Módulo para se conectar a APIs de mercado (ex: Alpha Vantage) e buscar dados históricos e em tempo real.
- **Processamento de Dados:** Funções para limpar, normalizar e transformar os dados brutos em um formato estruturado e pronto para análise.
- **Armazenamento Eficiente:** Suporte para armazenamento de dados no formato colunar Apache Parquet, ideal para análise de big data.
- **Arquitetura Modular:** O pipeline é dividido em múltiplos crates, separando as responsabilidades de coleta, processamento e utilitários.

---

## 🇺🇸 English Description

`rust-market-data-pipeline` is an open-source system for ingesting, processing, and storing financial market data. Built in Rust, this project is designed to be scalable and high-performance, with support for multiple data sources and storage formats.

This is the second in a series of five repositories focused on trading, the financial market, and AI, created to demonstrate the power of Rust in financial applications that require low latency and high reliability.

### Features

- **Data Connectors:** A module to connect to market data APIs (e.g., Alpha Vantage) and fetch historical and real-time data.
- **Data Processing:** Functions to clean, normalize, and transform raw data into a structured format ready for analysis.
- **Efficient Storage:** Support for storing data in the Apache Parquet columnar format, ideal for big data analytics.
- **Modular Architecture:** The pipeline is divided into multiple crates, separating the responsibilities of data collection, processing, and utilities.

---

## 🚀 Quick Start

### Pré-requisitos

- Rust (https://www.rust-lang.org/tools/install)
- Git
- Uma chave de API da [Alpha Vantage](https://www.alphavantage.co/support/#api-key) (gratuita)

### Instalação

1. Clone o repositório:
```bash
git clone https://github.com/your-username/rust-market-data-pipeline.git
cd rust-market-data-pipeline
```

2.  **Importante:** Abra o arquivo `examples/data_pipeline.rs` e substitua `"YOUR_API_KEY"` pela sua chave de API da Alpha Vantage.

3. Compile e execute o exemplo:
```bash
cargo run --example data_pipeline
```

### Exemplo de Saída

O exemplo irá buscar os dados diários para o ticker da IBM, processá-los e salvá-los em um arquivo Parquet.

```
Dados brutos recebidos:
shape: (100, 6)
┌────────────┬──────────┬──────────┬──────────┬──────────┬──────────┐
│ date       ┆ open     ┆ high     ┆ low      ┆ close    ┆ volume   │
│ ---        ┆ ---      ┆ ---      ┆ ---      ┆ ---      ┆ ---      │
│ str        ┆ f64      ┆ f64      ┆ f64      ┆ f64      ┆ i64      │
╞════════════╪══════════╪══════════╪══════════╪══════════╪══════════╡
│ 2024-05-23 ┆ 169.98   ┆ 171.49   ┆ 169.23   ┆ 170.83   ┆ 3456789  │
│ 2024-05-22 ┆ 170.01   ┆ 170.89   ┆ 169.5    ┆ 170.0    ┆ 2345678  │
│ ...        ┆ ...      ┆ ...      ┆ ...      ┆ ...      ┆ ...      │
│ 2024-01-02 ┆ 162.1    ┆ 163.48   ┆ 161.5    ┆ 163.0    ┆ 4567890  │
└────────────┴──────────┴──────────┴──────────┴──────────┴──────────┘
Dados processados e armazenados em data/ibm_daily.parquet
```

---

## 🏛️ Arquitetura

O sistema é projetado com uma arquitetura de pipeline, onde os dados fluem através de estágios de coleta, processamento e armazenamento.

![Arquitetura do Pipeline de Dados](https://i.imgur.com/U6R9HzkpkXEP.png)

### Crates

- `rmdp-core`: Orquestra o pipeline e contém a lógica de processamento.
- `rmdp-data`: Responsável pela conexão com fontes de dados externas.
- `rmdp-utils`: Funções utilitárias, como a configuração de logs.

---

## 🛣️ Roadmap

- [ ] Adicionar suporte para mais fontes de dados (ex: Yahoo Finance, CoinGecko).
- [ ] Implementar processamento de dados em streaming com Tokio.
- [ ] Adicionar um sistema de cache com Redis para evitar requisições repetidas.
- [ ] Integrar com um banco de dados de séries temporais (ex: InfluxDB, TimescaleDB).
- [ ] Criar um dashboard de monitoramento para o pipeline.

---

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request.

---

## 📜 Licença

Este projeto está licenciado sob a licença MIT.

---

## 👨‍💻 Autor

**Gabriel Demetrios Lafis**

*   Cientista de Dados | Analista de Dados | BI/BA
*   Formado em Análise e Desenvolvimento de Sistemas, Gestão da Tecnologia da Informação e Segurança Cibernética.

