#!/bin/bash

# Variables
CERT_NAME="server"
DAYS_VALID=365
P12_PASSWORD="your_password"

# Generate a private key
openssl genrsa -out ${CERT_NAME}.key 2048

# Generate a certificate signing request (CSR)
openssl req -new -key ${CERT_NAME}.key -out ${CERT_NAME}.csr \
    -subj "/C=US/ST=State/L=City/O=Organization/OU=Department/CN=localhost"

# Generate a self-signed certificate
openssl x509 -req -days $DAYS_VALID -in ${CERT_NAME}.csr -signkey ${CERT_NAME}.key -out ${CERT_NAME}.crt

# Convert the certificate and key to PKCS #12 format
openssl pkcs12 -export -out ${CERT_NAME}.p12 -inkey ${CERT_NAME}.key -in ${CERT_NAME}.crt -password pass:${P12_PASSWORD}

echo "Certificate and key have been generated and stored in ${CERT_NAME}.p12"