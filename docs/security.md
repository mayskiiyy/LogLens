# LogLens Security Model

## Authentication & Password Hashing
- Password hashes use **Argon2id** with secure random salts.
- Sessions use secure HttpOnly, SameSite cookies.

## Upload Safety & Zip Slip Safeguards
- Extracted archives are inspected prior to extraction.
- Paths are strictly sanitized to prevent directory traversal (`Zip Slip`).
- Extracted byte ratios and file entry counts are capped.

## Redaction Pipeline
- Automatic regex patterns mask authorization headers, bearer tokens, API keys, and passwords before export.
