mod wasm;

use sapir::{ark_ff::Field, constraint_system::ConstraintSystem};

/**
 * A simple circuit that multiplies two private inputs and exposes the result as a public input.
 */
pub fn my_synthesizer<F: Field>(cs: &mut ConstraintSystem<F>) {
    let a = cs.alloc_priv_input();
    let b = cs.alloc_priv_input();

    let c = a * b;

    cs.expose_public(c);
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_secq256k1::Fr;
    use sapir::{frontend::circuit::Circuit, spartan::spartan::Spartan};

    type Curve = ark_secq256k1::Projective;

    #[test]
    fn test_witness_gen() {
        let mut circuit = Circuit::new(my_synthesizer);
        let pub_inputs = vec![Fr::from(6u32)];
        let priv_inputs = vec![Fr::from(2u32), Fr::from(3u32)];
        let witness = circuit.gen_witness(&pub_inputs, &priv_inputs);

        assert!(circuit.is_sat(&witness, &pub_inputs));
    }

    #[test]
    fn test_prove() {
        let mut circuit = Circuit::new(my_synthesizer);
        let pub_inputs = vec![Fr::from(6u32)];
        let priv_inputs = vec![Fr::from(2u32), Fr::from(3u32)];
        let witness = circuit.gen_witness(&pub_inputs, &priv_inputs);

        let r1cs = circuit.to_r1cs();
        let spartan = Spartan::<Curve>::new(b"hello-sapir", r1cs);
        // Generate a proof
        let (proof, _) = spartan.prove(&witness, &pub_inputs);

        // Verify the proof
        spartan.verify(&proof, false);
    }
}
