use crate::Config;

use rustls::client::danger::ServerCertVerifier;

/// The certificate validation mechanism that allows any certificate.
#[derive(Debug)]
pub struct InsecureCertificateValidator {
    debug: bool,
}

impl InsecureCertificateValidator {
    /// Initialize a new instance.
    pub fn new(config: &Config) -> Self {
        InsecureCertificateValidator {
            debug: config.debug,
        }
    }
}

impl ServerCertVerifier for InsecureCertificateValidator {
    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        if self.debug {
            eprintln!("DEBUG <- (!) Certificate received, but not verified");
        }

        todo!()
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        if self.debug {
            eprintln!("DEBUG <- (!) Certificate received, but not verified");
        }
        todo!()
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        todo!()
    }
}
