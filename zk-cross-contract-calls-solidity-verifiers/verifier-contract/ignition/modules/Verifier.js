const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules")

const VerifierModule = buildModule("VerifierModule", (m) => {
    // We can use m.contract with the contract name and an array of constructor arguments.
    const verifier = m.contract("HonkVerifier", [])

    // Return the deployed contract instance for other modules to use
    return { verifier }
})

module.exports = VerifierModule