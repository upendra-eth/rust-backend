use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index)).
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// OpenSSL is a widely-used open-source library that provides a set of cryptographic functions and tools to secure communications on the internet. It works by implementing various cryptographic protocols and algorithms to ensure the confidentiality, integrity, and authenticity of data in transit.

// Here's an overview of how OpenSSL works:

// Cryptographic Algorithms: OpenSSL provides a wide range of cryptographic algorithms for encryption, decryption, hashing, and digital signatures. These algorithms are the building blocks of secure communication.

// Secure Communication Protocols: OpenSSL supports various secure communication protocols, including SSL (Secure Sockets Layer) and its successor TLS (Transport Layer Security). These protocols establish secure connections between clients (e.g., web browsers) and servers (e.g., web servers).

// Key Management: OpenSSL handles the generation, storage, and management of cryptographic keys. Keys are used to encrypt and decrypt data, as well as to establish secure connections between parties.

// Certificates: OpenSSL supports X.509 certificates, which are used to authenticate the identity of servers and clients. Certificates include public keys and are signed by trusted certificate authorities (CAs).

// Digital Signatures: OpenSSL allows the creation and verification of digital signatures, which are used to prove the authenticity and integrity of data. Digital signatures are crucial for ensuring that data has not been tampered with during transmission.

// Encryption and Decryption: OpenSSL supports symmetric and asymmetric encryption. Symmetric encryption uses the same key for both encryption and decryption, while asymmetric encryption uses a pair of public and private keys. Data can be encrypted with the recipient's public key and decrypted with their private key.

// Hashing: OpenSSL can compute cryptographic hash functions (e.g., SHA-256) to ensure data integrity. Hashes are used to verify that data has not been altered during transmission.

// Secure Random Number Generation: OpenSSL provides a secure source of random numbers, which is important for generating cryptographic keys and nonces.

// TLS Handshake: In a TLS handshake, the client and server exchange information to agree on encryption algorithms and establish a shared secret key. This key is used for secure communication during the session.

// Data Integrity and Confidentiality: Once a secure connection is established, data is encrypted to ensure confidentiality. Additionally, data is protected from tampering using digital signatures and integrity checks.

// Secure Socket Layer (SSL) and Transport Layer Security (TLS): OpenSSL is used by web servers and clients to enable secure HTTPS connections. When you visit a website with "https://" in the URL, it means that OpenSSL is likely being used to secure the data transfer.

// Trust Chain Verification: OpenSSL checks the trustworthiness of X.509 certificates by verifying that they are signed by a trusted certificate authority (CA). This helps prevent man-in-the-middle attacks.

// Integration: OpenSSL is integrated into various software and applications, including web servers (e.g., Apache, Nginx), email servers, VPNs, and other network-related tools.

// OpenSSL plays a critical role in securing data and communications over the internet. It is a fundamental component of modern cybersecurity and encryption practices, ensuring that sensitive information remains confidential and protected from unauthorized access or modification during transmission.

// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// Rustls and OpenSSL are two popular libraries for implementing TLS (Transport Layer Security) and SSL (Secure Sockets Layer) encryption in software applications, particularly for securing network communication. They serve similar purposes but have some key differences. Here's a comparison of Rustls and OpenSSL:

// Language:

// Rustls is written in the Rust programming language, which is known for its focus on safety and performance.
// OpenSSL is written in C, a language with a long history in the world of cryptography and networking.
// Safety and Memory Safety:

// Rustls places a strong emphasis on memory safety. Being written in Rust, it leverages Rust's safety features to prevent common programming errors like null pointer dereferences and buffer overflows. This makes it less prone to security vulnerabilities like buffer overflows.
// OpenSSL, being written in C, has had its share of security vulnerabilities over the years, largely due to memory management issues and unsafe practices in C.
// Integration and Popularity:

// OpenSSL has been widely adopted and integrated into many software applications, making it a well-established choice for securing network communication.
// Rustls is relatively newer and may not have the same level of integration and adoption as OpenSSL.
// Code Complexity:

// Rustls is designed with a cleaner and more modern codebase, which can be easier to work with and understand, especially for developers familiar with Rust.
// OpenSSL has a complex and sometimes convoluted codebase, which can make it challenging to work with and audit.
// Licensing:

// Rustls is licensed under the Apache License 2.0, which is a permissive open-source license.
// OpenSSL historically used a custom license that had some licensing complexities. However, it has transitioned to the Apache License 2.0 as well in later versions.
// Performance:

// OpenSSL has a reputation for being highly performant, and it's optimized for various hardware platforms.
// Rustls is designed for good performance but may not be as optimized as OpenSSL for some specific use cases.
// Features and Flexibility:

// OpenSSL is a feature-rich library with support for a wide range of cryptographic algorithms and protocols, making it suitable for a broad range of applications.
// Rustls, while capable, may not support all the same features and options as OpenSSL, especially in complex scenarios.
// Memory Overhead:

// Rustls generally has lower memory overhead due to its memory-safe design.
// OpenSSL may have a bit more memory overhead due to its C heritage and less focus on memory safety.
// Ecosystem and Documentation:

// OpenSSL has a well-established ecosystem, extensive documentation, and a large community of users and contributors.
// Rustls, being relatively newer, is still building its ecosystem and may have a smaller community.
// In summary, the choice between Rustls and OpenSSL depends on your specific requirements and constraints. If you prioritize memory safety, modern codebase, and Rust integration, Rustls may be a good choice. If you need a highly performant and widely adopted library with extensive features, OpenSSL remains a strong contender. The choice may also depend on the programming language and ecosystem of the software you're developing or the existing software stack you're working with.

// Regenerate
