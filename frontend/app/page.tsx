"use client";

export default function Home() {
  const onProveClick = async () => {
    const a = 1;
    const b = 2;
    const circuit = await import("circuit");
    await circuit.prepare();

    const c = a * b;

    const aBytes = new Uint8Array(32);
    const bBytes = new Uint8Array(32);
    const cBytes = new Uint8Array(32);

    aBytes[31] = a;
    bBytes[31] = b;
    cBytes[31] = c;

    const privInputs  = new Uint8Array(32 * 2);
    privInputs.set(aBytes, 0);
    privInputs.set(bBytes, 32);

    const proof = await circuit.prove(cBytes, privInputs);

    const verified = await circuit.verify(proof);
    if (verified) {
      window.alert("Proof generated and verified!")
    } else {
      window.alert("Something went wrong!")
    }
  }


  return (
    <div className="flex flex-col w-[100vw] items-center justify-center">
      <button onClick={onProveClick}>Prove</button>
    </div>
      );
}
