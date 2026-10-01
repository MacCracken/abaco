#!/usr/bin/env bash
# Regenerate the HTTPS currency-fetch test fixtures in tests/fixtures/tls/.
#
# tests/test_ccy_tls.tcyr serves these from a forked lib/tls.cyr native server
# on 127.0.0.1 and fetches through abaco's CurrencyCache_fetch. Nothing here is
# secret: the leaf keys exist only to let the test server complete a handshake.
# The CA private keys are made in a temporary directory and deleted, so no CA
# key is ever committed (and .gitignore ignores *.key / *.pem anyway, which is
# why the fixtures are named .crt / .der).
#
#   ca.crt               P-256 CA "abaco test CA" (PEM) — the trusted root
#   other-ca.crt         a second, unrelated P-256 CA (PEM) — the wrong root
#   server-cert.der      P-256 leaf from ca.crt, SAN DNS:localhost + IP:127.0.0.1
#   server-key.der       its key, PKCS#8 DER (lib/tls.cyr's tls_accept takes DER)
#   wrong-name-cert.der  P-256 leaf from ca.crt for DNS:abaco.invalid only, same
#                        key — a valid chain whose name matches neither
#                        "localhost" nor 127.0.0.1 (RFC 9525 / RFC 6761 .invalid)
#   dns-ip-cert.der      P-256 leaf from ca.crt whose only SAN is DNS:127.0.0.1,
#                        same key — an IP spelled as a DNS name, which must not
#                        match the IP literal 127.0.0.1 (RFC 9525 §6.3)
#   wild-ip-cert.der     the same with DNS:*.0.0.1 — a wildcard never matches
#                        an IP literal
#   rsa-ca.crt           RSA-2048 CA (PEM)
#   p384-cert.der        P-384 leaf signed by rsa-ca.crt with sha256WithRSA,
#                        SAN DNS:localhost + IP:127.0.0.1 — the client verifies
#                        an RSA chain signature and a P-384 CertificateVerify
#   p384-key.der         its key, PKCS#8 DER
#
# Every certificate is valid from 2025-01-01 to 2125-01-01 (an explicit start
# in the past, so a runner whose clock lags the generating machine still sees
# them as valid). Needs OpenSSL >= 3.4 for -not_before / -not_after.
#
# Usage: ./scripts/gen-tls-fixtures.sh [outdir]   (default tests/fixtures/tls)

set -euo pipefail

cd "$(dirname "$0")/.."
OUT="${1:-tests/fixtures/tls}"
mkdir -p "$OUT"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

NB=20250101000000Z
NA=21250101000000Z

# A self-signed CA: key type from $2 ("ec:P-256" / "rsa:2048").
mkca() {
    local name="$1" kind="$2" cn="$3"
    case "$kind" in
        ec:*)  openssl genpkey -algorithm EC -pkeyopt "ec_paramgen_curve:${kind#ec:}" -out "$TMP/$name.key" ;;
        rsa:*) openssl genpkey -algorithm RSA -pkeyopt "rsa_keygen_bits:${kind#rsa:}" -out "$TMP/$name.key" ;;
    esac
    openssl req -x509 -new -key "$TMP/$name.key" -subj "/CN=$cn" -sha256 \
        -not_before "$NB" -not_after "$NA" \
        -addext "basicConstraints=critical,CA:TRUE" \
        -addext "keyUsage=critical,keyCertSign,cRLSign" \
        -out "$OUT/$name.crt"
}

# A server leaf: key $2 (PEM in $TMP), issuer CA $3, SAN $4, DER cert to $5.
mkleaf() {
    local key="$1" ca="$2" san="$3" out="$4"
    openssl req -new -key "$TMP/$key" -subj "/CN=abaco test server" -out "$TMP/leaf.csr"
    printf '%s\n' "subjectAltName=$san" "basicConstraints=CA:FALSE" \
        "keyUsage=critical,digitalSignature" "extendedKeyUsage=serverAuth" > "$TMP/leaf.ext"
    openssl x509 -req -in "$TMP/leaf.csr" -CA "$OUT/$ca.crt" -CAkey "$TMP/$ca.key" \
        -CAcreateserial -CAserial "$TMP/$ca.srl" -sha256 \
        -not_before "$NB" -not_after "$NA" -extfile "$TMP/leaf.ext" \
        -outform DER -out "$OUT/$out"
}

mkca ca ec:P-256 "abaco test CA"
mkca other-ca ec:P-256 "abaco other CA"
mkca rsa-ca rsa:2048 "abaco test RSA CA"

openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out "$TMP/server.key"
openssl pkcs8 -topk8 -nocrypt -in "$TMP/server.key" -outform DER -out "$OUT/server-key.der"
mkleaf server.key ca "DNS:localhost,IP:127.0.0.1" server-cert.der
mkleaf server.key ca "DNS:abaco.invalid" wrong-name-cert.der
mkleaf server.key ca "DNS:127.0.0.1" dns-ip-cert.der
mkleaf server.key ca "DNS:*.0.0.1" wild-ip-cert.der

openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-384 -out "$TMP/p384.key"
openssl pkcs8 -topk8 -nocrypt -in "$TMP/p384.key" -outform DER -out "$OUT/p384-key.der"
mkleaf p384.key rsa-ca "DNS:localhost,IP:127.0.0.1" p384-cert.der

ls -l "$OUT"
