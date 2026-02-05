# Repository Snapshot (All text files)
- Root: `C:\Users\leopo\rust-studies`
- Generated at: 2026-02-04 09:54:37
- Git commit: abf790d
- Mode: full
- Files included: 53 (max 2000000 bytes per file, text-only heuristic)

# Table of Contents

- [Project Intelligence](#project-intelligence)
- [File Index](#file-index)
- [Files](#files)

---

# Project Intelligence

## 📦 Repository: rust-studies
- **Root**: `C:\Users\leopo\rust-studies`
- **Git commit**: abf790d
- **Generated at**: 2026-02-04 09:54:37

## 📁 Repository Tree

```
├── .cursor/
│   └── plans/
├── DOCUMENTACAO.md
├── certs-mtls/
│   ├── ca.pem
│   ├── client-cert.pem
│   ├── client-key.pem
│   ├── server-cert.pem
│   └── server-key.pem
├── repo_concat_all.md
└── scripts/
    └── concat_repo_all_text.py
```

## 🛠️ Stack Detected

- **Languages**: JavaScript/TypeScript
- **Frameworks**: Vite
- **Infrastructure**: Docker, Docker Compose

### Languages by file count
- **rust**: 28 files
- **json**: 5 files
- **markdown**: 4 files
- **toml**: 3 files
- **tsx**: 3 files
- **ts**: 3 files
- **python**: 1 files
- **powershell**: 1 files
- **html**: 1 files
- **css**: 1 files

## 🔐 Security Report

- **Files excluded by filename/path**: 1
  - Sample excluded files:
    - `chopebrain-ods-api/src/bin/gen_certs.rs`
- **Files redacted by content**: 5
  - Redacted files:
    - `chopebrain-ods-api/src/auth.rs`
    - `chopebrain-ods-api/src/config.rs`
    - `chopebrain-ods-api/src/handlers.rs`
    - `scripts/concat_repo_all_text.py`
    - `scripts/test-api.ps1`
  - Patterns detected:
    - `SECRET`: 7 occurrences
    - `PASSWORD`: 6 occurrences
    - `TOKEN`: 5 occurrences
    - `API_KEY`: 3 occurrences

## 🚀 How to Run

*(n/a)*

## 🎯 Entrypoints

*(n/a)*

## 🔐 Environment Variables (names only)

- `API_BASE` (appears 7x) - workflow-mvp/desktop/src/api.ts
- `CONCAT_ROOT` (appears 1x) - scripts/concat_repo_all_text.py
- `MAX_FILE_BYTES` (appears 1x) - scripts/concat_repo_all_text.py
- `CONCAT_ROOTS` (appears 1x) - scripts/concat_repo_all_text.py
- `CONCAT_OUT_BASENAME` (appears 1x) - scripts/concat_repo_all_text.py
- `CONCAT_OUT_PATH` (appears 1x) - scripts/concat_repo_all_text.py
- `CONCAT_MODE` (appears 1x) - scripts/concat_repo_all_text.py
- `SMART_TRUNCATE_BYTES` (appears 1x) - scripts/concat_repo_all_text.py
- `MAX_TOTAL_BYTES` (appears 1x) - scripts/concat_repo_all_text.py
- `CONCAT_SIDE_CAR_JSON` (appears 1x) - scripts/concat_repo_all_text.py

## 🌐 API Surface

*(n/a)*

## 🤖 RAG/LLM & Observability Signals

### RAG/LLM
- `scripts/concat_repo_all_text.py`
- `workflow-mvp/README.md`
- `workflow-mvp/backend/src/api.rs`
- `workflow-mvp/backend/src/main.rs`
- `workflow-mvp/backend/src/storage.rs`
- `workflow-mvp/desktop/package-lock.json`
- `workflow-mvp/desktop/src/App.tsx`

### Observability
- `scripts/concat_repo_all_text.py`

## ⚠️ Smells / Gaps

### Top TODOs/FIXMEs
- `scripts/concat_repo_all_text.py`: 14 occurrences
- `DOCUMENTACAO.md`: 2 occurrences

- **Tests present**: Yes (0 test files)
- **.env files found**: No .env files detected

### Largest files
- `scripts/concat_repo_all_text.py`: 85.8 KB
- `workflow-mvp/desktop/package-lock.json`: 80.4 KB
- `DOCUMENTACAO.md`: 25.1 KB
- `chopebrain-ods-api/src/handlers.rs`: 17.1 KB
- `workflow-mvp/desktop/src/App.tsx`: 10.8 KB
- `scripts/test-api.ps1`: 8.5 KB
- `.cursor/plans/api_rust_ods_choperia_42e63f58.plan.md`: 8.3 KB
- `workflow-mvp/backend/src/api.rs`: 6.3 KB
- `workflow-mvp/backend/src/engine/runtime.rs`: 5.6 KB
- `workflow-mvp/backend/src/compile.rs`: 5.6 KB

## File Index

| # | Path | Size (bytes) | Modified | Language |
|---|------|--------------|----------|----------|
| 1 | `.cursor/plans/api_rust_ods_choperia_42e63f58.plan.md` | 8481 | 2026-02-04T01:58:52.551730 | markdown |
| 2 | `.gitignore` | 371 | 2026-02-04T09:30:35.652031 | text |
| 3 | `DOCUMENTACAO.md` | 25679 | 2026-02-04T08:58:41.763998 | markdown |
| 4 | `chopebrain-ods-api/Cargo.toml` | 1020 | 2026-02-04T02:56:36.209245 | toml |
| 5 | `chopebrain-ods-api/README.md` | 4477 | 2026-02-04T08:55:24.102840 | markdown |
| 6 | `chopebrain-ods-api/src/auth.rs` | 4128 | 2026-02-04T02:55:50.462154 | rust |
| 7 | `chopebrain-ods-api/src/config.rs` | 4764 | 2026-02-04T02:20:40.971785 | rust |
| 8 | `chopebrain-ods-api/src/db.rs` | 1250 | 2026-02-04T02:55:33.208797 | rust |
| 9 | `chopebrain-ods-api/src/handlers.rs` | 17521 | 2026-02-04T03:11:38.982190 | rust |
| 10 | `chopebrain-ods-api/src/lib.rs` | 79 | 2026-02-04T01:59:47.895526 | rust |
| 11 | `chopebrain-ods-api/src/main.rs` | 2013 | 2026-02-04T02:55:26.514309 | rust |
| 12 | `chopebrain-ods-api/src/mtls.rs` | 3570 | 2026-02-04T02:57:40.576031 | rust |
| 13 | `scripts/concat_repo_all_text.py` | 87898 | 2026-02-04T09:54:24.476811 | python |
| 14 | `scripts/test-api.ps1` | 8693 | 2026-02-04T03:01:21.813607 | powershell |
| 15 | `workflow-mvp/README.md` | 4047 | 2026-02-04T09:28:10.859335 | markdown |
| 16 | `workflow-mvp/backend/Cargo.toml` | 652 | 2026-02-04T09:15:15.875104 | toml |
| 17 | `workflow-mvp/backend/Dockerfile` | 423 | 2026-02-04T09:18:26.540723 | text |
| 18 | `workflow-mvp/backend/src/api.rs` | 6446 | 2026-02-04T09:17:39.634305 | rust |
| 19 | `workflow-mvp/backend/src/compile.rs` | 5753 | 2026-02-04T09:10:36.304237 | rust |
| 20 | `workflow-mvp/backend/src/engine/expr.rs` | 4268 | 2026-02-04T09:09:04.893483 | rust |
| 21 | `workflow-mvp/backend/src/engine/mod.rs` | 269 | 2026-02-04T09:09:04.886761 | rust |
| 22 | `workflow-mvp/backend/src/engine/runtime.rs` | 5784 | 2026-02-04T09:15:40.500237 | rust |
| 23 | `workflow-mvp/backend/src/engine/steps/download_blob.rs` | 336 | 2026-02-04T09:07:25.274997 | rust |
| 24 | `workflow-mvp/backend/src/engine/steps/fault_end.rs` | 471 | 2026-02-04T09:08:06.429178 | rust |
| 25 | `workflow-mvp/backend/src/engine/steps/http_call.rs` | 1889 | 2026-02-04T09:08:06.393421 | rust |
| 26 | `workflow-mvp/backend/src/engine/steps/mod.rs` | 1984 | 2026-02-04T09:15:40.054937 | rust |
| 27 | `workflow-mvp/backend/src/engine/steps/publish_stream.rs` | 463 | 2026-02-04T09:08:06.437774 | rust |
| 28 | `workflow-mvp/backend/src/engine/steps/transform_json.rs` | 4514 | 2026-02-04T09:08:06.318480 | rust |
| 29 | `workflow-mvp/backend/src/engine/trace.rs` | 1930 | 2026-02-04T09:04:56.808400 | rust |
| 30 | `workflow-mvp/backend/src/engine/types.rs` | 1378 | 2026-02-04T09:04:48.144827 | rust |
| 31 | `workflow-mvp/backend/src/main.rs` | 765 | 2026-02-04T09:11:05.175393 | rust |
| 32 | `workflow-mvp/backend/src/models.rs` | 2925 | 2026-02-04T09:27:15.642977 | rust |
| 33 | `workflow-mvp/backend/src/schema.rs` | 4305 | 2026-02-04T09:05:57.343816 | rust |
| 34 | `workflow-mvp/backend/src/storage.rs` | 2763 | 2026-02-04T09:05:27.217034 | rust |
| 35 | `workflow-mvp/backend/src/validate.rs` | 4750 | 2026-02-04T09:16:11.011471 | rust |
| 36 | `workflow-mvp/desktop/index.html` | 397 | 2026-02-04T09:19:34.247253 | html |
| 37 | `workflow-mvp/desktop/package-lock.json` | 82366 | 2026-02-04T09:30:16.473742 | json |
| 38 | `workflow-mvp/desktop/package.json` | 718 | 2026-02-04T09:19:34.248691 | json |
| 39 | `workflow-mvp/desktop/src-tauri/Cargo.toml` | 529 | 2026-02-04T09:20:27.645706 | toml |
| 40 | `workflow-mvp/desktop/src-tauri/build.rs` | 42 | 2026-02-04T09:20:01.074515 | rust |
| 41 | `workflow-mvp/desktop/src-tauri/src/lib.rs` | 266 | 2026-02-04T09:20:27.645706 | rust |
| 42 | `workflow-mvp/desktop/src-tauri/src/main.rs` | 180 | 2026-02-04T09:20:01.090338 | rust |
| 43 | `workflow-mvp/desktop/src-tauri/tauri.conf.json` | 564 | 2026-02-04T09:20:00.852526 | json |
| 44 | `workflow-mvp/desktop/src/App.tsx` | 11017 | 2026-02-04T09:27:35.521524 | tsx |
| 45 | `workflow-mvp/desktop/src/Designer.tsx` | 3635 | 2026-02-04T09:28:53.853294 | tsx |
| 46 | `workflow-mvp/desktop/src/api.ts` | 3438 | 2026-02-04T09:22:44.259441 | ts |
| 47 | `workflow-mvp/desktop/src/demoWorkflow.ts` | 1721 | 2026-02-04T09:23:26.646700 | ts |
| 48 | `workflow-mvp/desktop/src/index.css` | 136 | 2026-02-04T09:19:34.488578 | css |
| 49 | `workflow-mvp/desktop/src/main.tsx` | 246 | 2026-02-04T09:19:34.489791 | tsx |
| 50 | `workflow-mvp/desktop/tsconfig.json` | 583 | 2026-02-04T09:19:34.484529 | json |
| 51 | `workflow-mvp/desktop/tsconfig.node.json` | 244 | 2026-02-04T09:19:34.486096 | json |
| 52 | `workflow-mvp/desktop/vite.config.ts` | 224 | 2026-02-04T09:19:34.247253 | ts |
| 53 | `workflow-mvp/docker-compose.yml` | 245 | 2026-02-04T09:18:16.134119 | yaml |
---

# Files

## [1] .cursor/plans/api_rust_ods_choperia_42e63f58.plan.md

````markdown
# FILE: .cursor/plans/api_rust_ods_choperia_42e63f58.plan.md
# FULL: C:\Users\leopo\rust-studies\.cursor\plans\api_rust_ods_choperia_42e63f58.plan.md
# SIZE: 8481 bytes
# MTIME: 2026-02-04T01:58:52.551730
# NOTE: Concatenated snapshot for review
---
name: API Rust ODS Choperia
overview: Plano para criar uma API em Rust (Axum ou Actix) que consulte o ODS da choperia com JWT, mTLS (certificados autoassinados gerados em Rust) e uso do mesmo .env do projeto Python, incluindo dois endpoints de hambúrgueres espelhando a lógica dos scripts Python.
todos: []
isProject: false
---

# API Rust ODS Choperia — Plano de implementação

## Contexto

- **Repositório alvo:** projeto que já possui (ou terá) o `.env` na raiz (com ODS_*), scripts Python de hambúrgueres e `app/db.py` / `app/config.py`. A API Rust será criada em um subdiretório (ex.: `chopebrain-ods-api/` ou `api-rust/`).
- **.env na raiz:** já contém `ODS_HOST`, `ODS_USER`, `ODS_PASSWORD`, `ODS_NAME`, `ODS_SSL_CA`. Incluir suporte a `ODS_PORT` (opcional; padrão 3306). Para JWT/mTLS: adicionar `JWT_SECRET`, `JWT_EXPIRATION_DAYS` (e opcionalmente caminho dos certs mTLS).
- **Referências Python (no repo onde o “outro Cursor” rodar):**
  - [scripts/contar_hamburgueres_vendidos_mes.py](scripts/contar_hamburgueres_vendidos_mes.py) — hambúrgueres vendidos no mês (item por `itenspedido.datahoracadastro`).
  - [scripts/identificar_hamburgueres_comanda_antiga.py](scripts/identificar_hamburgueres_comanda_antiga.py) — hambúrgueres do mês em pedidos abertos em mês anterior.
  - [app/db.py](app/db.py) e [app/config.py](app/config.py) — padrão de conexão ODS e leitura do mesmo `.env`.

---

## 1. Estrutura do projeto Rust

- **Local:** subdiretório na raiz do repo (ex.: `chopebrain-ods-api/`).
- **Crate:** binário da API + (opcional) binário ou subcomando para gerar certificados.
- **Carregamento do .env:** ao iniciar, o binário deve carregar o `.env` **da raiz do repositório** (não do diretório do crate). Opções:
  - Variável de ambiente `WORK_DIR` apontando para a raiz, ou
  - Procurar `.env` no diretório atual e em diretórios pais (ex.: até encontrar `Cargo.toml` da raiz ou um `.env` que contenha `ODS_HOST`).

**Dependências sugeridas (Cargo.toml):**

- **Web:** `axum` (ou `actix-web`).
- **Env:** `dotenvy` (compatível com o mesmo `.env` do Python).
- **MySQL:** `sqlx` com runtime `tokio` e feature `runtime-tokio-rustls` (ou `runtime-tokio-native-tls`) para SSL; usar `ODS_SSL_CA` como CA na conexão.
- **JWT:** `jsonwebtoken`; expiração e secret via env (`JWT_SECRET`, `JWT_EXPIRATION_DAYS`).
- **mTLS / TLS:** `rustls`, `rustls-pemfile`, `tokio-rustls`; servidor HTTPS com client auth.
- **Gerador de certs:** `rcgen` para CA, certificado do servidor e do cliente; salvar PEMs em `./certs-mtls/` (ou caminho configurável).

---

## 2. Variáveis de ambiente

Ler do mesmo `.env` da raiz:

| Variável | Uso |

|----------|-----|

| `ODS_HOST`, `ODS_PORT` (opcional, default 3306), `ODS_USER`, `ODS_PASSWORD`, `ODS_NAME` | Conexão MySQL ODS |

| `ODS_SSL_CA` | Caminho para o CA (ex.: `./certs/DigiCertGlobalRootCA.crt.pem`) para conexão SSL ao MySQL |

| `JWT_SECRET` | Assinatura do JWT |

| `JWT_EXPIRATION_DAYS` | Expiração do token em dias |

| (Opcional) `MTLS_SERVER_CERT`, `MTLS_SERVER_KEY`, `MTLS_CA_CERT` | Caminhos dos certs mTLS do servidor e CA; se não definidos, pode-se usar certs em `./certs-mtls/` por padrão |

---

## 3. Autenticação

- **Login:** `POST /auth/login` ou `POST /api/auth/login` com body JSON, por exemplo `{ "username": "...", "password": "..." }`. Alternativa: um único “segredo” configurável por env (ex.: `AUTH_SECRET` ou par username/password fixo) para simplificar.
  - Validar credenciais e, se OK, retornar `{ "token": "<JWT>" }`.
- **Proteção dos endpoints:** middleware que exige header `Authorization: Bearer <token>`, valida o JWT com `JWT_SECRET` e rejeita com 401 se inválido ou expirado.

---

## 4. mTLS (servidor HTTPS + certificado de cliente)

- **Servidor:** configurar TLS com rustls usando:
  - Certificado e chave do servidor (gerados pelo gerador de certs).
  - CA para validar certificados de cliente; configurar para **exigir** client certificate (mTLS).
- **Gerador de certificados (Rust):**
  - Um binário ou subcomando (ex.: `cargo run --bin gen-certs` ou `cargo run -- certs`) que:

    1. Gera CA autoassinada.
    2. Gera certificado do servidor assinado pela CA.
    3. Gera certificado do cliente assinado pela CA.

  - Salva em pasta configurável (ex.: `./certs-mtls/`): `ca.pem`, `server-cert.pem`, `server-key.pem`, `client-cert.pem`, `client-key.pem`.
  - Documentar no README: uso no Postman (Client Certificate: cert + private key) e variáveis de ambiente para caminhos dos certs, se aplicável.

---

## 5. Endpoints de hambúrgueres

Regra de “hambúrguer” (espelhar Python): categoria ODS (`etiquetas.descricao` = 'Hamburger' ou 'Hamburguer') **ou** nome do produto com palavras-chave (hamburguer, burger, cheeseburger, bacon burger, frango burger, artesanal burger, etc.).

- **POST /api/ods/hamburgueres-vendidos**  
  - Body: `{ "mes": "YYYY-MM" }` (ex.: `"2026-01"`).
  - **Lógica:** itens de pedido com `itenspedido.datahoracadastro` no mês dado; filtrar hambúrgueres por categoria e por nome (mesma regra dos scripts Python).  
  - **Retorno JSON:** totais (quantidade, valor, ticket médio), por categoria, por produto (ex.: top 10), lista de itens (data_venda_item, pedido, produto, categoria, quantidade, valor).  
  - **Referência de implementação/SQL:** [scripts/contar_hamburgueres_vendidos_mes.py](scripts/contar_hamburgueres_vendidos_mes.py).

- **POST /api/ods/hamburgueres-comandas-antigas**  
  - Body: `{ "mes": "YYYY-MM" }`.  
  - **Lógica:** mesmos itens hambúrguer do mês, mas apenas onde o pedido foi aberto em mês anterior (`pedidos.dataabertura` em ano/mês &lt; mês do item).  
  - **Retorno JSON:** totais, por produto, por pedido (id pedido, data abertura/fechamento, itens, quantidade, valor), detalhamento (data_venda_item, pedido, data_abertura, dias_diferenca, produto, quantidade, valor).  
  - **Referência:** [scripts/identificar_hamburgueres_comanda_antiga.py](scripts/identificar_hamburgueres_comanda_antiga.py) e query com `DATEDIFF(ip.datahoracadastro, p.dataabertura)` e filtros YEAR/MONTH no item e no pedido.

Ambos os endpoints devem exigir **JWT** (e, se o servidor estiver com mTLS, também certificado de cliente no Postman).

---

## 6. Conexão MySQL ODS

- Usar **sqlx** com pool assíncrono.
- String de conexão: `mysql://ODS_USER:ODS_PASSWORD@ODS_HOST:ODS_PORT/ODS_NAME` com opção SSL que use o arquivo `ODS_SSL_CA` como CA (equivalente ao que o Python faz em `app/db.py` com `get_ods_connection` e config em `app/config.py`).
- Garantir que o working directory (ou `WORK_DIR`) seja a raiz ao rodar a API, para que caminhos relativos como `ODS_SSL_CA=./certs/...` resolvam corretamente.

---

## 7. Documentação mínima (README)

- **Como rodar o gerador de certificados:** comando exato (ex.: `cargo run --bin gen-certs` ou `cargo run -- certs`) e onde os arquivos são salvos (`./certs-mtls/`).
- **Configuração Postman:**
  - Variáveis: base URL da API, token (obtido no login).
  - mTLS: como configurar Client Certificate (cert + private key) na requisição.
- **Exemplo de fluxo:**  

1) `POST /api/auth/login` com credenciais → obter token.

2) `POST /api/ods/hamburgueres-vendidos` com body `{"mes": "2026-01"}` e header `Authorization: Bearer <token>` (e client cert se mTLS estiver ativo).

---

## 8. Ordem sugerida de implementação

1. Crate Rust no subdiretório; `dotenvy` carregando `.env` da raiz; struct de config com ODS_*, JWT_*, (opcional) MTLS_*.
2. Conexão MySQL ODS com SSL (ODS_SSL_CA).
3. Endpoint de login e middleware JWT; endpoints de hambúrgueres protegidos por JWT.
4. Gerador de certs (rcgen) e leitura dos certs no servidor; ativar HTTPS com mTLS.
5. README com comandos, Postman e exemplo de chamadas.

---

## Diagrama de fluxo (resumo)

```mermaid
flowchart LR
  subgraph client [Cliente Postman]
    A[Login]
    B[Chamadas API]
  end
  subgraph api [API Rust]
    C[JWT Auth]
    D[MySQL ODS]
    E[hamburgueres-vendidos]
    F[hamburgueres-comandas-antigas]
  end
  A -->|username/password| C
  C -->|token| B
  B -->|Bearer + client cert| E
  B -->|Bearer + client cert| F
  E --> D
  F --> D
```

Este plano pode ser colado no “outro Cursor” (no repositório que contém os scripts Python e o `.env`) para implementação passo a passo; lá, as referências a `scripts/` e `app/` poderão ser abertas diretamente para espelhar o SQL e a regra de classificação de hambúrguer.
````

## [2] .gitignore

```
// FILE: .gitignore
// FULL: C:\Users\leopo\rust-studies\.gitignore
// SIZE: 371 bytes
// MTIME: 2026-02-04T09:30:35.652031
// NOTE: Concatenated snapshot for review
# Rust
target/
**/target/
**/*.rs.bk
Cargo.lock

# Env e segredos
.env
.env.api
.env.*.local

# Certificados e chaves
/certs/
/certs-mtls/
*.pem
*.key
!DigiCert*.pem

# IDE / OS
.idea/
.vscode/
*.swp
.DS_Store
Thumbs.db

# Logs
*.log

# Workflow MVP
workflow-mvp/desktop/node_modules/
workflow-mvp/desktop/dist/
workflow-mvp/backend/data/

```

## [3] DOCUMENTACAO.md

````markdown
# FILE: DOCUMENTACAO.md
# FULL: C:\Users\leopo\rust-studies\DOCUMENTACAO.md
# SIZE: 25679 bytes
# MTIME: 2026-02-04T08:58:41.763998
# NOTE: Concatenated snapshot for review
# Documentação completa — Chopebrain ODS API

API em Rust para o **ODS (Operational Data Store)** da Choperia: autenticação JWT, mTLS opcional e endpoints de relatórios de hambúrgueres (vendidos no mês e comandas antigas).

---

## 1. Visão geral

- **Objetivo:** Expor dados do banco MySQL do ODS via API REST, com autenticação e, opcionalmente, HTTPS com certificado de cliente (mTLS).
- **Stack:** Rust (edition 2021), Axum, SQLx (MySQL), JWT, RustLS (TLS).
- **Endpoints:**
  - `POST /api/auth/login` — Login (usuário/senha ou segredo único); retorna JWT.
  - `POST /api/ods/hamburgueres-vendidos` — Relatório de hambúrgueres vendidos no mês (requer JWT).
  - `POST /api/ods/hamburgueres-comandas-antigas` — Hambúrgueres em comandas abertas em período anterior ao mês da venda (requer JWT).

O `.env` (ou `.env.api`) fica na **raiz do repositório**, não dentro de `chopebrain-ods-api/`.

### 1.1 Diagrama de arquitetura (componentes)

```mermaid
flowchart TB
    subgraph Cliente["Cliente (Postman / Script / App)"]
        A[Request HTTP/HTTPS]
    end

    subgraph API["Chopebrain ODS API (Rust)"]
        direction TB
        B[main.rs]
        B --> C[config.rs]
        B --> D[db.rs]
        B --> E[handlers.rs]
        B --> F[mtls.rs]
        C --> G[.env / API_ENV_FILE]
        E --> H[auth.rs]
        E --> D
        F -.->|"Se certs existem"| TLS[HTTPS + mTLS]
        B --> |"Sem certs ou FORCE_HTTP"| HTTP[HTTP]
    end

    subgraph Rede["Camada de rede"]
        TLS
        HTTP
    end

    subgraph Dados["Dados"]
        MySQL[(MySQL ODS)]
    end

    A --> TLS
    A --> HTTP
    TLS --> E
    HTTP --> E
    D --> MySQL
    G -.->|"ODS_*, JWT_*, AUTH_*, MTLS_*"| C
```

### 1.2 Fluxo de decisão na inicialização (HTTP vs HTTPS)

```mermaid
flowchart LR
    START([Início]) --> LOAD[Carregar .env]
    LOAD --> CHECK_FORCE{FORCE_HTTP=1?}
    CHECK_FORCE -->|Sim| HTTP[Listen HTTP]
    CHECK_FORCE -->|Não| CHECK_CERTS{server-cert.pem e\nserver-key.pem existem?}
    CHECK_CERTS -->|Não| HTTP
    CHECK_CERTS -->|Sim| HTTPS[Bind HTTPS + mTLS]
    HTTP --> SERVE[Servir API]
    HTTPS --> SERVE
    SERVE --> END([API no ar])
```

---

## 2. Estrutura do projeto

```
rust-studies/
├── .env                    # Variáveis de ambiente (não versionado)
├── .env.api                # Alternativa para testes (ex.: scripts)
├── certs-mtls/            # Certificados mTLS (gerados por gen-certs; não versionado)
├── chopebrain-ods-api/     # Crate Rust da API
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── main.rs         # Entrypoint: carrega config, pool, sobe HTTP ou HTTPS
│       ├── lib.rs
│       ├── config.rs       # Carrega .env e Config (ODS, JWT, auth, mTLS)
│       ├── db.rs           # Pool MySQL com SSL (ODS_SSL_CA)
│       ├── auth.rs         # Login, JWT (create/decode), middleware require_jwt
│       ├── handlers.rs     # Rotas e handlers (login, hamburgueres-vendidos, comandas-antigas)
│       ├── mtls.rs        # HTTPS com mTLS (RustLS, axum-server)
│       └── bin/
│           └── gen_certs.rs  # Gera CA + server + client certs em certs-mtls/
├── scripts/
│   └── test-api.ps1       # Testes: gera certs, sobe API, login, endpoints
└── DOCUMENTACAO.md        # Este arquivo
```

---

## 3. Pré-requisitos

- **Rust** (edition 2021), com `cargo` no PATH.
- **MySQL** acessível (ODS) com as credenciais definidas no `.env`.
- Opcional: **PowerShell** 5.1+ para o script `scripts/test-api.ps1`.

---

## 4. Variáveis de ambiente

Todas as variáveis são lidas do arquivo `.env` (ou do arquivo indicado por `API_ENV_FILE`, ex.: `.env.api`). Caminhos relativos (ex.: `ODS_SSL_CA`, mTLS) são resolvidos em relação ao **diretório raiz** onde está o `.env` (`work_dir`).

### 4.1 Conexão MySQL (ODS)

| Variável       | Obrigatório | Descrição |
|----------------|-------------|-----------|
| `ODS_HOST`     | Sim         | Host do MySQL (ex.: `seu-servidor.mysql.database.azure.com`) |
| `ODS_PORT`     | Não         | Porta (padrão: `3306`) |
| `ODS_USER`     | Sim         | Usuário MySQL |
| `ODS_PASSWORD` | Sim         | Senha MySQL |
| `ODS_NAME`     | Sim         | Nome do banco de dados |
| `ODS_SSL_CA`   | Não         | Caminho para o certificado CA (ex.: `./certs/DigiCertGlobalRootCA.crt.pem`) para SSL na conexão |

### 4.2 JWT

| Variável             | Obrigatório | Descrição |
|----------------------|-------------|-----------|
| `JWT_SECRET`         | Não         | Segredo para assinar o token (padrão: `change-me-in-production`) |
| `JWT_EXPIRATION_DAYS`| Não         | Dias até expiração do token (padrão: `7`) |

### 4.3 Autenticação (login)

É necessário configurar **um** dos modos:

- **Usuário e senha:**  
  `AUTH_USERNAME` e `AUTH_PASSWORD`. O cliente envia `{ "username": "...", "password": "..." }`.

- **Segredo único:**  
  `AUTH_SECRET`. O cliente pode enviar `{ "secret": "valor" }` ou `{ "password": "valor" }` (ou username + password com a senha igual ao segredo).

### 4.4 mTLS (HTTPS + certificado de cliente)

| Variável            | Obrigatório | Descrição |
|---------------------|-------------|-----------|
| `MTLS_SERVER_CERT`  | Não         | Caminho do certificado do servidor (padrão: `./certs-mtls/server-cert.pem`) |
| `MTLS_SERVER_KEY`   | Não         | Caminho da chave privada do servidor (padrão: `./certs-mtls/server-key.pem`) |
| `MTLS_CA_CERT`      | Não         | Caminho do CA para validar certificados de cliente (padrão: `./certs-mtls/ca.pem`) |
| `FORCE_HTTP`        | Não         | Se `1`, desativa mTLS e sobe só HTTP (útil para testes locais) |

Se os arquivos de servidor (cert + key) existirem e `FORCE_HTTP` não for `1`, a API sobe em **HTTPS com mTLS** (cliente deve apresentar certificado aceito pela CA).

### 4.5 Servidor e diretório de trabalho

| Variável         | Obrigatório | Descrição |
|------------------|-------------|-----------|
| `LISTEN`         | Não         | Endereço e porta (padrão: `0.0.0.0:3000`), ex.: `127.0.0.1:30443` |
| `WORK_DIR`       | Não         | Diretório raiz onde está o `.env` (inferido automaticamente se não definido) |
| `API_ENV_FILE`   | Não         | Caminho absoluto ou relativo para um arquivo `.env` alternativo (ex.: `.env.api`) |
| `CERT_OUTPUT_DIR`| Não         | Diretório de saída do binário `gen-certs` (padrão: `./certs-mtls` em relação ao work_dir) |

### 4.6 Logs

| Variável   | Descrição |
|------------|-----------|
| `RUST_LOG` | Níveis de log (padrão: `chopebrain_ods_api=debug,tower_http=debug,info`) |

---

## 5. Como executar

### 5.1 Carregamento do .env

1. Se `API_ENV_FILE` estiver definido e for um arquivo, esse arquivo é carregado e o `work_dir` é o diretório do arquivo.
2. Se `WORK_DIR` estiver definido e existir `.env` nesse diretório, esse `.env` é usado.
3. Caso contrário, a API procura `.env` no diretório atual e em diretórios pais.

Recomendação: na raiz do repositório, ter um `.env` (ou usar `API_ENV_FILE` apontando para `.env.api` em testes).

### 5.2 Gerar certificados mTLS

Na raiz do repositório (ou com `WORK_DIR`/`API_ENV_FILE` adequados):

```bash
cargo run --manifest-path chopebrain-ods-api/Cargo.toml --bin gen-certs
```

Arquivos gerados em `./certs-mtls/` (ou em `CERT_OUTPUT_DIR`):

- `ca.pem` — Autoridade certificadora
- `server-cert.pem`, `server-key.pem` — Servidor (localhost / 127.0.0.1)
- `client-cert.pem`, `client-key.pem` — Cliente (Postman, script, etc.)

### 5.3 Subir a API

Na raiz do repositório:

```bash
cargo run --manifest-path chopebrain-ods-api/Cargo.toml
```

Ou de dentro de `chopebrain-ods-api/`:

```bash
cd chopebrain-ods-api
# PowerShell:
$env:WORK_DIR = ".."
# Bash:
# export WORK_DIR=..
cargo run
```

- Sem certificados mTLS (ou com `FORCE_HTTP=1`): **HTTP** em `http://0.0.0.0:3000` (ou o valor de `LISTEN`).
- Com certificados mTLS: **HTTPS** no mesmo endereço, exigindo certificado de cliente.

---

## 6. API — Endpoints

Base URL: `http://<host>:<port>` ou `https://<host>:<port>` conforme o modo.

### 6.1 POST /api/auth/login

**Autenticação:** Nenhuma.

**Request (JSON):**

- Modo usuário/senha: `{ "username": "usuario", "password": "senha" }`
- Modo segredo: `{ "secret": "valor_do_AUTH_SECRET" }` ou `{ "password": "valor_do_AUTH_SECRET" }`

**Resposta 200:**

```json
{ "token": "eyJ0eXAiOiJKV1QiLCJhbGc..." }
```

**Erros:** `401` — credenciais inválidas; `500` — erro ao gerar JWT.

**Sequência (login):**

```mermaid
sequenceDiagram
    participant C as Cliente
    participant A as API (handlers)
    participant Auth as auth.rs
    participant Config as Config (.env)

    C->>A: POST /api/auth/login { username, password } ou { secret }
    A->>Auth: validate_login(config, req)
    Auth->>Config: AUTH_USERNAME / AUTH_PASSWORD ou AUTH_SECRET
    Config-->>Auth: credenciais
    Auth-->>A: true/false
    alt credenciais OK
        A->>Auth: create_token(config, sub)
        Auth->>Auth: JWT HS256 (sub, exp, iat)
        Auth-->>A: token
        A-->>C: 200 { "token": "eyJ..." }
    else credenciais inválidas
        A-->>C: 401 { "error": "Credenciais inválidas" }
    end
```

---

### 6.2 POST /api/ods/hamburgueres-vendidos

**Autenticação:** `Authorization: Bearer <token>`. Em HTTPS com mTLS, também é necessário certificado de cliente.

**Request (JSON):**

```json
{ "mes": "2026-01" }
```

`mes` no formato **YYYY-MM**.

**Resposta 200:**

```json
{
  "totais": {
    "quantidade": 120,
    "valor": 4567.89,
    "ticket_medio": 38.07
  },
  "por_categoria": [
    { "categoria": "Hamburguer", "quantidade": 80, "valor": 3000.00 },
    { "categoria": null, "quantidade": 40, "valor": 1567.89 }
  ],
  "por_produto": [
    { "produto": "X-Tudo", "categoria": "Hamburguer", "quantidade": 45, "valor": 1800.00 }
  ],
  "itens": [
    {
      "data_venda_item": "2026-01-15 12:30",
      "pedido": 12345,
      "produto": "X-Tudo",
      "categoria": "Hamburguer",
      "quantidade": 1,
      "valor": 40.00
    }
  ]
}
```

- **totais:** quantidade total de itens (hambúrgueres), valor total e ticket médio.
- **por_categoria:** agregado por categoria (etiqueta).
- **por_produto:** até 10 produtos com maior quantidade.
- **itens:** lista de itens de venda no mês.

**Erros:** `400` — `mes` inválido (ex.: não YYYY-MM); `401` — token ausente/inválido; `500` — erro de banco (mensagem em `error`).

**Sequência (endpoint protegido, com ou sem mTLS):**

```mermaid
sequenceDiagram
    participant C as Cliente
    participant TLS as Camada TLS (se mTLS)
    participant M as Middleware require_jwt
    participant H as Handler (hamburgueres_*)
    participant DB as MySQL ODS

    C->>TLS: HTTPS Request (opcional: client cert)
    note over TLS: Se mTLS: valida client cert contra CA
    TLS->>M: Request + Authorization: Bearer &lt;token&gt;
    M->>M: decode_token(secret, token)
    alt token válido
        M->>H: next.run(request)
        H->>H: parse_mes("YYYY-MM")
        H->>DB: Queries (itenspedido, pedidos, produtos, etiquetas)
        DB-->>H: Rows
        H-->>M: Json(response)
        M-->>TLS: 200 + JSON
        TLS-->>C: Response
    else token ausente/inválido/expirado
        M-->>C: 401 { "error": "..." }
    end
```

---

### 6.3 POST /api/ods/hamburgueres-comandas-antigas

**Autenticação:** `Authorization: Bearer <token>`. Em HTTPS com mTLS, certificado de cliente obrigatório.

**Request (JSON):**

```json
{ "mes": "2026-01" }
```

Considera itens **vendidos no mês** indicado, mas em **pedidos (comandas) abertos em período anterior** (ano anterior ou mesmo ano com mês anterior).

**Resposta 200:**

```json
{
  "totais": {
    "quantidade": 15,
    "valor": 580.00,
    "ticket_medio": 38.67
  },
  "por_produto": [
    { "produto": "X-Bacon", "categoria": "Hamburguer", "quantidade": 8, "valor": 320.00 }
  ],
  "por_pedido": [
    {
      "pedido_id": 12001,
      "data_abertura": "2025-12-10 18:00",
      "data_fechamento": "2026-01-05 22:00",
      "itens": 3,
      "quantidade": 5,
      "valor": 195.00
    }
  ],
  "detalhamento": [
    {
      "data_venda_item": "2026-01-02 14:20",
      "pedido": 12001,
      "data_abertura": "2025-12-10",
      "dias_diferenca": 23,
      "produto": "X-Bacon",
      "quantidade": 2,
      "valor": 80.00
    }
  ]
}
```

**Erros:** mesmos códigos que em hamburgueres-vendidos.

---

## 7. Regra de “hambúrguer”

Um item entra nos relatórios de hambúrgueres se:

- A **categoria** (etiqueta) for `Hamburger` ou `Hamburguer` (case insensitive), **ou**
- O **nome do produto** contiver (case insensitive) uma das palavras-chave:  
  `hamburguer`, `burger`, `cheeseburger`, `bacon burger`, `frango burger`, `artesanal burger`.

Isso é aplicado em todas as queries dos dois endpoints ODS (filtro `HAMBURGER_CATEGORY_FILTER` em `handlers.rs`).

---

## 8. Banco de dados (ODS)

- **Motor:** MySQL.
- **Conexão:** SSL opcional via `ODS_SSL_CA` (modo Verify CA).

Tabelas utilizadas (nomes e colunas conforme implementação atual):

- **itenspedido:** `datahoracadastro`, `quantidade`, `valorunitario`, `codigopedido`, `codigoprodutodetalhe`, `codigoproduto`
- **pedidos:** `codigo`, `dataabertura`, `datafechamento`
- **produtodetalhe:** `codigo`
- **produtos:** `codigo`, `nome`, `codigoetiqueta`
- **etiquetas:** `codigo`, `descricao`

Os JOINs usam `codigo` / `codigopedido` / `codigoproduto` / `codigoetiqueta` conforme o schema do ODS.

### 8.1 Diagrama de relações (endpoints de hambúrgueres)

```mermaid
erDiagram
    itenspedido ||--o{ pedidos : "codigopedido"
    itenspedido ||--o{ produtodetalhe : "codigoprodutodetalhe"
    itenspedido ||--o{ produtos : "codigoproduto"
    produtos ||--o| etiquetas : "codigoetiqueta"
    pedidos {
        int codigo PK
        datetime dataabertura
        datetime datafechamento
    }
    itenspedido {
        datetime datahoracadastro
        int quantidade
        decimal valorunitario
        int codigopedido FK
        int codigoprodutodetalhe FK
        int codigoproduto FK
    }
    produtos {
        int codigo PK
        string nome
        int codigoetiqueta FK
    }
    etiquetas {
        int codigo PK
        string descricao
    }
    produtodetalhe {
        int codigo PK
    }
```

Fluxo lógico das queries: **itenspedido** (filtro por ano/mês em `datahoracadastro`) → **pedidos** (para comandas antigas: `dataabertura` em período anterior) → **produtodetalhe** → **produtos** → **etiquetas** (categoria). O filtro de “hambúrguer” é aplicado em `etiquetas.descricao` e `produtos.nome`.

---

## 9. Autenticação JWT

- **Algoritmo:** HS256 (segredo em `JWT_SECRET`).
- **Claims:** `sub` (identificador do usuário, ex.: username ou `"api"`), `exp`, `iat`.
- **Uso:** Header `Authorization: Bearer <token>` em todos os endpoints protegidos. O middleware `require_jwt` em `auth.rs` valida o token e rejeita com `401` se estiver ausente, inválido ou expirado.

### 9.1 Validação do login (validate_login)

O handler de login aceita dois modos de credencial; a ordem de checagem é a abaixo.

```mermaid
flowchart TD
    A[POST /api/auth/login] --> B{AUTH_SECRET\nconfigurado?}
    B -->|Sim| C{req.secret ou req.password\n== AUTH_SECRET?}
    C -->|Sim| OK[Login OK — cria JWT]
    C -->|Não| D{AUTH_USERNAME e\nAUTH_PASSWORD configurados?}
    B -->|Não| D
    D -->|Sim| E{req.username e req.password\nbatem?}
    E -->|Sim| OK
    E -->|Não| FAIL[401 Credenciais inválidas]
    D -->|Não| FAIL
```

---

## 10. mTLS (HTTPS + certificado de cliente) — detalhado

**mTLS (mutual TLS)** é TLS com autenticação bilateral: além do servidor se identificar com seu certificado (HTTPS normal), o **cliente** também precisa apresentar um certificado assinado por uma CA em que o servidor confia. Assim, só clientes que possuem um certificado válido conseguem estabelecer a conexão.

### 10.1 Hierarquia de certificados (gen-certs)

O binário `gen-certs` gera uma **CA (Certificate Authority)** autoassinada e, a partir dela, o certificado do servidor e o do cliente. Tudo fica em `certs-mtls/`.

```mermaid
flowchart TB
    subgraph CA["Autoridade certificadora (CA)"]
        CA_PEM["ca.pem\n(ChopeBrain ODS CA)"]
    end

    subgraph Servidor["Certificados do servidor"]
        S_CERT["server-cert.pem\n(localhost, 127.0.0.1)"]
        S_KEY["server-key.pem\n(chave privada)"]
    end

    subgraph Cliente["Certificados do cliente"]
        C_CERT["client-cert.pem\n(ChopeBrain API Client)"]
        C_KEY["client-key.pem\n(chave privada)"]
    end

    CA_PEM -->|"Assina"| S_CERT
    CA_PEM -->|"Assina"| C_CERT
    S_CERT --- S_KEY
    C_CERT --- C_KEY
```

| Arquivo           | Quem usa        | Uso |
|-------------------|----------------|-----|
| `ca.pem`          | Servidor       | Monta o **RootCertStore** para validar o certificado que o **cliente** envia. |
| `server-cert.pem` | Servidor       | Enviado ao cliente na handshake TLS (identidade do servidor). |
| `server-key.pem`  | Servidor       | Chave privada para decifrar e assinar (nunca enviada). |
| `client-cert.pem` | Cliente        | Enviado ao servidor na handshake (identidade do cliente). |
| `client-key.pem`  | Cliente        | Chave privada do cliente (nunca enviada). |

### 10.2 Quem usa o quê (visão geral)

```mermaid
flowchart LR
    subgraph Servidor["Servidor (API)"]
        SRV_STORE["RootCertStore\n(ca.pem)"]
        SRV_CERT["server-cert.pem"]
        SRV_KEY["server-key.pem"]
        VERIFIER["WebPkiClientVerifier\n(aceita só certs assinados pela CA)"]
        SRV_STORE --> VERIFIER
    end

    subgraph Cliente["Cliente (Postman, curl, app)"]
        CLI_CERT["client-cert.pem"]
        CLI_KEY["client-key.pem"]
    end

    Cliente -->|"1. Conexão TLS"| Servidor
    Servidor -->|"2. Envia server-cert"| Cliente
    Cliente -->|"3. Envia client-cert"| VERIFIER
    VERIFIER -->|"4. Valida assinatura com CA"| SRV_STORE
```

1. **Cliente** inicia conexão TLS ao servidor.
2. **Servidor** envia `server-cert.pem`; o cliente (opcionalmente) verifica contra uma CA de confiança — em teste com CA autoassinada pode desativar verificação.
3. **Servidor** pede o certificado do cliente (mTLS).
4. **Cliente** envia `client-cert.pem` (assinado com `client-key.pem`).
5. **Servidor** usa `ca.pem` no `WebPkiClientVerifier`: só aceita se o certificado do cliente for assinado por essa CA. Se não for, a conexão é encerrada antes de qualquer HTTP.

### 10.3 Handshake TLS com certificado de cliente (sequência)

```mermaid
sequenceDiagram
    participant C as Cliente
    participant S as Servidor (RustLS)

    C->>S: ClientHello (suítes cifra, etc.)
    S->>C: ServerHello + server-cert.pem
    S->>C: CertificateRequest (lista de CAs aceitas)
    S->>C: ServerHelloDone

    C->>C: Escolhe client-cert.pem + client-key.pem
    C->>S: client-cert.pem (Certificate)
    C->>S: CertificateVerify (assinatura com client-key)
    C->>S: ClientKeyExchange
    C->>S: Finished

    S->>S: WebPkiClientVerifier: verifica client-cert contra ca.pem
    alt Certificado válido (assinado pela CA)
        S->>C: Finished
        Note over C,S: Canal TLS estabelecido — HTTP/1.1 ou HTTP/2
    else Certificado inválido ou ausente
        S->>C: Fecha conexão (Alert)
    end
```

- **CertificateRequest:** o servidor indica que exige certificado de cliente e pode enviar a lista de CAs aceitas (no nosso caso, a CA de `ca.pem`).
- **CertificateVerify:** o cliente prova que possui a chave privada correspondente ao `client-cert.pem`.
- **WebPkiClientVerifier:** no servidor (código em `mtls.rs`), usa `RootCertStore` carregado com `ca.pem`; só certificados assinados por essa CA são aceitos.

### 10.4 Fluxo no código (mtls.rs)

```mermaid
flowchart TB
    A[main: mtls_enabled?] --> B{server-cert.pem e\nserver-key.pem existem?}
    B -->|Não ou FORCE_HTTP=1| C[Bind HTTP]
    B -->|Sim| D[Carregar server-cert.pem, server-key.pem]
    D --> E[Carregar ca.pem]
    E --> F[RootCertStore.add(ca)]
    F --> G[WebPkiClientVerifier::builder(root_store).build()]
    G --> H[ServerConfig com client_verifier]
    H --> I[axum_server::bind_rustls]
    I --> J[Servir HTTPS com mTLS]
```

- **RustLS** usa `ServerConfig` com `with_client_cert_verifier(client_verifier)`, então toda conexão precisa passar pela validação do certificado do cliente antes de chegar ao Axum.

### 10.5 Quando mTLS está ativo

- **Condição:** existência de `server-cert.pem` e `server-key.pem` (caminhos padrão `./certs-mtls/` ou variáveis `MTLS_SERVER_CERT` / `MTLS_SERVER_KEY`) **e** `FORCE_HTTP` ≠ `1`.
- **Comportamento:** o servidor escuta em HTTPS e **exige** certificado de cliente assinado pela CA (`ca.pem`). Sem certificado válido, a conexão TLS é recusada (não chega a retornar 401 HTTP).

### 10.6 Uso no Postman

1. **Settings** (ou na requisição) → **Certificate**.
2. Ativar **Client Certificate**.
3. **Certificate:** arquivo `client-cert.pem`.
4. **Private Key:** arquivo `client-key.pem`.
5. Em ambiente de teste com CA autoassinada, pode ser necessário desativar **SSL certificate verification** para o host.

### 10.7 Script de testes e FORCE_HTTP

Em alguns ambientes (ex.: PowerShell 5.1 no Windows) o uso de certificado cliente em PEM com `Invoke-RestMethod` ou `curl.exe` é limitado. O script pode definir `FORCE_HTTP=1` e `API_ENV_FILE` para subir a API em **HTTP** e testar apenas login e payloads dos endpoints, sem mTLS.

---

## 11. Script de testes (PowerShell)

**Arquivo:** `scripts/test-api.ps1`  
**Uso:** Executar na raiz do repositório: `.\scripts\test-api.ps1`

### 11.1 Fluxo do script (resumo)

```mermaid
flowchart TB
    subgraph Prep["Preparação"]
        A[Início] --> B{certs em certs-mtls/?}
        B -->|Não| C[cargo run --bin gen-certs]
        B -->|Sim| D[cargo build]
        C --> D
        D --> E[Liberar porta LISTEN]
        E --> F{.env.api existe?}
        F -->|Não| G[Erro e sair]
        F -->|Sim| H[Definir API_ENV_FILE, LISTEN, WORK_DIR]
    end

    H --> I[Iniciar API em background]
    I --> J[Aguardar porta em Listen]
    J --> K[POST /api/auth/login]
    K --> L{Login OK?}
    L -->|Não| M[Exibir erro e encerrar API]
    L -->|Sim| N[POST /api/ods/hamburgueres-vendidos com Bearer]
    N --> O[POST /api/ods/hamburgueres-comandas-antigas com Bearer]
    O --> P[Exibir respostas / erros]
    P --> Q[Encerrar processo da API]
    Q --> R[Fim]
    M --> R
```

O script:

1. Gera certificados em `certs-mtls/` se ainda não existirem (`gen-certs`).
2. Faz build da API.
3. Libera a porta configurada (ex.: 30443) se estiver em uso.
4. Cria/usa `.env.api` na raiz (variáveis mínimas: ODS_*, JWT_*, AUTH_SECRET). Define `API_ENV_FILE`, `LISTEN`, `WORK_DIR` e, em alguns ambientes, `FORCE_HTTP=1` para usar HTTP.
5. Inicia a API em background.
6. Aguarda a porta em escuta e faz:
   - Login em `POST /api/auth/login` (segredo ou usuário/senha conforme `.env.api`).
   - Chamadas a `hamburgueres-vendidos` e `hamburgueres-comandas-antigas` com o token.
7. Exibe respostas e, em caso de erro, o corpo da resposta (ex.: mensagem do MySQL).
8. Encerra o processo da API ao final.

Variáveis úteis no ambiente antes de rodar o script:

- `TEST_API_HTTP=1` — força uso de HTTP para os testes.
- `AUTH_SECRET` — segredo de login (se não estiver em `.env.api`).

---

## 12. Logs e troubleshooting

- **Logs:** `tracing` com nível padrão `chopebrain_ods_api=debug,tower_http=debug,info`. Ajuste com `RUST_LOG`.
- **Erros internos:** Em falhas de handler (ex.: MySQL), a API retorna JSON `{ "error": "mensagem" }` e registra causa e, em build debug, backtrace.
- **Configuração:** Se a API não encontrar o `.env`, verifique `WORK_DIR` ou `API_ENV_FILE` e o diretório de execução.
- **mTLS:** Se quiser forçar HTTP (ex.: testes sem certificado cliente), use `FORCE_HTTP=1`.
- **Porta em uso:** Altere `LISTEN` ou, no script, a variável `$ListenPort` e garanta que nenhum outro processo use a porta.

---

## 13. Resumo rápido

| Ação | Comando / Observação |
|------|----------------------|
| Carregar config | `.env` (ou `API_ENV_FILE`) na raiz do repo |
| Gerar certs mTLS | `cargo run --manifest-path chopebrain-ods-api/Cargo.toml --bin gen-certs` |
| Subir API | `cargo run --manifest-path chopebrain-ods-api/Cargo.toml` (na raiz) |
| Testar com script | `.\scripts\test-api.ps1` (raiz); requer `.env.api` com ODS_*, JWT_*, AUTH_* |
| Login | `POST /api/auth/login` com `username`/`password` ou `secret` |
| Endpoints ODS | `POST /api/ods/hamburgueres-vendidos` e `.../hamburgueres-comandas-antigas` com body `{ "mes": "YYYY-MM" }` e header `Authorization: Bearer <token>` |

Para mais detalhes do dia a dia (Postman, exemplos de body), consulte o `README.md` dentro de `chopebrain-ods-api/`.

````

## [4] chopebrain-ods-api/Cargo.toml

```toml
# FILE: chopebrain-ods-api/Cargo.toml
# FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\Cargo.toml
# SIZE: 1020 bytes
# MTIME: 2026-02-04T02:56:36.209245
# NOTE: Concatenated snapshot for review
[package]
name = "chopebrain-ods-api"
version = "0.1.0"
edition = "2021"
description = "API Rust ODS Choperia - JWT, mTLS, endpoints hambúrgueres"

[[bin]]
name = "chopebrain-ods-api"
path = "src/main.rs"

[[bin]]
name = "gen-certs"
path = "src/bin/gen_certs.rs"

[dependencies]
axum = { version = "0.7", features = ["json", "macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
dotenvy = "0.15"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "mysql", "chrono"] }
jsonwebtoken = "9"
chrono = { version = "0.4", features = ["serde"] }
axum-server = { version = "0.8", features = ["tls-rustls"] }
rustls = "0.23"
rustls-pemfile = "2"
tokio-rustls = "0.26"
futures-util = "0.3"
rcgen = "0.12"
time = "0.3"
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

```

## [5] chopebrain-ods-api/README.md

````markdown
# FILE: chopebrain-ods-api/README.md
# FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\README.md
# SIZE: 4477 bytes
# MTIME: 2026-02-04T08:55:24.102840
# NOTE: Concatenated snapshot for review
# Chopebrain ODS API

API Rust para o ODS da Choperia: autenticação JWT, mTLS (opcional) e endpoints de hambúrgueres (vendidos no mês e comandas antigas).

**Documentação completa:** na raiz do repositório, veja [../DOCUMENTACAO.md](../DOCUMENTACAO.md) (variáveis de ambiente, todos os endpoints, banco de dados, mTLS, script de testes e troubleshooting).

## Pré-requisitos

- Rust (edition 2021).
- `.env` na **raiz do repositório** (não dentro de `chopebrain-ods-api/`) com as variáveis ODS, JWT e, opcionalmente, mTLS (ver plano / documentação do projeto).

## Gerador de certificados (mTLS)

Para ativar HTTPS com mTLS (certificado de cliente obrigatório), gere os certificados na pasta padrão `./certs-mtls/` (relativa à raiz do repo):

```bash
# Na raiz do repositório (onde está o .env)
cargo run --manifest-path chopebrain-ods-api/Cargo.toml --bin gen-certs
```

Os arquivos são gravados em **`./certs-mtls/`** (ou no diretório definido por `CERT_OUTPUT_DIR`):

- `ca.pem` — CA
- `server-cert.pem`, `server-key.pem` — servidor
- `client-cert.pem`, `client-key.pem` — cliente (Postman, etc.)

Se esses arquivos existirem, a API sobe em HTTPS com mTLS; caso contrário, sobe em HTTP.

## Executando a API

Na raiz do repositório (para o binário encontrar o `.env`):

```bash
cargo run --manifest-path chopebrain-ods-api/Cargo.toml
```

Ou, de dentro de `chopebrain-ods-api/`, com `WORK_DIR` apontando para a raiz:

```bash
cd chopebrain-ods-api
$env:WORK_DIR = ".."   # PowerShell; no Bash: export WORK_DIR=..
cargo run
```

Endereço padrão: `http://0.0.0.0:3000` ou `https://0.0.0.0:3000` se mTLS estiver ativo. Use `LISTEN` para alterar (ex.: `LISTEN=127.0.0.1:8443`).

## Configuração no Postman

### Variáveis de ambiente (recomendado)

- **base_url**: `http://localhost:3000` (ou `https://localhost:3000` se mTLS)
- **token**: deixe vazio; preencha após o login (ou use um script para guardar o token da resposta do login).

### mTLS (Client Certificate)

Se a API estiver em HTTPS com mTLS:

1. Em **Settings** (ou na requisição) → **Certificate**.
2. Ative **Client Certificate**.
3. **Certificate**: arquivo `client-cert.pem` (ex.: `certs-mtls/client-cert.pem`).
4. **Private Key**: arquivo `client-key.pem` (ex.: `certs-mtls/client-key.pem`).
5. Em ambientes HTTPS autoassinados, pode ser necessário desativar **SSL certificate verification** apenas para testes.

### Autenticação (JWT)

- Tipo: **Bearer Token**.
- Valor: use a variável `{{token}}` após obter o token no login.

## Exemplo de fluxo

### 1. Login e obter token

**POST** `{{base_url}}/api/auth/login`

Body (JSON), um dos exemplos:

- Usuário/senha (se configurado no `.env`):
  ```json
  { "username": "seu_usuario", "password": "sua_senha" }
  ```
- Segredo único (se configurado `AUTH_SECRET`):
  ```json
  { "secret": "seu_segredo" }
  ```

Resposta exemplo:
```json
{ "token": "eyJ0eXAiOiJKV1QiLCJhbGc..." }
```

Copie o `token` para a variável **token** no Postman (ou use um script de teste que salve automaticamente).

### 2. Chamar endpoint protegido (hambúrgueres vendidos)

**POST** `{{base_url}}/api/ods/hamburgueres-vendidos`

- **Headers**: `Authorization: Bearer {{token}}`
- **Body** (JSON): `{ "mes": "2026-01" }`
- Se mTLS estiver ativo: Client Certificate configurado (cert + chave do cliente).

### 3. Hambúrgueres em comandas antigas

**POST** `{{base_url}}/api/ods/hamburgueres-comandas-antigas`

- **Headers**: `Authorization: Bearer {{token}}`
- **Body** (JSON): `{ "mes": "2026-01" }`
- Com mTLS: Client Certificate configurado.

## Variáveis de ambiente (resumo)

| Variável | Uso |
|----------|-----|
| `ODS_HOST`, `ODS_PORT` (opcional, default 3306), `ODS_USER`, `ODS_PASSWORD`, `ODS_NAME` | Conexão MySQL ODS |
| `ODS_SSL_CA` | Caminho para o CA (ex.: `./certs/DigiCertGlobalRootCA.crt.pem`) para SSL ao MySQL |
| `JWT_SECRET`, `JWT_EXPIRATION_DAYS` | Assinatura e expiração do JWT |
| `AUTH_USERNAME`, `AUTH_PASSWORD` ou `AUTH_SECRET` | Login (usuário/senha ou segredo único) |
| `MTLS_SERVER_CERT`, `MTLS_SERVER_KEY`, `MTLS_CA_CERT` | Caminhos dos certs mTLS (opcional; padrão `./certs-mtls/`) |
| `WORK_DIR` | Diretório raiz onde está o `.env` (opcional) |
| `LISTEN` | Endereço e porta (ex.: `0.0.0.0:3000`) |
| `CERT_OUTPUT_DIR` | Diretório de saída do `gen-certs` (opcional) |

````

## [6] chopebrain-ods-api/src/auth.rs

```rust
// FILE: chopebrain-ods-api/src/auth.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\auth.rs
// SIZE: 4128 bytes
// MTIME: 2026-02-04T02:55:50.462154
// NOTE: Concatenated snapshot for review
// SECURITY: Content redacted due to secret patterns: SECRET, PASSWORD, TOKEN
//! Autenticação JWT: login e validação de token.

use crate::config::Config;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken:REDACTED:TOKEN encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: REDACTED:PASSWORD
    #[serde(rename = "secret")]
    pub secret: REDACTED:SECRET
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// Valida credenciais: AUTH_USERNAME/AUTH_PASSWORD ou AUTH_SECRET (único segredo).
pub fn validate_login(cfg: &Config, req: &LoginRequest) -> bool {
    tracing::debug!("auth: tentativa de login (username presente={}, password presente={}, secret presente={})",
        req.username.is_some(), req.password.is_some(), req.secret.is_some());
    if let Some(secret) = &cfg.auth_secret {
        if req.secret.as_deref() == Some(secret.as_str()) {
            tracing::info!("auth: login OK via AUTH_SECRET (secret)");
            return true;
        }
        if req.password.as_deref() == Some(secret.as_str()) {
            tracing::info!("auth: login OK via AUTH_SECRET (password)");
            return true;
        }
    }
    if let (Some(u), Some(p)) = (&cfg.auth_username, &cfg.auth_password) {
        if req.username.as_deref() == Some(u.as_str()) && req.password.as_deref() == Some(p.as_str()) {
            tracing::info!("auth: login OK via AUTH_USERNAME/AUTH_PASSWORD (user={})", u);
            return true;
        }
    }
    tracing::warn!("auth: login recusado (credenciais inválidas)");
    false
}

pub fn create_token(cfg: &Config, sub: &str) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + Duration::days(cfg.jwt_expiration_days as i64);
    let claims = Claims {
        sub: sub.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
    )?;
    tracing::info!("auth: JWT criado para sub={}, expira em {} dias", sub, cfg.jwt_expiration_days);
    Ok(token)
}

pub fn decode_token(cfg: &Config, token: &str) -> Result<Claims, jsonwebtoken:REDACTED:TOKEN {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
}

/// Middleware: exige Authorization: Bearer <token> e valida JWT.
pub async fn require_jwt(
    State((config, _)): State<(Arc<Config>, Arc<sqlx::MySqlPool>)>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let path = request.uri().path().to_string();
    let auth = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));
    let token = match auth {
        Some(t) => t,
        None => {
            tracing::warn!("auth: {} - 401 Authorization header missing or invalid", path);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Authorization header missing or invalid" })),
            ));
        }
    };
    match decode_token(config.as_ref(), token) {
        Ok(claims) => {
            tracing::debug!("auth: {} - JWT válido sub={}", path, claims.sub);
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("auth: {} - 401 token inválido ou expirado: {}", path, e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid or expired token" })),
            ))
        }
    }
}

```

## [7] chopebrain-ods-api/src/config.rs

```rust
// FILE: chopebrain-ods-api/src/config.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\config.rs
// SIZE: 4764 bytes
// MTIME: 2026-02-04T02:20:40.971785
// NOTE: Concatenated snapshot for review
// SECURITY: Content redacted due to secret patterns: SECRET, PASSWORD
//! Configuração carregada do .env na raiz do repositório.

use std::path::PathBuf;

/// Encontra o diretório raiz onde está o .env (mesmo usado pelo Python).
/// 1. Usa API_ENV_FILE se definido (caminho para um .env alternativo; work_dir = diretório do arquivo).
/// 2. Usa WORK_DIR se definido.
/// 3. Procura .env no diretório atual e em diretórios pais.
pub fn find_env_dir() -> Result<PathBuf, std::io::Error> {
    if let Ok(env_file) = std::env::var("API_ENV_FILE") {
        let p = PathBuf::from(&env_file);
        if p.is_file() {
            return Ok(p.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf());
        }
    }
    if let Ok(work_dir) = std::env::var("WORK_DIR") {
        let p = PathBuf::from(work_dir);
        if p.join(".env").is_file() {
            return Ok(p);
        }
    }
    let mut current = std::env::current_dir()?;
    loop {
        let env_path = current.join(".env");
        if env_path.is_file() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    std::env::current_dir()
}

/// Carrega variáveis do .env da raiz do repo (não do diretório do crate).
/// Se API_ENV_FILE estiver definido, carrega esse arquivo.
pub fn load_dotenv_from_root() -> Result<PathBuf, dotenvy::Error> {
    let root = find_env_dir().map_err(|e| dotenvy::Error::Io(e))?;
    let env_path = std::env::var("API_ENV_FILE")
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .unwrap_or_else(|| root.join(".env"));
    dotenvy::from_path(&env_path)?;
    Ok(root)
}

#[derive(Clone, Debug)]
pub struct Config {
    pub ods_host: String,
    pub ods_port: u16,
    pub ods_user: String,
    pub ods_password: String,
    pub ods_name: String,
    pub ods_ssl_ca: Option<PathBuf>,
    pub jwt_secret: String,
    pub jwt_expiration_days: u64,
    pub auth_username: Option<String>,
    pub auth_password: REDACTED:PASSWORD
    pub auth_secret: REDACTED:SECRET
    pub mtls_server_cert: Option<PathBuf>,
    pub mtls_server_key: Option<PathBuf>,
    pub mtls_ca_cert: Option<PathBuf>,
    pub work_dir: PathBuf,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ConfigError(pub String);

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let work_dir = find_env_dir().map_err(|e| ConfigError(e.to_string()))?;
        let ods_host = std::env::var("ODS_HOST")
            .map_err(|_| ConfigError("ODS_HOST não definido".into()))?;
        let ods_port = std::env::var("ODS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3306);
        let ods_user = std::env::var("ODS_USER")
            .map_err(|_| ConfigError("ODS_USER não definido".into()))?;
        let ods_password = REDACTED:PASSWORD"ODS_PASSWORD")
            .map_err(|_| ConfigError("ODS_PASSWORD não definido".into()))?;
        let ods_name = std::env::var("ODS_NAME")
            .map_err(|_| ConfigError("ODS_NAME não definido".into()))?;
        let ods_ssl_ca = std::env::var("ODS_SSL_CA").ok().map(PathBuf::from);
        let jwt_secret = REDACTED:SECRET"JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".into());
        let jwt_expiration_days = std::env::var("JWT_EXPIRATION_DAYS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7);
        let auth_username = std::env::var("AUTH_USERNAME").ok();
        let auth_password = REDACTED:PASSWORD"AUTH_PASSWORD").ok();
        let auth_secret = REDACTED:SECRET"AUTH_SECRET").ok();
        let mtls_server_cert = std::env::var("MTLS_SERVER_CERT").ok().map(PathBuf::from);
        let mtls_server_key = std::env::var("MTLS_SERVER_KEY").ok().map(PathBuf::from);
        let mtls_ca_cert = std::env::var("MTLS_CA_CERT").ok().map(PathBuf::from);

        Ok(Self {
            ods_host,
            ods_port,
            ods_user,
            ods_password,
            ods_name,
            ods_ssl_ca,
            jwt_secret,
            jwt_expiration_days,
            auth_username,
            auth_password,
            auth_secret,
            mtls_server_cert,
            mtls_server_key,
            mtls_ca_cert,
            work_dir,
        })
    }

    /// Resolve caminho relativo em relação ao work_dir (raiz do repo).
    pub fn resolve_path(&self, path: &std::path::Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.work_dir.join(path)
        }
    }

    pub fn ods_ssl_ca_path(&self) -> Option<PathBuf> {
        self.ods_ssl_ca.as_ref().map(|p| self.resolve_path(p))
    }
}

```

## [8] chopebrain-ods-api/src/db.rs

```rust
// FILE: chopebrain-ods-api/src/db.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\db.rs
// SIZE: 1250 bytes
// MTIME: 2026-02-04T02:55:33.208797
// NOTE: Concatenated snapshot for review
//! Pool MySQL ODS com SSL (ODS_SSL_CA).

use crate::config::Config;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use sqlx::MySqlPool;

pub async fn create_pool(cfg: &Config) -> anyhow::Result<MySqlPool> {
    tracing::debug!("db: host={}:{}, database={}, user={}", cfg.ods_host, cfg.ods_port, cfg.ods_name, cfg.ods_user);

    let mut opts = MySqlConnectOptions::new()
        .host(&cfg.ods_host)
        .port(cfg.ods_port)
        .username(&cfg.ods_user)
        .password(&cfg.ods_password)
        .database(&cfg.ods_name);

    if let Some(ca_path) = cfg.ods_ssl_ca_path() {
        if ca_path.exists() {
            tracing::info!("db: SSL CA: {:?}", ca_path);
            opts = opts
                .ssl_mode(MySqlSslMode::VerifyCa)
                .ssl_ca(ca_path);
        } else {
            tracing::warn!("db: ODS_SSL_CA não encontrado: {:?}, usando SSL padrão", ca_path);
        }
    } else {
        tracing::debug!("db: ODS_SSL_CA não configurado");
    }

    let pool = MySqlPool::connect_with(opts).await.map_err(|e| {
        tracing::error!("db: erro ao conectar MySQL: {}", e);
        e
    })?;
    tracing::debug!("db: pool criado (size config padrão)");
    Ok(pool)
}

```

## [9] chopebrain-ods-api/src/handlers.rs

```rust
// FILE: chopebrain-ods-api/src/handlers.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\handlers.rs
// SIZE: 17521 bytes
// MTIME: 2026-02-04T03:11:38.982190
// NOTE: Concatenated snapshot for review
// SECURITY: Content redacted due to secret patterns: TOKEN
//! Rotas e handlers: login, hambúrgueres vendidos, hambúrgueres comandas antigas.

use crate::auth::{self, create_token, validate_login, LoginRequest, LoginResponse};
use crate::config::Config;
use axum::{
    middleware,
    routing::post,
    Router,
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use std::error::Error;
use std::sync::Arc;

/// Loga erro completo: mensagem, cadeia de causas e backtrace (em debug).
fn log_error_full(context: &str, e: &dyn Error) {
    tracing::error!("{}: {}", context, e);
    let mut source = e.source();
    while let Some(s) = source {
        tracing::error!("  caused by: {}", s);
        source = s.source();
    }
    #[cfg(debug_assertions)]
    {
        tracing::error!("  backtrace:\n{:?}", std::backtrace::Backtrace::capture());
    }
}

pub type AppState = (Arc<Config>, Arc<MySqlPool>);

pub fn router(config: Arc<Config>, pool: Arc<MySqlPool>) -> Router {
    let state: AppState = (config, pool);
    Router::new()
        .route("/api/auth/login", post(login))
        .route(
            "/api/ods/hamburgueres-vendidos",
            post(hamburgueres_vendidos)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth::require_jwt)),
        )
        .route(
            "/api/ods/hamburgueres-comandas-antigas",
            post(hamburgueres_comandas_antigas)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth::require_jwt)),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

async fn login(
    State((config, _)): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/auth/login");
    if !validate_login(config.as_ref(), &req) {
        tracing::warn!("handler: POST /api/auth/login -> 401 credenciais inválidas");
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Credenciais inválidas" })),
        ));
    }
    let sub = req
        .username
        .as_deref()
        .unwrap_or("api")
        .to_string();
    let token = REDACTED:TOKEN &sub).map_err(|e| {
        log_error_full("handler: POST /api/auth/login -> 500 erro ao criar JWT", e.as_ref());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    tracing::info!("handler: POST /api/auth/login -> 200 OK (token gerado para sub={})", sub);
    Ok(Json(LoginResponse { token }))
}

#[derive(Debug, Deserialize)]
pub struct MesRequest {
    pub mes: String, // "YYYY-MM"
}

// Regra hambúrguer: categoria etiquetas.descricao IN ('Hamburger','Hamburguer') OU nome do produto com palavras-chave.
const HAMBURGER_CATEGORY_FILTER: &str = "AND (LOWER(e.descricao) IN ('hamburger','hamburguer') OR LOWER(pr.nome) LIKE '%hamburguer%' OR LOWER(pr.nome) LIKE '%burger%' OR LOWER(pr.nome) LIKE '%cheeseburger%' OR LOWER(pr.nome) LIKE '%bacon burger%' OR LOWER(pr.nome) LIKE '%frango burger%' OR LOWER(pr.nome) LIKE '%artesanal burger%')";

#[derive(Debug, Serialize)]
pub struct HamburgueresVendidosResponse {
    pub totais: Totais,
    pub por_categoria: Vec<PorCategoria>,
    pub por_produto: Vec<PorProduto>,
    pub itens: Vec<ItemVenda>,
}

#[derive(Debug, Serialize)]
pub struct Totais {
    pub quantidade: i64,
    pub valor: f64,
    pub ticket_medio: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorCategoria {
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorProduto {
    pub produto: String,
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ItemVenda {
    pub data_venda_item: Option<String>,
    pub pedido: i64,
    pub produto: String,
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

async fn hamburgueres_vendidos(
    State((_config, pool)): State<AppState>,
    Json(body): Json<MesRequest>,
) -> Result<Json<HamburgueresVendidosResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/ods/hamburgueres-vendidos mes={}", body.mes);
    let (ano, mes) = parse_mes(&body.mes)?;
    tracing::debug!("handler: hamburgueres-vendidos ano={} mes={}", ano, mes);
    let total: Option<(i64, f64)> = sqlx::query_as(
        &format!(
            r#"
            SELECT CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED), COALESCE(SUM(ip.quantidade * ip.valorunitario), 0)
            FROM itenspedido ip
            INNER JOIN pedidos ped ON ped.codigo = ip.codigopedido
            INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
            INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
            LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
            WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
            {}
            "#,
            HAMBURGER_CATEGORY_FILTER
        ),
    )
    .bind(ano)
    .bind(mes)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (totais)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let (quantidade, valor) = total.unwrap_or((0, 0.0));
    let ticket_medio = if quantidade > 0 { valor / (quantidade as f64) } else { 0.0 };
    tracing::debug!("handler: hamburgueres-vendidos totais quantidade={} valor={}", quantidade, valor);

    let por_categoria: Vec<PorCategoria> = sqlx::query_as(&format!(
        r#"
        SELECT e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.id = ip.pedidoid
        INNER JOIN produtodetalhe pd ON pd.id = ip.produtodetalheid
        INNER JOIN produtos pr ON pr.id = pd.produtoid
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        GROUP BY e.descricao
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (por_categoria)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let por_produto: Vec<PorProduto> = sqlx::query_as(&format!(
        r#"
        SELECT pr.nome AS produto, e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        GROUP BY pr.codigo, pr.nome, e.descricao
        ORDER BY quantidade DESC
        LIMIT 10
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (por_produto)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let itens: Vec<ItemVenda> = sqlx::query_as(&format!(
        r#"
        SELECT DATE_FORMAT(ip.datahoracadastro, '%Y-%m-%d %H:%i') AS data_venda_item, ped.codigo AS pedido, pr.nome AS produto, e.descricao AS categoria, CAST(ip.quantidade AS SIGNED) AS quantidade, (ip.quantidade * ip.valorunitario) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.id = ip.pedidoid
        INNER JOIN produtodetalhe pd ON pd.id = ip.produtodetalheid
        INNER JOIN produtos pr ON pr.id = pd.produtoid
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        ORDER BY ip.datahoracadastro, ped.codigo
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (itens)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    tracing::info!("handler: POST /api/ods/hamburgueres-vendidos -> 200 OK (quantidade={}, valor={})", quantidade, valor);
    Ok(Json(HamburgueresVendidosResponse {
        totais: Totais {
            quantidade,
            valor,
            ticket_medio,
        },
        por_categoria,
        por_produto,
        itens,
    }))
}

fn parse_mes(mes: &str) -> Result<(i32, u32), (StatusCode, Json<serde_json::Value>)> {
    let parts: Vec<&str> = mes.split('-').collect();
    if parts.len() != 2 {
        tracing::warn!("handler: parse_mes inválido (esperado YYYY-MM): {:?}", mes);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mes deve ser YYYY-MM" })),
        ));
    }
    let ano: i32 = parts[0].parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "ano inválido" })),
        )
    })?;
    let mes: u32 = parts[1].parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mês inválido" })),
        )
    })?;
    if mes < 1 || mes > 12 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mês deve ser 01-12" })),
        ));
    }
    Ok((ano, mes))
}

// --- Hamburgueres comandas antigas ---

#[derive(Debug, Serialize)]
pub struct HamburgueresComandasAntigasResponse {
    pub totais: Totais,
    pub por_produto: Vec<PorProduto>,
    pub por_pedido: Vec<PorPedido>,
    pub detalhamento: Vec<DetalheComandaAntiga>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorPedido {
    pub pedido_id: i64,
    pub data_abertura: Option<String>,
    pub data_fechamento: Option<String>,
    pub itens: i64,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DetalheComandaAntiga {
    pub data_venda_item: Option<String>,
    pub pedido: i64,
    pub data_abertura: Option<String>,
    pub dias_diferenca: Option<i64>,
    pub produto: String,
    pub quantidade: i64,
    pub valor: f64,
}

async fn hamburgueres_comandas_antigas(
    State((_config, pool)): State<AppState>,
    Json(body): Json<MesRequest>,
) -> Result<Json<HamburgueresComandasAntigasResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/ods/hamburgueres-comandas-antigas mes={}", body.mes);
    let (ano, mes) = parse_mes(&body.mes)?;
    tracing::debug!("handler: hamburgueres-comandas-antigas ano={} mes={}", ano, mes);
    let comanda_antiga_filter = format!(
        "AND (YEAR(p.dataabertura) < {} OR (YEAR(p.dataabertura) = {} AND MONTH(p.dataabertura) < {}))",
        ano, ano, mes
    );
    let base_filter = format!(
        "WHERE YEAR(ip.datahoracadastro) = {} AND MONTH(ip.datahoracadastro) = {} {} ",
        ano, mes, comanda_antiga_filter
    );

    let total_quantidade: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED)
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (total_quantidade)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let total_valor: f64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(ip.quantidade * ip.valorunitario), 0)
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (total_valor)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let quantidade = total_quantidade;
    let valor = total_valor;
    let ticket_medio = if quantidade > 0 { valor / (quantidade as f64) } else { 0.0 };

    let por_produto: Vec<PorProduto> = sqlx::query_as(&format!(
        r#"
        SELECT pr.nome AS produto, e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        GROUP BY pr.codigo, pr.nome, e.descricao
        ORDER BY quantidade DESC
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (por_produto)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let por_pedido: Vec<PorPedido> = sqlx::query_as(&format!(
        r#"
        SELECT p.codigo AS pedido_id, DATE_FORMAT(p.dataabertura, '%Y-%m-%d %H:%i') AS data_abertura, DATE_FORMAT(p.datafechamento, '%Y-%m-%d %H:%i') AS data_fechamento,
               COUNT(*) AS itens, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        GROUP BY p.codigo, p.dataabertura, p.datafechamento
        ORDER BY p.dataabertura
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (por_pedido)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let detalhamento: Vec<DetalheComandaAntiga> = sqlx::query_as(&format!(
        r#"
        SELECT DATE_FORMAT(ip.datahoracadastro, '%Y-%m-%d %H:%i') AS data_venda_item, p.codigo AS pedido, DATE_FORMAT(p.dataabertura, '%Y-%m-%d') AS data_abertura,
               DATEDIFF(ip.datahoracadastro, p.dataabertura) AS dias_diferenca, pr.nome AS produto, CAST(ip.quantidade AS SIGNED) AS quantidade, (ip.quantidade * ip.valorunitario) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        ORDER BY ip.datahoracadastro, p.codigo
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (detalhamento)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    tracing::info!("handler: POST /api/ods/hamburgueres-comandas-antigas -> 200 OK (quantidade={}, valor={})", quantidade, valor);
    Ok(Json(HamburgueresComandasAntigasResponse {
        totais: Totais {
            quantidade,
            valor,
            ticket_medio,
        },
        por_produto,
        por_pedido,
        detalhamento,
    }))
}

```

## [10] chopebrain-ods-api/src/lib.rs

```rust
// FILE: chopebrain-ods-api/src/lib.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\lib.rs
// SIZE: 79 bytes
// MTIME: 2026-02-04T01:59:47.895526
// NOTE: Concatenated snapshot for review
pub mod config;
pub mod db;
pub mod auth;
pub mod handlers;
pub mod mtls;

```

## [11] chopebrain-ods-api/src/main.rs

```rust
// FILE: chopebrain-ods-api/src/main.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\main.rs
// SIZE: 2013 bytes
// MTIME: 2026-02-04T02:55:26.514309
// NOTE: Concatenated snapshot for review
use chopebrain_ods_api::{config, db, handlers, mtls};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "chopebrain_ods_api=debug,tower_http=debug,info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().with_target(true).with_thread_ids(false))
        .init();

    tracing::info!("=== Iniciando Chopebrain ODS API ===");

    let work_dir = config::load_dotenv_from_root()?;
    tracing::info!("config: .env carregado da raiz: {}", work_dir.display());

    let cfg = config::Config::from_env().map_err(|e| {
        tracing::error!("config: falha ao carregar: {}", e);
        anyhow::anyhow!("{}", e)
    })?;
    let cfg = Arc::new(cfg);
    tracing::debug!("config: ODS_HOST={}, ODS_NAME={}, JWT_EXPIRATION_DAYS={}", cfg.ods_host, cfg.ods_name, cfg.jwt_expiration_days);

    tracing::info!("db: criando pool MySQL ODS...");
    let pool = db::create_pool(&cfg).await.map_err(|e| {
        tracing::error!("db: falha ao conectar: {}", e);
        anyhow::anyhow!("{}", e)
    })?;
    let pool = Arc::new(pool);
    tracing::info!("db: pool MySQL ODS conectado com sucesso");

    let app = handlers::router(cfg.clone(), pool);

    let listen = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let addr: std::net::SocketAddr = listen.parse()?;

    if mtls::mtls_enabled(&cfg) {
        tracing::info!("server: iniciando HTTPS com mTLS em https://{}", addr);
        mtls::serve_mtls(app, addr, &cfg).await?;
    } else {
        tracing::info!("server: iniciando HTTP em http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("server: escutando em {}", addr);
        axum::serve(listener, app).await?;
    }
    Ok(())
}

```

## [12] chopebrain-ods-api/src/mtls.rs

```rust
// FILE: chopebrain-ods-api/src/mtls.rs
// FULL: C:\Users\leopo\rust-studies\chopebrain-ods-api\src\mtls.rs
// SIZE: 3570 bytes
// MTIME: 2026-02-04T02:57:40.576031
// NOTE: Concatenated snapshot for review
//! Servidor HTTPS com mTLS (exige certificado do cliente).

use crate::config::Config;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use rustls::server::WebPkiClientVerifier;
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

pub fn mtls_enabled(cfg: &Config) -> bool {
    if std::env::var("FORCE_HTTP").as_deref() == Ok("1") {
        return false;
    }
    let server_cert = cfg
        .mtls_server_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .or_else(|| Some(cfg.work_dir.join("certs-mtls").join("server-cert.pem")));
    let server_key = cfg
        .mtls_server_key
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .or_else(|| Some(cfg.work_dir.join("certs-mtls").join("server-key.pem")));
    server_cert.map(|p| p.exists()).unwrap_or(false)
        && server_key.map(|p| p.exists()).unwrap_or(false)
}

fn load_cert(path: &Path) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    certs(&mut reader).collect::<Result<Vec<_>, _>>().map_err(|e| anyhow::anyhow!("{:?}", e))
}

fn load_private_key(
    path: &Path,
) -> anyhow::Result<rustls::pki_types::PrivateKeyDer<'static>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    private_key(&mut reader)?
        .ok_or_else(|| anyhow::anyhow!("Nenhuma chave privada encontrada em {:?}", path))
}

fn load_ca_certs(
    path: &Path,
) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    certs(&mut reader).collect::<Result<Vec<_>, _>>().map_err(|e| anyhow::anyhow!("{:?}", e))
}

pub async fn serve_mtls(
    app: Router,
    addr: std::net::SocketAddr,
    cfg: &Config,
) -> anyhow::Result<()> {
    let server_cert_path = cfg
        .mtls_server_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("server-cert.pem"));
    let server_key_path = cfg
        .mtls_server_key
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("server-key.pem"));
    let ca_path = cfg
        .mtls_ca_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("ca.pem"));

    let certs = load_cert(&server_cert_path)?;
    let key = load_private_key(&server_key_path)?;
    let ca_certs = load_ca_certs(&ca_path)?;

    let mut root_store = rustls::RootCertStore::empty();
    for cert in ca_certs {
        root_store.add(cert).map_err(|e| anyhow::anyhow!("CA cert: {}", e))?;
    }

    let client_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .map_err(|e| anyhow::anyhow!("client verifier: {}", e))?;

    let mut config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(certs, key)
        .map_err(|e| anyhow::anyhow!("Server cert: {}", e))?;

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    let rustls_config = RustlsConfig::from_config(Arc::new(config));
    axum_server::bind_rustls(addr, rustls_config)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("serve_mtls: {}", e))?;
    Ok(())
}

```

## [13] scripts/concat_repo_all_text.py

`````python
# FILE: scripts/concat_repo_all_text.py
# FULL: C:\Users\leopo\rust-studies\scripts\concat_repo_all_text.py
# SIZE: 87898 bytes
# MTIME: 2026-02-04T09:54:24.476811
# NOTE: Concatenated snapshot for review
# SECURITY: Content redacted due to secret patterns: API_KEY, PASSWORD, TOKEN
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Concatena TODOS os arquivos de TEXTO de 1..N projetos em um .md por projeto,
respeitando .gitignore, evitando binários e arquivos muito grandes.
Evita auto-inclusão do próprio arquivo MD gerado e QUALQUER item relacionado a ".git".

Gera um MD rico em inteligência arquitetural com:
- Project Intelligence: tree, stack detectada, comandos úteis, entrypoints, env vars, API routes
- File Index: tabela com metadados de todos os arquivos
- Modos FULL e SMART para controle de tamanho
- Side-car JSON opcional com metadados estruturados

MELHORIAS:
- Exclui diretórios de dependências (.venv, node_modules, __pycache__, etc.)
- Detecta e exclui diretórios muito grandes (>100MB por padrão)
- Lista expandida de padrões de diretórios de bibliotecas e dependências
- Filtragem inteligente para evitar sobrecarregar o arquivo MD final
- Auto-detecção do ROOT do repositório: procura por .git subindo diretórios a partir do script
- Project Intelligence: análise automática de stack, comandos, entrypoints, env vars, rotas API
- Modo SMART: trunca arquivos grandes mantendo header, assinaturas e footer
- Side-car JSON: metadados estruturados para análise programática

CLI:
  # Modo single (auto-detecta ROOT do repo onde o script está):
  python concat_repo_all_text.py
  
  # Modo multi (processa múltiplos projetos):
  python concat_repo_all_text.py "C:\\Projetos\\projA" "C:\\Projetos\\projB"

AUTO-DETECÇÃO (quando executado sem argumentos e sem env vars):
- ROOT: começa no diretório do script e sobe procurando por .git. Se encontrar, usa esse diretório.
         Se não encontrar, usa o diretório do script como ROOT.
- OUT_PATH: se CONCAT_OUT_PATH não estiver definido, usa <ROOT>/repo_concat_all.md

ENV VARS (sobrescrevem defaults):
- CONCAT_ROOT: define ROOT explicitamente (sobrescreve auto-detecção)
- CONCAT_OUT_PATH: define caminho completo do arquivo de saída (sobrescreve default)
- CONCAT_OUT_BASENAME: nome do arquivo de saída (padrão: "repo_concat_all.md")
- CONCAT_ROOTS: lista de projetos separados por | ou ; (modo multi via env)
- MAX_FILE_BYTES: tamanho máximo por arquivo (padrão: 2000000 = 2MB)
- CONCAT_MODE: modo de concatenação - "full" ou "smart" (padrão: "smart")
- SMART_TRUNCATE_BYTES: tamanho em bytes para aplicar truncamento smart (padrão: 200000 = 200KB)
- MAX_TOTAL_BYTES: tamanho máximo total do MD (0 = sem limite, padrão: 0)
- CONCAT_SIDE_CAR_JSON: gerar JSON com metadados ("1", "true", "yes" para ativar, padrão: desativado)

PROJECT INTELLIGENCE:
O MD gerado inclui automaticamente:
- Repository Tree: estrutura de diretórios (até profundidade 4)
- Stack Detected: linguagens, frameworks e infra detectados
- How to Run: comandos extraídos de Makefile, package.json, docker-compose, etc.
- Entrypoints: arquivos principais prováveis (main.py, app.py, etc.)
- Environment Variables: nomes de variáveis (sem valores) e onde aparecem
- API Surface: rotas detectadas (FastAPI, Express, etc.)
- RAG/LLM & Observability: sinais de bibliotecas relacionadas
- Smells/Gaps: TODOs, presença de testes, .env real, maiores arquivos

MODO SMART:
No modo "smart" (padrão), arquivos maiores que SMART_TRUNCATE_BYTES são truncados mantendo:
- Primeiras 120 linhas (header)
- Assinaturas detectadas (def/class/import em Python, export/function em JS/TS, etc.)
- Últimas 80 linhas (footer)
- Aviso de truncamento

O modo "full" mantém o comportamento original (conteúdo completo, respeitando MAX_FILE_BYTES).
"""

from __future__ import annotations
import os
import sys  
import subprocess
import shutil
import re
import json
from datetime import datetime
from typing import Iterable, List, Set, Optional, Tuple, Dict, Any

# ----- Auto-detecção do ROOT do repositório -----
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

def find_repo_root(start_dir: str) -> str:
    """
    Procura por um diretório .git subindo a partir de start_dir até a raiz do filesystem.
    Retorna o primeiro diretório encontrado que contém .git, ou start_dir se não encontrar.
    """
    current = os.path.abspath(start_dir)
    root_path = os.path.abspath(os.sep)  # Raiz do filesystem (/, C:\, etc.)
    
    while True:
        git_dir = os.path.join(current, ".git")
        if os.path.isdir(git_dir):
            return current
        # Se chegou na raiz do filesystem, para
        if current == root_path or os.path.dirname(current) == current:
            break
        current = os.path.dirname(current)
    
    # Se não encontrou .git, retorna o diretório inicial
    return start_dir

# ----- Defaults / Env (com auto-detecção) -----
DEFAULT_ROOT = find_repo_root(SCRIPT_DIR)
ROOT = os.getenv("CONCAT_ROOT", DEFAULT_ROOT)
MAX_FILE_BYTES = int(os.getenv("MAX_FILE_BYTES", "2000000"))  # 2MB
CONCAT_ROOTS_ENV = os.getenv("CONCAT_ROOTS", "").strip()
CONCAT_OUT_BASENAME = (os.getenv("CONCAT_OUT_BASENAME", "repo_concat_all.md").strip() or "repo_concat_all.md")
# OUT_PATH: se CONCAT_OUT_PATH não estiver definido, usa <ROOT>/<CONCAT_OUT_BASENAME>
OUT_PATH = os.getenv("CONCAT_OUT_PATH", os.path.join(ROOT, CONCAT_OUT_BASENAME))
# Novos env vars para modos e limites
CONCAT_MODE = os.getenv("CONCAT_MODE", "smart").strip().lower()  # "full" ou "smart"
SMART_TRUNCATE_BYTES = int(os.getenv("SMART_TRUNCATE_BYTES", "200000"))  # 200KB
MAX_TOTAL_BYTES = int(os.getenv("MAX_TOTAL_BYTES", "0"))  # 0 = sem limite
CONCAT_SIDE_CAR_JSON = os.getenv("CONCAT_SIDE_CAR_JSON", "").strip().lower() in ("1", "true", "yes")

# Extensões de binários/arquivos indesejados
EXCLUDE_EXTS: Set[str] = {
    ".png", ".jpg", ".jpeg", ".gif", ".webp", ".bmp", ".ico", ".svg",
    ".mp3", ".wav", ".ogg", ".flac", ".mp4", ".mkv", ".webm", ".mov", ".avi",
    ".woff", ".woff2", ".ttf", ".otf", ".eot",
    ".zip", ".rar", ".7z", ".tar", ".gz", ".bz2", ".xz",
    ".apk", ".ipa", ".dmg", ".iso",
    ".exe", ".dll", ".so", ".dylib", ".o", ".a", ".obj", ".class", ".jar",
    ".sqlite", ".db", ".fdb", ".mdb", ".accdb", ".parquet", ".xls", ".xlsx",
    ".pdf", ".psd",
}

# Regex patterns para exclusão de arquivos sensíveis (case-insensitive)
# CRÍTICO: Estes padrões bloqueiam arquivos que NUNCA devem aparecer no MD
EXCLUDE_BASENAME_REGEX = [
    r'^\.env',  # .env, .env.local, .env.prod, .env.development, etc. (qualquer .env*)
    r'^\.envrc$',
    r'^\.npmrc$',
    r'^\.pypirc$',
    r'credentials.*\.json$',  # credentials.json, credentials-prod.json, etc.
    r'token.*\.json$',
    r'service[_-]?account.*\.json$',  # service-account.json, service_account.json
    r'.*[_-]sa\.json$',  # qualquer arquivo terminando em -sa.json ou _sa.json
    r'id[_-]?rsa',  # id_rsa, id-rsa, idrsa (com ou sem .pub)
    r'id[_-]?ed25519',  # id_ed25519, id-ed25519
    r'known_hosts$',
    r'authorized_keys$',  # chaves SSH autorizadas
    r'secrets.*\.(json|yaml|yml)$',
    r'private.*\.(json|yaml|yml)$',
    r'oauth.*\.(json|yaml|yml)$',
    r'apikey.*\.(json|yaml|yml)$',
    r'gcp.*\.json$',
    r'firebase.*\.json$',
]

EXCLUDE_PATH_REGEX = [
    r'[/\\]\.env',  # Qualquer .env em qualquer subpasta
    r'[/\\]secrets[/\\]',
    r'[/\\]secret[/\\]',
    r'[/\\]keys[/\\]',
    r'[/\\]certs[/\\]',
    r'[/\\]\.ssh[/\\]',
    r'[/\\]\.aws[/\\]',
    r'[/\\]private[/\\]',
    r'[/\\]credentials[/\\]',
]

# Extensões de arquivos sensíveis (chaves, certificados, bancos de dados locais)
# CRÍTICO: Arquivos com estas extensões são SEMPRE excluídos
SENSITIVE_EXTS: Set[str] = {
    ".pem", ".key", ".p12", ".pfx", ".crt", ".cer", ".jks", ".kdbx",  # Chaves e certificados
    ".sqlite", ".sqlite3", ".db", ".fdb", ".mdb", ".accdb",  # Bancos de dados locais
}

FALLBACK_PRUNE_DIRS: Set[str] = {
    # Git e controle de versão
    ".git", ".svn", ".hg", ".bzr",
    
    # Node.js / JavaScript
    "node_modules", "npm-debug.log*", "yarn-debug.log*", "yarn-error.log*",
    ".next", ".nuxt", ".vuepress", ".cache", ".turbo", "dist", "build", "out",
    "coverage", ".nyc_output", ".jest", ".eslintcache",
    
    # Python
    ".venv", "venv", "env", ".env", ".virtualenv", "__pycache__", ".pytest_cache",
    ".mypy_cache", ".coverage", ".tox", ".nox", "site-packages", ".Python",
    "pip-log.txt", "pip-delete-this-directory.txt", ".pytest_cache",
    
    # Java
    "target", ".gradle", ".mvn", "build", "out", ".idea", ".vscode",
    
    # C/C++
    "Debug", "Release", "x64", "x86", "obj", "bin", ".vs", "CMakeFiles",
    
    # Go
    "vendor", "go.sum", "go.mod",
    
    # Rust
    "target", "Cargo.lock",
    
    # Ruby
    "vendor", "bundle", ".bundle", "Gemfile.lock",
    
    # PHP
    "vendor", "composer.lock",
    
    # Outros
    ".DS_Store", "Thumbs.db", "*.log", "logs", "tmp", "temp", ".tmp",
    "coverage", "test-results", ".nyc_output", ".nyc_output",
    "bower_components", ".sass-cache", ".parcel-cache",
    "android", "ios", "platforms", "plugins", "www",
    ".expo", ".expo-shared", "expo-env.d.ts",
    ".docusaurus", "docs", "documentation",
    "backup", "backups", ".backup", ".backups",
    "archive", "archives", ".archive", ".archives",
    "old", "deprecated", "legacy",
    "test", "tests", "spec", "specs", "__tests__", "__spec__",
    "mocks", "fixtures", "stubs", "doubles",
    "examples", "samples", "demos", "playground",
    "docs", "documentation", "wiki", "guides",
    "assets", "static", "public", "media", "images", "img", "icons",
    "fonts", "css", "styles", "themes", "skins",
    "locales", "i18n", "translations", "lang",
    "config", "configs", "settings", "conf",
    "secrets", "credentials", "keys", "certs", "certificates",
    "migrations", "seeds", "fixtures", "factories",
    "deploy", "deployment", "deployments", "infra", "infrastructure",
    "docker", "containers", "k8s", "kubernetes", "helm",
    "terraform", "ansible", "chef", "puppet",
    "scripts", "tools", "utilities", "bin", "sbin",
    "data", "databases", "db", "dbs", "storage",
    "cache", "caches", ".cache", "cached",
    "temp", "temporary", ".temp", ".tmp",
    "runtime", "runtimes", "binaries", "executables",
    "packages", "pkg", "pkgs", "libraries",
    "modules", "components", "widgets", "plugins", "extensions",
    "addons", "add-ons", "extensions", "extras",
    "third-party", "third_party", "external", "externals",
    "dependencies", "deps", "dev-dependencies", "dev_dependencies",
    "peer-dependencies", "peer_dependencies", "optional-dependencies",
    "bundled-dependencies", "bundled_dependencies",
    "workspaces", "monorepo", "packages", "apps",
    "generated", "auto-generated", "auto_generated", "build-generated",
    "compiled", "transpiled", "minified", "uglified",
    "sourcemaps", "source-maps", "source_maps",
    "types", "typings", "definitions", "interfaces",
    "schemas", "models", "entities", "dto", "dtos",
    "enums", "constants", "configs", "settings",
    "utils", "utilities", "helpers", "helpers",
    "validators", "validations", "sanitizers", "formatters",
    "parsers", "serializers", "deserializers", "mappers",
    "repositories", "repos", "services", "controllers",
    "middleware", "middlewares", "guards", "interceptors",
    "decorators", "annotations", "attributes", "metadata",
    "events", "listeners", "handlers", "processors",
    "workers", "jobs", "tasks", "schedulers", "cron",
    "queues", "streams", "pipes", "filters", "transformers",
    "adapters", "facades", "proxies", "wrappers",
    "factories", "builders", "creators", "generators",
    "singletons", "managers", "providers", "injectors",
    "resolvers", "loaders", "initializers", "bootstrap",
    "startup", "shutdown", "cleanup", "teardown",
    "migrations", "seeds", "fixtures", "factories",
    "fakers", "mocks", "stubs", "doubles", "spies",
    "test-utils", "test_utils", "test-helpers", "test_helpers",
    "test-data", "test_data", "test-fixtures", "test_fixtures",
    "e2e", "integration", "unit", "spec", "specs",
    "stories", "storybook", "chromatic",
    "cypress", "playwright", "selenium", "webdriver",
    "jest", "mocha", "jasmine", "karma", "vitest",
    "eslint", "prettier", "husky", "lint-staged",
    "commitizen", "commitlint", "conventional-changelog",
    "semantic-release", "release", "releases",
    "changelog", "changelogs", "history", "histories",
    "version", "versions", "tags", "branches",
    "main", "master", "develop", "dev", "staging", "stage",
    "production", "prod", "preview", "demo", "sandbox",
    "experimental", "beta", "alpha", "rc", "release-candidate",
    "hotfix", "bugfix", "feature", "enhancement", "improvement",
    "refactor", "cleanup", "optimization", "performance",
    "security", "vulnerability", "patch", "fix", "bug",
    "issue", "issues", "ticket", "tickets", "task", "tasks",
    "todo", "todos", "note", "notes", "comment", "comments",
    "review", "reviews", "feedback", "suggestions",
    "proposal", "proposals", "rfc", "rfcs", "adr", "adrs",
    "decision", "decisions", "meeting", "meetings",
    "agenda", "agendas", "minutes", "summary", "summaries",
    "report", "reports", "analysis", "analyses",
    "research", "investigation", "exploration", "experiment",
    "experiments", "proof-of-concept", "poc", "prototype",
    "prototypes", "mockup", "mockups", "wireframe", "wireframes",
    "design", "designs", "ui", "ux", "frontend", "backend",
    "apis", "rest", "graphql", "grpc", "soap",
    "websocket", "websockets", "sse", "sse", "webhook", "webhooks",
    "microservice", "microservices", "service", "services",
    "gateway", "gateways", "proxy", "proxies", "load-balancer",
    "load-balancers", "reverse-proxy", "reverse-proxies",
    "cdn", "cdn", "edge", "edges", "cache", "caches",
    "database", "databases", "db", "dbs", "sql", "nosql",
    "mongodb", "mysql", "postgresql", "postgres", "sqlite",
    "redis", "memcached", "elasticsearch", "solr",
    "kafka", "rabbitmq", "activemq", "sqs", "sns",
    "s3", "gcs", "azure-blob", "azure-blob-storage",
    "cloud", "aws", "azure", "gcp", "google-cloud",
    "docker", "kubernetes", "k8s", "helm", "terraform",
    "ansible", "chef", "puppet", "salt", "vagrant",
    "jenkins", "gitlab-ci", "github-actions", "azure-devops",
    "circleci", "travis-ci", "appveyor", "bamboo",
    "teamcity", "buildkite", "drone", "concourse",
    "monitoring", "observability", "logging", "tracing",
    "metrics", "alerts", "dashboards", "grafana",
    "prometheus", "influxdb", "datadog", "newrelic",
    "sentry", "rollbar", "bugsnag", "honeybadger",
    "security", "vulnerability", "scanning", "auditing",
    "compliance", "governance", "policies", "standards",
    "documentation", "docs", "wiki", "guides", "tutorials",
    "examples", "samples", "demos", "playground",
    "sandbox", "experimental", "beta", "alpha",
    "deprecated", "legacy", "old", "archive", "backup"
}

LANG_BY_EXT = {
    ".ts": "ts", ".tsx": "tsx",
    ".js": "js", ".jsx": "jsx", ".mjs": "js", ".cjs": "js",
    ".py": "python", ".java": "java", ".kt": "kotlin", ".go": "go",
    ".rb": "ruby", ".php": "php", ".rs": "rust", ".cpp": "cpp", ".cc": "cpp", ".cxx": "cpp", ".c": "c", ".h": "c",
    ".cs": "csharp", ".swift": "swift", ".scala": "scala",
    ".sh": "bash", ".bat": "bat", ".ps1": "powershell",
    ".json": "json", ".yml": "yaml", ".yaml": "yaml", ".toml": "toml", ".ini": "ini", ".conf": "ini",
    ".sql": "sql", ".md": "markdown", ".txt": "text", ".env": "dotenv",
    ".html": "html", ".htm": "html", ".css": "css",
}

# ---------- Utilidades ----------
def is_git_repo(root: str) -> bool:
    return os.path.isdir(os.path.join(root, ".git"))

def have_cmd(cmd: str) -> bool:
    return shutil.which(cmd) is not None

def path_has_git(rel_path: str) -> bool:
    """
    Exclui SOMENTE diretórios/itens dentro de `.git*` (ex.: `.git/`, `.git/objects/`, etc).
    IMPORTANTE: NÃO exclui arquivos como `.gitignore`, `.gitattributes`, `.gitmodules`.
    """
    p = rel_path.replace("\\", "/").lower()
    # Segmentos .git* (mas permite arquivos "dot-git" comuns)
    allowed_dot_git_files = {".gitignore", ".gitattributes", ".gitmodules", ".gitkeep"}
    for seg in p.split("/"):
        if seg == ".git":
            return True
        if seg.startswith(".git") and seg not in allowed_dot_git_files:
            return True
    return False

def is_dependency_dir(dirname: str, dir_path: str = None) -> bool:
    """
    Detecta se um diretório é de dependências, bibliotecas ou arquivos gerados
    que devem ser excluídos para evitar sobrecarregar o arquivo MD final.
    """
    dirname_lower = dirname.lower()
    
    # Diretórios do projeto que NÃO devem ser excluídos
    project_dirs = {"src", "api", "libs", "lib", "modules", "components", "utils", "helpers", "backend", "frontend", "tests", "scripts", "desktop"}
    if dirname_lower in project_dirs:
        return False
    
    # Verifica se está na lista de diretórios conhecidos
    if dirname_lower in FALLBACK_PRUNE_DIRS:
        return True
    
    # Se o diretório for muito grande, considera como dependência
    if dir_path and is_large_directory(dir_path):
        return True
    
    # Padrões comuns de diretórios de dependências
    dependency_patterns = [
        # Python
        r"__pycache__", r"\.pyc$", r"\.pyo$", r"\.pyd$", r"\.egg-info$",
        r"\.dist-info$", r"site-packages", r"\.virtualenv", r"\.env",
        
        # Node.js
        r"node_modules", r"\.npm", r"\.yarn", r"\.pnpm", r"\.cache",
        r"\.next", r"\.nuxt", r"\.vuepress", r"\.turbo", r"dist",
        r"build", r"out", r"coverage", r"\.nyc_output", r"\.jest",
        
        # Java
        r"target", r"\.gradle", r"\.mvn", r"\.idea", r"\.vscode",
        r"\.classpath", r"\.project", r"\.settings",
        
        # C/C++
        r"Debug", r"Release", r"x64", r"x86", r"obj", r"bin",
        r"\.vs", r"CMakeFiles", r"\.cmake",
        
        # Go
        r"vendor", r"go\.sum", r"go\.mod",
        
        # Rust
        r"target", r"Cargo\.lock",
        
        # Ruby
        r"vendor", r"bundle", r"\.bundle", r"Gemfile\.lock",
        
        # PHP
        r"vendor", r"composer\.lock",
        
        # Padrões gerais
        r"\.DS_Store", r"Thumbs\.db", r"\.log$", r"logs", r"tmp",
        r"temp", r"\.tmp", r"\.cache", r"cached", r"runtime",
        r"runtimes", r"binaries", r"executables", r"packages",
        r"pkg", r"pkgs", r"libs", r"libraries", r"modules",
        r"components", r"widgets", r"plugins", r"extensions",
        r"addons", r"add-ons", r"extras", r"third-party",
        r"third_party", r"external", r"externals", r"dependencies",
        r"deps", r"dev-dependencies", r"dev_dependencies",
        r"peer-dependencies", r"peer_dependencies", r"optional-dependencies",
        r"bundled-dependencies", r"bundled_dependencies", r"workspaces",
        r"monorepo", r"apps", r"generated", r"auto-generated",
        r"auto_generated", r"build-generated", r"compiled",
        r"transpiled", r"minified", r"uglified", r"sourcemaps",
        r"source-maps", r"source_maps", r"types", r"typings",
        r"definitions", r"interfaces", r"schemas", r"models",
        r"entities", r"dto", r"dtos", r"enums", r"constants",
        r"configs", r"settings", r"utils", r"utilities", r"helpers",
        r"validators", r"validations", r"sanitizers", r"formatters",
        r"parsers", r"serializers", r"deserializers", r"mappers",
        r"repositories", r"repos", r"services", r"controllers",
        r"middleware", r"middlewares", r"guards", r"interceptors",
        r"decorators", r"annotations", r"attributes", r"metadata",
        r"events", r"listeners", r"handlers", r"processors",
        r"workers", r"jobs", r"tasks", r"schedulers", r"cron",
        r"queues", r"streams", r"pipes", r"filters", r"transformers",
        r"adapters", r"facades", r"proxies", r"wrappers",
        r"factories", r"builders", r"creators", r"generators",
        r"singletons", r"managers", r"providers", r"injectors",
        r"resolvers", r"loaders", r"initializers", r"bootstrap",
        r"startup", r"shutdown", r"cleanup", r"teardown",
        r"migrations", r"seeds", r"fixtures", r"factories",
        r"fakers", r"mocks", r"stubs", r"doubles", r"spies",
        r"test-utils", r"test_utils", r"test-helpers", r"test_helpers",
        r"test-data", r"test_data", r"test-fixtures", r"test_fixtures",
        r"e2e", r"integration", r"unit", r"spec", r"specs",
        r"stories", r"storybook", r"chromatic", r"cypress",
        r"playwright", r"selenium", r"webdriver", r"jest", r"mocha",
        r"jasmine", r"karma", r"vitest", r"eslint", r"prettier",
        r"husky", r"lint-staged", r"commitizen", r"commitlint",
        r"conventional-changelog", r"semantic-release", r"release",
        r"releases", r"changelog", r"changelogs", r"history",
        r"histories", r"version", r"versions", r"tags", r"branches",
        r"main", r"master", r"develop", r"dev", r"staging", r"stage",
        r"production", r"prod", r"preview", r"demo", r"sandbox",
        r"experimental", r"beta", r"alpha", r"rc", r"release-candidate",
        r"hotfix", r"bugfix", r"feature", r"enhancement", r"improvement",
        r"refactor", r"cleanup", r"optimization", r"performance",
        r"security", r"vulnerability", r"patch", r"fix", r"bug",
        r"issue", r"issues", r"ticket", r"tickets", r"task", r"tasks",
        r"todo", r"todos", r"note", r"notes", r"comment", r"comments",
        r"review", r"reviews", r"feedback", r"suggestions",
        r"proposal", r"proposals", r"rfc", r"rfcs", r"adr", r"adrs",
        r"decision", r"decisions", r"meeting", r"meetings",
        r"agenda", r"agendas", r"minutes", r"summary", r"summaries",
        r"report", r"reports", r"analysis", r"analyses",
        r"research", r"investigation", r"exploration", r"experiment",
        r"experiments", r"proof-of-concept", r"poc", r"prototype",
        r"prototypes", r"mockup", r"mockups", r"wireframe", r"wireframes",
        r"design", r"designs", r"ui", r"ux",
        r"api", r"apis", r"rest", r"graphql", r"grpc", r"soap",
        r"websocket", r"websockets", r"sse", r"webhook", r"webhooks",
        r"microservice", r"microservices", r"service", r"services",
        r"gateway", r"gateways", r"proxy", r"proxies", r"load-balancer",
        r"load-balancers", r"reverse-proxy", r"reverse-proxies",
        r"cdn", r"edge", r"edges", r"cache", r"caches",
        r"database", r"databases", r"db", r"dbs", r"sql", r"nosql",
        r"mongodb", r"mysql", r"postgresql", r"postgres", r"sqlite",
        r"redis", r"memcached", r"elasticsearch", r"solr",
        r"kafka", r"rabbitmq", r"activemq", r"sqs", r"sns",
        r"s3", r"gcs", r"azure-blob", r"azure-blob-storage",
        r"cloud", r"aws", r"azure", r"gcp", r"google-cloud",
        r"docker", r"kubernetes", r"k8s", r"helm", r"terraform",
        r"ansible", r"chef", r"puppet", r"salt", r"vagrant",
        r"jenkins", r"gitlab-ci", r"github-actions", r"azure-devops",
        r"circleci", r"travis-ci", r"appveyor", r"bamboo",
        r"teamcity", r"buildkite", r"drone", r"concourse",
        r"monitoring", r"observability", r"logging", r"tracing",
        r"metrics", r"alerts", r"dashboards", r"grafana",
        r"prometheus", r"influxdb", r"datadog", r"newrelic",
        r"sentry", r"rollbar", r"bugsnag", r"honeybadger",
        r"security", r"vulnerability", r"scanning", r"auditing",
        r"compliance", r"governance", r"policies", r"standards",
        r"documentation", r"docs", r"wiki", r"guides", r"tutorials",
        r"examples", r"samples", r"demos", r"playground",
        r"sandbox", r"experimental", r"beta", r"alpha",
        r"deprecated", r"legacy", r"old", r"archive", r"backup"
    ]
    
    import re
    for pattern in dependency_patterns:
        if re.search(pattern, dirname_lower):
            return True
    
    return False

def run_git_lsfiles(root: str) -> Optional[List[str]]:
    """
    Lista arquivos usando git ls-files.
    IMPORTANTE: Se os arquivos não estiverem rastreados pelo git, retorna None
    para que o fallback use o filesystem diretamente.
    """
    if not is_git_repo(root) or not have_cmd("git"):
        return None
    try:
        # Usar --cached (rastreados) e --others (não rastreados, mas não ignorados)
        # --exclude-standard respeita .gitignore
        out = subprocess.check_output(
            ["git", "-C", root, "ls-files", "--cached", "--others", "--exclude-standard"],
            stderr=subprocess.DEVNULL,
        )
        files = out.decode("utf-8", errors="replace").splitlines()
        files = [f.replace("\\", "/") for f in files if f and not f.endswith("/")]
        # Filtro extra: nunca incluir diretórios `.git*`
        files = [f for f in files if not path_has_git(f)]
        return files if files else None
    except Exception:
        return None

def try_pathspec_iter(root: str) -> Optional[Iterable[str]]:
    try:
        import pathspec  # type: ignore
    except Exception:
        return None
    gi_path = os.path.join(root, ".gitignore")
    if not os.path.exists(gi_path):
        return None
    with open(gi_path, "r", encoding="utf-8", errors="replace") as f:
        lines = f.readlines()
    spec = pathspec.PathSpec.from_lines("gitwildmatch", lines)

    def walker() -> Iterable[str]:
        for dirpath, dirnames, filenames in os.walk(root):
            rel_dir = os.path.relpath(dirpath, root)
            rel_dir = "" if rel_dir == "." else rel_dir
            # Poda SOMENTE .git* (o resto deve ser controlado pelo .gitignore)
            dirnames[:] = [d for d in dirnames if not d.lower().startswith(".git")]
            # arquivos
            for name in filenames:
                rel = os.path.join(rel_dir, name) if rel_dir else name
                rel = rel.replace("\\", "/")
                if path_has_git(rel):
                    continue
                if not spec.match_file(rel):
                    yield rel
    return walker()

def list_all_files_fallback(root: str) -> Iterable[str]:
    for dirpath, dirnames, filenames in os.walk(root):
        # Fallback sem .gitignore: podar SOMENTE `.git*`
        dirnames[:] = [d for d in dirnames if not d.lower().startswith(".git")]
        rel_dir = os.path.relpath(dirpath, root)
        rel_dir = "" if rel_dir == "." else rel_dir
        for name in filenames:
            rel = os.path.join(rel_dir, name) if rel_dir else name
            rel = rel.replace("\\", "/")
            if path_has_git(rel):
                continue
            yield rel

def choose_fence_for(content: str) -> str:
    if "```" not in content:
        return "```"
    if "````" not in content:
        return "````"
    return "`````"

def lang_for(path: str) -> str:
    _, ext = os.path.splitext(path.lower())
    return LANG_BY_EXT.get(ext, "")

def is_probably_binary(abs_path: str) -> bool:
    try:
        size = os.path.getsize(abs_path)
        if size > MAX_FILE_BYTES:
            return True
        with open(abs_path, "rb") as f:
            chunk = f.read(4096)
        if b"\x00" in chunk:
            return True
        textish = sum((c in (9, 10, 13)) or (32 <= c <= 126) or (160 <= c <= 255) for c in chunk)
        return (len(chunk) > 0 and (textish / len(chunk) < 0.85))
    except Exception:
        return True

def is_large_directory(dir_path: str, max_dir_size_mb: int = 100) -> bool:
    """
    Verifica se um diretório é muito grande (acima do limite especificado em MB).
    Útil para evitar processar diretórios de dependências muito grandes.
    """
    try:
        total_size = 0
        max_size_bytes = max_dir_size_mb * 1024 * 1024
        
        for dirpath, dirnames, filenames in os.walk(dir_path):
            # Para cada arquivo no diretório
            for filename in filenames:
                filepath = os.path.join(dirpath, filename)
                try:
                    total_size += os.path.getsize(filepath)
                    # Se já passou do limite, retorna True imediatamente
                    if total_size > max_size_bytes:
                        return True
                except (OSError, IOError):
                    continue
                    
        return total_size > max_size_bytes
    except Exception:
        return True  # Em caso de erro, considera como grande para ser seguro

def should_exclude(path: str, basename: str = None) -> bool:
    """
    Função centralizada para decidir se um arquivo deve ser excluído.
    CRÍTICO: Esta função é chamada ANTES de ler qualquer arquivo do disco.
    Normaliza path, aplica todos os padrões de exclusão.
    
    Retorna True se o arquivo deve ser excluído por segurança ou por ser pesado/inútil.
    """
    if basename is None:
        basename = os.path.basename(path)
    
    # Normalizar path (sempre usar /) - compatível Windows/Linux
    norm_path = path.replace("\\", "/")
    basename_lower = basename.lower()
    norm_path_lower = norm_path.lower()
    
    # 1. Verificar basename com regex (mais rápido, verifica primeiro)
    for pattern in EXCLUDE_BASENAME_REGEX:
        if re.search(pattern, basename, re.IGNORECASE):
            return True  # Excluído por nome (ex: .env, credentials.json)
    
    # 2. Verificar caminho com regex (ex: /secrets/, /.ssh/)
    for pattern in EXCLUDE_PATH_REGEX:
        if re.search(pattern, norm_path, re.IGNORECASE):
            return True  # Excluído por estar em diretório sensível
    
    # 3. Verificar extensão sensível (ex: .pem, .key, .sqlite)
    ext = os.path.splitext(path.lower())[1]
    if ext in SENSITIVE_EXTS:
        return True  # Excluído por extensão (chaves, certificados, DBs locais)
    
    # 4. Verificar diretórios sensíveis (qualquer segmento do path)
    # Nota: NÃO excluímos "dependências" por heurística aqui — isso deve ficar a cargo do .gitignore.
    path_segments = norm_path.lower().split("/")
    for segment in path_segments:
        # Diretórios sensíveis por padrão (secrets, keys, etc.)
        if any(sensitive_dir in segment for sensitive_dir in ["secrets", "secret", "keys", "certs", "private", "credentials", ".ssh", ".aws"]):
            return True
    
    return False  # Arquivo permitido (ainda pode ser filtrado por should_include)

def redact_secrets(content: str) -> Tuple[str, List[Dict[str, str]]]:
    """
    Detecta e redige segredos no conteúdo.
    Retorna (conteúdo redigido, lista de padrões detectados com tipos).
    """
    if not content:
        return content, []
    
    redactions = []
    redacted_content = content
    
    # Padrões de detecção e redaction (ordem importa - mais específicos primeiro)
    patterns = [
        # Chaves privadas PEM (deve vir primeiro para não ser confundido)
        (r'(-----BEGIN\s+.*PRIVATE\s+KEY-----[\s\S]*?-----END\s+.*PRIVATE\s+KEY-----)', "PRIVATE_KEY", False),
        # Service account JSON (private_key field)
        (r'("private_key"\s*:\s*")([^"]+)(")', "SERVICE_ACCOUNT_KEY", True),
        # OpenAI keys
        (r'(sk-[a-zA-Z0-9]{20,})', "OPENAI_KEY", False),
        # Google API keys
        (r'(AIza[0-9A-Za-z_-]{35})', "GOOGLE_API_KEY", False),
        # JWT tokens (eyJ...)
        (r'(eyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,})', "JWT_TOKEN", False),
        # AWS keys
        (r'(AKIA[0-9A-Z]{16})', "AWS_ACCESS_KEY", False),
        (r'(aws_secret_access_key\s*[:=]\s*["\']?)([a-zA-Z0-9/+=]{20,})', "AWS_SECRET_KEY", True),
        # Azure
        (r'(azure_client_secret\s*[:=]\s*["\']?)([a-zA-Z0-9_-]{20,})', "AZURE_CLIENT_SECRET", True),
        # Senhas e tokens (case-insensitive)
        (r'(password\s*[:=]\s*["\']?)([^\s"\'\n]{8,})', "PASSWORD", True),
        (r'(mysql_password\s*[:=]\s*["\']?)([^\s"\'\n]{8,})', "MYSQL_PASSWORD", True),
        (r'(secret\s*[:=]\s*["\']?)([^\s"\'\n]{8,})', "SECRET", True),
        (r'(token\s*[:=]\s*["\']?)([^\s"\'\n]{8,})', "TOKEN", True),
        (r'(api_key\s*[:=]\s*["\']?)([^\s"\'\n]{8,})', "API_KEY", True),
        # Bearer tokens
        (r'(Bearer\s+)([a-zA-Z0-9_-]{40,})', "BEARER_TOKEN", True),
    ]
    
    # Processar padrões em ordem reversa para manter índices corretos
    for pattern, pattern_type, has_groups in reversed(patterns):
        matches = list(re.finditer(pattern, redacted_content, re.IGNORECASE | re.MULTILINE | re.DOTALL))
        if matches:
            redactions.append({
                "pattern": pattern_type,
                "count": len(matches)
            })
            # Redigir cada match (processar de trás para frente para manter índices)
            for match in reversed(matches):
                start, end = match.span()
                if has_groups and len(match.groups()) >= 2:
                    # Padrão com grupos (ex: "password=" + valor)
                    # Manter o prefixo, redigir o valor
                    prefix = match.group(1)
                    redacted_value = f"REDACTED:{pattern_type}"
                    replacement = prefix + redacted_value
                    if len(match.groups()) >= 3:
                        # Tem sufixo também (ex: JSON com aspas)
                        suffix = match.group(3)
                        replacement = prefix + redacted_value + suffix
                else:
                    # Padrão simples, substituir tudo
                    replacement = f"REDACTED:{pattern_type}"
                
                redacted_content = (
                    redacted_content[:start] +
                    replacement +
                    redacted_content[end:]
                )
    
    return redacted_content, redactions

def should_include(path: str, root: str, exclude_basenames: Set[str]) -> bool:
    """
    Decide se um arquivo deve ser incluído no MD.
    CHAMAR should_exclude() PRIMEIRO antes de qualquer outra verificação.
    """
    # 1. PRIMEIRO: Verificar exclusão por segurança (decisão centralizada)
    if should_exclude(path):
        return False
    
    # 2. Nunca incluir .git (diretórios ou arquivos com ".git" no nome)
    if path_has_git(path):
        return False
    
    # 3. Não incluir o próprio arquivo de saída (ou variantes)
    if os.path.basename(path).lower() in exclude_basenames:
        return False
    
    # 4. Verificar extensões binárias
    ext = os.path.splitext(path.lower())[1]
    if ext in EXCLUDE_EXTS:
        return False
    
    # 5. Verificar se é binário (última verificação, mais custosa)
    abs_path = os.path.join(root, path)
    return not is_probably_binary(abs_path)

def read_text(abs_path: str) -> Optional[str]:
    try:
        with open(abs_path, "r", encoding="utf-8", errors="replace") as f:
            return f.read()
    except Exception:
        return None

def get_git_commit(root: str) -> Optional[str]:
    if not is_git_repo(root) or not have_cmd("git"):
        return None
    try:
        out = subprocess.check_output(
            ["git", "-C", root, "rev-parse", "--short", "HEAD"],
            stderr=subprocess.DEVNULL,
        )
        return out.decode().strip()
    except Exception:
        return None

def bytes_len(s: str) -> int:
    return len(s.encode("utf-8", errors="replace"))

# ---------- Project Intelligence Functions ----------
def render_tree(root: str, max_depth: int = 4, max_lines: int = 400) -> str:
    """Gera uma representação em árvore do diretório, limitada por profundidade e linhas."""
    lines = []
    root_abs = os.path.abspath(root)
    
    def walk_tree(current_dir: str, prefix: str = "", depth: int = 0, is_last: bool = True):
        if depth > max_depth or len(lines) >= max_lines:
            return
        
        try:
            items = []
            for item in sorted(os.listdir(current_dir)):
                item_path = os.path.join(current_dir, item)
                # Pular .git e dependências
                if item.startswith(".git") or is_dependency_dir(item, item_path):
                    continue
                if os.path.isdir(item_path) or os.path.isfile(item_path):
                    items.append((item, os.path.isdir(item_path)))
            
            for idx, (item, is_dir) in enumerate(items):
                if len(lines) >= max_lines:
                    lines.append(f"{prefix}... (truncated)")
                    return
                
                is_last_item = (idx == len(items) - 1)
                connector = "└── " if is_last_item else "├── "
                lines.append(f"{prefix}{connector}{item}{'/' if is_dir else ''}")
                
                if is_dir and depth < max_depth:
                    extension = "    " if is_last_item else "│   "
                    walk_tree(
                        os.path.join(current_dir, item),
                        prefix + extension,
                        depth + 1,
                        is_last_item
                    )
        except (OSError, PermissionError):
            pass
    
    walk_tree(root_abs)
    if len(lines) >= max_lines:
        lines.append("... (tree truncated at max_lines)")
    return "\n".join(lines) if lines else "(empty)"

def detect_stack(root: str, file_list: List[str]) -> Dict[str, Any]:
    """Detecta stack tecnológica baseado em arquivos presentes."""
    stack = {"languages": [], "frameworks": [], "infra": [], "signals": []}
    file_set = {f.lower() for f in file_list}
    basenames = {os.path.basename(f).lower() for f in file_list}
    
    # Python
    if any("pyproject.toml" in f or "requirements.txt" in f or "poetry.lock" in f or "setup.py" in f for f in file_list):
        stack["languages"].append("Python")
        if any("fastapi" in f or "fastapi" in str(file_set) for f in file_list):
            stack["frameworks"].append("FastAPI")
        if any("flask" in f or "flask" in str(file_set) for f in file_list):
            stack["frameworks"].append("Flask")
        if any("django" in f for f in file_list):
            stack["frameworks"].append("Django")
    
    # Node.js
    if "package.json" in basenames:
        stack["languages"].append("JavaScript/TypeScript")
        if any("vite.config" in f for f in file_list):
            stack["frameworks"].append("Vite")
        if any("next.config" in f for f in file_list):
            stack["frameworks"].append("Next.js")
        if any("express" in f for f in file_list):
            stack["frameworks"].append("Express")
    
    # Go
    if "go.mod" in basenames:
        stack["languages"].append("Go")
        if any("fiber" in f or "gin" in f for f in file_list):
            stack["frameworks"].append("Go Web Framework")
    
    # Infra
    if "dockerfile" in basenames or any("dockerfile" in f.lower() for f in file_list):
        stack["infra"].append("Docker")
    if "docker-compose.yml" in basenames or "docker-compose.yaml" in basenames:
        stack["infra"].append("Docker Compose")
    if any(".github/workflows" in f for f in file_list):
        stack["infra"].append("GitHub Actions")
    if any("azure-pipelines" in f or ".azure" in f for f in file_list):
        stack["infra"].append("Azure Pipelines")
    if any("terraform" in f for f in file_list):
        stack["infra"].append("Terraform")
    if any("helm" in f for f in file_list):
        stack["infra"].append("Helm")
    
    return stack

def extract_commands(root: str) -> Dict[str, List[str]]:
    """Extrai comandos úteis de Makefile, package.json, pyproject.toml, docker-compose."""
    commands = {"make": [], "npm": [], "poetry": [], "docker_compose": [], "workflows": []}
    
    # Makefile
    makefile_path = os.path.join(root, "Makefile")
    if os.path.exists(makefile_path):
        try:
            with open(makefile_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                # Regex simples para targets
                targets = re.findall(r'^([a-zA-Z0-9_-]+):', content, re.MULTILINE)
                commands["make"] = [t for t in targets if not t.startswith(".")][:20]
        except Exception:
            pass
    
    # package.json
    pkg_path = os.path.join(root, "package.json")
    if os.path.exists(pkg_path):
        try:
            with open(pkg_path, "r", encoding="utf-8", errors="replace") as f:
                data = json.load(f)
                if "scripts" in data:
                    commands["npm"] = list(data["scripts"].keys())[:20]
        except Exception:
            pass
    
    # pyproject.toml (poetry/uv)
    pyproject_path = os.path.join(root, "pyproject.toml")
    if os.path.exists(pyproject_path):
        try:
            with open(pyproject_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                if "[tool.poetry.scripts]" in content or "[tool.poetry]" in content:
                    commands["poetry"].append("poetry install")
                    commands["poetry"].append("poetry run pytest")
        except Exception:
            pass
    
    # docker-compose.yml
    compose_paths = [
        os.path.join(root, "docker-compose.yml"),
        os.path.join(root, "docker-compose.yaml"),
    ]
    for compose_path in compose_paths:
        if os.path.exists(compose_path):
            try:
                with open(compose_path, "r", encoding="utf-8", errors="replace") as f:
                    content = f.read()
                    # Regex simples para serviços
                    services = re.findall(r'^\s+([a-zA-Z0-9_-]+):\s*$', content, re.MULTILINE)
                    # Filtrar apenas os que estão dentro de "services:"
                    in_services = False
                    found_services = []
                    for line in content.splitlines():
                        if "services:" in line.lower():
                            in_services = True
                            continue
                        if in_services and line.strip() and not line.startswith(" ") and ":" in line:
                            break
                        if in_services and re.match(r'^\s+[a-zA-Z0-9_-]+:\s*$', line):
                            found_services.append(line.strip().rstrip(":"))
                    commands["docker_compose"] = found_services[:20]
            except Exception:
                pass
    
    # GitHub Actions workflows
    workflows_dir = os.path.join(root, ".github", "workflows")
    if os.path.isdir(workflows_dir):
        try:
            for fname in os.listdir(workflows_dir):
                if fname.endswith((".yml", ".yaml")):
                    commands["workflows"].append(fname)
        except Exception:
            pass
    
    return commands

def extract_entrypoints(file_list: List[str]) -> List[str]:
    """Detecta entrypoints prováveis do projeto."""
    entrypoints = []
    common_names = {
        "app.py", "main.py", "server.py", "index.py", "__main__.py",
        "main.go", "main.ts", "main.js", "index.ts", "index.js",
        "src/main.ts", "src/index.ts", "src/main.js", "src/index.js",
    }
    for f in file_list:
        basename = os.path.basename(f).lower()
        if basename in common_names or f.lower() in [n.lower() for n in common_names]:
            entrypoints.append(f)
    return sorted(entrypoints)[:10]

def extract_env_vars(root: str, selected_files: List[str]) -> Dict[str, Dict[str, Any]]:
    """
    Extrai nomes de variáveis de ambiente (SEM VALORES) do código.
    CRÍTICO: NUNCA inclui valores de env vars, apenas nomes e onde aparecem.
    """
    env_vars: Dict[str, Dict[str, Any]] = {}
    
    # Padrões de regex para diferentes linguagens (captura apenas o nome)
    patterns = [
        (r'os\.getenv\(["\']([^"\']+)["\']', "Python"),
        (r'os\.environ\[["\']([^"\']+)["\']', "Python"),
        (r'process\.env\.([A-Z_][A-Z0-9_]*)', "JavaScript"),
        (r'process\.env\[["\']([^"\']+)["\']', "JavaScript"),
        (r'\$\{([A-Z_][A-Z0-9_]*)\}', "YAML/Shell"),
        (r'\$([A-Z_][A-Z0-9_]*)', "Shell"),
    ]
    
    # Primeiro, tentar .env.example (apenas nomes, sem valores)
    env_example_paths = [
        os.path.join(root, ".env.example"),
        os.path.join(root, ".env.sample"),
        os.path.join(root, "env.example"),
    ]
    for env_path in env_example_paths:
        # Verificar se não foi excluído
        rel_env_path = os.path.relpath(env_path, root).replace("\\", "/")
        if should_exclude(rel_env_path):
            continue
        if os.path.exists(env_path):
            try:
                with open(env_path, "r", encoding="utf-8", errors="replace") as f:
                    for line in f:
                        line = line.strip()
                        if line and not line.startswith("#") and "=" in line:
                            # Extrair apenas o nome (antes do =), ignorar valor
                            var_name = line.split("=")[0].strip()
                            if var_name and var_name.replace("_", "").isalnum():
                                if var_name not in env_vars:
                                    env_vars[var_name] = {"count": 0, "samples": []}
                                env_vars[var_name]["count"] += 1
                                if len(env_vars[var_name]["samples"]) < 3:
                                    env_vars[var_name]["samples"].append(rel_env_path)
            except Exception:
                pass
    
    # Varredura em arquivos de código (apenas nomes, nunca valores)
    for rel_path in selected_files[:500]:  # Limitar para performance
        # Double-check: não processar arquivos excluídos
        if should_exclude(rel_path):
            continue
        abs_path = os.path.join(root, rel_path)
        if not os.path.isfile(abs_path):
            continue
        ext = os.path.splitext(rel_path)[1].lower()
        if ext not in {".py", ".js", ".ts", ".jsx", ".tsx", ".sh", ".yaml", ".yml"}:
            continue
        
        try:
            with open(abs_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                for pattern, lang in patterns:
                    matches = re.finditer(pattern, content)
                    for match in matches:
                        var_name = match.group(1)  # Apenas o nome, nunca o valor
                        if var_name not in env_vars:
                            env_vars[var_name] = {"count": 0, "samples": []}
                        env_vars[var_name]["count"] += 1
                        if len(env_vars[var_name]["samples"]) < 3 and rel_path not in env_vars[var_name]["samples"]:
                            env_vars[var_name]["samples"].append(rel_path)
        except Exception:
            continue
    
    return env_vars

def extract_api_routes(selected_files: List[str], root: str) -> List[Dict[str, str]]:
    """Extrai rotas de API usando heurísticas para FastAPI, Express, etc."""
    routes = []
    
    # Padrões FastAPI
    fastapi_pattern = re.compile(r'@(app|router)\.(get|post|put|delete|patch|head|options)\s*\(\s*["\']([^"\']+)["\']')
    
    # Padrões Express
    express_pattern = re.compile(r'(app|router)\.(get|post|put|delete|patch|head|options)\s*\(\s*["\']([^"\']+)["\']')
    
    for rel_path in selected_files:
        if not rel_path.endswith((".py", ".js", ".ts")):
            continue
        abs_path = os.path.join(root, rel_path)
        try:
            with open(abs_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                # FastAPI
                for match in fastapi_pattern.finditer(content):
                    method = match.group(2).upper()
                    path = match.group(3)
                    routes.append({"method": method, "path": path, "file": rel_path})
                # Express
                for match in express_pattern.finditer(content):
                    method = match.group(2).upper()
                    path = match.group(3)
                    routes.append({"method": method, "path": path, "file": rel_path})
        except Exception:
            continue
    
    return routes[:100]  # Limitar

def scan_signals(selected_files: List[str], root: str) -> Dict[str, List[str]]:
    """Escaneia sinais de RAG/LLM e observabilidade."""
    signals = {"rag_llm": [], "observability": []}
    
    rag_keywords = [
        "langchain", "llama_index", "openai", "anthropic", "qdrant", "chroma",
        "pinecone", "embeddings", "vector", "retriever", "llm", "rag"
    ]
    obs_keywords = [
        "opentelemetry", "prometheus", "sentry", "loguru", "structlog",
        "datadog", "newrelic", "grafana", "jaeger", "zipkin"
    ]
    
    for rel_path in selected_files:
        abs_path = os.path.join(root, rel_path)
        if not os.path.isfile(abs_path):
            continue
        try:
            with open(abs_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read().lower()
                for keyword in rag_keywords:
                    if keyword in content and rel_path not in signals["rag_llm"]:
                        signals["rag_llm"].append(rel_path)
                        break
                for keyword in obs_keywords:
                    if keyword in content and rel_path not in signals["observability"]:
                        signals["observability"].append(rel_path)
                        break
        except Exception:
            continue
    
    return signals

def find_env_files_in_filesystem(root: str) -> List[Dict[str, Any]]:
    """
    Procura arquivos .env* no filesystem real (não só na lista de arquivos).
    Retorna lista com path e se foi excluded.
    """
    env_files = []
    
    def walk_for_env(current_dir: str, depth: int = 0, max_depth: int = 10):
        if depth > max_depth:
            return
        try:
            for item in os.listdir(current_dir):
                item_path = os.path.join(current_dir, item)
                # Pular .git e dependências
                if item.startswith(".git") or is_dependency_dir(item, item_path):
                    continue
                
                if os.path.isfile(item_path):
                    basename_lower = item.lower()
                    if basename_lower.startswith(".env"):
                        rel_path = os.path.relpath(item_path, root)
                        rel_path_norm = rel_path.replace("\\", "/")
                        is_excluded = should_exclude(rel_path_norm, item)
                        env_files.append({
                            "path": rel_path_norm,
                            "excluded": is_excluded
                        })
                elif os.path.isdir(item_path):
                    walk_for_env(item_path, depth + 1, max_depth)
        except (OSError, PermissionError):
            pass
    
    walk_for_env(root)
    return env_files

def scan_smells(root: str, selected_files: List[str], all_files: List[str]) -> Dict[str, Any]:
    """Escaneia smells e gaps: TODOs, testes, .env real (filesystem + lista), maiores arquivos."""
    smells = {
        "todos": [],
        "tests_present": False,
        "tests_count": 0,
        "env_files": [],  # Lista de dicts com path e excluded
        "largest_files": []
    }
    
    todo_pattern = re.compile(r'\b(TODO|FIXME|HACK|XXX|NOTE|BUG)\b', re.IGNORECASE)
    todo_counts: Dict[str, int] = {}
    
    file_sizes: List[Tuple[str, int]] = []
    
    for rel_path in selected_files:
        abs_path = os.path.join(root, rel_path)
        if not os.path.isfile(abs_path):
            continue
        
        # Tamanho do arquivo
        try:
            size = os.path.getsize(abs_path)
            file_sizes.append((rel_path, size))
        except Exception:
            pass
        
        # TODOs
        try:
            with open(abs_path, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
                matches = len(todo_pattern.findall(content))
                if matches > 0:
                    todo_counts[rel_path] = matches
        except Exception:
            pass
        
        # Testes
        if "test" in rel_path.lower() or "spec" in rel_path.lower():
            smells["tests_present"] = True
            if rel_path.endswith((".py", ".js", ".ts", ".go")):
                smells["tests_count"] += 1
    
    # Top 10 TODOs
    smells["todos"] = sorted(todo_counts.items(), key=lambda x: x[1], reverse=True)[:10]
    
    # .env: verificar filesystem real E lista de arquivos
    env_from_filesystem = find_env_files_in_filesystem(root)
    # Também verificar na lista de arquivos (pode ter .env que não está no filesystem ainda)
    for rel_path in all_files:
        basename = os.path.basename(rel_path).lower()
        if basename.startswith(".env"):
            rel_path_norm = rel_path.replace("\\", "/")
            is_excluded = should_exclude(rel_path_norm, basename)
            # Evitar duplicatas
            if not any(e["path"] == rel_path_norm for e in env_from_filesystem):
                env_from_filesystem.append({
                    "path": rel_path_norm,
                    "excluded": is_excluded
                })
    
    smells["env_files"] = env_from_filesystem
    
    # Top 10 maiores arquivos
    smells["largest_files"] = sorted(file_sizes, key=lambda x: x[1], reverse=True)[:10]
    
    return smells

def smart_extract(rel_path: str, content: str, lang: str, smart_truncate_bytes: int) -> str:
    """Extrai conteúdo inteligente de arquivos grandes: header, assinaturas, footer."""
    content_bytes = len(content.encode("utf-8"))
    if content_bytes <= smart_truncate_bytes:
        return content
    
    lines = content.splitlines()
    header_lines = 120
    footer_lines = 80
    
    # Header
    header = "\n".join(lines[:header_lines])
    
    # Assinaturas (def/class/import em Python; export/function/class em TS/JS; func/type em Go)
    signatures = []
    signature_patterns = {
        "python": re.compile(r'^(def |class |import |from )', re.MULTILINE),
        "ts": re.compile(r'^(export |function |class |import )', re.MULTILINE),
        "js": re.compile(r'^(export |function |class |import |const |let )', re.MULTILINE),
        "go": re.compile(r'^(func |type |import )', re.MULTILINE),
    }
    
    pattern = signature_patterns.get(lang, None)
    if pattern:
        for match in pattern.finditer(content):
            line_num = content[:match.start()].count("\n")
            line = lines[line_num] if line_num < len(lines) else ""
            if line.strip() and line not in signatures:
                signatures.append(line)
                if len(signatures) >= 50:
                    break
    
    # Footer
    footer = "\n".join(lines[-footer_lines:]) if len(lines) > footer_lines else ""
    
    # Montar resultado
    parts = [header]
    if signatures:
        parts.append("\n// ... signatures ...\n")
        parts.append("\n".join(signatures[:50]))
    parts.append("\n\n[TRUNCATED: smart mode - showing header, signatures, and footer]\n\n")
    if footer:
        parts.append(footer)
    
    return "\n".join(parts)

# ---------- Core ----------
def build_file_list(root: str) -> List[str]:
    """
    Constrói lista de arquivos do repositório.
    Prioridade:
    1. git ls-files --cached --others --exclude-standard (respeita .gitignore)
    2. pathspec lendo .gitignore (se disponível)
    3. filesystem direto (fallback) — sem git/pathspec não dá para respeitar .gitignore 100%

    Objetivo: considerar "todos os arquivos" do projeto, exceto os ignorados pelo `.gitignore`,
    mantendo apenas uma camada mínima de segurança via should_exclude().
    """
    files = run_git_lsfiles(root)
    if files is not None and len(files) > 0:
        return sorted(files)

    # Tentar pathspec (respeita .gitignore)
    ps = try_pathspec_iter(root)
    if ps is not None:
        ps_list = list(ps)
        if len(ps_list) > 0:
            return sorted(ps_list)
    
    # Fallback: filesystem direto (inclui tudo, exceto `.git*`)
    return sorted(list(list_all_files_fallback(root)))

def prioritize_files_for_smart(files: List[str]) -> List[str]:
    """Prioriza arquivos para modo SMART: docs primeiro, depois código importante, depois resto."""
    priority_patterns = [
        # Prioridade 1: Documentação
        (lambda f: any(f.lower().startswith(p) for p in ["readme", "docs/", "domain.md", "cursor_rules.md"]), 1),
        # Prioridade 2: Configuração do projeto
        (lambda f: any(f.lower().endswith(p) for p in ["pyproject.toml", "requirements.txt", "package.json", "docker-compose.yml", "docker-compose.yaml", "makefile"]), 2),
        # Prioridade 3: Entrypoints e código principal
        (lambda f: any(f.lower().endswith(p) for p in ["main.py", "app.py", "server.py", "index.py", "main.go", "main.ts", "main.js", "index.ts", "index.js"]), 3),
        (lambda f: "/api/" in f.lower() or "/routes/" in f.lower() or "/routers/" in f.lower() or "/controllers/" in f.lower(), 3),
        # Prioridade 4: Models, schemas, entities
        (lambda f: "/models/" in f.lower() or "/entities/" in f.lower() or "/schemas/" in f.lower(), 4),
        (lambda f: "/migrations/" in f.lower(), 4),
        # Prioridade 5: Resto
        (lambda f: True, 5),
    ]
    
    prioritized = {1: [], 2: [], 3: [], 4: [], 5: []}
    seen = set()
    
    for f in files:
        if f in seen:
            continue
        for pattern_func, priority in priority_patterns:
            if pattern_func(f):
                prioritized[priority].append(f)
                seen.add(f)
                break
    
    result = []
    for priority in sorted(prioritized.keys()):
        result.extend(sorted(prioritized[priority]))
    
    return result

def safety_gate_scan(md_content: str) -> Tuple[str, List[Dict[str, Any]]]:
    """
    Safety Gate: escaneia o MD final gerado procurando por segredos.
    Se encontrar, redige e retorna (conteúdo sanitizado, lista de findings).
    
    CRÍTICO: Usa regex específicos para formatos reais de segredos, evitando falsos positivos
    em comentários de documentação.
    """
    findings = []
    safe_content = md_content
    lines = safe_content.splitlines()
    
    def is_comment_line(line_num: int) -> bool:
        """Verifica se a linha é um comentário (heurística simples)."""
        if line_num < 1 or line_num > len(lines):
            return False
        line = lines[line_num - 1].strip()
        # Ignorar linhas que são claramente comentários
        return line.startswith("#") or line.startswith("//") or line.startswith("--")
    
    def is_placeholder_value(value: str) -> bool:
        """Verifica se o valor é um placeholder/documentação, não um segredo real."""
        value_lower = value.lower()
        # Ignorar valores que são claramente placeholders
        placeholder_patterns = [
            "redacted:",
            "example",
            "placeholder",
            "xxx",
            "your_",
            "secret123",  # Exemplo comum em docs
            "senha123",   # Exemplo comum em docs
            "abc123",     # Exemplo comum
            "test",
            "demo",
        ]
        return any(ph in value_lower for ph in placeholder_patterns)
    
    # Padrões críticos que NUNCA devem aparecer no MD
    # ORDEM: mais específicos primeiro, genéricos por último
    critical_patterns = [
        # 1. Chaves privadas PEM (formato específico)
        (r'(-----BEGIN\s+.*PRIVATE\s+KEY-----[\s\S]*?-----END\s+.*PRIVATE\s+KEY-----)', "PRIVATE_KEY", False),
        # 2. OpenAI keys (formato específico: sk-...)
        (r'(sk-[a-zA-Z0-9]{20,})', "OPENAI_KEY", False),
        # 3. Google API keys (formato específico: AIza...)
        (r'(AIza[0-9A-Za-z_-]{35})', "GOOGLE_API_KEY", False),
        # 4. JWT tokens (formato específico: eyJ...)
        (r'(eyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,})', "JWT_TOKEN", False),
        # 5. AWS keys (formato específico: AKIA...)
        (r'(AKIA[0-9A-Z]{16})', "AWS_ACCESS_KEY", False),
        # 6. Service account JSON (formato específico)
        (r'("type"\s*:\s*"service_account"[\s\S]{0,500}?"private_key"\s*:\s*")([^"]+)(")', "SERVICE_ACCOUNT_KEY", True),
        # 7. Senhas e secrets genéricos (apenas se não for comentário)
        # PASSWORD: manter genérico mas verificar contexto
        (r'(password\s*[:=]\s*["\']?)([^\s"\'\n]{12,})', "PASSWORD", True),
        (r'(mysql_password\s*[:=]\s*["\']?)([^\s"\'\n]{12,})', "MYSQL_PASSWORD", True),
        (r'(secret\s*[:=]\s*["\']?)([^\s"\'\n]{12,})', "SECRET", True),
        # 8. TOKEN: apenas se for formato JWT ou Bearer (já coberto acima, mas manter genérico para outros casos)
        (r'(token\s*[:=]\s*["\']?)([a-zA-Z0-9_-]{40,})', "TOKEN", True),
        # 9. API_KEY: apenas se for formato específico (já coberto acima, mas manter para outros formatos)
        (r'(api_key\s*[:=]\s*["\']?)([a-zA-Z0-9_-]{32,})', "API_KEY", True),
    ]
    
    for pattern, pattern_type, check_comment in critical_patterns:
        matches = list(re.finditer(pattern, safe_content, re.IGNORECASE | re.MULTILINE | re.DOTALL))
        if matches:
            for match in matches:
                start, end = match.span()
                line_num = safe_content[:start].count("\n") + 1
                
                # Ignorar se for comentário (apenas para padrões genéricos)
                if check_comment and is_comment_line(line_num):
                    continue
                
                # Ignorar se o valor for um placeholder/documentação
                if check_comment and len(match.groups()) >= 2:
                    value = match.group(2) if len(match.groups()) >= 2 else ""
                    if is_placeholder_value(value):
                        continue
                
                # Redigir
                if len(match.groups()) >= 2:
                    prefix = match.group(1)
                    replacement = prefix + f"REDACTED:{pattern_type}"
                    if len(match.groups()) >= 3:
                        suffix = match.group(3)
                        replacement = prefix + f"REDACTED:{pattern_type}" + suffix
                else:
                    replacement = f"REDACTED:{pattern_type}"
                
                safe_content = safe_content[:start] + replacement + safe_content[end:]
                
                findings.append({
                    "pattern": pattern_type,
                    "line": line_num,
                    "action": "redacted"
                })
    
    return safe_content, findings

def concat_repo(root: str, out_path: str, exclude_basenames: Set[str]) -> Tuple[int, int, bool]:
    """
    Concatena arquivos do repositório em um MD.
    Retorna (included_count, considered_count, safety_gate_failed)
    """
    files = build_file_list(root)
    
    # PRIMEIRO: Filtrar usando should_exclude() antes de qualquer processamento
    selected = []
    excluded_by_security = []
    for p in files:
        if should_exclude(p):
            excluded_by_security.append(p)
        elif should_include(p, root, exclude_basenames):
            selected.append(p)
    
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    commit = get_git_commit(root)
    mode = CONCAT_MODE if CONCAT_MODE in ("full", "smart") else "smart"
    
    # Estatísticas de segurança
    security_stats = {
        "excluded_by_filename": len(excluded_by_security),
        "excluded_files": excluded_by_security[:20],  # Limitar para não poluir
        "redacted_files": [],
        "redaction_patterns": {}
    }
    
    # Priorizar arquivos no modo SMART
    if mode == "smart":
        selected = prioritize_files_for_smart(selected)
    else:
        selected.sort()
    
    # Coletar metadados dos arquivos (APENAS se não foi excluded)
    file_metadata: List[Dict[str, Any]] = []
    for rel in selected:
        # Garantir que não foi excluded (double-check)
        if should_exclude(rel):
            continue
        abs_path = os.path.join(root, rel)
        try:
            stat = os.stat(abs_path)
            file_metadata.append({
                "path": rel,
                "size": stat.st_size,
                "mtime": datetime.fromtimestamp(stat.st_mtime).isoformat(),
                "lang": lang_for(rel)
            })
        except Exception:
            file_metadata.append({
                "path": rel,
                "size": 0,
                "mtime": "",
                "lang": lang_for(rel)
            })
    
    # Project Intelligence
    print("[INFO] Analisando projeto...")
    stack = detect_stack(root, selected)
    commands = extract_commands(root)
    entrypoints = extract_entrypoints(selected)
    env_vars = extract_env_vars(root, selected)
    api_routes = extract_api_routes(selected, root)
    signals = scan_signals(selected, root)
    smells = scan_smells(root, selected, files)  # Passar all_files para detectar .env em qualquer lugar
    tree = render_tree(root, max_depth=4, max_lines=400)
    
    # Contar linguagens por extensão
    lang_counts: Dict[str, int] = {}
    for rel in selected:
        lang = lang_for(rel)
        if lang:
            lang_counts[lang] = lang_counts.get(lang, 0) + 1
    
    # Nome do repo (basename do ROOT)
    repo_name = os.path.basename(os.path.abspath(root))
    
    # Construir seção de Project Intelligence
    intel_lines = [
        "# Project Intelligence",
        "",
        f"## 📦 Repository: {repo_name}",
        f"- **Root**: `{os.path.abspath(root)}`",
        f"- **Git commit**: {commit if commit else '(n/a)'}",
        f"- **Generated at**: {now}",
        "",
        "## 📁 Repository Tree",
        "",
        "```",
        tree,
        "```",
        "",
        "## 🛠️ Stack Detected",
        "",
    ]
    
    if stack["languages"]:
        intel_lines.append(f"- **Languages**: {', '.join(stack['languages'])}")
    if stack["frameworks"]:
        intel_lines.append(f"- **Frameworks**: {', '.join(stack['frameworks'])}")
    if stack["infra"]:
        intel_lines.append(f"- **Infrastructure**: {', '.join(stack['infra'])}")
    if not stack["languages"] and not stack["frameworks"]:
        intel_lines.append("- *(n/a)*")
    
    # Linguagens por extensão
    if lang_counts:
        intel_lines.append("")
        intel_lines.append("### Languages by file count")
        sorted_langs = sorted(lang_counts.items(), key=lambda x: x[1], reverse=True)
        for lang, count in sorted_langs[:10]:
            intel_lines.append(f"- **{lang}**: {count} files")
    
    # Security Report (será atualizado após processar arquivos)
    security_report_placeholder = "<<SECURITY_REPORT_PLACEHOLDER>>"
    intel_lines.extend([
        "",
        "## 🔐 Security Report",
        "",
        security_report_placeholder,
        "",
        "## 🚀 How to Run",
        "",
    ])
    
    if commands["make"]:
        intel_lines.append("### Makefile Targets")
        for target in commands["make"]:
            intel_lines.append(f"- `make {target}`")
        intel_lines.append("")
    
    if commands["npm"]:
        intel_lines.append("### npm scripts")
        for script in commands["npm"]:
            intel_lines.append(f"- `npm run {script}`")
        intel_lines.append("")
    
    if commands["poetry"]:
        intel_lines.append("### Poetry commands")
        for cmd in commands["poetry"]:
            intel_lines.append(f"- `{cmd}`")
        intel_lines.append("")
    
    if commands["docker_compose"]:
        intel_lines.append("### Docker Compose services")
        for svc in commands["docker_compose"]:
            intel_lines.append(f"- `docker-compose up {svc}`")
        intel_lines.append("")
    
    if commands["workflows"]:
        intel_lines.append("### GitHub Actions workflows")
        for wf in commands["workflows"]:
            intel_lines.append(f"- `.github/workflows/{wf}`")
        intel_lines.append("")
    
    if not any(commands.values()):
        intel_lines.append("*(n/a)*")
        intel_lines.append("")
    
    intel_lines.extend([
        "## 🎯 Entrypoints",
        "",
    ])
    if entrypoints:
        for ep in entrypoints:
            intel_lines.append(f"- `{ep}`")
    else:
        intel_lines.append("*(n/a)*")
    intel_lines.append("")
    
    intel_lines.extend([
        "## 🔐 Environment Variables (names only)",
        "",
    ])
    if env_vars:
        sorted_vars = sorted(env_vars.items(), key=lambda x: x[1]["count"], reverse=True)
        for var_name, info in sorted_vars[:50]:
            samples_str = ", ".join(info["samples"][:3])
            intel_lines.append(f"- `{var_name}` (appears {info['count']}x) - {samples_str}")
    else:
        intel_lines.append("*(n/a)*")
    intel_lines.append("")
    
    intel_lines.extend([
        "## 🌐 API Surface",
        "",
    ])
    if api_routes:
        for route in api_routes[:50]:
            intel_lines.append(f"- `{route['method']} {route['path']}` → `{route['file']}`")
    else:
        intel_lines.append("*(n/a)*")
    intel_lines.append("")
    
    intel_lines.extend([
        "## 🤖 RAG/LLM & Observability Signals",
        "",
    ])
    if signals["rag_llm"]:
        intel_lines.append("### RAG/LLM")
        for f in signals["rag_llm"][:10]:
            intel_lines.append(f"- `{f}`")
        intel_lines.append("")
    if signals["observability"]:
        intel_lines.append("### Observability")
        for f in signals["observability"][:10]:
            intel_lines.append(f"- `{f}`")
        intel_lines.append("")
    if not signals["rag_llm"] and not signals["observability"]:
        intel_lines.append("*(n/a)*")
        intel_lines.append("")
    
    intel_lines.extend([
        "## ⚠️ Smells / Gaps",
        "",
    ])
    if smells["todos"]:
        intel_lines.append("### Top TODOs/FIXMEs")
        for path, count in smells["todos"]:
            intel_lines.append(f"- `{path}`: {count} occurrences")
        intel_lines.append("")
    
    intel_lines.append(f"- **Tests present**: {'Yes' if smells['tests_present'] else 'No'} ({smells['tests_count']} test files)")
    if smells['env_files']:
        total_env = len(smells['env_files'])
        excluded_env = sum(1 for e in smells['env_files'] if e.get('excluded', False))
        included_env = total_env - excluded_env
        intel_lines.append(f"- **.env files found**: {total_env} file(s) detected")
        if excluded_env > 0:
            intel_lines.append(f"  - {excluded_env} excluded (security), {included_env} included")
        else:
            intel_lines.append(f"  - All {total_env} included")
        for env_info in smells['env_files'][:5]:
            status = "EXCLUDED" if env_info.get('excluded', False) else "included"
            intel_lines.append(f"  - `{env_info['path']}` ({status})")
    else:
        intel_lines.append("- **.env files found**: No .env files detected")
    intel_lines.append("")
    
    if smells["largest_files"]:
        intel_lines.append("### Largest files")
        for path, size in smells["largest_files"]:
            size_kb = size / 1024
            intel_lines.append(f"- `{path}`: {size_kb:.1f} KB")
        intel_lines.append("")
    
    intelligence_section = "\n".join(intel_lines)
    
    # TOC
    toc_lines = [
        "# Table of Contents",
        "",
        "- [Project Intelligence](#project-intelligence)",
        "- [File Index](#file-index)",
        "- [Files](#files)",
        "",
        "---",
        "",
    ]
    toc = "\n".join(toc_lines)
    
    # File Index
    file_index_lines = [
        "## File Index",
        "",
        "| # | Path | Size (bytes) | Modified | Language |",
        "|---|------|--------------|----------|----------|",
    ]
    for idx, meta in enumerate(file_metadata, 1):
        file_index_lines.append(
            f"| {idx} | `{meta['path']}` | {meta['size']} | {meta['mtime']} | {meta['lang'] or 'text'} |"
        )
    file_index = "\n".join(file_index_lines)
    
    # Header
    header = [
        "# Repository Snapshot (All text files)",
        f"- Root: `{os.path.abspath(root)}`",
        f"- Generated at: {now}",
        f"- Git commit: {commit if commit else '(n/a)'}",
        f"- Mode: {mode}",
        f"- Files included: {len(selected)} (max {MAX_FILE_BYTES} bytes per file, text-only heuristic)",
        "",
    ]
    
    # Files section
    files_header = [
        "---",
        "",
        "# Files",
        "",
    ]
    
    sections: List[str] = []
    total_bytes = bytes_len("\n".join(header + [toc, intelligence_section, file_index] + files_header))
    included = 0
    truncated_count = 0
    
    for idx, rel in enumerate(selected, start=1):
        if MAX_TOTAL_BYTES > 0 and total_bytes >= MAX_TOTAL_BYTES:
            sections.append(f"\n[OUTPUT TRUNCATED BY MAX_TOTAL_BYTES]\n")
            break
        
        # DOUBLE-CHECK: nunca processar arquivos excluded
        if should_exclude(rel):
            continue
        
        abs_path = os.path.join(root, rel)
        content = read_text(abs_path)
        if content is None:
            continue
        
        # Detectar e redigir segredos no conteúdo
        redacted_content, redactions = redact_secrets(content)
        is_redacted = len(redactions) > 0
        file_redactions = redactions  # Guardar para usar depois
        if is_redacted:
            content = redacted_content
            security_stats["redacted_files"].append(rel)
            # Contar padrões
            for redaction in redactions:
                pattern = redaction["pattern"]
                security_stats["redaction_patterns"][pattern] = (
                    security_stats["redaction_patterns"].get(pattern, 0) + redaction["count"]
                )
        
        # Aplicar modo smart se necessário (após redaction)
        lang = lang_for(rel)
        is_truncated = False
        if mode == "smart" and not is_redacted:
            original_size = len(content.encode("utf-8"))
            if original_size > SMART_TRUNCATE_BYTES:
                content = smart_extract(rel, content, lang, SMART_TRUNCATE_BYTES)
                is_truncated = True
                truncated_count += 1
        
        # Metadados do arquivo
        meta = next((m for m in file_metadata if m["path"] == rel), None)
        size_str = f"{meta['size']}" if meta else "?"
        mtime_str = meta["mtime"] if meta else "?"
        
        fence = choose_fence_for(content)
        comment_prefix = "#" if lang in {"python","bash","sh","yaml","toml","ini","sql","markdown","text","dotenv"} else "//"
        comment_lines = [
            f"{comment_prefix} FILE: {rel}",
            f"{comment_prefix} FULL: {os.path.abspath(abs_path)}",
            f"{comment_prefix} SIZE: {size_str} bytes",
            f"{comment_prefix} MTIME: {mtime_str}",
            f"{comment_prefix} NOTE: Concatenated snapshot for review",
        ]
        if is_redacted:
            patterns_found = ", ".join(set(r["pattern"] for r in file_redactions))
            comment_lines.append(f"{comment_prefix} SECURITY: Content redacted due to secret patterns: {patterns_found}")
        if is_truncated:
            comment_lines.append(f"{comment_prefix} TRUNCATED: smart mode applied")
        
        code_header = f"{fence}{lang}" if lang else fence
        code_block = (
            f"## [{idx}] {rel}\n\n"
            f"{code_header}\n"
            + "\n".join(comment_lines) + "\n"
            + content + "\n"
            f"{fence}\n"
        )
        
        block_bytes = bytes_len(code_block)
        if MAX_TOTAL_BYTES > 0 and total_bytes + block_bytes > MAX_TOTAL_BYTES:
            sections.append(f"\n[OUTPUT TRUNCATED BY MAX_TOTAL_BYTES]\n")
            break
        
        sections.append(code_block)
        total_bytes += block_bytes
        included += 1
    
    # Construir Security Report completo agora que temos todas as estatísticas
    security_report_lines = [
        f"- **Files excluded by filename/path**: {security_stats['excluded_by_filename']}",
    ]
    if security_stats['excluded_files']:
        security_report_lines.append("  - Sample excluded files:")
        for exc_file in security_stats['excluded_files'][:10]:
            security_report_lines.append(f"    - `{exc_file}`")
    
    security_report_lines.append(f"- **Files redacted by content**: {len(security_stats['redacted_files'])}")
    if security_stats['redacted_files']:
        security_report_lines.append("  - Redacted files:")
        for red_file in security_stats['redacted_files'][:10]:
            security_report_lines.append(f"    - `{red_file}`")
    
    if security_stats['redaction_patterns']:
        security_report_lines.append("  - Patterns detected:")
        for pattern, count in sorted(security_stats['redaction_patterns'].items(), key=lambda x: x[1], reverse=True):
            security_report_lines.append(f"    - `{pattern}`: {count} occurrences")
    
    security_report = "\n".join(security_report_lines)
    intelligence_section = intelligence_section.replace(security_report_placeholder, security_report)
    
    final_text = "\n".join(header + [toc, intelligence_section, file_index] + files_header + sections)
    
    # Safety Gate: escanear MD final por segredos
    print("[INFO] Executando Safety Gate...")
    safe_final_text, safety_findings = safety_gate_scan(final_text)
    safety_gate_failed = len(safety_findings) > 0
    
    if safety_gate_failed:
        # Listar tipos de padrões detectados (sem valores)
        pattern_types = sorted(set(f["pattern"] for f in safety_findings))
        print(f"[SECURITY] Safety Gate triggered! {len(safety_findings)} segredo(s) detectado(s)", file=sys.stderr)
        print(f"[SECURITY] Tipos detectados: {', '.join(pattern_types)}", file=sys.stderr)
        print(f"[ERRO] Safety Gate detectou {len(safety_findings)} segredo(s) no MD gerado!", file=sys.stderr)
        
        # Reescrever MD com conteúdo sanitizado
        final_text = safe_final_text
        
        # Gerar security_report.json
        security_report_path = os.path.splitext(out_path)[0] + "_security_report.json"
        security_report_data = {
            "generated_at": now,
            "root": os.path.abspath(root),
            "commit": commit,
            "safety_gate_findings": safety_findings,
            "action_taken": "MD reescrito com valores redigidos",
            "excluded_files": security_stats["excluded_files"],
            "redacted_files": security_stats["redacted_files"],
            "redaction_patterns": security_stats["redaction_patterns"]
        }
        try:
            with open(security_report_path, "w", encoding="utf-8") as f:
                json.dump(security_report_data, f, indent=2, ensure_ascii=False)
            print(f"[ERRO] Security report gerado: {security_report_path}", file=sys.stderr)
        except Exception as e:
            print(f"[ERRO] Falha ao gerar security report: {e}", file=sys.stderr)
    else:
        print("[OK] Safety Gate: nenhum segredo detectado no MD final")
    
    # Side-car JSON
    if CONCAT_SIDE_CAR_JSON:
        json_path = os.path.splitext(out_path)[0] + ".meta.json"
        json_data = {
            "root": os.path.abspath(root),
            "generated_at": now,
            "commit": commit,
            "mode": mode,
            "stack_detected": stack,
            "entrypoints": entrypoints,
            "commands": commands,
            "env_vars": {k: {"count": v["count"], "samples": v["samples"]} for k, v in env_vars.items()},
            "api_routes": api_routes[:50],
            "smells": {
                "todos": smells["todos"],
                "tests_present": smells["tests_present"],
                "tests_count": smells["tests_count"],
                "env_files": smells["env_files"],
                "largest_files": [{"path": p, "size": s} for p, s in smells["largest_files"]]
            },
            "security": {
                "excluded_by_filename": security_stats["excluded_by_filename"],
                "excluded_files": security_stats["excluded_files"],
                "redacted_files": security_stats["redacted_files"],
                "redaction_patterns": security_stats["redaction_patterns"]
            },
            "files_included": file_metadata,
            "signals": signals,
        }
        try:
            with open(json_path, "w", encoding="utf-8") as f:
                json.dump(json_data, f, indent=2, ensure_ascii=False)
            print(f"[INFO] Side-car JSON gerado: {json_path}")
        except Exception as e:
            print(f"[WARN] Falha ao gerar side-car JSON: {e}", file=sys.stderr)
    
    # Sobrescrita garantida
    try:
        if os.path.exists(out_path):
            os.remove(out_path)
    except Exception:
        pass
    
    os.makedirs(os.path.dirname(os.path.abspath(out_path)) or ".", exist_ok=True)
    with open(out_path, "w", encoding="utf-8", errors="replace") as f:
        f.write(final_text)
    
    if truncated_count > 0:
        print(f"[INFO] {truncated_count} arquivos truncados no modo smart")
    
    return included, len(selected), safety_gate_failed

def parse_roots_from_env(env_value: str) -> List[str]:
    if not env_value:
        return []
    raw = env_value.replace("|", "\n").replace(";", "\n").splitlines()
    roots = [p.strip().strip('"').strip("'") for p in raw if p.strip()]
    return roots

def resolve_projects() -> Tuple[List[str], bool]:
    cli_roots = [arg for arg in sys.argv[1:] if arg.strip()]
    if cli_roots:
        return cli_roots, True
    env_roots = parse_roots_from_env(CONCAT_ROOTS_ENV)
    if env_roots:
        return env_roots, True
    return [ROOT], False

def main():
    roots, is_multi = resolve_projects()

    if is_multi:
        print(f"[INFO] Projetos a processar ({len(roots)}):")
        for r in roots:
            print("  -", os.path.abspath(r))
        print(f"[INFO] Saída por projeto: <root>\\{CONCAT_OUT_BASENAME}\n")

        any_error = False
        for r in roots:
            if not os.path.isdir(r):
                print(f"[ERRO] ROOT não encontrado: {r}", file=sys.stderr)
                any_error = True
                continue
            out_path = os.path.join(r, CONCAT_OUT_BASENAME)
            exclude_basenames = {os.path.basename(out_path).lower()}
            included, considered, safety_failed = concat_repo(r, out_path, exclude_basenames)
            
            if safety_failed:
                print(f"[ERRO] Safety Gate falhou para {r}!", file=sys.stderr)
                any_error = True
            
            print(f"[OK] Gerado: {out_path}")
            print(f"     Arquivos considerados: {considered} | incluídos (texto): {included}\n")
        if any_error:
            # Se Safety Gate falhou, usar exit code 3 (segurança)
            # Caso contrário, exit code 2 (erro geral)
            sys.exit(3)

    else:
        r = roots[0]
        if not os.path.isdir(r):
            print(f"[ERRO] ROOT não encontrado: {r}", file=sys.stderr)
            sys.exit(2)
        out_path = OUT_PATH
        exclude_basenames = {os.path.basename(out_path).lower()}
        included, considered, safety_failed = concat_repo(r, out_path, exclude_basenames)
        
        if safety_failed:
            print(f"[ERRO] Safety Gate detectou segredos! Build falhou com exit code 3", file=sys.stderr)
            sys.exit(3)
        
        print(f"[OK] Gerado: {out_path}")
        print(f"Arquivos considerados (pós .gitignore): {considered} | incluídos (texto): {included}")

if __name__ == "__main__":
    main()

# ============================================================================
# QUICK SANITY TESTS (comentários para validação manual)
# ============================================================================
#
# Cenário 1: Repo com .git, script no root, sem env/args
#   - Colocar concat_repo_all_text.py na raiz do repo (onde tem .git)
#   - Executar: python concat_repo_all_text.py
#   - Esperado: [OK] e gera <repo>/repo_concat_all.md
#
# Cenário 2: Script em subpasta tools/, repo tem .git acima
#   - Colocar concat_repo_all_text.py em tools/concat_repo_all_text.py
#   - Executar: python tools/concat_repo_all_text.py
#   - Esperado: encontra .git acima e usa repo root, gera <repo>/repo_concat_all.md
#
# Cenário 3: Repo sem .git
#   - Executar script em diretório sem .git
#   - Esperado: usa pasta do script como ROOT, gera <script_dir>/repo_concat_all.md
#
# Cenário 4: Com CONCAT_ROOT e/ou CONCAT_OUT_PATH definidos
#   - Setar env vars: CONCAT_ROOT=C:\outro\repo CONCAT_OUT_PATH=C:\saida.md
#   - Executar: python concat_repo_all_text.py
#   - Esperado: usa valores das env vars (comportamento antigo mantido)
#
# ============================================================================
# SECURITY TESTS (validação de segurança)
# ============================================================================
#
# Teste 1: Arquivo .env em subpasta
#   - Criar backend/.env com: PASSWORD=REDACTED:PASSWORD API_KEY=REDACTED:API_KEY
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * backend/.env NÃO aparece no MD (excluído por nome)
#     * Security Report lista backend/.env como excluído
#     * Nenhum valor de senha/chave aparece no MD
#
# Teste 2: Service account JSON
#   - Criar config/service-account.json com campo "private_key"
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * Se o arquivo passar (não for excluído), o campo private_key deve ser redigido
#     * Security Report lista o arquivo como redigido
#     * Padrão SERVICE_ACCOUNT_KEY aparece no relatório
#
# Teste 3: Security Report
#   - Criar múltiplos arquivos sensíveis: .env, secrets/key.pem, credentials.json
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * Security Report mostra quantidade de arquivos excluídos
#     * Lista de sample files excluídos (até 10)
#     * Se algum arquivo for redigido, mostra padrões detectados
#
# Teste 4: Detecção de .env em qualquer pasta
#   - Criar: backend/.env, frontend/.env.local, .env.production
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * Todos os .env são excluídos do MD
#     * Smells/Gaps lista todos os .env encontrados (mesmo excluídos)
#     * Security Report mostra exclusões
#
# Teste 5: Auto-exclusão do próprio MD
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * repo_concat_all.md NÃO aparece no próprio conteúdo
#     * Não há loop infinito ou auto-inclusão
#
# Teste 6: Safety Gate - Detecção de segredos no MD final
#   - Criar um arquivo de código (ex: config.py) com: API_KEY="REDACTED:API_KEY"
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * O arquivo config.py é incluído no MD
#     * Safety Gate detecta o segredo no MD final
#     * MD é reescrito com REDACTED:OPENAI_KEY
#     * security_report.json é gerado com findings
#     * Script retorna exit code 3
#     * Verificar security_report.json contém:
#       - safety_gate_findings com pattern e linha
#       - action_taken: "MD reescrito com valores redigidos"
#
# Teste 7: Safety Gate - Múltiplos padrões
#   - Criar arquivo test_secrets.py com:
#     * PASSWORD="REDACTED:PASSWORD"
#     * TOKEN="REDACTED:TOKEN"
#     * API_KEY="REDACTED:API_KEY"
#   - Executar: python concat_repo_all_text.py
#   - Esperado:
#     * Safety Gate detecta todos os padrões
#     * security_report.json lista todos os findings
#     * MD final não contém nenhum valor real
#
# ============================================================================
`````

## [14] scripts/test-api.ps1

```powershell
// FILE: scripts/test-api.ps1
// FULL: C:\Users\leopo\rust-studies\scripts\test-api.ps1
// SIZE: 8693 bytes
// MTIME: 2026-02-04T03:01:21.813607
// NOTE: Concatenated snapshot for review
// SECURITY: Content redacted due to secret patterns: SECRET, TOKEN
# Script de teste da API Chopebrain ODS
# Gera certificados (servidor + cliente), sobe a API em HTTPS com mTLS e testa login + endpoints.
# Executar na raiz do repositório: .\scripts\test-api.ps1

$ErrorActionPreference = "Stop"
$RepoRoot = if ($PSScriptRoot) { Split-Path $PSScriptRoot -Parent } else { Get-Location }
$ApiDir = Join-Path $RepoRoot "chopebrain-ods-api"
$CertsDir = Join-Path $RepoRoot "certs-mtls"
$ListenPort = 30443
# No Windows com .NET Framework, client cert PEM não funciona com curl/Invoke-RestMethod; use HTTP para o script
$UseHttp = ($env:TEST_API_HTTP -eq "1") -or ($PSVersionTable.PSVersion.Major -lt 6 -and -not (Test-Path "C:\Program Files\dotnet\shared\Microsoft.NETCore.App\5.0*" -ErrorAction SilentlyContinue))
$BaseUrl = if ($UseHttp) { "http://127.0.0.1:$ListenPort" } else { "https://127.0.0.1:$ListenPort" }
$ClientCert = Join-Path $CertsDir "client-cert.pem"
$ClientKey = Join-Path $CertsDir "client-key.pem"
$CurlExe = if (Test-Path "C:\Windows\System32\curl.exe") { "C:\Windows\System32\curl.exe" } else { "curl.exe" }

# Credencial de teste (deve bater com AUTH_SECRET no .env)
$AuthSecret = REDACTED:SECRET
if (-not $AuthSecret) { $AuthSecret = "REDACTED:SECRET" }

# No Windows, curl.exe (Schannel) não aceita PEM para client cert; usamos Invoke-RestMethod + cert .NET
function Get-ClientCertificate {
    $cert = [System.Security.Cryptography.X509Certificates.X509Certificate2]::CreateFromPemFile($ClientCert, $ClientKey)
    return $cert
}

function Write-Step { param($Msg) Write-Host "`n==> $Msg" -ForegroundColor Cyan }
function Write-Ok   { param($Msg) Write-Host "    OK: $Msg" -ForegroundColor Green }
function Write-Fail { param($Msg) Write-Host "    FALHA: $Msg" -ForegroundColor Red }

function Get-ResponseBodyFromError {
    param($ErrorRecord)
    $body = $null
    if ($ErrorRecord.ErrorDetails.Message) { $body = $ErrorRecord.ErrorDetails.Message }
    if ($body) { return $body }
    if ($ErrorRecord.Exception.Response) {
        try {
            $stream = $ErrorRecord.Exception.Response.GetResponseStream()
            if ($stream) {
                $reader = New-Object System.IO.StreamReader($stream)
                $body = $reader.ReadToEnd()
                $reader.Close()
            }
        } catch {}
    }
    return $body
}

Push-Location $RepoRoot
$apiProcess = $null

try {
    # 1) Gerar certificados (servidor + cliente) se não existirem
    Write-Step "Certificados mTLS (servidor + cliente)"
    if (-not (Test-Path (Join-Path $CertsDir "server-cert.pem"))) {
        Write-Host "    Gerando certificados em $CertsDir ..."
        cargo run --manifest-path (Join-Path $ApiDir "Cargo.toml") --bin gen-certs 2>&1 | Out-Null
        if (-not (Test-Path $ClientCert)) { throw "gen-certs não criou client-cert.pem" }
        Write-Ok "Certificados gerados (ca, server-cert/key, client-cert/key)"
    } else {
        Write-Ok "Certificados já existem em $CertsDir"
    }

    # 2) Build da API
    Write-Step "Build da API"
    cargo build --manifest-path (Join-Path $ApiDir "Cargo.toml") | Out-Null
    $exePath = Join-Path $ApiDir "target\debug\chopebrain-ods-api.exe"
    if (-not (Test-Path $exePath)) {
        Write-Fail "Executável não encontrado: $exePath"
        exit 1
    }
    Write-Ok "Build concluído"

    # 3) Liberar porta se estiver em uso (execução anterior)
    $existing = Get-NetTCPConnection -LocalPort $ListenPort -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($existing) {
        Stop-Process -Id $existing.OwningProcess -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 1
    }

    # 4) Subir a API em background (HTTPS + mTLS)
    Write-Step "Iniciando API em $BaseUrl (HTTPS + mTLS)"
    $ListenAddr = "127.0.0.1:$ListenPort"
    $EnvApiFile = Join-Path $RepoRoot ".env.api"
    if (-not (Test-Path $EnvApiFile)) {
        Write-Fail "Arquivo .env.api não encontrado em $EnvApiFile (crie com ODS_*, JWT_*, AUTH_SECRET)"
        exit 1
    }
    $env:API_ENV_FILE = $EnvApiFile
    $env:LISTEN = $ListenAddr
    $env:WORK_DIR = $RepoRoot
    $env:RUST_BACKTRACE = "1"
    if ($UseHttp) { $env:FORCE_HTTP = "1"; Write-Host "    (modo HTTP para testes - sem mTLS)" -ForegroundColor Gray }
    $apiProcess = Start-Process -FilePath $exePath -WorkingDirectory $RepoRoot -PassThru
    Start-Sleep -Seconds 8

    # Verificar se a porta abriu
    $portOpen = $false
    for ($i = 0; $i -lt 5; $i++) {
        $conn = Get-NetTCPConnection -LocalPort $ListenPort -State Listen -ErrorAction SilentlyContinue
        if ($conn) { $portOpen = $true; break }
        Start-Sleep -Seconds 2
    }
    if (-not $portOpen) {
        Stop-Process -Id $apiProcess.Id -Force -ErrorAction SilentlyContinue
        Write-Fail "Porta $ListenPort não está em LISTEN (API pode ter falhado ao iniciar)"
        exit 1
    }
    Write-Ok "Porta $ListenPort em escuta"

    $loginBody = @{ secret = REDACTED:SECRET } | ConvertTo-Json
    $headers = @{ "Content-Type" = "application/json" }

    # 5) Aguardar API responder
    $maxAttempts = 15
    $attempt = 0
    $r = $null
    $params = @{ Uri = "$BaseUrl/api/auth/login"; Method = "Post"; Body = $loginBody; Headers = $headers; ErrorAction = "Stop" }
    if (-not $UseHttp) {
        try { $clientCert = Get-ClientCertificate } catch { $env:FORCE_HTTP = "1"; $UseHttp = $true; $BaseUrl = "http://127.0.0.1:$ListenPort" }
        if (-not $UseHttp) { $params["Certificate"] = $clientCert; $params["SkipCertificateCheck"] = $true }
    }
    while ($attempt -lt $maxAttempts) {
        try {
            $r = Invoke-RestMethod @params
            if ($r.token) { break }
        } catch {}
        $attempt++
        Start-Sleep -Seconds 1
    }
    if (-not $r -or -not $r.token) {
        Stop-Process -Id $apiProcess.Id -Force -ErrorAction SilentlyContinue
        Write-Fail "API não respondeu em $BaseUrl (login)"
        exit 1
    }
    Write-Ok "API respondendo"

    # 6) Login e obter token
    Write-Step "Login (POST /api/auth/login)"
    $loginJson = Invoke-RestMethod @params
    $token = REDACTED:TOKEN
    if (-not $token) { throw "Resposta sem token" }
    Write-Ok "Token obtido"

    # 7) Hamburgueres vendidos
    Write-Step "Hamburgueres vendidos (POST /api/ods/hamburgueres-vendidos)"
    $mes = Get-Date -Format "yyyy-MM"
    $bodyVendidos = @{ mes = $mes } | ConvertTo-Json
    $authParams = @{ Uri = "$BaseUrl/api/ods/hamburgueres-vendidos"; Method = "Post"; Body = $bodyVendidos; Headers = @{ "Content-Type" = "application/json"; "Authorization" = "Bearer $token" } }
    if (-not $UseHttp) { $authParams["Certificate"] = $clientCert; $authParams["SkipCertificateCheck"] = $true }
    try {
        $respVendidos = Invoke-RestMethod @authParams
        if ($respVendidos.totais) {
            Write-Ok "Resposta com 'totais' (quantidade: $($respVendidos.totais.quantidade), valor: $($respVendidos.totais.valor))"
        } else {
            Write-Ok "Resposta recebida"
        }
    } catch {
        if ($_.Exception.Response.StatusCode.value__ -eq 500) {
            $body = Get-ResponseBodyFromError -ErrorRecord $_
            Write-Host "    Aviso: 500 Erro Interno" -ForegroundColor Yellow
            if ($body) { Write-Host "    Detalhe (resposta da API): $body" -ForegroundColor Gray }
            else { Write-Host "    (verifique conexão ODS/MySQL e .env.api)" -ForegroundColor Gray }
        } else { throw }
    }

    # 8) Hamburgueres comandas antigas
    Write-Step "Hamburgueres comandas antigas (POST /api/ods/hamburgueres-comandas-antigas)"
    $authParams.Uri = "$BaseUrl/api/ods/hamburgueres-comandas-antigas"
    try {
        $respComandas = Invoke-RestMethod @authParams
        Write-Ok "Resposta recebida"
    } catch {
        if ($_.Exception.Response.StatusCode.value__ -eq 500) {
            $body = Get-ResponseBodyFromError -ErrorRecord $_
            Write-Host "    Aviso: 500 Erro Interno" -ForegroundColor Yellow
            if ($body) { Write-Host "    Detalhe (resposta da API): $body" -ForegroundColor Gray }
            else { Write-Host "    (verifique conexão ODS/MySQL)" -ForegroundColor Gray }
        } else { throw }
    }

    Write-Host ""
    Write-Host "=== Todos os testes passaram. ===" -ForegroundColor Green
}
catch {
    Write-Fail $_.Exception.Message
    exit 1
}
finally {
    if ($apiProcess -and -not $apiProcess.HasExited) {
        Stop-Process -Id $apiProcess.Id -Force -ErrorAction SilentlyContinue
    }
    Pop-Location
}

```

## [15] workflow-mvp/README.md

````markdown
# FILE: workflow-mvp/README.md
# FULL: C:\Users\leopo\rust-studies\workflow-mvp\README.md
# SIZE: 4047 bytes
# MTIME: 2026-02-04T09:28:10.859335
# NOTE: Concatenated snapshot for review
# Workflow Engine MVP (BPEL-lite + Tauri + Axum)

MVP E2E: motor de workflows estilo BPEL simplificado com DSL versionado, compilador (graph → DSL), runtime e desktop com skin retrô (98.css).

## Estrutura

- **backend/** — API Rust (Axum). Compila graph → DSL, valida, persiste workflows/executions, executa engine.
- **desktop/** — App Tauri 2 + React + TypeScript + Vite. Designer (React Flow), Inspector, Compile/Validate/Save/Load/Run.
- **docker-compose.yml** — Sobe o backend na porta 8080 com volume para dados.

## Pré-requisitos

- Rust (backend)
- Node.js 18+ (desktop)
- Docker e Docker Compose (para rodar o backend em container)

## Como rodar

### 1. Backend (API)

**Com Docker (recomendado):**

```bash
docker compose up --build
```

O backend sobe em **http://localhost:8080**. Dados em `./backend/data` (volume).

**Sem Docker:**

```bash
cd backend
cargo run
```

Crie a pasta `backend/data` (ou defina `DATA_DIR`) para persistir workflows e executions.

### 2. Desktop (Tauri)

```bash
cd desktop
npm install
npm run tauri dev
```

Na primeira vez o Tauri compila o Rust do app; o Vite sobe em http://localhost:1420.

### 3. Fluxo de uso

1. **Desenhar** — Use "Load Demo" ou adicione nós pelo Toolbox e conecte com arestas.
2. **Compile** — Gera a DSL a partir do grafo (aparece na aba Source).
3. **Validate** — Mostra erros/avisos (Problems).
4. **Save** — Persiste workflow (graph + DSL) no backend; o ID fica em localStorage.
5. **Load** — Informe o ID e carregue um workflow salvo.
6. **Run Dry** — Executa em modo dry-run (sem side-effects reais); trace na área Execution.
7. **Run** — Executa de verdade (transform_json, http_call, etc.); resultado em Execution.

## Endpoints da API (backend)

| Método | Rota | Descrição |
|--------|------|-----------|
| GET | /health | Health check |
| GET | /connectors/schema | Schema dos step kinds (Inspector/Toolbox) |
| POST | /workflows/compile | Body: `{ nodes, edges }` → `{ ok, workflowDefinition?, problems }` |
| POST | /workflows/validate | Body: `{ graph? }` ou `{ dsl? }` → `{ problems }` |
| POST | /workflows/save | Body: `{ id?, name, graph, dsl }` → `{ id, name, saved_at }` |
| GET | /workflows/:id | Workflow salvo |
| POST | /workflows/:id/run | Body: `{ mode: "dry-run"\|"run", inputContext? }` → ExecutionResult |
| GET | /executions/:id | Resultado de execução |
| POST | /workflows/:id/deploy | Fake deploy (retorna deployment_id) |

## DSL (exemplo)

```json
{
  "version": 1,
  "id": "...",
  "name": "Ingest Pipeline",
  "start": "step_trigger",
  "steps": {
    "step_trigger": { "kind": "trigger.redis_stream", "next": "step_transform", "config": {}, "retry": {}, "onFault": "step_fault" },
    "step_transform": { "kind": "activity.transform_json", "next": "step_if", "config": { "expr": "$.amount" }, "onFault": "step_fault" },
    "step_if": { "kind": "control.if", "config": { "expr": "$.amount > 100" }, "then": "step_publish_high", "else": "step_publish_low", "onFault": "step_fault" },
    "step_publish_high": { "kind": "activity.publish_stream", "next": "step_end", "config": { "stream": "high" }, "onFault": "step_fault" },
    "step_publish_low": { "kind": "activity.publish_stream", "next": "step_end", "config": { "stream": "low" }, "onFault": "step_fault" },
    "step_end": { "kind": "control.end" },
    "step_fault": { "kind": "control.fault_end", "config": { "reason": "faulted" } }
  }
}
```

Step kinds MVP: `trigger.redis_stream`, `activity.download_blob`, `activity.transform_json`, `activity.http_call`, `activity.publish_stream`, `control.if`, `control.end`, `control.fault_end`.

## Done criteria

- [x] Compile: graph → DSL
- [x] Validate: ciclos, then/else em if, campos obrigatórios
- [x] Run dry: percorre steps e gera trace
- [x] Run: transform_json e stubs; resultados em GET /executions/:id
- [x] Desktop: desenhar, compilar, validar, salvar, carregar, executar e ver trace

````

## [16] workflow-mvp/backend/Cargo.toml

```toml
# FILE: workflow-mvp/backend/Cargo.toml
# FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\Cargo.toml
# SIZE: 652 bytes
# MTIME: 2026-02-04T09:15:15.875104
# NOTE: Concatenated snapshot for review
[package]
name = "workflow-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["json", "macros"] }
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
reqwest = { version = "0.11", features = ["json", "blocking"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
lazy_static = "1.4"

```

## [17] workflow-mvp/backend/Dockerfile

```
// FILE: workflow-mvp/backend/Dockerfile
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\Dockerfile
// SIZE: 423 bytes
// MTIME: 2026-02-04T09:18:26.540723
// NOTE: Concatenated snapshot for review
FROM rust:1.76-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/workflow-backend /app/workflow-backend
ENV DATA_DIR=/app/data
VOLUME /app/data
EXPOSE 8080
CMD ["/app/workflow-backend"]

```

## [18] workflow-mvp/backend/src/api.rs

```rust
// FILE: workflow-mvp/backend/src/api.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\api.rs
// SIZE: 6446 bytes
// MTIME: 2026-02-04T09:17:39.634305
// NOTE: Concatenated snapshot for review
//! API Axum: health, schema, compile, validate, save, get workflow, run, get execution, deploy.

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::compile::{compile, CompileResult};
use crate::engine::{run_workflow, ExecutionResult};
use crate::models::{Graph, SavedWorkflow, WorkflowDefinition};
use crate::schema::connector_schemas;
use crate::storage;
use crate::validate::{validate_dsl, validate_graph, ValidateRequest, ValidateResult};

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    Router::new()
        .route("/health", get(health))
        .route("/connectors/schema", get(connectors_schema))
        .route("/workflows/compile", post(workflows_compile))
        .route("/workflows/validate", post(workflows_validate))
        .route("/workflows/save", post(workflows_save))
        .route("/workflows/:id", get(workflows_get))
        .route("/workflows/:id/run", post(workflows_run))
        .route("/workflows/:id/deploy", post(workflows_deploy))
        .route("/executions/:id", get(executions_get))
        .layer(cors)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn connectors_schema() -> Json<Vec<crate::schema::ConnectorSchema>> {
    Json(connector_schemas())
}

#[derive(Deserialize)]
struct CompileBody {
    nodes: Vec<crate::models::GraphNode>,
    edges: Vec<crate::models::GraphEdge>,
    #[serde(rename = "workflowId")]
    workflow_id: Option<String>,
    name: Option<String>,
}

async fn workflows_compile(Json(body): Json<CompileBody>) -> Json<CompileResult> {
    let graph = Graph {
        nodes: body.nodes,
        edges: body.edges,
    };
    let result = compile(graph, body.workflow_id, body.name);
    Json(result)
}

async fn workflows_validate(Json(body): Json<serde_json::Value>) -> Json<ValidateResult> {
    let problems = if body.get("graph").is_some() {
        match serde_json::from_value::<Graph>(body.get("graph").cloned().unwrap_or_default()) {
            Ok(graph) => validate_graph(&graph).problems,
            Err(_) => vec![crate::validate::ValidationProblem {
                code: "INVALID_GRAPH".to_string(),
                message: "Invalid graph JSON".to_string(),
                node_id: None,
                step_id: None,
                severity: "error".to_string(),
            }],
        }
    } else if body.get("dsl").is_some() {
        match serde_json::from_value::<WorkflowDefinition>(body.get("dsl").cloned().unwrap_or_default()) {
            Ok(dsl) => validate_dsl(&dsl),
            Err(_) => vec![crate::validate::ValidationProblem {
                code: "INVALID_DSL".to_string(),
                message: "Invalid DSL JSON".to_string(),
                node_id: None,
                step_id: None,
                severity: "error".to_string(),
            }],
        }
    } else {
        vec![]
    };
    Json(ValidateResult { problems })
}

#[derive(Deserialize)]
struct SaveBody {
    id: Option<String>,
    name: String,
    graph: Graph,
    dsl: WorkflowDefinition,
}

#[derive(serde::Serialize)]
struct SaveResponse {
    id: String,
    name: String,
    #[serde(rename = "saved_at")]
    saved_at: String,
}

async fn workflows_save(Json(body): Json<SaveBody>) -> Result<Json<SaveResponse>, (StatusCode, String)> {
    let st = storage::storage();
    let w = st.save_workflow(body.id, body.name.clone(), body.graph, body.dsl)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(SaveResponse {
        id: w.id,
        name: w.name,
        saved_at: w.saved_at,
    }))
}

async fn workflows_get(Path(id): Path<String>) -> Result<Json<SavedWorkflow>, (StatusCode, String)> {
    let st = storage::storage();
    let w = st.get_workflow(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Workflow not found".to_string()))?;
    Ok(Json(w))
}

#[derive(Deserialize)]
struct RunBody {
    mode: String,
    #[serde(rename = "inputContext")]
    input_context: Option<InputContext>,
}

#[derive(Deserialize, Default)]
struct InputContext {
    #[serde(default)]
    vars: serde_json::Value,
}

async fn workflows_run(
    Path(id): Path<String>,
    Json(body): Json<RunBody>,
) -> Result<Json<ExecutionResult>, (StatusCode, String)> {
    let st = storage::storage();
    let w = st.get_workflow(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Workflow not found".to_string()))?;

    let execution_id = Uuid::new_v4().to_string();
    let correlation_id = Uuid::new_v4().to_string();
    let vars = body
        .input_context
        .map(|c| c.vars)
        .unwrap_or(serde_json::json!({}));
    let mode = if body.mode == "run" { "run" } else { "dry-run" };

    let workflow = w.dsl.clone();
    let result = tokio::task::spawn_blocking(move || {
        run_workflow(
            &workflow,
            vars,
            mode,
            &execution_id,
            &correlation_id,
        )
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    st.save_execution(&result.execution_id, &result)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(result))
}

async fn workflows_deploy(Path(id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("deploy (fake) workflow id={}", id);
    Json(serde_json::json!({
        "deployment_id": Uuid::new_v4().to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn executions_get(Path(id): Path<String>) -> Result<Json<ExecutionResult>, (StatusCode, String)> {
    let st = storage::storage();
    let r = st.get_execution(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Execution not found".to_string()))?;
    Ok(Json(r))
}

```

## [19] workflow-mvp/backend/src/compile.rs

```rust
// FILE: workflow-mvp/backend/src/compile.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\compile.rs
// SIZE: 5753 bytes
// MTIME: 2026-02-04T09:10:36.304237
// NOTE: Concatenated snapshot for review
//! Compilador: Graph (frontend) -> WorkflowDefinition (DSL).

use crate::models::{Graph, GraphEdge, GraphNode, RetryConfig, StepDef, WorkflowDefinition};
use std::collections::{HashMap, HashSet};

#[derive(Debug, serde::Serialize)]
pub struct CompileResult {
    pub ok: bool,
    #[serde(rename = "workflowDefinition", skip_serializing_if = "Option::is_none")]
    pub workflow_definition: Option<WorkflowDefinition>,
    pub problems: Vec<CompileProblem>,
}

#[derive(Debug, serde::Serialize)]
pub struct CompileProblem {
    pub code: String,
    pub message: String,
}

pub fn compile(graph: Graph, workflow_id: Option<String>, name: Option<String>) -> CompileResult {
    let mut problems = Vec::new();
    let workflow_id = workflow_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let name = name.unwrap_or_else(|| "Unnamed".to_string());

    // Find start: single node with type starting with "trigger."
    let triggers: Vec<_> = graph
        .nodes
        .iter()
        .filter(|n| n.node_type.starts_with("trigger."))
        .collect();
    if triggers.is_empty() {
        problems.push(CompileProblem {
            code: "NO_TRIGGER".to_string(),
            message: "No trigger node found (need one node with type trigger.*)".to_string(),
        });
        return CompileResult {
            ok: false,
            workflow_definition: None,
            problems,
        };
    }
    if triggers.len() > 1 {
        problems.push(CompileProblem {
            code: "MULTIPLE_TRIGGERS".to_string(),
            message: "Multiple trigger nodes found (only one allowed)".to_string(),
        });
        return CompileResult {
            ok: false,
            workflow_definition: None,
            problems,
        };
    }
    let start_id = triggers[0].id.clone();

    // Build step id -> (next, then, else, onFault) from edges
    let mut next_map: HashMap<String, String> = HashMap::new();
    let mut then_map: HashMap<String, String> = HashMap::new();
    let mut else_map: HashMap<String, String> = HashMap::new();
    let mut on_fault_map: HashMap<String, String> = HashMap::new();

    for edge in &graph.edges {
        let label = edge
            .label
            .as_deref()
            .unwrap_or("next")
            .to_lowercase();
        let target = edge.target.clone();
        match label.as_str() {
            "then" => {
                then_map.insert(edge.source.clone(), target);
            }
            "else" => {
                else_map.insert(edge.source.clone(), target);
            }
            "onfault" | "on_fault" => {
                on_fault_map.insert(edge.source.clone(), target);
            }
            _ => {
                next_map.insert(edge.source.clone(), target);
            }
        }
    }

    // Build steps from nodes
    let node_ids: HashSet<_> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
    let mut steps: HashMap<String, StepDef> = HashMap::new();

    for node in &graph.nodes {
        let next = next_map.get(&node.id).cloned();
        let then = then_map.get(&node.id).cloned();
        let else_b = else_map.get(&node.id).cloned();
        let on_fault = on_fault_map.get(&node.id).cloned();

        if node.node_type == "control.if" {
            if then.is_none() || else_b.is_none() {
                problems.push(CompileProblem {
                    code: "IF_MISSING_BRANCHES".to_string(),
                    message: format!(
                        "control.if node '{}' must have exactly one 'then' and one 'else' edge",
                        node.id
                    ),
                });
            }
        }

        let step = StepDef {
            kind: node.node_type.clone(),
            next,
            then,
            r#else: else_b,
            config: node.data.clone(),
            retry: Some(RetryConfig::default()),
            on_fault,
        };
        steps.insert(node.id.clone(), step);
    }

    // Validate DAG: no cycles in main flow (follow next/then/else, ignore onFault for cycle check)
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    if let Some(cycle) = find_cycle(&start_id, &steps, &mut visited, &mut stack) {
        problems.push(CompileProblem {
            code: "CYCLE".to_string(),
            message: format!("Cycle detected in flow: {}", cycle),
        });
    }

    if !problems.is_empty() {
        return CompileResult {
            ok: false,
            workflow_definition: None,
            problems,
        };
    }

    let workflow_definition = WorkflowDefinition {
        version: 1,
        id: workflow_id,
        name,
        start: start_id,
        steps,
    };

    CompileResult {
        ok: true,
        workflow_definition: Some(workflow_definition),
        problems,
    }
}

fn find_cycle(
    node: &str,
    steps: &HashMap<String, StepDef>,
    visited: &mut HashSet<String>,
    stack: &mut HashSet<String>,
) -> Option<String> {
    if stack.contains(node) {
        return Some(node.to_string());
    }
    if visited.contains(node) {
        return None;
    }
    visited.insert(node.to_string());
    stack.insert(node.to_string());

    let step = steps.get(node)?;
    let nexts = vec![
        step.next.as_ref(),
        step.then.as_ref(),
        step.r#else.as_ref(),
    ];
    for n in nexts.into_iter().flatten() {
        if let Some(cycle) = find_cycle(n, steps, visited, stack) {
            stack.remove(node);
            return Some(cycle);
        }
    }
    stack.remove(node);
    None
}

```

## [20] workflow-mvp/backend/src/engine/expr.rs

```rust
// FILE: workflow-mvp/backend/src/engine/expr.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\expr.rs
// SIZE: 4268 bytes
// MTIME: 2026-02-04T09:09:04.893483
// NOTE: Concatenated snapshot for review
//! Avaliação de expressões simples para control.if (ex: $.amount > 100).

use crate::engine::StepFault;
use crate::models::Context;

fn get_from_path(value: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
    let path = path.trim_start_matches('$').trim_start_matches('.');
    if path.is_empty() {
        return Some(value.clone());
    }
    let mut current = value;
    for part in path.split('.') {
        current = current.get(part)?;
    }
    Some(current.clone())
}

/// Avalia expressão booleana sobre o context (vars + last). Ex: "$.amount > 100"
pub fn eval_bool(ctx: &Context, expr: &str) -> Result<bool, StepFault> {
    let mut combined = serde_json::Map::new();
    if let Some(obj) = ctx.vars.as_object() {
        for (k, v) in obj {
            combined.insert(k.clone(), v.clone());
        }
    }
    if let Some(obj) = ctx.last.as_object() {
        for (k, v) in obj {
            combined.insert(k.clone(), v.clone());
        }
    }
    let value = serde_json::Value::Object(combined);
    eval_simple_expr(&value, expr)
}

fn eval_simple_expr(value: &serde_json::Value, expr: &str) -> Result<bool, StepFault> {
    let expr = expr.trim();
    if let Some(rest) = expr.strip_prefix("$.") {
        let parts: Vec<&str> = rest.splitn(2, ' ').collect();
        if parts.len() < 2 {
            return Ok(get_from_path(value, &format!("$.{}", rest)).is_some());
        }
        let path = parts[0];
        let op_val = parts[1].trim();
        let left = get_from_path(value, path).unwrap_or(serde_json::Value::Null);

        let (op, val_str) = if let Some(i) = op_val.find(">=") {
            (">=", op_val[i + 2..].trim())
        } else if let Some(i) = op_val.find("<=") {
            ("<=", op_val[i + 2..].trim())
        } else if let Some(i) = op_val.find("!=") {
            ("!=", op_val[i + 2..].trim())
        } else if let Some(i) = op_val.find("==") {
            ("==", op_val[i + 2..].trim())
        } else if let Some(i) = op_val.find('>') {
            (">", op_val[i + 1..].trim())
        } else if let Some(i) = op_val.find('<') {
            ("<", op_val[i + 1..].trim())
        } else {
            return Err(StepFault {
                code: "EXPR_PARSE".to_string(),
                message: "Expected comparison operator".to_string(),
            });
        };

        let right: serde_json::Value = if val_str.starts_with('"') && val_str.ends_with('"') {
            serde_json::Value::String(val_str[1..val_str.len() - 1].to_string())
        } else if let Ok(n) = val_str.parse::<i64>() {
            serde_json::Number::from(n).into()
        } else if let Ok(n) = val_str.parse::<f64>() {
            serde_json::Number::from_f64(n).map(Into::into).unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        };

        let result = match op {
            ">" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l > r
            }
            ">=" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l >= r
            }
            "<" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l < r
            }
            "<=" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l <= r
            }
            "==" => left == right,
            "!=" => left != right,
            _ => false,
        };
        return Ok(result);
    }
    Err(StepFault {
        code: "EXPR_PARSE".to_string(),
        message: format!("Unsupported expression: {}", expr),
    })
}

```

## [21] workflow-mvp/backend/src/engine/mod.rs

```rust
// FILE: workflow-mvp/backend/src/engine/mod.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\mod.rs
// SIZE: 269 bytes
// MTIME: 2026-02-04T09:09:04.886761
// NOTE: Concatenated snapshot for review
pub mod types;
pub mod trace;
pub mod runtime;
pub mod steps;
pub mod expr;

pub use types::{ExecutionResult, ExecutionStatus, TraceEvent, StepFault, ExecutionMetrics};
pub use trace::TraceRecorder;
pub use runtime::run_workflow;
pub use steps::execute_step;

```

## [22] workflow-mvp/backend/src/engine/runtime.rs

```rust
// FILE: workflow-mvp/backend/src/engine/runtime.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\runtime.rs
// SIZE: 5784 bytes
// MTIME: 2026-02-04T09:15:40.500237
// NOTE: Concatenated snapshot for review
//! Runtime: executa WorkflowDefinition com context inicial e produz ExecutionResult.

use crate::engine::expr;
use crate::engine::steps::execute_step;
use crate::engine::trace::TraceRecorder;
use crate::engine::types::{ExecutionMetrics, ExecutionResult, ExecutionStatus};
use crate::models::{Context, StepDef, WorkflowDefinition};
use chrono::Utc;
use std::time::Instant;

pub fn run_workflow(
    workflow: &WorkflowDefinition,
    initial_vars: serde_json::Value,
    mode: &str,
    execution_id: &str,
    correlation_id: &str,
) -> ExecutionResult {
    let start_wall = Instant::now();
    let mut trace = TraceRecorder::new();
    let mut steps_executed = 0u32;

    let mut meta = serde_json::Map::new();
    meta.insert(
        "correlation_id".to_string(),
        serde_json::Value::String(correlation_id.to_string()),
    );
    meta.insert(
        "workflow_id".to_string(),
        serde_json::Value::String(workflow.id.clone()),
    );
    meta.insert(
        "execution_id".to_string(),
        serde_json::Value::String(execution_id.to_string()),
    );

    let mut ctx = Context {
        vars: initial_vars,
        last: serde_json::Value::Object(serde_json::Map::new()),
        meta: serde_json::Value::Object(meta),
    };

    let mut current_id = workflow.start.clone();

    loop {
        let step = match workflow.steps.get(&current_id) {
            Some(s) => s,
            None => {
                trace.step_faulted(
                    &current_id,
                    serde_json::json!({"error": "Step not found" }),
                );
                return finish(ExecutionStatus::Faulted, ctx, trace, steps_executed, start_wall, execution_id);
            }
        };

        trace.step_started(&current_id);

        if step.kind == "control.end" {
            trace.step_succeeded(&current_id, serde_json::json!({}));
            steps_executed += 1;
            return finish(ExecutionStatus::Succeeded, ctx, trace, steps_executed, start_wall, execution_id);
        }

        if step.kind == "control.fault_end" {
            let _ = execute_step(step, &ctx, mode);
            trace.step_faulted(
                &current_id,
                step.config.get("reason").cloned().unwrap_or(serde_json::json!("faulted")),
            );
            steps_executed += 1;
            return finish(ExecutionStatus::Faulted, ctx, trace, steps_executed, start_wall, execution_id);
        }

        if step.kind == "control.if" {
            let expr_str = step.config.get("expr").and_then(|v| v.as_str()).unwrap_or("false");
            let branch = match expr::eval_bool(&ctx, expr_str) {
                Ok(true) => "then",
                Ok(false) => "else",
                Err(e) => {
                    trace.step_faulted(
                        &current_id,
                        serde_json::json!({ "code": e.code, "message": e.message }),
                    );
                    steps_executed += 1;
                    return finish(ExecutionStatus::Faulted, ctx, trace, steps_executed, start_wall, execution_id);
                }
            };
            trace.step_succeeded(
                &current_id,
                serde_json::json!({ "branch": branch }),
            );
            steps_executed += 1;
            current_id = step
                .next_step(Some(branch))
                .unwrap_or_else(|| current_id);
            continue;
        }

        let default_retry = crate::models::RetryConfig::default();
        let retry = step.retry.as_ref().unwrap_or(&default_retry);
        let mut attempt = 0u32;
        let next_id = loop {
            attempt += 1;
            match execute_step(step, &ctx, mode) {
                Ok(patch) => {
                    ctx.last = patch;
                    trace.step_succeeded(&current_id, ctx.last.clone());
                    steps_executed += 1;
                    break step.next_step(None);
                }
                Err(fault) => {
                    trace.step_faulted(
                        &current_id,
                        serde_json::json!({ "code": fault.code, "message": fault.message }),
                    );
                    if attempt >= retry.max_attempts {
                        if let Some(ref on_fault) = step.on_fault {
                            current_id = on_fault.clone();
                            break Some(on_fault.clone());
                        }
                        return finish(ExecutionStatus::Faulted, ctx, trace, steps_executed, start_wall, execution_id);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(
                        retry.backoff_ms * (retry.multiplier as u64).pow(attempt - 1),
                    ));
                }
            }
        };

        if let Some(id) = next_id {
            current_id = id;
        } else {
            return finish(ExecutionStatus::Faulted, ctx, trace, steps_executed, start_wall, execution_id);
        }
    }
}


fn finish(
    status: ExecutionStatus,
    ctx: Context,
    trace: TraceRecorder,
    steps_executed: u32,
    start_wall: Instant,
    execution_id: &str,
) -> ExecutionResult {
    ExecutionResult {
        execution_id: execution_id.to_string(),
        status,
        final_context: serde_json::json!({
            "vars": ctx.vars,
            "last": ctx.last,
            "meta": ctx.meta
        }),
        trace: trace.into_events(),
        metrics: ExecutionMetrics {
            steps_executed,
            total_duration_ms: start_wall.elapsed().as_millis() as u64,
        },
    }
}

```

## [23] workflow-mvp/backend/src/engine/steps/download_blob.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/download_blob.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\download_blob.rs
// SIZE: 336 bytes
// MTIME: 2026-02-04T09:07:25.274997
// NOTE: Concatenated snapshot for review
//! Stub: simula download e retorna bytes/size.

use crate::engine::StepFault;
use crate::models::Context;

pub fn run(
    _ctx: &Context,
    _config: &serde_json::Value,
    _mode: &str,
) -> Result<serde_json::Value, StepFault> {
    Ok(serde_json::json!({
        "bytes": "<simulated>",
        "size": 0
    }))
}

```

## [24] workflow-mvp/backend/src/engine/steps/fault_end.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/fault_end.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\fault_end.rs
// SIZE: 471 bytes
// MTIME: 2026-02-04T09:08:06.429178
// NOTE: Concatenated snapshot for review
//! control.fault_end: sinaliza parada com fault.

use crate::engine::StepFault;
use crate::models::Context;

pub fn run(
    _ctx: &Context,
    config: &serde_json::Value,
    _mode: &str,
) -> Result<serde_json::Value, StepFault> {
    let reason = config
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("faulted");
    Err(StepFault {
        code: "FAULT_END".to_string(),
        message: reason.to_string(),
    })
}

```

## [25] workflow-mvp/backend/src/engine/steps/http_call.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/http_call.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\http_call.rs
// SIZE: 1889 bytes
// MTIME: 2026-02-04T09:08:06.393421
// NOTE: Concatenated snapshot for review
//! activity.http_call: em run faz reqwest; em dry-run simula.

use crate::engine::StepFault;
use crate::models::Context;

pub fn run(
    ctx: &Context,
    config: &serde_json::Value,
    mode: &str,
) -> Result<serde_json::Value, StepFault> {
    let url = config
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StepFault {
            code: "CONFIG".to_string(),
            message: "http_call requires 'url'".to_string(),
        })?;
    let method = config.get("method").and_then(|v| v.as_str()).unwrap_or("GET");

    if mode == "dry-run" {
        return Ok(serde_json::json!({
            "simulated": true,
            "url": url,
            "method": method,
            "status": 200,
            "body": {}
        }));
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| StepFault {
            code: "HTTP_CLIENT".to_string(),
            message: e.to_string(),
        })?;

    let resp = match method.to_uppercase().as_str() {
        "GET" => client.get(url).send(),
        "POST" => {
            let body = config.get("body").cloned().unwrap_or(serde_json::Value::Null);
            client.post(url).json(&body).send()
        }
        "PUT" => {
            let body = config.get("body").cloned().unwrap_or(serde_json::Value::Null);
            client.put(url).json(&body).send()
        }
        _ => client.get(url).send(),
    };

    let resp = resp.map_err(|e| StepFault {
        code: "HTTP_ERROR".to_string(),
        message: e.to_string(),
    })?;

    let status = resp.status().as_u16();
    let body: serde_json::Value = resp.json().unwrap_or(serde_json::Value::Null);

    Ok(serde_json::json!({
        "status": status,
        "body": body
    }))
}

```

## [26] workflow-mvp/backend/src/engine/steps/mod.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/mod.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\mod.rs
// SIZE: 1984 bytes
// MTIME: 2026-02-04T09:15:40.054937
// NOTE: Concatenated snapshot for review
//! Step handlers: cada kind executa e retorna patch ou StepFault.

use crate::engine::StepFault;
use crate::models::{Context, StepDef};
use std::sync::Arc;

pub mod download_blob;
pub mod fault_end;
pub mod http_call;
pub mod publish_stream;
pub mod transform_json;

type StepHandler = Arc<dyn Fn(&Context, &serde_json::Value, &str) -> Result<serde_json::Value, StepFault> + Send + Sync>;

fn registry() -> std::collections::HashMap<String, StepHandler> {
    let mut m = std::collections::HashMap::new();
    m.insert(
        "trigger.redis_stream".to_string(),
        Arc::new(|ctx: &Context, _config: &serde_json::Value, _mode: &str| Ok(ctx.last.clone())) as StepHandler,
    );
    m.insert(
        "activity.download_blob".to_string(),
        Arc::new(download_blob::run) as StepHandler,
    );
    m.insert(
        "activity.transform_json".to_string(),
        Arc::new(transform_json::run) as StepHandler,
    );
    m.insert(
        "activity.http_call".to_string(),
        Arc::new(http_call::run) as StepHandler,
    );
    m.insert(
        "activity.publish_stream".to_string(),
        Arc::new(publish_stream::run) as StepHandler,
    );
    m.insert(
        "control.end".to_string(),
        Arc::new(|_ctx: &Context, _config: &serde_json::Value, _mode: &str| Ok(serde_json::json!({}))) as StepHandler,
    );
    m.insert(
        "control.fault_end".to_string(),
        Arc::new(fault_end::run) as StepHandler,
    );
    m
}

lazy_static::lazy_static! {
    static ref HANDLERS: std::collections::HashMap<String, StepHandler> = registry();
}

pub fn execute_step(
    step: &StepDef,
    ctx: &Context,
    mode: &str,
) -> Result<serde_json::Value, StepFault> {
    let handler = HANDLERS.get(step.kind.as_str()).ok_or_else(|| StepFault {
        code: "UNKNOWN_KIND".to_string(),
        message: format!("Unknown step kind: {}", step.kind),
    })?;
    handler(ctx, &step.config, mode)
}

```

## [27] workflow-mvp/backend/src/engine/steps/publish_stream.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/publish_stream.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\publish_stream.rs
// SIZE: 463 bytes
// MTIME: 2026-02-04T09:08:06.437774
// NOTE: Concatenated snapshot for review
//! Stub: registra "would publish" no trace; retorna patch.

use crate::engine::StepFault;
use crate::models::Context;

pub fn run(
    _ctx: &Context,
    config: &serde_json::Value,
    _mode: &str,
) -> Result<serde_json::Value, StepFault> {
    let stream = config.get("stream").and_then(|v| v.as_str()).unwrap_or("default");
    Ok(serde_json::json!({
        "published": true,
        "stream": stream,
        "simulated": true
    }))
}

```

## [28] workflow-mvp/backend/src/engine/steps/transform_json.rs

```rust
// FILE: workflow-mvp/backend/src/engine/steps/transform_json.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\steps\transform_json.rs
// SIZE: 4514 bytes
// MTIME: 2026-02-04T09:08:06.318480
// NOTE: Concatenated snapshot for review
//! Transforma JSON usando expressão simples (ex: $.amount ou path).

use crate::engine::StepFault;
use crate::models::Context;

fn get_from_path(value: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
    let path = path.trim_start_matches('$').trim_start_matches('.');
    if path.is_empty() {
        return Some(value.clone());
    }
    let mut current = value;
    for part in path.split('.') {
        current = current.get(part)?;
    }
    Some(current.clone())
}

fn eval_simple_expr(value: &serde_json::Value, expr: &str) -> Result<bool, StepFault> {
    // Support simple comparisons: $.amount > 100, $.x == "y", etc.
    let expr = expr.trim();
    let value_for_path = |path: &str| -> Option<serde_json::Value> {
        get_from_path(value, path)
    };

    if let Some(rest) = expr.strip_prefix("$.") {
        let (path, op_val) = rest.split_once(' ').ok_or_else(|| StepFault {
            code: "EXPR_PARSE".to_string(),
            message: "Expected: $.path op value".to_string(),
        })?;
        let (op, val_str) = op_val.split_at(2);
        let val_str = val_str.trim();
        let left = value_for_path(path).unwrap_or(serde_json::Value::Null);
        let right: serde_json::Value = if val_str.starts_with('"') && val_str.ends_with('"') {
            serde_json::Value::String(val_str[1..val_str.len() - 1].to_string())
        } else if let Ok(n) = val_str.parse::<i64>() {
            serde_json::Value::Number(n.into())
        } else if let Ok(n) = val_str.parse::<f64>() {
            serde_json::Number::from_f64(n).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        };

        let result = match op.trim() {
            ">" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l > r
            }
            ">=" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l >= r
            }
            "<" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l < r
            }
            "<=" => {
                let l = left.as_f64().or_else(|| left.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                let r = right.as_f64().or_else(|| right.as_i64().map(|x| x as f64)).unwrap_or(0.0);
                l <= r
            }
            "==" => left == right,
            "!=" => left != right,
            _ => false,
        };
        return Ok(result);
    }

    Err(StepFault {
        code: "EXPR_PARSE".to_string(),
        message: format!("Unsupported expression: {}", expr),
    })
}

pub fn run(
    ctx: &Context,
    config: &serde_json::Value,
    _mode: &str,
) -> Result<serde_json::Value, StepFault> {
    let expr = config
        .get("expr")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StepFault {
            code: "CONFIG".to_string(),
            message: "transform_json requires 'expr'".to_string(),
        })?;
    let input_path = config.get("inputPath").and_then(|v| v.as_str()).unwrap_or("$");
    let input = if input_path == "$" || input_path == "." {
        ctx.last.clone()
    } else {
        get_from_path(&ctx.vars, input_path).unwrap_or(ctx.last.clone())
    };
    if input.is_null() {
        return Ok(serde_json::json!({ "result": null }));
    }
    // Simple path extraction: $.field returns value at field
    if expr.starts_with('$') && !expr.contains(' ') {
        let value = get_from_path(&input, expr).unwrap_or(serde_json::Value::Null);
        return Ok(serde_json::json!({ "result": value }));
    }
    // Comparison expression for control.if usage
    if expr.contains('>') || expr.contains('<') || expr.contains("==") || expr.contains("!=") {
        let result = eval_simple_expr(&input, expr)?;
        return Ok(serde_json::json!({ "result": result }));
    }
    let value = get_from_path(&input, expr).unwrap_or(serde_json::Value::Null);
    Ok(serde_json::json!({ "result": value }))
}

```

## [29] workflow-mvp/backend/src/engine/trace.rs

```rust
// FILE: workflow-mvp/backend/src/engine/trace.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\trace.rs
// SIZE: 1930 bytes
// MTIME: 2026-02-04T09:04:56.808400
// NOTE: Concatenated snapshot for review
//! Coletor de eventos de execução (trace).

use super::types::TraceEvent;
use chrono::Utc;
use std::time::Instant;

pub struct TraceRecorder {
    events: Vec<TraceEvent>,
    step_start: Option<(String, Instant)>,
}

impl TraceRecorder {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            step_start: None,
        }
    }

    pub fn step_started(&mut self, step_id: &str) {
        let _ = self.step_start.take();
        self.step_start = Some((step_id.to_string(), Instant::now()));
        self.events.push(TraceEvent {
            step_id: step_id.to_string(),
            event_type: "step_started".to_string(),
            payload: serde_json::json!({}),
            ts: Utc::now().to_rfc3339(),
            duration_ms: None,
        });
    }

    pub fn step_succeeded(&mut self, step_id: &str, payload: serde_json::Value) {
        let duration_ms = self
            .step_start
            .take()
            .filter(|(s, _)| s == step_id)
            .map(|(_, t)| t.elapsed().as_millis() as u64);
        self.events.push(TraceEvent {
            step_id: step_id.to_string(),
            event_type: "step_succeeded".to_string(),
            payload,
            ts: Utc::now().to_rfc3339(),
            duration_ms,
        });
    }

    pub fn step_faulted(&mut self, step_id: &str, payload: serde_json::Value) {
        let duration_ms = self
            .step_start
            .take()
            .filter(|(s, _)| s == step_id)
            .map(|(_, t)| t.elapsed().as_millis() as u64);
        self.events.push(TraceEvent {
            step_id: step_id.to_string(),
            event_type: "step_faulted".to_string(),
            payload,
            ts: Utc::now().to_rfc3339(),
            duration_ms,
        });
    }

    pub fn into_events(self) -> Vec<TraceEvent> {
        self.events
    }
}

```

## [30] workflow-mvp/backend/src/engine/types.rs

```rust
// FILE: workflow-mvp/backend/src/engine/types.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\engine\types.rs
// SIZE: 1378 bytes
// MTIME: 2026-02-04T09:04:48.144827
// NOTE: Concatenated snapshot for review
//! Tipos do runtime: ExecutionResult, TraceEvent, StepFault.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionStatus {
    Succeeded,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub execution_id: String,
    pub status: ExecutionStatus,
    #[serde(rename = "finalContext")]
    pub final_context: serde_json::Value,
    pub trace: Vec<TraceEvent>,
    #[serde(default)]
    pub metrics: ExecutionMetrics,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    #[serde(rename = "stepsExecuted")]
    pub steps_executed: u32,
    #[serde(rename = "totalDurationMs")]
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    #[serde(rename = "stepId")]
    pub step_id: String,
    #[serde(rename = "eventType")]
    pub event_type: String, // "step_started" | "step_succeeded" | "step_faulted"
    #[serde(default)]
    pub payload: serde_json::Value,
    pub ts: String,
    #[serde(rename = "durationMs", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct StepFault {
    pub code: String,
    pub message: String,
}

```

## [31] workflow-mvp/backend/src/main.rs

```rust
// FILE: workflow-mvp/backend/src/main.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\main.rs
// SIZE: 765 bytes
// MTIME: 2026-02-04T09:11:05.175393
// NOTE: Concatenated snapshot for review
mod api;
mod compile;
mod engine;
mod models;
mod schema;
mod storage;
mod validate;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    storage::init_storage()?;
    let app = api::router();
    let addr: std::net::SocketAddr = "0.0.0.0:8080".parse()?;
    tracing::info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

```

## [32] workflow-mvp/backend/src/models.rs

```rust
// FILE: workflow-mvp/backend/src/models.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\models.rs
// SIZE: 2925 bytes
// MTIME: 2026-02-04T09:27:15.642977
// NOTE: Concatenated snapshot for review
//! DSL canônico e modelos de Graph (frontend).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow definition (DSL canônico versionado).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub version: u32,
    pub id: String,
    pub name: String,
    pub start: String,
    #[serde(rename = "steps")]
    pub steps: HashMap<String, StepDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDef {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<String>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub r#else: Option<String>,
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryConfig>,
    #[serde(rename = "onFault", skip_serializing_if = "Option::is_none")]
    pub on_fault: Option<String>,
}

impl StepDef {
    pub fn next_step(&self, branch: Option<&str>) -> Option<String> {
        match branch {
            Some("then") => self.then.clone(),
            Some("else") => self.r#else.clone(),
            _ => self.next.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    #[serde(rename = "maxAttempts")]
    pub max_attempts: u32,
    #[serde(rename = "backoffMs")]
    pub backoff_ms: u64,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff_ms: 200,
            multiplier: 2.0,
        }
    }
}

/// Context (variáveis do processo).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    #[serde(default)]
    pub vars: serde_json::Value,
    #[serde(default)]
    pub last: serde_json::Value,
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// Graph do frontend (React Flow).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub label: Option<String>,
}

/// Workflow salvo (graph + dsl).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWorkflow {
    pub id: String,
    pub name: String,
    pub graph: Graph,
    pub dsl: WorkflowDefinition,
    #[serde(rename = "saved_at")]
    pub saved_at: String,
}

```

## [33] workflow-mvp/backend/src/schema.rs

```rust
// FILE: workflow-mvp/backend/src/schema.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\schema.rs
// SIZE: 4305 bytes
// MTIME: 2026-02-04T09:05:57.343816
// NOTE: Concatenated snapshot for review
//! Schema dos step kinds para o Inspector (GET /connectors/schema).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorSchema {
    pub kind: String,
    pub fields: Vec<FieldSchema>,
    pub supports: SupportsSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
}

impl FieldSchema {
    pub fn new(name: &str, field_type: &str, required: bool) -> Self {
        Self {
            name: name.to_string(),
            field_type: field_type.to_string(),
            required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportsSchema {
    pub next: bool,
    pub then: bool,
    #[serde(rename = "else")]
    pub else_branch: bool,
    #[serde(rename = "onFault")]
    pub on_fault: bool,
}

pub fn connector_schemas() -> Vec<ConnectorSchema> {
    vec![
        ConnectorSchema {
            kind: "trigger.redis_stream".to_string(),
            fields: vec![
                FieldSchema::new("stream", "string", true),
                FieldSchema::new("group", "string", false),
            ],
            supports: SupportsSchema {
                next: true,
                then: false,
                else_branch: false,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "activity.download_blob".to_string(),
            fields: vec![
                FieldSchema::new("url", "string", true),
                FieldSchema::new("bucket", "string", false),
            ],
            supports: SupportsSchema {
                next: true,
                then: false,
                else_branch: false,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "activity.transform_json".to_string(),
            fields: vec![
                FieldSchema::new("expr", "string", true),
                FieldSchema::new("inputPath", "string", false),
            ],
            supports: SupportsSchema {
                next: true,
                then: false,
                else_branch: false,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "activity.http_call".to_string(),
            fields: vec![
                FieldSchema::new("url", "string", true),
                FieldSchema::new("method", "string", false),
                FieldSchema::new("body", "string", false),
            ],
            supports: SupportsSchema {
                next: true,
                then: false,
                else_branch: false,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "activity.publish_stream".to_string(),
            fields: vec![
                FieldSchema::new("stream", "string", true),
                FieldSchema::new("key", "string", false),
            ],
            supports: SupportsSchema {
                next: true,
                then: false,
                else_branch: false,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "control.if".to_string(),
            fields: vec![FieldSchema::new("expr", "string", true)],
            supports: SupportsSchema {
                next: false,
                then: true,
                else_branch: true,
                on_fault: true,
            },
        },
        ConnectorSchema {
            kind: "control.end".to_string(),
            fields: vec![],
            supports: SupportsSchema {
                next: false,
                then: false,
                else_branch: false,
                on_fault: false,
            },
        },
        ConnectorSchema {
            kind: "control.fault_end".to_string(),
            fields: vec![FieldSchema::new("reason", "string", false)],
            supports: SupportsSchema {
                next: false,
                then: false,
                else_branch: false,
                on_fault: false,
            },
        },
    ]
}

```

## [34] workflow-mvp/backend/src/storage.rs

```rust
// FILE: workflow-mvp/backend/src/storage.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\storage.rs
// SIZE: 2763 bytes
// MTIME: 2026-02-04T09:05:27.217034
// NOTE: Concatenated snapshot for review
//! Persistência de workflows e executions em arquivos.

use crate::engine::ExecutionResult;
use crate::models::{Graph, SavedWorkflow, WorkflowDefinition};
use anyhow::Result;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

fn data_dir() -> PathBuf {
    std::env::var("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./data"))
}

fn ensure_dir(p: &Path) -> Result<()> {
    if !p.exists() {
        std::fs::create_dir_all(p)?;
    }
    Ok(())
}

pub struct Storage {
    base: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let base = data_dir();
        ensure_dir(&base)?;
        ensure_dir(&base.join("workflows"))?;
        ensure_dir(&base.join("executions"))?;
        Ok(Self { base })
    }

    pub fn save_workflow(
        &self,
        id: Option<String>,
        name: String,
        graph: Graph,
        dsl: WorkflowDefinition,
    ) -> Result<SavedWorkflow> {
        let id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let saved_at = chrono::Utc::now().to_rfc3339();
        let w = SavedWorkflow {
            id: id.clone(),
            name,
            graph,
            dsl,
            saved_at,
        };
        let path = self.base.join("workflows").join(format!("{}.json", id));
        let j = serde_json::to_string_pretty(&w)?;
        std::fs::write(path, j)?;
        Ok(w)
    }

    pub fn get_workflow(&self, id: &str) -> Result<Option<SavedWorkflow>> {
        let path = self.base.join("workflows").join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let s = std::fs::read_to_string(path)?;
        let w: SavedWorkflow = serde_json::from_str(&s)?;
        Ok(Some(w))
    }

    pub fn save_execution(&self, execution_id: &str, result: &ExecutionResult) -> Result<()> {
        let path = self.base.join("executions").join(format!("{}.json", execution_id));
        let j = serde_json::to_string_pretty(result)?;
        std::fs::write(path, j)?;
        Ok(())
    }

    pub fn get_execution(&self, execution_id: &str) -> Result<Option<ExecutionResult>> {
        let path = self.base.join("executions").join(format!("{}.json", execution_id));
        if !path.exists() {
            return Ok(None);
        }
        let s = std::fs::read_to_string(path)?;
        let r: ExecutionResult = serde_json::from_str(&s)?;
        Ok(Some(r))
    }
}

static STORAGE: Lazy<Storage> = Lazy::new(|| {
    Storage::new().expect("storage init")
});

pub fn init_storage() -> Result<()> {
    Lazy::force(&STORAGE);
    Ok(())
}

pub fn storage() -> &'static Storage {
    &STORAGE
}

```

## [35] workflow-mvp/backend/src/validate.rs

```rust
// FILE: workflow-mvp/backend/src/validate.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\backend\src\validate.rs
// SIZE: 4750 bytes
// MTIME: 2026-02-04T09:16:11.011471
// NOTE: Concatenated snapshot for review
//! Validador: Graph ou DSL -> problems (erros e warnings).

use crate::compile;
use crate::models::{Graph, WorkflowDefinition};
use crate::schema::connector_schemas;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct ValidateRequest {
    pub graph: Option<Graph>,
    pub dsl: Option<WorkflowDefinition>,
}

#[derive(Debug, Serialize)]
pub struct ValidateResult {
    pub problems: Vec<ValidationProblem>,
}

#[derive(Debug, Serialize)]
pub struct ValidationProblem {
    pub code: String,
    pub message: String,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "stepId", skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    pub severity: String, // "error" | "warning"
}

pub fn validate_graph(graph: &Graph) -> ValidateResult {
    let compile_result = compile::compile(graph.clone(), None, None);
    let mut problems: Vec<ValidationProblem> = compile_result
        .problems
        .into_iter()
        .map(|p| ValidationProblem {
            code: p.code,
            message: p.message,
            node_id: None,
            step_id: None,
            severity: "error".to_string(),
        })
        .collect();

    if let Some(ref wf) = compile_result.workflow_definition {
        let dsl_problems = validate_dsl(wf);
        for p in dsl_problems {
            problems.push(p);
        }
    }
    ValidateResult { problems }
}

pub fn validate_dsl(wf: &WorkflowDefinition) -> Vec<ValidationProblem> {
    let mut problems = Vec::new();
    let schemas = connector_schemas();
    let schema_map: std::collections::HashMap<_, _> = schemas
        .iter()
        .map(|s| (s.kind.as_str(), s))
        .collect();

    let step_ids: HashSet<String> = wf.steps.keys().cloned().collect();

    for (step_id, step) in &wf.steps {
        let kind = step.kind.as_str();
        if let Some(schema) = schema_map.get(kind) {
            for field in &schema.fields {
                if field.required {
                    let config = &step.config;
                    let val = config.get(&field.name);
                    if val.is_none() || (val.is_some() && val.as_ref().and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(false)) {
                        problems.push(ValidationProblem {
                            code: "REQUIRED_FIELD".to_string(),
                            message: format!("Step '{}': required field '{}' missing or empty", step_id, field.name),
                            node_id: None,
                            step_id: Some(step_id.clone()),
                            severity: "error".to_string(),
                        });
                    }
                }
            }

            if kind == "control.if" {
                if step.then.is_none() || step.r#else.is_none() {
                    problems.push(ValidationProblem {
                        code: "IF_BRANCHES".to_string(),
                        message: "control.if must have 'then' and 'else' steps".to_string(),
                        node_id: None,
                        step_id: Some(step_id.clone()),
                        severity: "error".to_string(),
                    });
                }
            }
        } else {
            problems.push(ValidationProblem {
                code: "UNKNOWN_KIND".to_string(),
                message: format!("Unknown step kind: {}", kind),
                node_id: None,
                step_id: Some(step_id.clone()),
                severity: "error".to_string(),
            });
        }

        // Check references exist
        for ref_name in [&step.next, &step.then, &step.r#else, &step.on_fault] {
            if let Some(id) = ref_name {
                if !step_ids.contains(id.as_str()) {
                    problems.push(ValidationProblem {
                        code: "INVALID_REF".to_string(),
                        message: format!("Step '{}' references non-existent step '{}'", step_id, id),
                        node_id: None,
                        step_id: Some(step_id.clone()),
                        severity: "error".to_string(),
                    });
                }
            }
        }
    }

    if !wf.steps.contains_key(&wf.start) {
        problems.push(ValidationProblem {
            code: "INVALID_START".to_string(),
            message: format!("Start step '{}' not found in steps", wf.start),
            node_id: None,
            step_id: None,
            severity: "error".to_string(),
        });
    }

    problems
}

```

## [36] workflow-mvp/desktop/index.html

```html
// FILE: workflow-mvp/desktop/index.html
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\index.html
// SIZE: 397 bytes
// MTIME: 2026-02-04T09:19:34.247253
// NOTE: Concatenated snapshot for review
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Workflow Engine</title>
    <link rel="stylesheet" href="https://unpkg.com/98.css@0.1.20/build/98.css" />
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>

```

## [37] workflow-mvp/desktop/package-lock.json

```json
// FILE: workflow-mvp/desktop/package-lock.json
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\package-lock.json
// SIZE: 82366 bytes
// MTIME: 2026-02-04T09:30:16.473742
// NOTE: Concatenated snapshot for review
{
  "name": "workflow-desktop",
  "version": "0.1.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "workflow-desktop",
      "version": "0.1.0",
      "dependencies": {
        "@lumino/widgets": "^2.4.0",
        "@xyflow/react": "^12.3.6",
        "98.css": "^0.1.20",
        "react": "^18.3.1",
        "react-dom": "^18.3.1",
        "uuid": "^9.0.1"
      },
      "devDependencies": {
        "@tauri-apps/cli": "^2.0.0",
        "@types/react": "^18.3.12",
        "@types/react-dom": "^18.3.1",
        "@types/uuid": "^9.0.8",
        "@vitejs/plugin-react": "^4.3.3",
        "typescript": "~5.6.2",
        "vite": "^5.4.10"
      }
    },
    "node_modules/@babel/code-frame": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/code-frame/-/code-frame-7.29.0.tgz",
      "integrity": "sha512-9NhCeYjq9+3uxgdtp20LSiJXJvN0FeCtNGpJxuMFZ1Kv3cWUNb6DOhJwUvcVCzKGR66cw4njwM6hrJLqgOwbcw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/helper-validator-identifier": "^7.28.5",
        "js-tokens": "^4.0.0",
        "picocolors": "^1.1.1"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/compat-data": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/compat-data/-/compat-data-7.29.0.tgz",
      "integrity": "sha512-T1NCJqT/j9+cn8fvkt7jtwbLBfLC/1y1c7NtCeXFRgzGTsafi68MRv8yzkYSapBnFA6L3U2VSc02ciDzoAJhJg==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/core": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/core/-/core-7.29.0.tgz",
      "integrity": "sha512-CGOfOJqWjg2qW/Mb6zNsDm+u5vFQ8DxXfbM09z69p5Z6+mE1ikP2jUXw+j42Pf1XTYED2Rni5f95npYeuwMDQA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/code-frame": "^7.29.0",
        "@babel/generator": "^7.29.0",
        "@babel/helper-compilation-targets": "^7.28.6",
        "@babel/helper-module-transforms": "^7.28.6",
        "@babel/helpers": "^7.28.6",
        "@babel/parser": "^7.29.0",
        "@babel/template": "^7.28.6",
        "@babel/traverse": "^7.29.0",
        "@babel/types": "^7.29.0",
        "@jridgewell/remapping": "^2.3.5",
        "convert-source-map": "^2.0.0",
        "debug": "^4.1.0",
        "gensync": "^1.0.0-beta.2",
        "json5": "^2.2.3",
        "semver": "^6.3.1"
      },
      "engines": {
        "node": ">=6.9.0"
      },
      "funding": {
        "type": "opencollective",
        "url": "https://opencollective.com/babel"
      }
    },
    "node_modules/@babel/generator": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/generator/-/generator-7.29.0.tgz",
      "integrity": "sha512-vSH118/wwM/pLR38g/Sgk05sNtro6TlTJKuiMXDaZqPUfjTFcudpCOt00IhOfj+1BFAX+UFAlzCU+6WXr3GLFQ==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/parser": "^7.29.0",
        "@babel/types": "^7.29.0",
        "@jridgewell/gen-mapping": "^0.3.12",
        "@jridgewell/trace-mapping": "^0.3.28",
        "jsesc": "^3.0.2"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-compilation-targets": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/helper-compilation-targets/-/helper-compilation-targets-7.28.6.tgz",
      "integrity": "sha512-JYtls3hqi15fcx5GaSNL7SCTJ2MNmjrkHXg4FSpOA/grxK8KwyZ5bubHsCq8FXCkua6xhuaaBit+3b7+VZRfcA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/compat-data": "^7.28.6",
        "@babel/helper-validator-option": "^7.27.1",
        "browserslist": "^4.24.0",
        "lru-cache": "^5.1.1",
        "semver": "^6.3.1"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-globals": {
      "version": "7.28.0",
      "resolved": "https://registry.npmjs.org/@babel/helper-globals/-/helper-globals-7.28.0.tgz",
      "integrity": "sha512-+W6cISkXFa1jXsDEdYA8HeevQT/FULhxzR99pxphltZcVaugps53THCeiWA8SguxxpSp3gKPiuYfSWopkLQ4hw==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-module-imports": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/helper-module-imports/-/helper-module-imports-7.28.6.tgz",
      "integrity": "sha512-l5XkZK7r7wa9LucGw9LwZyyCUscb4x37JWTPz7swwFE/0FMQAGpiWUZn8u9DzkSBWEcK25jmvubfpw2dnAMdbw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/traverse": "^7.28.6",
        "@babel/types": "^7.28.6"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-module-transforms": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/helper-module-transforms/-/helper-module-transforms-7.28.6.tgz",
      "integrity": "sha512-67oXFAYr2cDLDVGLXTEABjdBJZ6drElUSI7WKp70NrpyISso3plG9SAGEF6y7zbha/wOzUByWWTJvEDVNIUGcA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/helper-module-imports": "^7.28.6",
        "@babel/helper-validator-identifier": "^7.28.5",
        "@babel/traverse": "^7.28.6"
      },
      "engines": {
        "node": ">=6.9.0"
      },
      "peerDependencies": {
        "@babel/core": "^7.0.0"
      }
    },
    "node_modules/@babel/helper-plugin-utils": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/helper-plugin-utils/-/helper-plugin-utils-7.28.6.tgz",
      "integrity": "sha512-S9gzZ/bz83GRysI7gAD4wPT/AI3uCnY+9xn+Mx/KPs2JwHJIz1W8PZkg2cqyt3RNOBM8ejcXhV6y8Og7ly/Dug==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-string-parser": {
      "version": "7.27.1",
      "resolved": "https://registry.npmjs.org/@babel/helper-string-parser/-/helper-string-parser-7.27.1.tgz",
      "integrity": "sha512-qMlSxKbpRlAridDExk92nSobyDdpPijUq2DW6oDnUqd0iOGxmQjyqhMIihI9+zv4LPyZdRje2cavWPbCbWm3eA==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-validator-identifier": {
      "version": "7.28.5",
      "resolved": "https://registry.npmjs.org/@babel/helper-validator-identifier/-/helper-validator-identifier-7.28.5.tgz",
      "integrity": "sha512-qSs4ifwzKJSV39ucNjsvc6WVHs6b7S03sOh2OcHF9UHfVPqWWALUsNUVzhSBiItjRZoLHx7nIarVjqKVusUZ1Q==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helper-validator-option": {
      "version": "7.27.1",
      "resolved": "https://registry.npmjs.org/@babel/helper-validator-option/-/helper-validator-option-7.27.1.tgz",
      "integrity": "sha512-YvjJow9FxbhFFKDSuFnVCe2WxXk1zWc22fFePVNEaWJEu8IrZVlda6N0uHwzZrUM1il7NC9Mlp4MaJYbYd9JSg==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/helpers": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/helpers/-/helpers-7.28.6.tgz",
      "integrity": "sha512-xOBvwq86HHdB7WUDTfKfT/Vuxh7gElQ+Sfti2Cy6yIWNW05P8iUslOVcZ4/sKbE+/jQaukQAdz/gf3724kYdqw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/template": "^7.28.6",
        "@babel/types": "^7.28.6"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/parser": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/parser/-/parser-7.29.0.tgz",
      "integrity": "sha512-IyDgFV5GeDUVX4YdF/3CPULtVGSXXMLh1xVIgdCgxApktqnQV0r7/8Nqthg+8YLGaAtdyIlo2qIdZrbCv4+7ww==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/types": "^7.29.0"
      },
      "bin": {
        "parser": "bin/babel-parser.js"
      },
      "engines": {
        "node": ">=6.0.0"
      }
    },
    "node_modules/@babel/plugin-transform-react-jsx-self": {
      "version": "7.27.1",
      "resolved": "https://registry.npmjs.org/@babel/plugin-transform-react-jsx-self/-/plugin-transform-react-jsx-self-7.27.1.tgz",
      "integrity": "sha512-6UzkCs+ejGdZ5mFFC/OCUrv028ab2fp1znZmCZjAOBKiBK2jXD1O+BPSfX8X2qjJ75fZBMSnQn3Rq2mrBJK2mw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/helper-plugin-utils": "^7.27.1"
      },
      "engines": {
        "node": ">=6.9.0"
      },
      "peerDependencies": {
        "@babel/core": "^7.0.0-0"
      }
    },
    "node_modules/@babel/plugin-transform-react-jsx-source": {
      "version": "7.27.1",
      "resolved": "https://registry.npmjs.org/@babel/plugin-transform-react-jsx-source/-/plugin-transform-react-jsx-source-7.27.1.tgz",
      "integrity": "sha512-zbwoTsBruTeKB9hSq73ha66iFeJHuaFkUbwvqElnygoNbj/jHRsSeokowZFN3CZ64IvEqcmmkVe89OPXc7ldAw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/helper-plugin-utils": "^7.27.1"
      },
      "engines": {
        "node": ">=6.9.0"
      },
      "peerDependencies": {
        "@babel/core": "^7.0.0-0"
      }
    },
    "node_modules/@babel/template": {
      "version": "7.28.6",
      "resolved": "https://registry.npmjs.org/@babel/template/-/template-7.28.6.tgz",
      "integrity": "sha512-YA6Ma2KsCdGb+WC6UpBVFJGXL58MDA6oyONbjyF/+5sBgxY/dwkhLogbMT2GXXyU84/IhRw/2D1Os1B/giz+BQ==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/code-frame": "^7.28.6",
        "@babel/parser": "^7.28.6",
        "@babel/types": "^7.28.6"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/traverse": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/traverse/-/traverse-7.29.0.tgz",
      "integrity": "sha512-4HPiQr0X7+waHfyXPZpWPfWL/J7dcN1mx9gL6WdQVMbPnF3+ZhSMs8tCxN7oHddJE9fhNE7+lxdnlyemKfJRuA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/code-frame": "^7.29.0",
        "@babel/generator": "^7.29.0",
        "@babel/helper-globals": "^7.28.0",
        "@babel/parser": "^7.29.0",
        "@babel/template": "^7.28.6",
        "@babel/types": "^7.29.0",
        "debug": "^4.3.1"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@babel/types": {
      "version": "7.29.0",
      "resolved": "https://registry.npmjs.org/@babel/types/-/types-7.29.0.tgz",
      "integrity": "sha512-LwdZHpScM4Qz8Xw2iKSzS+cfglZzJGvofQICy7W7v4caru4EaAmyUuO6BGrbyQ2mYV11W0U8j5mBhd14dd3B0A==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/helper-string-parser": "^7.27.1",
        "@babel/helper-validator-identifier": "^7.28.5"
      },
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/@esbuild/aix-ppc64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/aix-ppc64/-/aix-ppc64-0.21.5.tgz",
      "integrity": "sha512-1SDgH6ZSPTlggy1yI6+Dbkiz8xzpHJEVAlF/AM1tHPLsf5STom9rwtjE4hKAF20FfXXNTFqEYXyJNWh1GiZedQ==",
      "cpu": [
        "ppc64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "aix"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/android-arm": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/android-arm/-/android-arm-0.21.5.tgz",
      "integrity": "sha512-vCPvzSjpPHEi1siZdlvAlsPxXl7WbOVUBBAowWug4rJHb68Ox8KualB+1ocNvT5fjv6wpkX6o/iEpbDrf68zcg==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "android"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/android-arm64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/android-arm64/-/android-arm64-0.21.5.tgz",
      "integrity": "sha512-c0uX9VAUBQ7dTDCjq+wdyGLowMdtR/GoC2U5IYk/7D1H1JYC0qseD7+11iMP2mRLN9RcCMRcjC4YMclCzGwS/A==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "android"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/android-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/android-x64/-/android-x64-0.21.5.tgz",
      "integrity": "sha512-D7aPRUUNHRBwHxzxRvp856rjUHRFW1SdQATKXH2hqA0kAZb1hKmi02OpYRacl0TxIGz/ZmXWlbZgjwWYaCakTA==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "android"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/darwin-arm64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/darwin-arm64/-/darwin-arm64-0.21.5.tgz",
      "integrity": "sha512-DwqXqZyuk5AiWWf3UfLiRDJ5EDd49zg6O9wclZ7kUMv2WRFr4HKjXp/5t8JZ11QbQfUS6/cRCKGwYhtNAY88kQ==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "darwin"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/darwin-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/darwin-x64/-/darwin-x64-0.21.5.tgz",
      "integrity": "sha512-se/JjF8NlmKVG4kNIuyWMV/22ZaerB+qaSi5MdrXtd6R08kvs2qCN4C09miupktDitvh8jRFflwGFBQcxZRjbw==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "darwin"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/freebsd-arm64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/freebsd-arm64/-/freebsd-arm64-0.21.5.tgz",
      "integrity": "sha512-5JcRxxRDUJLX8JXp/wcBCy3pENnCgBR9bN6JsY4OmhfUtIHe3ZW0mawA7+RDAcMLrMIZaf03NlQiX9DGyB8h4g==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "freebsd"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/freebsd-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/freebsd-x64/-/freebsd-x64-0.21.5.tgz",
      "integrity": "sha512-J95kNBj1zkbMXtHVH29bBriQygMXqoVQOQYA+ISs0/2l3T9/kj42ow2mpqerRBxDJnmkUDCaQT/dfNXWX/ZZCQ==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "freebsd"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-arm": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-arm/-/linux-arm-0.21.5.tgz",
      "integrity": "sha512-bPb5AHZtbeNGjCKVZ9UGqGwo8EUu4cLq68E95A53KlxAPRmUyYv2D6F0uUI65XisGOL1hBP5mTronbgo+0bFcA==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-arm64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-arm64/-/linux-arm64-0.21.5.tgz",
      "integrity": "sha512-ibKvmyYzKsBeX8d8I7MH/TMfWDXBF3db4qM6sy+7re0YXya+K1cem3on9XgdT2EQGMu4hQyZhan7TeQ8XkGp4Q==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-ia32": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-ia32/-/linux-ia32-0.21.5.tgz",
      "integrity": "sha512-YvjXDqLRqPDl2dvRODYmmhz4rPeVKYvppfGYKSNGdyZkA01046pLWyRKKI3ax8fbJoK5QbxblURkwK/MWY18Tg==",
      "cpu": [
        "ia32"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-loong64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-loong64/-/linux-loong64-0.21.5.tgz",
      "integrity": "sha512-uHf1BmMG8qEvzdrzAqg2SIG/02+4/DHB6a9Kbya0XDvwDEKCoC8ZRWI5JJvNdUjtciBGFQ5PuBlpEOXQj+JQSg==",
      "cpu": [
        "loong64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-mips64el": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-mips64el/-/linux-mips64el-0.21.5.tgz",
      "integrity": "sha512-IajOmO+KJK23bj52dFSNCMsz1QP1DqM6cwLUv3W1QwyxkyIWecfafnI555fvSGqEKwjMXVLokcV5ygHW5b3Jbg==",
      "cpu": [
        "mips64el"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-ppc64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-ppc64/-/linux-ppc64-0.21.5.tgz",
      "integrity": "sha512-1hHV/Z4OEfMwpLO8rp7CvlhBDnjsC3CttJXIhBi+5Aj5r+MBvy4egg7wCbe//hSsT+RvDAG7s81tAvpL2XAE4w==",
      "cpu": [
        "ppc64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-riscv64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-riscv64/-/linux-riscv64-0.21.5.tgz",
      "integrity": "sha512-2HdXDMd9GMgTGrPWnJzP2ALSokE/0O5HhTUvWIbD3YdjME8JwvSCnNGBnTThKGEB91OZhzrJ4qIIxk/SBmyDDA==",
      "cpu": [
        "riscv64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-s390x": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-s390x/-/linux-s390x-0.21.5.tgz",
      "integrity": "sha512-zus5sxzqBJD3eXxwvjN1yQkRepANgxE9lgOW2qLnmr8ikMTphkjgXu1HR01K4FJg8h1kEEDAqDcZQtbrRnB41A==",
      "cpu": [
        "s390x"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/linux-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.21.5.tgz",
      "integrity": "sha512-1rYdTpyv03iycF1+BhzrzQJCdOuAOtaqHTWJZCWvijKD2N5Xu0TtVC8/+1faWqcP9iBCWOmjmhoH94dH82BxPQ==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/netbsd-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/netbsd-x64/-/netbsd-x64-0.21.5.tgz",
      "integrity": "sha512-Woi2MXzXjMULccIwMnLciyZH4nCIMpWQAs049KEeMvOcNADVxo0UBIQPfSmxB3CWKedngg7sWZdLvLczpe0tLg==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "netbsd"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/openbsd-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/openbsd-x64/-/openbsd-x64-0.21.5.tgz",
      "integrity": "sha512-HLNNw99xsvx12lFBUwoT8EVCsSvRNDVxNpjZ7bPn947b8gJPzeHWyNVhFsaerc0n3TsbOINvRP2byTZ5LKezow==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "openbsd"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/sunos-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/sunos-x64/-/sunos-x64-0.21.5.tgz",
      "integrity": "sha512-6+gjmFpfy0BHU5Tpptkuh8+uw3mnrvgs+dSPQXQOv3ekbordwnzTVEb4qnIvQcYXq6gzkyTnoZ9dZG+D4garKg==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "sunos"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/win32-arm64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/win32-arm64/-/win32-arm64-0.21.5.tgz",
      "integrity": "sha512-Z0gOTd75VvXqyq7nsl93zwahcTROgqvuAcYDUr+vOv8uHhNSKROyU961kgtCD1e95IqPKSQKH7tBTslnS3tA8A==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/win32-ia32": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/win32-ia32/-/win32-ia32-0.21.5.tgz",
      "integrity": "sha512-SWXFF1CL2RVNMaVs+BBClwtfZSvDgtL//G/smwAc5oVK/UPu2Gu9tIaRgFmYFFKrmg3SyAjSrElf0TiJ1v8fYA==",
      "cpu": [
        "ia32"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@esbuild/win32-x64": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/@esbuild/win32-x64/-/win32-x64-0.21.5.tgz",
      "integrity": "sha512-tQd/1efJuzPC6rCFwEvLtci/xNFcTZknmXs98FYDfGE4wP9ClFV98nyKrzJKVPMhdDnjzLhdUyMX4PsQAPjwIw==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/@jridgewell/gen-mapping": {
      "version": "0.3.13",
      "resolved": "https://registry.npmjs.org/@jridgewell/gen-mapping/-/gen-mapping-0.3.13.tgz",
      "integrity": "sha512-2kkt/7niJ6MgEPxF0bYdQ6etZaA+fQvDcLKckhy1yIQOzaoKjBBjSj63/aLVjYE3qhRt5dvM+uUyfCg6UKCBbA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@jridgewell/sourcemap-codec": "^1.5.0",
        "@jridgewell/trace-mapping": "^0.3.24"
      }
    },
    "node_modules/@jridgewell/remapping": {
      "version": "2.3.5",
      "resolved": "https://registry.npmjs.org/@jridgewell/remapping/-/remapping-2.3.5.tgz",
      "integrity": "sha512-LI9u/+laYG4Ds1TDKSJW2YPrIlcVYOwi2fUC6xB43lueCjgxV4lffOCZCtYFiH6TNOX+tQKXx97T4IKHbhyHEQ==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@jridgewell/gen-mapping": "^0.3.5",
        "@jridgewell/trace-mapping": "^0.3.24"
      }
    },
    "node_modules/@jridgewell/resolve-uri": {
      "version": "3.1.2",
      "resolved": "https://registry.npmjs.org/@jridgewell/resolve-uri/-/resolve-uri-3.1.2.tgz",
      "integrity": "sha512-bRISgCIjP20/tbWSPWMEi54QVPRZExkuD9lJL+UIxUKtwVJA8wW1Trb1jMs1RFXo1CBTNZ/5hpC9QvmKWdopKw==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.0.0"
      }
    },
    "node_modules/@jridgewell/sourcemap-codec": {
      "version": "1.5.5",
      "resolved": "https://registry.npmjs.org/@jridgewell/sourcemap-codec/-/sourcemap-codec-1.5.5.tgz",
      "integrity": "sha512-cYQ9310grqxueWbl+WuIUIaiUaDcj7WOq5fVhEljNVgRfOUhY9fy2zTvfoqWsnebh8Sl70VScFbICvJnLKB0Og==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/@jridgewell/trace-mapping": {
      "version": "0.3.31",
      "resolved": "https://registry.npmjs.org/@jridgewell/trace-mapping/-/trace-mapping-0.3.31.tgz",
      "integrity": "sha512-zzNR+SdQSDJzc8joaeP8QQoCQr8NuYx2dIIytl1QeBEZHJ9uW6hebsrYgbz8hJwUQao3TWCMtmfV8Nu1twOLAw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@jridgewell/resolve-uri": "^3.1.0",
        "@jridgewell/sourcemap-codec": "^1.4.14"
      }
    },
    "node_modules/@lumino/algorithm": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/algorithm/-/algorithm-2.0.4.tgz",
      "integrity": "sha512-gddBhESPqu25KWLeAK9Kz8tS9Ph7P45i0CNG7Ia4XMhK9PHLtTsBdJTC9jP+MqhbzC8zDT/4ekvYRV9ojRPj7Q==",
      "license": "BSD-3-Clause"
    },
    "node_modules/@lumino/collections": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/collections/-/collections-2.0.4.tgz",
      "integrity": "sha512-D/Py9L5HET6+XUYGxFqDEEth4B65X2c7B/GQVRR8q5Fl7EArVL6e98ZXw8BMkuPcTNa0zlENpCKXzlcoJZxXgQ==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4"
      }
    },
    "node_modules/@lumino/commands": {
      "version": "2.3.3",
      "resolved": "https://registry.npmjs.org/@lumino/commands/-/commands-2.3.3.tgz",
      "integrity": "sha512-7Ci0QdFzt4NKFMhULr19sJPpOLHJw/oYlq6Pb0/Kq1s05+cIoLimr5wiyjkbAlNoGO/8A8SEBGHy3uctZz6G3A==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4",
        "@lumino/coreutils": "^2.2.2",
        "@lumino/disposable": "^2.1.5",
        "@lumino/domutils": "^2.0.4",
        "@lumino/keyboard": "^2.0.4",
        "@lumino/signaling": "^2.1.5",
        "@lumino/virtualdom": "^2.0.4"
      }
    },
    "node_modules/@lumino/coreutils": {
      "version": "2.2.2",
      "resolved": "https://registry.npmjs.org/@lumino/coreutils/-/coreutils-2.2.2.tgz",
      "integrity": "sha512-zaKJaK7rawPATn2BGHkbMrR6oK3s9PxNe9KreLwWF2dB4ZBHDiEmNLRyHRorfJ7XqVOEXAsAAj0jFn+qJPC/4Q==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4"
      }
    },
    "node_modules/@lumino/disposable": {
      "version": "2.1.5",
      "resolved": "https://registry.npmjs.org/@lumino/disposable/-/disposable-2.1.5.tgz",
      "integrity": "sha512-hO9AkJK0oEGzxopuxI8LaZqwzSNwXJTGCdr5K4gh6al+zxpN7rOCh6Aq3zDxkIHJU4zybxv8r02ardx9XJsG3A==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/signaling": "^2.1.5"
      }
    },
    "node_modules/@lumino/domutils": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/domutils/-/domutils-2.0.4.tgz",
      "integrity": "sha512-naYGUQn3e0CLtz/tjKOZP8SOBg0SW7EguhkxLpNUXlVUvx7rVsfr0VI22FVL+jgI0FbxXpEkxpSMxtK73jxJAg==",
      "license": "BSD-3-Clause"
    },
    "node_modules/@lumino/dragdrop": {
      "version": "2.1.7",
      "resolved": "https://registry.npmjs.org/@lumino/dragdrop/-/dragdrop-2.1.7.tgz",
      "integrity": "sha512-oa5EtBC37EiiJsuIFVcK1DywpEb4dnQCMzWnymvjlBXG3/fAIC+65Q/iLoNUWkMz1pc3ET2SZWDporlznxlEbw==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/coreutils": "^2.2.2",
        "@lumino/disposable": "^2.1.5"
      }
    },
    "node_modules/@lumino/keyboard": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/keyboard/-/keyboard-2.0.4.tgz",
      "integrity": "sha512-kIVkdSz8F5wtZr8hZp0CMX+E0eMCOnFH6XCT7j2UBQ80ERJHFy0eX+IbNo3dtRQ7+CcDhBV4hQquFNFa+/04QQ==",
      "license": "BSD-3-Clause"
    },
    "node_modules/@lumino/messaging": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/messaging/-/messaging-2.0.4.tgz",
      "integrity": "sha512-NbZnchAPOciSe9Qn/g6EzG0LRaw7bygFIXbCD440ZhzvugdBeAerwYhrA795jkXPNrrl3olp5AlO0cBB/XZNtg==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4",
        "@lumino/collections": "^2.0.4"
      }
    },
    "node_modules/@lumino/properties": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/properties/-/properties-2.0.4.tgz",
      "integrity": "sha512-XsL2qLZk+1FbfuTrkyjciI8PMDw3YcaBkqVQ+iv7OOJf9bUlrmTpCMY0Hu5d3hV2W3TWlRsdbvRRLEBJSKv0iA==",
      "license": "BSD-3-Clause"
    },
    "node_modules/@lumino/signaling": {
      "version": "2.1.5",
      "resolved": "https://registry.npmjs.org/@lumino/signaling/-/signaling-2.1.5.tgz",
      "integrity": "sha512-Wkx6WR45ynmKBlW0GBEoh4xk9+QluKr1JHuMftqcStBHSQBCnN54UKRRDbySXHGRhhx6p4neu7sGomgQSlQK8w==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4",
        "@lumino/coreutils": "^2.2.2"
      }
    },
    "node_modules/@lumino/virtualdom": {
      "version": "2.0.4",
      "resolved": "https://registry.npmjs.org/@lumino/virtualdom/-/virtualdom-2.0.4.tgz",
      "integrity": "sha512-7MFthA9KUsqZTGm/D98FZt1QupjIGyd3XyB4SIugn6DQAqhjBiyykCZydnRq3qmuMHybQel33dNIbHpzyNyQwA==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4"
      }
    },
    "node_modules/@lumino/widgets": {
      "version": "2.7.4",
      "resolved": "https://registry.npmjs.org/@lumino/widgets/-/widgets-2.7.4.tgz",
      "integrity": "sha512-mZTMRBoUe02fecfWCmZLrdnZkA9MouygTgxftw08lIVFL/rJzCFjYB/RfaeindBlKzsprRWlQZfc7z8YH14n/w==",
      "license": "BSD-3-Clause",
      "dependencies": {
        "@lumino/algorithm": "^2.0.4",
        "@lumino/commands": "^2.3.3",
        "@lumino/coreutils": "^2.2.2",
        "@lumino/disposable": "^2.1.5",
        "@lumino/domutils": "^2.0.4",
        "@lumino/dragdrop": "^2.1.7",
        "@lumino/keyboard": "^2.0.4",
        "@lumino/messaging": "^2.0.4",
        "@lumino/properties": "^2.0.4",
        "@lumino/signaling": "^2.1.5",
        "@lumino/virtualdom": "^2.0.4"
      }
    },
    "node_modules/@rolldown/pluginutils": {
      "version": "1.0.0-beta.27",
      "resolved": "https://registry.npmjs.org/@rolldown/pluginutils/-/pluginutils-1.0.0-beta.27.tgz",
      "integrity": "sha512-+d0F4MKMCbeVUJwG96uQ4SgAznZNSq93I3V+9NHA4OpvqG8mRCpGdKmK8l/dl02h2CCDHwW2FqilnTyDcAnqjA==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/@rollup/rollup-android-arm-eabi": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-android-arm-eabi/-/rollup-android-arm-eabi-4.57.1.tgz",
      "integrity": "sha512-A6ehUVSiSaaliTxai040ZpZ2zTevHYbvu/lDoeAteHI8QnaosIzm4qwtezfRg1jOYaUmnzLX1AOD6Z+UJjtifg==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "android"
      ]
    },
    "node_modules/@rollup/rollup-android-arm64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-android-arm64/-/rollup-android-arm64-4.57.1.tgz",
      "integrity": "sha512-dQaAddCY9YgkFHZcFNS/606Exo8vcLHwArFZ7vxXq4rigo2bb494/xKMMwRRQW6ug7Js6yXmBZhSBRuBvCCQ3w==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "android"
      ]
    },
    "node_modules/@rollup/rollup-darwin-arm64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-darwin-arm64/-/rollup-darwin-arm64-4.57.1.tgz",
      "integrity": "sha512-crNPrwJOrRxagUYeMn/DZwqN88SDmwaJ8Cvi/TN1HnWBU7GwknckyosC2gd0IqYRsHDEnXf328o9/HC6OkPgOg==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "darwin"
      ]
    },
    "node_modules/@rollup/rollup-darwin-x64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-darwin-x64/-/rollup-darwin-x64-4.57.1.tgz",
      "integrity": "sha512-Ji8g8ChVbKrhFtig5QBV7iMaJrGtpHelkB3lsaKzadFBe58gmjfGXAOfI5FV0lYMH8wiqsxKQ1C9B0YTRXVy4w==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "darwin"
      ]
    },
    "node_modules/@rollup/rollup-freebsd-arm64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-freebsd-arm64/-/rollup-freebsd-arm64-4.57.1.tgz",
      "integrity": "sha512-R+/WwhsjmwodAcz65guCGFRkMb4gKWTcIeLy60JJQbXrJ97BOXHxnkPFrP+YwFlaS0m+uWJTstrUA9o+UchFug==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "freebsd"
      ]
    },
    "node_modules/@rollup/rollup-freebsd-x64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-freebsd-x64/-/rollup-freebsd-x64-4.57.1.tgz",
      "integrity": "sha512-IEQTCHeiTOnAUC3IDQdzRAGj3jOAYNr9kBguI7MQAAZK3caezRrg0GxAb6Hchg4lxdZEI5Oq3iov/w/hnFWY9Q==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "freebsd"
      ]
    },
    "node_modules/@rollup/rollup-linux-arm-gnueabihf": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-arm-gnueabihf/-/rollup-linux-arm-gnueabihf-4.57.1.tgz",
      "integrity": "sha512-F8sWbhZ7tyuEfsmOxwc2giKDQzN3+kuBLPwwZGyVkLlKGdV1nvnNwYD0fKQ8+XS6hp9nY7B+ZeK01EBUE7aHaw==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-arm-musleabihf": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-arm-musleabihf/-/rollup-linux-arm-musleabihf-4.57.1.tgz",
      "integrity": "sha512-rGfNUfn0GIeXtBP1wL5MnzSj98+PZe/AXaGBCRmT0ts80lU5CATYGxXukeTX39XBKsxzFpEeK+Mrp9faXOlmrw==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-arm64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-arm64-gnu/-/rollup-linux-arm64-gnu-4.57.1.tgz",
      "integrity": "sha512-MMtej3YHWeg/0klK2Qodf3yrNzz6CGjo2UntLvk2RSPlhzgLvYEB3frRvbEF2wRKh1Z2fDIg9KRPe1fawv7C+g==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-arm64-musl": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-arm64-musl/-/rollup-linux-arm64-musl-4.57.1.tgz",
      "integrity": "sha512-1a/qhaaOXhqXGpMFMET9VqwZakkljWHLmZOX48R0I/YLbhdxr1m4gtG1Hq7++VhVUmf+L3sTAf9op4JlhQ5u1Q==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-loong64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-loong64-gnu/-/rollup-linux-loong64-gnu-4.57.1.tgz",
      "integrity": "sha512-QWO6RQTZ/cqYtJMtxhkRkidoNGXc7ERPbZN7dVW5SdURuLeVU7lwKMpo18XdcmpWYd0qsP1bwKPf7DNSUinhvA==",
      "cpu": [
        "loong64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-loong64-musl": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-loong64-musl/-/rollup-linux-loong64-musl-4.57.1.tgz",
      "integrity": "sha512-xpObYIf+8gprgWaPP32xiN5RVTi/s5FCR+XMXSKmhfoJjrpRAjCuuqQXyxUa/eJTdAE6eJ+KDKaoEqjZQxh3Gw==",
      "cpu": [
        "loong64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-ppc64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-ppc64-gnu/-/rollup-linux-ppc64-gnu-4.57.1.tgz",
      "integrity": "sha512-4BrCgrpZo4hvzMDKRqEaW1zeecScDCR+2nZ86ATLhAoJ5FQ+lbHVD3ttKe74/c7tNT9c6F2viwB3ufwp01Oh2w==",
      "cpu": [
        "ppc64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-ppc64-musl": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-ppc64-musl/-/rollup-linux-ppc64-musl-4.57.1.tgz",
      "integrity": "sha512-NOlUuzesGauESAyEYFSe3QTUguL+lvrN1HtwEEsU2rOwdUDeTMJdO5dUYl/2hKf9jWydJrO9OL/XSSf65R5+Xw==",
      "cpu": [
        "ppc64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-riscv64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-riscv64-gnu/-/rollup-linux-riscv64-gnu-4.57.1.tgz",
      "integrity": "sha512-ptA88htVp0AwUUqhVghwDIKlvJMD/fmL/wrQj99PRHFRAG6Z5nbWoWG4o81Nt9FT+IuqUQi+L31ZKAFeJ5Is+A==",
      "cpu": [
        "riscv64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-riscv64-musl": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-riscv64-musl/-/rollup-linux-riscv64-musl-4.57.1.tgz",
      "integrity": "sha512-S51t7aMMTNdmAMPpBg7OOsTdn4tySRQvklmL3RpDRyknk87+Sp3xaumlatU+ppQ+5raY7sSTcC2beGgvhENfuw==",
      "cpu": [
        "riscv64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-s390x-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-s390x-gnu/-/rollup-linux-s390x-gnu-4.57.1.tgz",
      "integrity": "sha512-Bl00OFnVFkL82FHbEqy3k5CUCKH6OEJL54KCyx2oqsmZnFTR8IoNqBF+mjQVcRCT5sB6yOvK8A37LNm/kPJiZg==",
      "cpu": [
        "s390x"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-x64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-x64-gnu/-/rollup-linux-x64-gnu-4.57.1.tgz",
      "integrity": "sha512-ABca4ceT4N+Tv/GtotnWAeXZUZuM/9AQyCyKYyKnpk4yoA7QIAuBt6Hkgpw8kActYlew2mvckXkvx0FfoInnLg==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-linux-x64-musl": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-linux-x64-musl/-/rollup-linux-x64-musl-4.57.1.tgz",
      "integrity": "sha512-HFps0JeGtuOR2convgRRkHCekD7j+gdAuXM+/i6kGzQtFhlCtQkpwtNzkNj6QhCDp7DRJ7+qC/1Vg2jt5iSOFw==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "linux"
      ]
    },
    "node_modules/@rollup/rollup-openbsd-x64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-openbsd-x64/-/rollup-openbsd-x64-4.57.1.tgz",
      "integrity": "sha512-H+hXEv9gdVQuDTgnqD+SQffoWoc0Of59AStSzTEj/feWTBAnSfSD3+Dql1ZruJQxmykT/JVY0dE8Ka7z0DH1hw==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "openbsd"
      ]
    },
    "node_modules/@rollup/rollup-openharmony-arm64": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-openharmony-arm64/-/rollup-openharmony-arm64-4.57.1.tgz",
      "integrity": "sha512-4wYoDpNg6o/oPximyc/NG+mYUejZrCU2q+2w6YZqrAs2UcNUChIZXjtafAiiZSUc7On8v5NyNj34Kzj/Ltk6dQ==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "openharmony"
      ]
    },
    "node_modules/@rollup/rollup-win32-arm64-msvc": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-win32-arm64-msvc/-/rollup-win32-arm64-msvc-4.57.1.tgz",
      "integrity": "sha512-O54mtsV/6LW3P8qdTcamQmuC990HDfR71lo44oZMZlXU4tzLrbvTii87Ni9opq60ds0YzuAlEr/GNwuNluZyMQ==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ]
    },
    "node_modules/@rollup/rollup-win32-ia32-msvc": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-win32-ia32-msvc/-/rollup-win32-ia32-msvc-4.57.1.tgz",
      "integrity": "sha512-P3dLS+IerxCT/7D2q2FYcRdWRl22dNbrbBEtxdWhXrfIMPP9lQhb5h4Du04mdl5Woq05jVCDPCMF7Ub0NAjIew==",
      "cpu": [
        "ia32"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ]
    },
    "node_modules/@rollup/rollup-win32-x64-gnu": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-win32-x64-gnu/-/rollup-win32-x64-gnu-4.57.1.tgz",
      "integrity": "sha512-VMBH2eOOaKGtIJYleXsi2B8CPVADrh+TyNxJ4mWPnKfLB/DBUmzW+5m1xUrcwWoMfSLagIRpjUFeW5CO5hyciQ==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ]
    },
    "node_modules/@rollup/rollup-win32-x64-msvc": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/@rollup/rollup-win32-x64-msvc/-/rollup-win32-x64-msvc-4.57.1.tgz",
      "integrity": "sha512-mxRFDdHIWRxg3UfIIAwCm6NzvxG0jDX/wBN6KsQFTvKFqqg9vTrWUE68qEjHt19A5wwx5X5aUi2zuZT7YR0jrA==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "win32"
      ]
    },
    "node_modules/@tauri-apps/cli": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli/-/cli-2.10.0.tgz",
      "integrity": "sha512-ZwT0T+7bw4+DPCSWzmviwq5XbXlM0cNoleDKOYPFYqcZqeKY31KlpoMW/MOON/tOFBPgi31a2v3w9gliqwL2+Q==",
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "bin": {
        "tauri": "tauri.js"
      },
      "engines": {
        "node": ">= 10"
      },
      "funding": {
        "type": "opencollective",
        "url": "https://opencollective.com/tauri"
      },
      "optionalDependencies": {
        "@tauri-apps/cli-darwin-arm64": "2.10.0",
        "@tauri-apps/cli-darwin-x64": "2.10.0",
        "@tauri-apps/cli-linux-arm-gnueabihf": "2.10.0",
        "@tauri-apps/cli-linux-arm64-gnu": "2.10.0",
        "@tauri-apps/cli-linux-arm64-musl": "2.10.0",
        "@tauri-apps/cli-linux-riscv64-gnu": "2.10.0",
        "@tauri-apps/cli-linux-x64-gnu": "2.10.0",
        "@tauri-apps/cli-linux-x64-musl": "2.10.0",
        "@tauri-apps/cli-win32-arm64-msvc": "2.10.0",
        "@tauri-apps/cli-win32-ia32-msvc": "2.10.0",
        "@tauri-apps/cli-win32-x64-msvc": "2.10.0"
      }
    },
    "node_modules/@tauri-apps/cli-darwin-arm64": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-darwin-arm64/-/cli-darwin-arm64-2.10.0.tgz",
      "integrity": "sha512-avqHD4HRjrMamE/7R/kzJPcAJnZs0IIS+1nkDP5b+TNBn3py7N2aIo9LIpy+VQq0AkN8G5dDpZtOOBkmWt/zjA==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "darwin"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-darwin-x64": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-darwin-x64/-/cli-darwin-x64-2.10.0.tgz",
      "integrity": "sha512-keDmlvJRStzVFjZTd0xYkBONLtgBC9eMTpmXnBXzsHuawV2q9PvDo2x6D5mhuoMVrJ9QWjgaPKBBCFks4dK71Q==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "darwin"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-arm-gnueabihf": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-arm-gnueabihf/-/cli-linux-arm-gnueabihf-2.10.0.tgz",
      "integrity": "sha512-e5u0VfLZsMAC9iHaOEANumgl6lfnJx0Dtjkd8IJpysZ8jp0tJ6wrIkto2OzQgzcYyRCKgX72aKE0PFgZputA8g==",
      "cpu": [
        "arm"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-arm64-gnu": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-arm64-gnu/-/cli-linux-arm64-gnu-2.10.0.tgz",
      "integrity": "sha512-YrYYk2dfmBs5m+OIMCrb+JH/oo+4FtlpcrTCgiFYc7vcs6m3QDd1TTyWu0u01ewsCtK2kOdluhr/zKku+KP7HA==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-arm64-musl": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-arm64-musl/-/cli-linux-arm64-musl-2.10.0.tgz",
      "integrity": "sha512-GUoPdVJmrJRIXFfW3Rkt+eGK9ygOdyISACZfC/bCSfOnGt8kNdQIQr5WRH9QUaTVFIwxMlQyV3m+yXYP+xhSVA==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-riscv64-gnu": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-riscv64-gnu/-/cli-linux-riscv64-gnu-2.10.0.tgz",
      "integrity": "sha512-JO7s3TlSxshwsoKNCDkyvsx5gw2QAs/Y2GbR5UE2d5kkU138ATKoPOtxn8G1fFT1aDW4LH0rYAAfBpGkDyJJnw==",
      "cpu": [
        "riscv64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-x64-gnu": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-x64-gnu/-/cli-linux-x64-gnu-2.10.0.tgz",
      "integrity": "sha512-Uvh4SUUp4A6DVRSMWjelww0GnZI3PlVy7VS+DRF5napKuIehVjGl9XD0uKoCoxwAQBLctvipyEK+pDXpJeoHng==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-linux-x64-musl": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-linux-x64-musl/-/cli-linux-x64-musl-2.10.0.tgz",
      "integrity": "sha512-AP0KRK6bJuTpQ8kMNWvhIpKUkQJfcPFeba7QshOQZjJ8wOS6emwTN4K5g/d3AbCMo0RRdnZWwu67MlmtJyxC1Q==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "linux"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-win32-arm64-msvc": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-win32-arm64-msvc/-/cli-win32-arm64-msvc-2.10.0.tgz",
      "integrity": "sha512-97DXVU3dJystrq7W41IX+82JEorLNY+3+ECYxvXWqkq7DBN6FsA08x/EFGE8N/b0LTOui9X2dvpGGoeZKKV08g==",
      "cpu": [
        "arm64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-win32-ia32-msvc": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-win32-ia32-msvc/-/cli-win32-ia32-msvc-2.10.0.tgz",
      "integrity": "sha512-EHyQ1iwrWy1CwMalEm9z2a6L5isQ121pe7FcA2xe4VWMJp+GHSDDGvbTv/OPdkt2Lyr7DAZBpZHM6nvlHXEc4A==",
      "cpu": [
        "ia32"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@tauri-apps/cli-win32-x64-msvc": {
      "version": "2.10.0",
      "resolved": "https://registry.npmjs.org/@tauri-apps/cli-win32-x64-msvc/-/cli-win32-x64-msvc-2.10.0.tgz",
      "integrity": "sha512-NTpyQxkpzGmU6ceWBTY2xRIEaS0ZLbVx1HE1zTA3TY/pV3+cPoPPOs+7YScr4IMzXMtOw7tLw5LEXo5oIG3qaQ==",
      "cpu": [
        "x64"
      ],
      "dev": true,
      "license": "Apache-2.0 OR MIT",
      "optional": true,
      "os": [
        "win32"
      ],
      "engines": {
        "node": ">= 10"
      }
    },
    "node_modules/@types/babel__core": {
      "version": "7.20.5",
      "resolved": "https://registry.npmjs.org/@types/babel__core/-/babel__core-7.20.5.tgz",
      "integrity": "sha512-qoQprZvz5wQFJwMDqeseRXWv3rqMvhgpbXFfVyWhbx9X47POIA6i/+dXefEmZKoAgOaTdaIgNSMqMIU61yRyzA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/parser": "^7.20.7",
        "@babel/types": "^7.20.7",
        "@types/babel__generator": "*",
        "@types/babel__template": "*",
        "@types/babel__traverse": "*"
      }
    },
    "node_modules/@types/babel__generator": {
      "version": "7.27.0",
      "resolved": "https://registry.npmjs.org/@types/babel__generator/-/babel__generator-7.27.0.tgz",
      "integrity": "sha512-ufFd2Xi92OAVPYsy+P4n7/U7e68fex0+Ee8gSG9KX7eo084CWiQ4sdxktvdl0bOPupXtVJPY19zk6EwWqUQ8lg==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/types": "^7.0.0"
      }
    },
    "node_modules/@types/babel__template": {
      "version": "7.4.4",
      "resolved": "https://registry.npmjs.org/@types/babel__template/-/babel__template-7.4.4.tgz",
      "integrity": "sha512-h/NUaSyG5EyxBIp8YRxo4RMe2/qQgvyowRwVMzhYhBCONbW8PUsg4lkFMrhgZhUe5z3L3MiLDuvyJ/CaPa2A8A==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/parser": "^7.1.0",
        "@babel/types": "^7.0.0"
      }
    },
    "node_modules/@types/babel__traverse": {
      "version": "7.28.0",
      "resolved": "https://registry.npmjs.org/@types/babel__traverse/-/babel__traverse-7.28.0.tgz",
      "integrity": "sha512-8PvcXf70gTDZBgt9ptxJ8elBeBjcLOAcOtoO/mPJjtji1+CdGbHgm77om1GrsPxsiE+uXIpNSK64UYaIwQXd4Q==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/types": "^7.28.2"
      }
    },
    "node_modules/@types/d3-color": {
      "version": "3.1.3",
      "resolved": "https://registry.npmjs.org/@types/d3-color/-/d3-color-3.1.3.tgz",
      "integrity": "sha512-iO90scth9WAbmgv7ogoq57O9YpKmFBbmoEoCHDB2xMBY0+/KVrqAaCDyCE16dUspeOvIxFFRI+0sEtqDqy2b4A==",
      "license": "MIT"
    },
    "node_modules/@types/d3-drag": {
      "version": "3.0.7",
      "resolved": "https://registry.npmjs.org/@types/d3-drag/-/d3-drag-3.0.7.tgz",
      "integrity": "sha512-HE3jVKlzU9AaMazNufooRJ5ZpWmLIoc90A37WU2JMmeq28w1FQqCZswHZ3xR+SuxYftzHq6WU6KJHvqxKzTxxQ==",
      "license": "MIT",
      "dependencies": {
        "@types/d3-selection": "*"
      }
    },
    "node_modules/@types/d3-interpolate": {
      "version": "3.0.4",
      "resolved": "https://registry.npmjs.org/@types/d3-interpolate/-/d3-interpolate-3.0.4.tgz",
      "integrity": "sha512-mgLPETlrpVV1YRJIglr4Ez47g7Yxjl1lj7YKsiMCb27VJH9W8NVM6Bb9d8kkpG/uAQS5AmbA48q2IAolKKo1MA==",
      "license": "MIT",
      "dependencies": {
        "@types/d3-color": "*"
      }
    },
    "node_modules/@types/d3-selection": {
      "version": "3.0.11",
      "resolved": "https://registry.npmjs.org/@types/d3-selection/-/d3-selection-3.0.11.tgz",
      "integrity": "sha512-bhAXu23DJWsrI45xafYpkQ4NtcKMwWnAC/vKrd2l+nxMFuvOT3XMYTIj2opv8vq8AO5Yh7Qac/nSeP/3zjTK0w==",
      "license": "MIT"
    },
    "node_modules/@types/d3-transition": {
      "version": "3.0.9",
      "resolved": "https://registry.npmjs.org/@types/d3-transition/-/d3-transition-3.0.9.tgz",
      "integrity": "sha512-uZS5shfxzO3rGlu0cC3bjmMFKsXv+SmZZcgp0KD22ts4uGXp5EVYGzu/0YdwZeKmddhcAccYtREJKkPfXkZuCg==",
      "license": "MIT",
      "dependencies": {
        "@types/d3-selection": "*"
      }
    },
    "node_modules/@types/d3-zoom": {
      "version": "3.0.8",
      "resolved": "https://registry.npmjs.org/@types/d3-zoom/-/d3-zoom-3.0.8.tgz",
      "integrity": "sha512-iqMC4/YlFCSlO8+2Ii1GGGliCAY4XdeG748w5vQUbevlbDu0zSjH/+jojorQVBK/se0j6DUFNPBGSqD3YWYnDw==",
      "license": "MIT",
      "dependencies": {
        "@types/d3-interpolate": "*",
        "@types/d3-selection": "*"
      }
    },
    "node_modules/@types/estree": {
      "version": "1.0.8",
      "resolved": "https://registry.npmjs.org/@types/estree/-/estree-1.0.8.tgz",
      "integrity": "sha512-dWHzHa2WqEXI/O1E9OjrocMTKJl2mSrEolh1Iomrv6U+JuNwaHXsXx9bLu5gG7BUWFIN0skIQJQ/L1rIex4X6w==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/@types/prop-types": {
      "version": "15.7.15",
      "resolved": "https://registry.npmjs.org/@types/prop-types/-/prop-types-15.7.15.tgz",
      "integrity": "sha512-F6bEyamV9jKGAFBEmlQnesRPGOQqS2+Uwi0Em15xenOxHaf2hv6L8YCVn3rPdPJOiJfPiCnLIRyvwVaqMY3MIw==",
      "devOptional": true,
      "license": "MIT"
    },
    "node_modules/@types/react": {
      "version": "18.3.27",
      "resolved": "https://registry.npmjs.org/@types/react/-/react-18.3.27.tgz",
      "integrity": "sha512-cisd7gxkzjBKU2GgdYrTdtQx1SORymWyaAFhaxQPK9bYO9ot3Y5OikQRvY0VYQtvwjeQnizCINJAenh/V7MK2w==",
      "devOptional": true,
      "license": "MIT",
      "dependencies": {
        "@types/prop-types": "*",
        "csstype": "^3.2.2"
      }
    },
    "node_modules/@types/react-dom": {
      "version": "18.3.7",
      "resolved": "https://registry.npmjs.org/@types/react-dom/-/react-dom-18.3.7.tgz",
      "integrity": "sha512-MEe3UeoENYVFXzoXEWsvcpg6ZvlrFNlOQ7EOsvhI3CfAXwzPfO8Qwuxd40nepsYKqyyVQnTdEfv68q91yLcKrQ==",
      "dev": true,
      "license": "MIT",
      "peerDependencies": {
        "@types/react": "^18.0.0"
      }
    },
    "node_modules/@types/uuid": {
      "version": "9.0.8",
      "resolved": "https://registry.npmjs.org/@types/uuid/-/uuid-9.0.8.tgz",
      "integrity": "sha512-jg+97EGIcY9AGHJJRaaPVgetKDsrTgbRjQ5Msgjh/DQKEFl0DtyRr/VCOyD1T2R1MNeWPK/u7JoGhlDZnKBAfA==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/@vitejs/plugin-react": {
      "version": "4.7.0",
      "resolved": "https://registry.npmjs.org/@vitejs/plugin-react/-/plugin-react-4.7.0.tgz",
      "integrity": "sha512-gUu9hwfWvvEDBBmgtAowQCojwZmJ5mcLn3aufeCsitijs3+f2NsrPtlAWIR6OPiqljl96GVCUbLe0HyqIpVaoA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@babel/core": "^7.28.0",
        "@babel/plugin-transform-react-jsx-self": "^7.27.1",
        "@babel/plugin-transform-react-jsx-source": "^7.27.1",
        "@rolldown/pluginutils": "1.0.0-beta.27",
        "@types/babel__core": "^7.20.5",
        "react-refresh": "^0.17.0"
      },
      "engines": {
        "node": "^14.18.0 || >=16.0.0"
      },
      "peerDependencies": {
        "vite": "^4.2.0 || ^5.0.0 || ^6.0.0 || ^7.0.0"
      }
    },
    "node_modules/@xyflow/react": {
      "version": "12.10.0",
      "resolved": "https://registry.npmjs.org/@xyflow/react/-/react-12.10.0.tgz",
      "integrity": "sha512-eOtz3whDMWrB4KWVatIBrKuxECHqip6PfA8fTpaS2RUGVpiEAe+nqDKsLqkViVWxDGreq0lWX71Xth/SPAzXiw==",
      "license": "MIT",
      "dependencies": {
        "@xyflow/system": "0.0.74",
        "classcat": "^5.0.3",
        "zustand": "^4.4.0"
      },
      "peerDependencies": {
        "react": ">=17",
        "react-dom": ">=17"
      }
    },
    "node_modules/@xyflow/system": {
      "version": "0.0.74",
      "resolved": "https://registry.npmjs.org/@xyflow/system/-/system-0.0.74.tgz",
      "integrity": "sha512-7v7B/PkiVrkdZzSbL+inGAo6tkR/WQHHG0/jhSvLQToCsfa8YubOGmBYd1s08tpKpihdHDZFwzQZeR69QSBb4Q==",
      "license": "MIT",
      "dependencies": {
        "@types/d3-drag": "^3.0.7",
        "@types/d3-interpolate": "^3.0.4",
        "@types/d3-selection": "^3.0.10",
        "@types/d3-transition": "^3.0.8",
        "@types/d3-zoom": "^3.0.8",
        "d3-drag": "^3.0.0",
        "d3-interpolate": "^3.0.1",
        "d3-selection": "^3.0.0",
        "d3-zoom": "^3.0.0"
      }
    },
    "node_modules/98.css": {
      "version": "0.1.21",
      "resolved": "https://registry.npmjs.org/98.css/-/98.css-0.1.21.tgz",
      "integrity": "sha512-ddk5qtUWyapM0Bzd5jwGExoE5fdSEGrP+F5VbYjyZLf2c9UVmn6w2NPTvCsoD4BWdGsjdLjlkQGhWwWTJcYQJQ==",
      "license": "MIT"
    },
    "node_modules/baseline-browser-mapping": {
      "version": "2.9.19",
      "resolved": "https://registry.npmjs.org/baseline-browser-mapping/-/baseline-browser-mapping-2.9.19.tgz",
      "integrity": "sha512-ipDqC8FrAl/76p2SSWKSI+H9tFwm7vYqXQrItCuiVPt26Km0jS+NzSsBWAaBusvSbQcfJG+JitdMm+wZAgTYqg==",
      "dev": true,
      "license": "Apache-2.0",
      "bin": {
        "baseline-browser-mapping": "dist/cli.js"
      }
    },
    "node_modules/browserslist": {
      "version": "4.28.1",
      "resolved": "https://registry.npmjs.org/browserslist/-/browserslist-4.28.1.tgz",
      "integrity": "sha512-ZC5Bd0LgJXgwGqUknZY/vkUQ04r8NXnJZ3yYi4vDmSiZmC/pdSN0NbNRPxZpbtO4uAfDUAFffO8IZoM3Gj8IkA==",
      "dev": true,
      "funding": [
        {
          "type": "opencollective",
          "url": "https://opencollective.com/browserslist"
        },
        {
          "type": "tidelift",
          "url": "https://tidelift.com/funding/github/npm/browserslist"
        },
        {
          "type": "github",
          "url": "https://github.com/sponsors/ai"
        }
      ],
      "license": "MIT",
      "dependencies": {
        "baseline-browser-mapping": "^2.9.0",
        "caniuse-lite": "^1.0.30001759",
        "electron-to-chromium": "^1.5.263",
        "node-releases": "^2.0.27",
        "update-browserslist-db": "^1.2.0"
      },
      "bin": {
        "browserslist": "cli.js"
      },
      "engines": {
        "node": "^6 || ^7 || ^8 || ^9 || ^10 || ^11 || ^12 || >=13.7"
      }
    },
    "node_modules/caniuse-lite": {
      "version": "1.0.30001767",
      "resolved": "https://registry.npmjs.org/caniuse-lite/-/caniuse-lite-1.0.30001767.tgz",
      "integrity": "sha512-34+zUAMhSH+r+9eKmYG+k2Rpt8XttfE4yXAjoZvkAPs15xcYQhyBYdalJ65BzivAvGRMViEjy6oKr/S91loekQ==",
      "dev": true,
      "funding": [
        {
          "type": "opencollective",
          "url": "https://opencollective.com/browserslist"
        },
        {
          "type": "tidelift",
          "url": "https://tidelift.com/funding/github/npm/caniuse-lite"
        },
        {
          "type": "github",
          "url": "https://github.com/sponsors/ai"
        }
      ],
      "license": "CC-BY-4.0"
    },
    "node_modules/classcat": {
      "version": "5.0.5",
      "resolved": "https://registry.npmjs.org/classcat/-/classcat-5.0.5.tgz",
      "integrity": "sha512-JhZUT7JFcQy/EzW605k/ktHtncoo9vnyW/2GspNYwFlN1C/WmjuV/xtS04e9SOkL2sTdw0VAZ2UGCcQ9lR6p6w==",
      "license": "MIT"
    },
    "node_modules/convert-source-map": {
      "version": "2.0.0",
      "resolved": "https://registry.npmjs.org/convert-source-map/-/convert-source-map-2.0.0.tgz",
      "integrity": "sha512-Kvp459HrV2FEJ1CAsi1Ku+MY3kasH19TFykTz2xWmMeq6bk2NU3XXvfJ+Q61m0xktWwt+1HSYf3JZsTms3aRJg==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/csstype": {
      "version": "3.2.3",
      "resolved": "https://registry.npmjs.org/csstype/-/csstype-3.2.3.tgz",
      "integrity": "sha512-z1HGKcYy2xA8AGQfwrn0PAy+PB7X/GSj3UVJW9qKyn43xWa+gl5nXmU4qqLMRzWVLFC8KusUX8T/0kCiOYpAIQ==",
      "devOptional": true,
      "license": "MIT"
    },
    "node_modules/d3-color": {
      "version": "3.1.0",
      "resolved": "https://registry.npmjs.org/d3-color/-/d3-color-3.1.0.tgz",
      "integrity": "sha512-zg/chbXyeBtMQ1LbD/WSoW2DpC3I0mpmPdW+ynRTj/x2DAWYrIY7qeZIHidozwV24m4iavr15lNwIwLxRmOxhA==",
      "license": "ISC",
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-dispatch": {
      "version": "3.0.1",
      "resolved": "https://registry.npmjs.org/d3-dispatch/-/d3-dispatch-3.0.1.tgz",
      "integrity": "sha512-rzUyPU/S7rwUflMyLc1ETDeBj0NRuHKKAcvukozwhshr6g6c5d8zh4c2gQjY2bZ0dXeGLWc1PF174P2tVvKhfg==",
      "license": "ISC",
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-drag": {
      "version": "3.0.0",
      "resolved": "https://registry.npmjs.org/d3-drag/-/d3-drag-3.0.0.tgz",
      "integrity": "sha512-pWbUJLdETVA8lQNJecMxoXfH6x+mO2UQo8rSmZ+QqxcbyA3hfeprFgIT//HW2nlHChWeIIMwS2Fq+gEARkhTkg==",
      "license": "ISC",
      "dependencies": {
        "d3-dispatch": "1 - 3",
        "d3-selection": "3"
      },
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-ease": {
      "version": "3.0.1",
      "resolved": "https://registry.npmjs.org/d3-ease/-/d3-ease-3.0.1.tgz",
      "integrity": "sha512-wR/XK3D3XcLIZwpbvQwQ5fK+8Ykds1ip7A2Txe0yxncXSdq1L9skcG7blcedkOX+ZcgxGAmLX1FrRGbADwzi0w==",
      "license": "BSD-3-Clause",
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-interpolate": {
      "version": "3.0.1",
      "resolved": "https://registry.npmjs.org/d3-interpolate/-/d3-interpolate-3.0.1.tgz",
      "integrity": "sha512-3bYs1rOD33uo8aqJfKP3JWPAibgw8Zm2+L9vBKEHJ2Rg+viTR7o5Mmv5mZcieN+FRYaAOWX5SJATX6k1PWz72g==",
      "license": "ISC",
      "dependencies": {
        "d3-color": "1 - 3"
      },
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-selection": {
      "version": "3.0.0",
      "resolved": "https://registry.npmjs.org/d3-selection/-/d3-selection-3.0.0.tgz",
      "integrity": "sha512-fmTRWbNMmsmWq6xJV8D19U/gw/bwrHfNXxrIN+HfZgnzqTHp9jOmKMhsTUjXOJnZOdZY9Q28y4yebKzqDKlxlQ==",
      "license": "ISC",
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-timer": {
      "version": "3.0.1",
      "resolved": "https://registry.npmjs.org/d3-timer/-/d3-timer-3.0.1.tgz",
      "integrity": "sha512-ndfJ/JxxMd3nw31uyKoY2naivF+r29V+Lc0svZxe1JvvIRmi8hUsrMvdOwgS1o6uBHmiz91geQ0ylPP0aj1VUA==",
      "license": "ISC",
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/d3-transition": {
      "version": "3.0.1",
      "resolved": "https://registry.npmjs.org/d3-transition/-/d3-transition-3.0.1.tgz",
      "integrity": "sha512-ApKvfjsSR6tg06xrL434C0WydLr7JewBB3V+/39RMHsaXTOG0zmt/OAXeng5M5LBm0ojmxJrpomQVZ1aPvBL4w==",
      "license": "ISC",
      "dependencies": {
        "d3-color": "1 - 3",
        "d3-dispatch": "1 - 3",
        "d3-ease": "1 - 3",
        "d3-interpolate": "1 - 3",
        "d3-timer": "1 - 3"
      },
      "engines": {
        "node": ">=12"
      },
      "peerDependencies": {
        "d3-selection": "2 - 3"
      }
    },
    "node_modules/d3-zoom": {
      "version": "3.0.0",
      "resolved": "https://registry.npmjs.org/d3-zoom/-/d3-zoom-3.0.0.tgz",
      "integrity": "sha512-b8AmV3kfQaqWAuacbPuNbL6vahnOJflOhexLzMMNLga62+/nh0JzvJ0aO/5a5MVgUFGS7Hu1P9P03o3fJkDCyw==",
      "license": "ISC",
      "dependencies": {
        "d3-dispatch": "1 - 3",
        "d3-drag": "2 - 3",
        "d3-interpolate": "1 - 3",
        "d3-selection": "2 - 3",
        "d3-transition": "2 - 3"
      },
      "engines": {
        "node": ">=12"
      }
    },
    "node_modules/debug": {
      "version": "4.4.3",
      "resolved": "https://registry.npmjs.org/debug/-/debug-4.4.3.tgz",
      "integrity": "sha512-RGwwWnwQvkVfavKVt22FGLw+xYSdzARwm0ru6DhTVA3umU5hZc28V3kO4stgYryrTlLpuvgI9GiijltAjNbcqA==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "ms": "^2.1.3"
      },
      "engines": {
        "node": ">=6.0"
      },
      "peerDependenciesMeta": {
        "supports-color": {
          "optional": true
        }
      }
    },
    "node_modules/electron-to-chromium": {
      "version": "1.5.286",
      "resolved": "https://registry.npmjs.org/electron-to-chromium/-/electron-to-chromium-1.5.286.tgz",
      "integrity": "sha512-9tfDXhJ4RKFNerfjdCcZfufu49vg620741MNs26a9+bhLThdB+plgMeou98CAaHu/WATj2iHOOHTp1hWtABj2A==",
      "dev": true,
      "license": "ISC"
    },
    "node_modules/esbuild": {
      "version": "0.21.5",
      "resolved": "https://registry.npmjs.org/esbuild/-/esbuild-0.21.5.tgz",
      "integrity": "sha512-mg3OPMV4hXywwpoDxu3Qda5xCKQi+vCTZq8S9J/EpkhB2HzKXq4SNFZE3+NK93JYxc8VMSep+lOUSC/RVKaBqw==",
      "dev": true,
      "hasInstallScript": true,
      "license": "MIT",
      "bin": {
        "esbuild": "bin/esbuild"
      },
      "engines": {
        "node": ">=12"
      },
      "optionalDependencies": {
        "@esbuild/aix-ppc64": "0.21.5",
        "@esbuild/android-arm": "0.21.5",
        "@esbuild/android-arm64": "0.21.5",
        "@esbuild/android-x64": "0.21.5",
        "@esbuild/darwin-arm64": "0.21.5",
        "@esbuild/darwin-x64": "0.21.5",
        "@esbuild/freebsd-arm64": "0.21.5",
        "@esbuild/freebsd-x64": "0.21.5",
        "@esbuild/linux-arm": "0.21.5",
        "@esbuild/linux-arm64": "0.21.5",
        "@esbuild/linux-ia32": "0.21.5",
        "@esbuild/linux-loong64": "0.21.5",
        "@esbuild/linux-mips64el": "0.21.5",
        "@esbuild/linux-ppc64": "0.21.5",
        "@esbuild/linux-riscv64": "0.21.5",
        "@esbuild/linux-s390x": "0.21.5",
        "@esbuild/linux-x64": "0.21.5",
        "@esbuild/netbsd-x64": "0.21.5",
        "@esbuild/openbsd-x64": "0.21.5",
        "@esbuild/sunos-x64": "0.21.5",
        "@esbuild/win32-arm64": "0.21.5",
        "@esbuild/win32-ia32": "0.21.5",
        "@esbuild/win32-x64": "0.21.5"
      }
    },
    "node_modules/escalade": {
      "version": "3.2.0",
      "resolved": "https://registry.npmjs.org/escalade/-/escalade-3.2.0.tgz",
      "integrity": "sha512-WUj2qlxaQtO4g6Pq5c29GTcWGDyd8itL8zTlipgECz3JesAiiOKotd8JU6otB3PACgG6xkJUyVhboMS+bje/jA==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6"
      }
    },
    "node_modules/fsevents": {
      "version": "2.3.3",
      "resolved": "https://registry.npmjs.org/fsevents/-/fsevents-2.3.3.tgz",
      "integrity": "sha512-5xoDfX+fL7faATnagmWPpbFtwh/R77WmMMqqHGS65C3vvB0YHrgF+B1YmZ3441tMj5n63k0212XNoJwzlhffQw==",
      "dev": true,
      "hasInstallScript": true,
      "license": "MIT",
      "optional": true,
      "os": [
        "darwin"
      ],
      "engines": {
        "node": "^8.16.0 || ^10.6.0 || >=11.0.0"
      }
    },
    "node_modules/gensync": {
      "version": "1.0.0-beta.2",
      "resolved": "https://registry.npmjs.org/gensync/-/gensync-1.0.0-beta.2.tgz",
      "integrity": "sha512-3hN7NaskYvMDLQY55gnW3NQ+mesEAepTqlg+VEbj7zzqEMBVNhzcGYYeqFo/TlYz6eQiFcp1HcsCZO+nGgS8zg==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=6.9.0"
      }
    },
    "node_modules/js-tokens": {
      "version": "4.0.0",
      "resolved": "https://registry.npmjs.org/js-tokens/-/js-tokens-4.0.0.tgz",
      "integrity": "sha512-RdJUflcE3cUzKiMqQgsCu06FPu9UdIJO0beYbPhHN4k6apgJtifcoCtT9bcxOpYBtpD2kCM6Sbzg4CausW/PKQ==",
      "license": "MIT"
    },
    "node_modules/jsesc": {
      "version": "3.1.0",
      "resolved": "https://registry.npmjs.org/jsesc/-/jsesc-3.1.0.tgz",
      "integrity": "sha512-/sM3dO2FOzXjKQhJuo0Q173wf2KOo8t4I8vHy6lF9poUp7bKT0/NHE8fPX23PwfhnykfqnC2xRxOnVw5XuGIaA==",
      "dev": true,
      "license": "MIT",
      "bin": {
        "jsesc": "bin/jsesc"
      },
      "engines": {
        "node": ">=6"
      }
    },
    "node_modules/json5": {
      "version": "2.2.3",
      "resolved": "https://registry.npmjs.org/json5/-/json5-2.2.3.tgz",
      "integrity": "sha512-XmOWe7eyHYH14cLdVPoyg+GOH3rYX++KpzrylJwSW98t3Nk+U8XOl8FWKOgwtzdb8lXGf6zYwDUzeHMWfxasyg==",
      "dev": true,
      "license": "MIT",
      "bin": {
        "json5": "lib/cli.js"
      },
      "engines": {
        "node": ">=6"
      }
    },
    "node_modules/loose-envify": {
      "version": "1.4.0",
      "resolved": "https://registry.npmjs.org/loose-envify/-/loose-envify-1.4.0.tgz",
      "integrity": "sha512-lyuxPGr/Wfhrlem2CL/UcnUc1zcqKAImBDzukY7Y5F/yQiNdko6+fRLevlw1HgMySw7f611UIY408EtxRSoK3Q==",
      "license": "MIT",
      "dependencies": {
        "js-tokens": "^3.0.0 || ^4.0.0"
      },
      "bin": {
        "loose-envify": "cli.js"
      }
    },
    "node_modules/lru-cache": {
      "version": "5.1.1",
      "resolved": "https://registry.npmjs.org/lru-cache/-/lru-cache-5.1.1.tgz",
      "integrity": "sha512-KpNARQA3Iwv+jTA0utUVVbrh+Jlrr1Fv0e56GGzAFOXN7dk/FviaDW8LHmK52DlcH4WP2n6gI8vN1aesBFgo9w==",
      "dev": true,
      "license": "ISC",
      "dependencies": {
        "yallist": "^3.0.2"
      }
    },
    "node_modules/ms": {
      "version": "2.1.3",
      "resolved": "https://registry.npmjs.org/ms/-/ms-2.1.3.tgz",
      "integrity": "sha512-6FlzubTLZG3J2a/NVCAleEhjzq5oxgHyaCU9yYXvcLsvoVaHJq/s5xXI6/XXP6tz7R9xAOtHnSO/tXtF3WRTlA==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/nanoid": {
      "version": "3.3.11",
      "resolved": "https://registry.npmjs.org/nanoid/-/nanoid-3.3.11.tgz",
      "integrity": "sha512-N8SpfPUnUp1bK+PMYW8qSWdl9U+wwNWI4QKxOYDy9JAro3WMX7p2OeVRF9v+347pnakNevPmiHhNmZ2HbFA76w==",
      "dev": true,
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/ai"
        }
      ],
      "license": "MIT",
      "bin": {
        "nanoid": "bin/nanoid.cjs"
      },
      "engines": {
        "node": "^10 || ^12 || ^13.7 || ^14 || >=15.0.1"
      }
    },
    "node_modules/node-releases": {
      "version": "2.0.27",
      "resolved": "https://registry.npmjs.org/node-releases/-/node-releases-2.0.27.tgz",
      "integrity": "sha512-nmh3lCkYZ3grZvqcCH+fjmQ7X+H0OeZgP40OierEaAptX4XofMh5kwNbWh7lBduUzCcV/8kZ+NDLCwm2iorIlA==",
      "dev": true,
      "license": "MIT"
    },
    "node_modules/picocolors": {
      "version": "1.1.1",
      "resolved": "https://registry.npmjs.org/picocolors/-/picocolors-1.1.1.tgz",
      "integrity": "sha512-xceH2snhtb5M9liqDsmEw56le376mTZkEX/jEb/RxNFyegNul7eNslCXP9FDj/Lcu0X8KEyMceP2ntpaHrDEVA==",
      "dev": true,
      "license": "ISC"
    },
    "node_modules/postcss": {
      "version": "8.5.6",
      "resolved": "https://registry.npmjs.org/postcss/-/postcss-8.5.6.tgz",
      "integrity": "sha512-3Ybi1tAuwAP9s0r1UQ2J4n5Y0G05bJkpUIO0/bI9MhwmD70S5aTWbXGBwxHrelT+XM1k6dM0pk+SwNkpTRN7Pg==",
      "dev": true,
      "funding": [
        {
          "type": "opencollective",
          "url": "https://opencollective.com/postcss/"
        },
        {
          "type": "tidelift",
          "url": "https://tidelift.com/funding/github/npm/postcss"
        },
        {
          "type": "github",
          "url": "https://github.com/sponsors/ai"
        }
      ],
      "license": "MIT",
      "dependencies": {
        "nanoid": "^3.3.11",
        "picocolors": "^1.1.1",
        "source-map-js": "^1.2.1"
      },
      "engines": {
        "node": "^10 || ^12 || >=14"
      }
    },
    "node_modules/react": {
      "version": "18.3.1",
      "resolved": "https://registry.npmjs.org/react/-/react-18.3.1.tgz",
      "integrity": "sha512-wS+hAgJShR0KhEvPJArfuPVN1+Hz1t0Y6n5jLrGQbkb4urgPE/0Rve+1kMB1v/oWgHgm4WIcV+i7F2pTVj+2iQ==",
      "license": "MIT",
      "dependencies": {
        "loose-envify": "^1.1.0"
      },
      "engines": {
        "node": ">=0.10.0"
      }
    },
    "node_modules/react-dom": {
      "version": "18.3.1",
      "resolved": "https://registry.npmjs.org/react-dom/-/react-dom-18.3.1.tgz",
      "integrity": "sha512-5m4nQKp+rZRb09LNH59GM4BxTh9251/ylbKIbpe7TpGxfJ+9kv6BLkLBXIjjspbgbnIBNqlI23tRnTWT0snUIw==",
      "license": "MIT",
      "dependencies": {
        "loose-envify": "^1.1.0",
        "scheduler": "^0.23.2"
      },
      "peerDependencies": {
        "react": "^18.3.1"
      }
    },
    "node_modules/react-refresh": {
      "version": "0.17.0",
      "resolved": "https://registry.npmjs.org/react-refresh/-/react-refresh-0.17.0.tgz",
      "integrity": "sha512-z6F7K9bV85EfseRCp2bzrpyQ0Gkw1uLoCel9XBVWPg/TjRj94SkJzUTGfOa4bs7iJvBWtQG0Wq7wnI0syw3EBQ==",
      "dev": true,
      "license": "MIT",
      "engines": {
        "node": ">=0.10.0"
      }
    },
    "node_modules/rollup": {
      "version": "4.57.1",
      "resolved": "https://registry.npmjs.org/rollup/-/rollup-4.57.1.tgz",
      "integrity": "sha512-oQL6lgK3e2QZeQ7gcgIkS2YZPg5slw37hYufJ3edKlfQSGGm8ICoxswK15ntSzF/a8+h7ekRy7k7oWc3BQ7y8A==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "@types/estree": "1.0.8"
      },
      "bin": {
        "rollup": "dist/bin/rollup"
      },
      "engines": {
        "node": ">=18.0.0",
        "npm": ">=8.0.0"
      },
      "optionalDependencies": {
        "@rollup/rollup-android-arm-eabi": "4.57.1",
        "@rollup/rollup-android-arm64": "4.57.1",
        "@rollup/rollup-darwin-arm64": "4.57.1",
        "@rollup/rollup-darwin-x64": "4.57.1",
        "@rollup/rollup-freebsd-arm64": "4.57.1",
        "@rollup/rollup-freebsd-x64": "4.57.1",
        "@rollup/rollup-linux-arm-gnueabihf": "4.57.1",
        "@rollup/rollup-linux-arm-musleabihf": "4.57.1",
        "@rollup/rollup-linux-arm64-gnu": "4.57.1",
        "@rollup/rollup-linux-arm64-musl": "4.57.1",
        "@rollup/rollup-linux-loong64-gnu": "4.57.1",
        "@rollup/rollup-linux-loong64-musl": "4.57.1",
        "@rollup/rollup-linux-ppc64-gnu": "4.57.1",
        "@rollup/rollup-linux-ppc64-musl": "4.57.1",
        "@rollup/rollup-linux-riscv64-gnu": "4.57.1",
        "@rollup/rollup-linux-riscv64-musl": "4.57.1",
        "@rollup/rollup-linux-s390x-gnu": "4.57.1",
        "@rollup/rollup-linux-x64-gnu": "4.57.1",
        "@rollup/rollup-linux-x64-musl": "4.57.1",
        "@rollup/rollup-openbsd-x64": "4.57.1",
        "@rollup/rollup-openharmony-arm64": "4.57.1",
        "@rollup/rollup-win32-arm64-msvc": "4.57.1",
        "@rollup/rollup-win32-ia32-msvc": "4.57.1",
        "@rollup/rollup-win32-x64-gnu": "4.57.1",
        "@rollup/rollup-win32-x64-msvc": "4.57.1",
        "fsevents": "~2.3.2"
      }
    },
    "node_modules/scheduler": {
      "version": "0.23.2",
      "resolved": "https://registry.npmjs.org/scheduler/-/scheduler-0.23.2.tgz",
      "integrity": "sha512-UOShsPwz7NrMUqhR6t0hWjFduvOzbtv7toDH1/hIrfRNIDBnnBWd0CwJTGvTpngVlmwGCdP9/Zl/tVrDqcuYzQ==",
      "license": "MIT",
      "dependencies": {
        "loose-envify": "^1.1.0"
      }
    },
    "node_modules/semver": {
      "version": "6.3.1",
      "resolved": "https://registry.npmjs.org/semver/-/semver-6.3.1.tgz",
      "integrity": "sha512-BR7VvDCVHO+q2xBEWskxS6DJE1qRnb7DxzUrogb71CWoSficBxYsiAGd+Kl0mmq/MprG9yArRkyrQxTO6XjMzA==",
      "dev": true,
      "license": "ISC",
      "bin": {
        "semver": "bin/semver.js"
      }
    },
    "node_modules/source-map-js": {
      "version": "1.2.1",
      "resolved": "https://registry.npmjs.org/source-map-js/-/source-map-js-1.2.1.tgz",
      "integrity": "sha512-UXWMKhLOwVKb728IUtQPXxfYU+usdybtUrK/8uGE8CQMvrhOpwvzDBwj0QhSL7MQc7vIsISBG8VQ8+IDQxpfQA==",
      "dev": true,
      "license": "BSD-3-Clause",
      "engines": {
        "node": ">=0.10.0"
      }
    },
    "node_modules/typescript": {
      "version": "5.6.3",
      "resolved": "https://registry.npmjs.org/typescript/-/typescript-5.6.3.tgz",
      "integrity": "sha512-hjcS1mhfuyi4WW8IWtjP7brDrG2cuDZukyrYrSauoXGNgx0S7zceP07adYkJycEr56BOUTNPzbInooiN3fn1qw==",
      "dev": true,
      "license": "Apache-2.0",
      "bin": {
        "tsc": "bin/tsc",
        "tsserver": "bin/tsserver"
      },
      "engines": {
        "node": ">=14.17"
      }
    },
    "node_modules/update-browserslist-db": {
      "version": "1.2.3",
      "resolved": "https://registry.npmjs.org/update-browserslist-db/-/update-browserslist-db-1.2.3.tgz",
      "integrity": "sha512-Js0m9cx+qOgDxo0eMiFGEueWztz+d4+M3rGlmKPT+T4IS/jP4ylw3Nwpu6cpTTP8R1MAC1kF4VbdLt3ARf209w==",
      "dev": true,
      "funding": [
        {
          "type": "opencollective",
          "url": "https://opencollective.com/browserslist"
        },
        {
          "type": "tidelift",
          "url": "https://tidelift.com/funding/github/npm/browserslist"
        },
        {
          "type": "github",
          "url": "https://github.com/sponsors/ai"
        }
      ],
      "license": "MIT",
      "dependencies": {
        "escalade": "^3.2.0",
        "picocolors": "^1.1.1"
      },
      "bin": {
        "update-browserslist-db": "cli.js"
      },
      "peerDependencies": {
        "browserslist": ">= 4.21.0"
      }
    },
    "node_modules/use-sync-external-store": {
      "version": "1.6.0",
      "resolved": "https://registry.npmjs.org/use-sync-external-store/-/use-sync-external-store-1.6.0.tgz",
      "integrity": "sha512-Pp6GSwGP/NrPIrxVFAIkOQeyw8lFenOHijQWkUTrDvrF4ALqylP2C/KCkeS9dpUM3KvYRQhna5vt7IL95+ZQ9w==",
      "license": "MIT",
      "peerDependencies": {
        "react": "^16.8.0 || ^17.0.0 || ^18.0.0 || ^19.0.0"
      }
    },
    "node_modules/uuid": {
      "version": "9.0.1",
      "resolved": "https://registry.npmjs.org/uuid/-/uuid-9.0.1.tgz",
      "integrity": "sha512-b+1eJOlsR9K8HJpow9Ok3fiWOWSIcIzXodvv0rQjVoOVNpWMpxf1wZNpt4y9h10odCNrqnYp1OBzRktckBe3sA==",
      "funding": [
        "https://github.com/sponsors/broofa",
        "https://github.com/sponsors/ctavan"
      ],
      "license": "MIT",
      "bin": {
        "uuid": "dist/bin/uuid"
      }
    },
    "node_modules/vite": {
      "version": "5.4.21",
      "resolved": "https://registry.npmjs.org/vite/-/vite-5.4.21.tgz",
      "integrity": "sha512-o5a9xKjbtuhY6Bi5S3+HvbRERmouabWbyUcpXXUA1u+GNUKoROi9byOJ8M0nHbHYHkYICiMlqxkg1KkYmm25Sw==",
      "dev": true,
      "license": "MIT",
      "dependencies": {
        "esbuild": "^0.21.3",
        "postcss": "^8.4.43",
        "rollup": "^4.20.0"
      },
      "bin": {
        "vite": "bin/vite.js"
      },
      "engines": {
        "node": "^18.0.0 || >=20.0.0"
      },
      "funding": {
        "url": "https://github.com/vitejs/vite?sponsor=1"
      },
      "optionalDependencies": {
        "fsevents": "~2.3.3"
      },
      "peerDependencies": {
        "@types/node": "^18.0.0 || >=20.0.0",
        "less": "*",
        "lightningcss": "^1.21.0",
        "sass": "*",
        "sass-embedded": "*",
        "stylus": "*",
        "sugarss": "*",
        "terser": "^5.4.0"
      },
      "peerDependenciesMeta": {
        "@types/node": {
          "optional": true
        },
        "less": {
          "optional": true
        },
        "lightningcss": {
          "optional": true
        },
        "sass": {
          "optional": true
        },
        "sass-embedded": {
          "optional": true
        },
        "stylus": {
          "optional": true
        },
        "sugarss": {
          "optional": true
        },
        "terser": {
          "optional": true
        }
      }
    },
    "node_modules/yallist": {
      "version": "3.1.1",
      "resolved": "https://registry.npmjs.org/yallist/-/yallist-3.1.1.tgz",
      "integrity": "sha512-a4UGQaWPH59mOXUYnAG2ewncQS4i4F43Tv3JoAM+s2VDAmS9NsK8GpDMLrCHPksFT7h3K6TOoUNn2pb7RoXx4g==",
      "dev": true,
      "license": "ISC"
    },
    "node_modules/zustand": {
      "version": "4.5.7",
      "resolved": "https://registry.npmjs.org/zustand/-/zustand-4.5.7.tgz",
      "integrity": "sha512-CHOUy7mu3lbD6o6LJLfllpjkzhHXSBlX8B9+qPddUsIfeF5S/UZ5q0kmCsnRqT1UHFQZchNFDDzMbQsuesHWlw==",
      "license": "MIT",
      "dependencies": {
        "use-sync-external-store": "^1.2.2"
      },
      "engines": {
        "node": ">=12.7.0"
      },
      "peerDependencies": {
        "@types/react": ">=16.8",
        "immer": ">=9.0.6",
        "react": ">=16.8"
      },
      "peerDependenciesMeta": {
        "@types/react": {
          "optional": true
        },
        "immer": {
          "optional": true
        },
        "react": {
          "optional": true
        }
      }
    }
  }
}

```

## [38] workflow-mvp/desktop/package.json

```json
// FILE: workflow-mvp/desktop/package.json
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\package.json
// SIZE: 718 bytes
// MTIME: 2026-02-04T09:19:34.248691
// NOTE: Concatenated snapshot for review
{
  "name": "workflow-desktop",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  },
  "dependencies": {
    "98.css": "^0.1.20",
    "@lumino/widgets": "^2.4.0",
    "@xyflow/react": "^12.3.6",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "uuid": "^9.0.1"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@types/react": "^18.3.12",
    "@types/react-dom": "^18.3.1",
    "@types/uuid": "^9.0.8",
    "@vitejs/plugin-react": "^4.3.3",
    "typescript": "~5.6.2",
    "vite": "^5.4.10"
  }
}

```

## [39] workflow-mvp/desktop/src-tauri/Cargo.toml

```toml
# FILE: workflow-mvp/desktop/src-tauri/Cargo.toml
# FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src-tauri\Cargo.toml
# SIZE: 529 bytes
# MTIME: 2026-02-04T09:20:27.645706
# NOTE: Concatenated snapshot for review
[package]
name = "workflow-desktop"
version = "0.1.0"
description = "Workflow Engine Desktop"
authors = ["you"]
edition = "2021"

[lib]
name = "workflow_desktop_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[[bin]]
name = "workflow-desktop"
path = "src/main.rs"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

```

## [40] workflow-mvp/desktop/src-tauri/build.rs

```rust
// FILE: workflow-mvp/desktop/src-tauri/build.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src-tauri\build.rs
// SIZE: 42 bytes
// MTIME: 2026-02-04T09:20:01.074515
// NOTE: Concatenated snapshot for review
fn main() {
    tauri_build::build()
}

```

## [41] workflow-mvp/desktop/src-tauri/src/lib.rs

```rust
// FILE: workflow-mvp/desktop/src-tauri/src/lib.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src-tauri\src\lib.rs
// SIZE: 266 bytes
// MTIME: 2026-02-04T09:20:27.645706
// NOTE: Concatenated snapshot for review
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

```

## [42] workflow-mvp/desktop/src-tauri/src/main.rs

```rust
// FILE: workflow-mvp/desktop/src-tauri/src/main.rs
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src-tauri\src\main.rs
// SIZE: 180 bytes
// MTIME: 2026-02-04T09:20:01.090338
// NOTE: Concatenated snapshot for review
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    workflow_desktop_lib::run()
}

```

## [43] workflow-mvp/desktop/src-tauri/tauri.conf.json

```json
// FILE: workflow-mvp/desktop/src-tauri/tauri.conf.json
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src-tauri\tauri.conf.json
// SIZE: 564 bytes
// MTIME: 2026-02-04T09:20:00.852526
// NOTE: Concatenated snapshot for review
{
  "productName": "Workflow Engine",
  "version": "0.1.0",
  "identifier": "com.workflow.engine",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Workflow Engine",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {},
  "plugins": {}
}

```

## [44] workflow-mvp/desktop/src/App.tsx

```tsx
// FILE: workflow-mvp/desktop/src/App.tsx
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\App.tsx
// SIZE: 11017 bytes
// MTIME: 2026-02-04T09:27:35.521524
// NOTE: Concatenated snapshot for review
import React, { useState, useCallback, useEffect } from "react";
import {
  compileWorkflow,
  validateWorkflow,
  saveWorkflow,
  getWorkflow,
  runWorkflow,
  getConnectorSchemas,
} from "./api";
import { graphToFlow, flowToGraph, Designer } from "./Designer";
import type { Node, Edge } from "@xyflow/react";
import type { Graph, WorkflowDefinition, ValidationProblem, ConnectorSchema, ExecutionResult } from "./api";
import { getDemoWorkflow } from "./demoWorkflow";

const STORAGE_WORKFLOW = "lastWorkflowId";
const STORAGE_EXECUTION = "lastExecutionId";

export default function App() {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [lastDsl, setLastDsl] = useState<WorkflowDefinition | null>(null);
  const [problems, setProblems] = useState<ValidationProblem[]>([]);
  const [logs, setLogs] = useState<string[]>([]);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [selectedEdge, setSelectedEdge] = useState<Edge | null>(null);
  const [workflowId, setWorkflowId] = useState<string>(() => localStorage.getItem(STORAGE_WORKFLOW) ?? "");
  const [executionResult, setExecutionResult] = useState<ExecutionResult | null>(null);
  const [schemas, setSchemas] = useState<ConnectorSchema[]>([]);
  const [loadId, setLoadId] = useState("");

  const log = useCallback((msg: string) => {
    setLogs((prev) => [...prev, `[${new Date().toISOString().slice(11, 19)}] ${msg}`]);
  }, []);

  useEffect(() => {
    getConnectorSchemas().then(setSchemas).catch(() => log("Failed to load connector schemas"));
  }, [log]);

  const graph = flowToGraph(nodes, edges);

  const handleCompile = useCallback(() => {
    compileWorkflow({
      nodes: graph.nodes,
      edges: graph.edges,
    }).then((res) => {
      if (res.ok && res.workflowDefinition) {
        setLastDsl(res.workflowDefinition);
        log("Compile OK");
      } else {
        log("Compile failed: " + res.problems.map((p) => p.message).join("; "));
      }
    }).catch((e) => log("Compile error: " + (e as Error).message));
  }, [graph, log]);

  const handleValidate = useCallback(() => {
    validateWorkflow({ graph }).then((res) => {
      setProblems(res.problems);
      log(res.problems.length ? `Validate: ${res.problems.length} problem(s)` : "Validate OK");
    }).catch((e) => log("Validate error: " + (e as Error).message));
  }, [graph, log]);

  const handleSave = useCallback(() => {
    if (!lastDsl) {
      log("Compile first before Save");
      return;
    }
    saveWorkflow({
      id: workflowId || undefined,
      name: lastDsl.name || "Unnamed",
      graph,
      dsl: lastDsl,
    }).then((res) => {
      setWorkflowId(res.id);
      localStorage.setItem(STORAGE_WORKFLOW, res.id);
      log("Saved: " + res.id);
    }).catch((e) => log("Save error: " + (e as Error).message));
  }, [lastDsl, workflowId, graph, log]);

  const handleLoad = useCallback(() => {
    const id = loadId || workflowId;
    if (!id) {
      log("Enter workflow ID to load");
      return;
    }
    getWorkflow(id).then((w) => {
      const { nodes: n, edges: e } = graphToFlow(w.graph);
      setNodes(n);
      setEdges(e);
      setLastDsl(w.dsl);
      setWorkflowId(w.id);
      localStorage.setItem(STORAGE_WORKFLOW, w.id);
      log("Loaded: " + w.id);
    }).catch((e) => log("Load error: " + (e as Error).message));
  }, [loadId, workflowId, log]);

  const handleRun = useCallback(
    (mode: "dry-run" | "run") => {
      const id = workflowId;
      if (!id) {
        log("Save workflow first to Run");
        return;
      }
      log(`Run ${mode} started`);
      runWorkflow(id, {
        mode,
        inputContext: { vars: { amount: 150 } },
      })
        .then((res) => {
          setExecutionResult(res);
          localStorage.setItem(STORAGE_EXECUTION, res.executionId);
          log(`Run ${mode} finished: ${res.status}`);
        })
        .catch((e) => log("Run error: " + (e as Error).message));
    },
    [workflowId, log]
  );

  const handleLoadDemo = useCallback(() => {
    const demo = getDemoWorkflow();
    const { nodes: n, edges: e } = graphToFlow(demo);
    setNodes(n);
    setEdges(e);
    setLastDsl(null);
    log("Loaded Demo workflow");
  }, [log]);

  const handleNew = useCallback(() => {
    setNodes([]);
    setEdges([]);
    setLastDsl(null);
    setProblems([]);
    log("New workflow");
  }, [log]);

  return (
    <div className="window" style={{ margin: 8, height: "calc(100vh - 16px)" }}>
      <div className="title-bar">
        <div className="title-bar-text">Workflow Engine</div>
      </div>
      <div className="window-body">
        <div className="field-row" style={{ marginBottom: 8, flexWrap: "wrap", gap: 4 }}>
          <button className="button" onClick={handleNew}>New</button>
          <button className="button" onClick={handleLoadDemo}>Load Demo</button>
          <button className="button" onClick={handleCompile}>Compile</button>
          <button className="button" onClick={handleValidate}>Validate</button>
          <button className="button" onClick={handleSave}>Save</button>
          <input
            className="input"
            placeholder="Workflow ID"
            value={loadId || workflowId}
            onChange={(e) => setLoadId(e.target.value)}
            style={{ width: 200 }}
          />
          <button className="button" onClick={handleLoad}>Load</button>
          <button className="button" onClick={() => handleRun("dry-run")}>Run Dry</button>
          <button className="button" onClick={() => handleRun("run")}>Run</button>
        </div>

        <div style={{ display: "flex", flex: 1, minHeight: 0, gap: 8 }}>
          <div className="window" style={{ flex: "1 1 60%", minWidth: 0, display: "flex", flexDirection: "column" }}>
            <div className="title-bar"><div className="title-bar-text">Designer</div></div>
            <div className="window-body" style={{ flex: 1, minHeight: 300 }}>
              <Designer
                nodes={nodes}
                edges={edges}
                onNodesChange={setNodes}
                onEdgesChange={setEdges}
                onSelection={(n, e) => {
                  setSelectedNode(n ?? null);
                  setSelectedEdge(e ?? null);
                }}
              />
            </div>
          </div>

          <div style={{ flex: "0 0 280px", display: "flex", flexDirection: "column", gap: 8 }}>
            <div className="window" style={{ flex: 1, minHeight: 200 }}>
              <div className="title-bar"><div className="title-bar-text">Toolbox</div></div>
              <div className="window-body">
                {schemas.map((s) => (
                  <button
                    key={s.kind}
                    className="button"
                    style={{ display: "block", marginBottom: 4, width: "100%", textAlign: "left" }}
                    onClick={() => {
                      const id = crypto.randomUUID?.() ?? `n_${Date.now()}`;
                      setNodes((prev) => [
                        ...prev,
                        {
                          id,
                          type: "default",
                          position: { x: 100 + (prev.length % 4) * 200, y: 80 + Math.floor(prev.length / 4) * 100 },
                          data: { label: s.kind },
                        },
                      ]);
                      log("Added " + s.kind);
                    }}
                  >
                    {s.kind}
                  </button>
                ))}
              </div>
            </div>
            <div className="window" style={{ flex: 1, minHeight: 200 }}>
              <div className="title-bar"><div className="title-bar-text">Inspector</div></div>
              <div className="window-body">
                {selectedNode ? (
                  <div>
                    <p><strong>Node:</strong> {selectedNode.id}</p>
                    <p><strong>Type:</strong> {(selectedNode.data?.label as string) ?? ""}</p>
                  </div>
                ) : selectedEdge ? (
                  <div>
                    <p><strong>Edge:</strong> {selectedEdge.source} → {selectedEdge.target}</p>
                    <p><strong>Label:</strong> {(selectedEdge.label as string) ?? "next"}</p>
                  </div>
                ) : (
                  <p>Select a node or edge</p>
                )}
              </div>
            </div>

            {lastDsl && (
              <div className="window" style={{ maxHeight: 200, overflow: "auto" }}>
                <div className="title-bar"><div className="title-bar-text">Source (DSL)</div></div>
                <div className="window-body">
                  <pre style={{ fontSize: 10 }}>{JSON.stringify(lastDsl, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        </div>

        <div style={{ display: "flex", marginTop: 8, gap: 8 }}>
          <div className="window" style={{ flex: 1, maxHeight: 180 }}>
            <div className="title-bar"><div className="title-bar-text">Problems</div></div>
            <div className="window-body" style={{ overflow: "auto" }}>
              {problems.length === 0 ? (
                <p>No problems</p>
              ) : (
                <ul>
                  {problems.map((p, i) => (
                    <li key={i}>{p.severity}: {p.message}</li>
                  ))}
                </ul>
              )}
            </div>
          </div>
          <div className="window" style={{ flex: 1, maxHeight: 180 }}>
            <div className="title-bar"><div className="title-bar-text">Console</div></div>
            <div className="window-body" style={{ overflow: "auto", fontFamily: "monospace", fontSize: 11 }}>
              {logs.slice(-20).map((l, i) => (
                <div key={i}>{l}</div>
              ))}
            </div>
          </div>
        </div>

        {executionResult && (
          <div className="window" style={{ marginTop: 8, maxHeight: 220 }}>
            <div className="title-bar"><div className="title-bar-text">Execution: {executionResult.executionId}</div></div>
            <div className="window-body" style={{ overflow: "auto" }}>
              <p><strong>Status:</strong> {executionResult.status}</p>
              <p><strong>Steps:</strong> {executionResult.trace?.length ?? 0}</p>
              <ul>
                {(executionResult.trace ?? []).map((ev, i) => (
                  <li key={i}>{ev.stepId} – {ev.eventType} {ev.durationMs != null ? `(${ev.durationMs}ms)` : ""}</li>
                ))}
              </ul>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

```

## [45] workflow-mvp/desktop/src/Designer.tsx

```tsx
// FILE: workflow-mvp/desktop/src/Designer.tsx
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\Designer.tsx
// SIZE: 3635 bytes
// MTIME: 2026-02-04T09:28:53.853294
// NOTE: Concatenated snapshot for review
import React, { useCallback } from "react";
import {
  ReactFlow,
  Controls,
  Background,
  Connection,
  Edge,
  Node,
  BackgroundVariant,
  Panel,
  OnNodesChange,
  OnEdgesChange,
  applyNodeChanges,
  applyEdgeChanges,
  NodeChange,
  EdgeChange,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import type { GraphNode, GraphEdge } from "./api";

export function flowNodeFromGraphNode(n: GraphNode, index: number): Node {
  return {
    id: n.id,
    type: "default",
    position: { x: 250 + (index % 4) * 220, y: 100 + Math.floor(index / 4) * 120 },
    data: { label: n.type, ...n.data },
  };
}

function flowEdgeFromGraphEdge(e: GraphEdge): Edge {
  return {
    id: e.id,
    source: e.source,
    target: e.target,
    label: e.label ?? "next",
  };
}

export function graphToFlow(graph: { nodes: GraphNode[]; edges: GraphEdge[] }): { nodes: Node[]; edges: Edge[] } {
  const nodes = graph.nodes.map((n, i) => flowNodeFromGraphNode(n, i));
  const edges = graph.edges.map(flowEdgeFromGraphEdge);
  return { nodes, edges };
}

export function flowToGraph(nodes: Node[], edges: Edge[]): { nodes: GraphNode[]; edges: GraphEdge[] } {
  return {
    nodes: nodes.map((n) => ({
      id: n.id,
      type: (n.data?.label as string) ?? (n.data?.type as string) ?? "activity.transform_json",
      data: Object.fromEntries(Object.entries(n.data ?? {}).filter(([k]) => k !== "label")),
    })),
    edges: edges.map((e) => ({
      id: e.id,
      source: e.source,
      target: e.target,
      label: (e.label as string) ?? "next",
    })),
  };
}

type DesignerProps = {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: (nodes: Node[]) => void;
  onEdgesChange: (edges: Edge[]) => void;
  onSelection?: (node: Node | null, edge: Edge | null) => void;
};

export function Designer({
  nodes,
  edges,
  onNodesChange,
  onEdgesChange,
  onSelection,
}: DesignerProps) {
  const onNodesChangeInner: OnNodesChange = useCallback(
    (ch: NodeChange[]) => {
      onNodesChange(applyNodeChanges(ch, nodes));
    },
    [nodes, onNodesChange]
  );

  const onEdgesChangeInner: OnEdgesChange = useCallback(
    (ch: EdgeChange[]) => {
      onEdgesChange(applyEdgeChanges(ch, edges));
    },
    [edges, onEdgesChange]
  );

  const onConnect = useCallback(
    (c: Connection) => {
      if (!c.source || !c.target) return;
      const newEdge: Edge = {
        id: `e_${Date.now()}`,
        source: c.source,
        target: c.target,
        label: "next",
      };
      onEdgesChange([...edges, newEdge]);
    },
    [edges, onEdgesChange]
  );

  const onNodeClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      onSelection?.(node, null);
    },
    [onSelection]
  );
  const onEdgeClick = useCallback(
    (_: React.MouseEvent, edge: Edge) => {
      onSelection?.(null, edge);
    },
    [onSelection]
  );
  const onPaneClick = useCallback(() => {
    onSelection?.(null, null);
  }, [onSelection]);

  return (
    <div style={{ width: "100%", height: "100%", minHeight: 400 }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChangeInner}
        onEdgesChange={onEdgesChangeInner}
        onConnect={onConnect}
        onNodeClick={onNodeClick}
        onEdgeClick={onEdgeClick}
        onPaneClick={onPaneClick}
        fitView
      >
        <Background variant={BackgroundVariant.Dots} />
        <Controls />
        <Panel position="top-left">Designer</Panel>
      </ReactFlow>
    </div>
  );
}

```

## [46] workflow-mvp/desktop/src/api.ts

```ts
// FILE: workflow-mvp/desktop/src/api.ts
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\api.ts
// SIZE: 3438 bytes
// MTIME: 2026-02-04T09:22:44.259441
// NOTE: Concatenated snapshot for review
const API_BASE = "http://localhost:8080";

export type GraphNode = { id: string; type: string; data: Record<string, unknown> };
export type GraphEdge = { id: string; source: string; target: string; label?: string };
export type Graph = { nodes: GraphNode[]; edges: GraphEdge[] };

export type WorkflowDefinition = {
  version: number;
  id: string;
  name: string;
  start: string;
  steps: Record<string, unknown>;
};

export type CompileResult = {
  ok: boolean;
  workflowDefinition?: WorkflowDefinition;
  problems: { code: string; message: string }[];
};

export type ValidationProblem = {
  code: string;
  message: string;
  nodeId?: string;
  stepId?: string;
  severity: string;
};

export type ConnectorSchema = {
  kind: string;
  fields: { name: string; type: string; required: boolean }[];
  supports: { next: boolean; then: boolean; else: boolean; onFault: boolean };
};

export type TraceEvent = {
  stepId: string;
  eventType: string;
  payload: unknown;
  ts: string;
  durationMs?: number;
};

export type ExecutionResult = {
  executionId: string;
  status: string;
  finalContext: unknown;
  trace: TraceEvent[];
  metrics: { stepsExecuted: number; totalDurationMs: number };
};

export async function getConnectorSchemas(): Promise<ConnectorSchema[]> {
  const r = await fetch(`${API_BASE}/connectors/schema`);
  return r.json();
}

export async function compileWorkflow(body: {
  nodes: GraphNode[];
  edges: GraphEdge[];
  workflowId?: string;
  name?: string;
}): Promise<CompileResult> {
  const r = await fetch(`${API_BASE}/workflows/compile`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  return r.json();
}

export async function validateWorkflow(body: { graph?: Graph; dsl?: WorkflowDefinition }): Promise<{ problems: ValidationProblem[] }> {
  const r = await fetch(`${API_BASE}/workflows/validate`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  return r.json();
}

export async function saveWorkflow(body: {
  id?: string;
  name: string;
  graph: Graph;
  dsl: WorkflowDefinition;
}): Promise<{ id: string; name: string; saved_at: string }> {
  const r = await fetch(`${API_BASE}/workflows/save`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

export async function getWorkflow(id: string): Promise<{ id: string; name: string; graph: Graph; dsl: WorkflowDefinition; saved_at: string }> {
  const r = await fetch(`${API_BASE}/workflows/${id}`);
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

export async function runWorkflow(
  id: string,
  body: { mode: "dry-run" | "run"; inputContext?: { vars: Record<string, unknown> } }
): Promise<ExecutionResult> {
  const r = await fetch(`${API_BASE}/workflows/${id}/run`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

export async function getExecution(id: string): Promise<ExecutionResult> {
  const r = await fetch(`${API_BASE}/executions/${id}`);
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

```

## [47] workflow-mvp/desktop/src/demoWorkflow.ts

```ts
// FILE: workflow-mvp/desktop/src/demoWorkflow.ts
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\demoWorkflow.ts
// SIZE: 1721 bytes
// MTIME: 2026-02-04T09:23:26.646700
// NOTE: Concatenated snapshot for review
import type { Graph } from "./api";

export function getDemoWorkflow(): Graph {
  return {
    nodes: [
      { id: "step_trigger", type: "trigger.redis_stream", data: { stream: "demo", group: "" } },
      { id: "step_transform", type: "activity.transform_json", data: { expr: "$.amount", inputPath: "$" } },
      { id: "step_if", type: "control.if", data: { expr: "$.amount > 100" } },
      { id: "step_publish_high", type: "activity.publish_stream", data: { stream: "high", key: "" } },
      { id: "step_publish_low", type: "activity.publish_stream", data: { stream: "low", key: "" } },
      { id: "step_end", type: "control.end", data: {} },
      { id: "step_fault", type: "control.fault_end", data: { reason: "faulted" } },
    ],
    edges: [
      { id: "e1", source: "step_trigger", target: "step_transform", label: "next" },
      { id: "e2", source: "step_transform", target: "step_if", label: "next" },
      { id: "e3", source: "step_if", target: "step_publish_high", label: "then" },
      { id: "e4", source: "step_if", target: "step_publish_low", label: "else" },
      { id: "e5", source: "step_publish_high", target: "step_end", label: "next" },
      { id: "e6", source: "step_publish_low", target: "step_end", label: "next" },
      { id: "e7", source: "step_trigger", target: "step_fault", label: "onFault" },
      { id: "e8", source: "step_transform", target: "step_fault", label: "onFault" },
      { id: "e9", source: "step_if", target: "step_fault", label: "onFault" },
      { id: "e10", source: "step_publish_high", target: "step_fault", label: "onFault" },
      { id: "e11", source: "step_publish_low", target: "step_fault", label: "onFault" },
    ],
  };
}

```

## [48] workflow-mvp/desktop/src/index.css

```css
// FILE: workflow-mvp/desktop/src/index.css
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\index.css
// SIZE: 136 bytes
// MTIME: 2026-02-04T09:19:34.488578
// NOTE: Concatenated snapshot for review
body {
  margin: 0;
  font-family: "MS Sans Serif", Tahoma, sans-serif;
  background: #c0c0c0;
}
#root {
  min-height: 100vh;
}

```

## [49] workflow-mvp/desktop/src/main.tsx

```tsx
// FILE: workflow-mvp/desktop/src/main.tsx
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\src\main.tsx
// SIZE: 246 bytes
// MTIME: 2026-02-04T09:19:34.489791
// NOTE: Concatenated snapshot for review
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

```

## [50] workflow-mvp/desktop/tsconfig.json

```json
// FILE: workflow-mvp/desktop/tsconfig.json
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\tsconfig.json
// SIZE: 583 bytes
// MTIME: 2026-02-04T09:19:34.484529
// NOTE: Concatenated snapshot for review
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}

```

## [51] workflow-mvp/desktop/tsconfig.node.json

```json
// FILE: workflow-mvp/desktop/tsconfig.node.json
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\tsconfig.node.json
// SIZE: 244 bytes
// MTIME: 2026-02-04T09:19:34.486096
// NOTE: Concatenated snapshot for review
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true,
    "strict": true
  },
  "include": ["vite.config.ts"]
}

```

## [52] workflow-mvp/desktop/vite.config.ts

```ts
// FILE: workflow-mvp/desktop/vite.config.ts
// FULL: C:\Users\leopo\rust-studies\workflow-mvp\desktop\vite.config.ts
// SIZE: 224 bytes
// MTIME: 2026-02-04T09:19:34.247253
// NOTE: Concatenated snapshot for review
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});

```

## [53] workflow-mvp/docker-compose.yml

```yaml
# FILE: workflow-mvp/docker-compose.yml
# FULL: C:\Users\leopo\rust-studies\workflow-mvp\docker-compose.yml
# SIZE: 245 bytes
# MTIME: 2026-02-04T09:18:16.134119
# NOTE: Concatenated snapshot for review
services:
  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    volumes:
      - backend-data:/app/data
    environment:
      - DATA_DIR=/app/data

volumes:
  backend-data:

```
