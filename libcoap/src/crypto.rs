use std::fmt::Debug;

#[derive(Debug)]
pub struct CoapClientCryptoIdentity {
    pub identity: Box<CoapCryptoPskIdentity>,
    pub key: Box<CoapCryptoPsk>,
}

#[derive(Debug)]
pub struct CoapServerCryptoHint {
    pub hint: Box<CoapCryptoPskIdentity>,
    pub key: Box<CoapCryptoPsk>,
}

pub type CoapCryptoPskIdentity = [u8];
pub type CoapCryptoPsk = [u8];

pub trait CoapClientCryptoProvider: Debug {
    fn provide_info_for_hint(&mut self, hint: Option<&CoapCryptoPskIdentity>) -> Option<CoapClientCryptoIdentity>;
}

pub trait CoapServerCryptoProvider: Debug {
    fn provide_key_for_identity(&mut self, identity: &CoapCryptoPskIdentity) -> Option<Box<CoapCryptoPsk>>;

    fn provide_hint_for_sni(&mut self, sni: Option<&str>) -> Option<CoapServerCryptoHint>;
}

// TODO DTLS PKI/RPK