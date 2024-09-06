use wasm_bindgen::prelude::*;
use base64::{encode_config, decode_config, STANDARD, URL_SAFE};

/// Base64 encoding function for Ethereum smart contracts
/// Encodes input bytes into a Base64 string.
#[wasm_bindgen]
pub fn base64_encode(data: &[u8]) -> String {
    encode_config(data, STANDARD)
}

/// Base64 decoding function for Ethereum smart contracts
/// Decodes a Base64 string back into bytes. Now with proper error handling.
#[wasm_bindgen]
pub fn base64_decode(encoded_data: &str) -> Result<Vec<u8>, JsValue> {
    decode_config(encoded_data, STANDARD).map_err(|e| JsValue::from_str(&format!("Invalid Base64 input: {}", e)))
}

/// URL-safe Base64 encoding function
/// Encodes input bytes into a URL-safe Base64 string.
#[wasm_bindgen]
pub fn base64_encode_url_safe(data: &[u8]) -> String {
    encode_config(data, URL_SAFE)
}

/// URL-safe Base64 decoding function
/// Decodes a URL-safe Base64 string back into bytes. Now with proper error handling.
#[wasm_bindgen]
pub fn base64_decode_url_safe(encoded_data: &str) -> Result<Vec<u8>, JsValue> {
    decode_config(encoded_data, URL_SAFE).map_err(|e| JsValue::from_str(&format!("Invalid Base64 input: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let data = b"Hello, Ethereum!";
        let encoded = base64_encode(data);
        assert_eq!(encoded, "SGVsbG8sIEV0aGVyZXVtIQ==");
    }

    #[test]
    fn test_base64_decode() {
        let encoded = "SGVsbG8sIEV0aGVyZXVtIQ==";
        let decoded = base64_decode(encoded).expect("Failed to decode");
        assert_eq!(decoded, b"Hello, Ethereum!");
    }

    #[test]
    fn test_base64_encode_url_safe() {
        let data = b"Hello, Ethereum!";
        let encoded = base64_encode_url_safe(data);
        assert_eq!(encoded, "SGVsbG8sIEV0aGVyZXVtIQ==");
    }

    #[test]
    fn test_base64_decode_url_safe() {
        let encoded = "SGVsbG8sIEV0aGVyZXVtIQ==";
        let decoded = base64_decode_url_safe(encoded).expect("Failed to decode");
        assert_eq!(decoded, b"Hello, Ethereum!");
    }

    #[test]
    fn test_invalid_base64_input() {
        let invalid_data = "InvalidBase64";
        let result = base64_decode(invalid_data);
        assert!(result.is_err(), "Expected error on invalid Base64 input");
    }
}
