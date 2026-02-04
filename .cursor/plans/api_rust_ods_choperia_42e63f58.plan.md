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