// src/cli/consts/constants.rs

pub const FILE_NAME_AES_GCM_KEY: &str = ".aes_key";
pub const SHA_STR_HELP: &str = "Your strings come here and will go through a SHA algorithm.";
pub const SHA_FILE_HELP: &str = "Enter your filenames here and its contents will get processed inside a SHA algorithm.";
pub const AESGCM_STR_HELP: &str = "Your strings come here and will go through an AES-GCM algorithm.";
pub const AESGCM_FILE_HELP: &str = "Enter your filenames here and its contents will get processed inside a AES-GCM algorithm.";
pub const SHA_TYPE_HELP: &str = "The SHA algorithm you pick.";
pub const AESGCM_TYPE_HELP: &str = "The AES-GCM algorithm you pick.";
pub const NO_VERBOSE_HELP: &str = "Silence extra commentary.";
pub const AESGCM_KEY_GEN_HELP: &str = "Saves your AES-Key in a file calleed '.aes_key'. DO NOT COMMIT THIS ON GITHUB OR ANY OTHER PLATFORM.";
pub const AESGCM_KEYFILE_HELP: &str = "Use '.aes_key' to decrypt your encryption.";
pub const BASE64_TYPE_HELP: &str = "The Base64 algorithm you pick.";