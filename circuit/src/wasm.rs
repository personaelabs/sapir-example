// This file is used to embed the circuit/prover into the wasm binary.
use crate::my_synthesizer;

type Curve = ark_secq256k1::Projective;
use sapir::embed_to_wasm;
use sapir::wasm::prelude::*;

const DOMAIN_STR: &[u8] = b"hello-sapir";
embed_to_wasm!(my_synthesizer, Curve, DOMAIN_STR);
