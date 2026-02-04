# Chopebrain ODS API

API Rust para o ODS da Choperia: autenticação JWT, mTLS (opcional) e endpoints de hambúrgueres (vendidos no mês e comandas antigas).

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
