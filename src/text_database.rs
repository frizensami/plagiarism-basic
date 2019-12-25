use std::collections::HashMap;
type TextOwnerID = String;

/// Stores the corpus of trusted and untrusted strings
struct TextDatabase {
    /// Map
    trusted_texts: HashMap<TextOwnerID, String>,
    /// 
    untrusted_texts: HashMap<TextOwnerID, String>
}

