// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Consensus Algorithm — kanonische Konsens-Implementierung (MVP-Start).
//! Specs: ATC-CONSENSUS-301..307 (DRAFT, SCR-0071) · Evidence: .atc/evidence/
//! Consensus hashing is not protocol-frozen. The MVP sequencing/selection
//! path uses SHA-256 as a standard cryptographic primitive; this does not
//! establish the final protocol hash. TownHash remains development-only and
//! is prohibited from consensus/security use until independently reviewed.

pub mod hash;
pub mod poh;
pub mod selection;
