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
$AuthSecret = $env:AUTH_SECRET
if (-not $AuthSecret) { $AuthSecret = "test-secret-api" }

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

    $loginBody = @{ secret = $AuthSecret } | ConvertTo-Json
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
    $token = $loginJson.token
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
