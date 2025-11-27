const hre = require('hardhat')

async function main() {
    console.log('Deploying Verifier contract to', hre.network.name, '...\n')

    // Get signers
    const [deployer] = await hre.ethers.getSigners()
    console.log('Deploying with account:', deployer.address)

    // Get account balance
    try {
        const balance = await hre.ethers.provider.getBalance(deployer.address)
        console.log('Account balance:', hre.ethers.formatEther(balance), 'PAS\n') 
    } catch (error) {
        console.log('Could not fetch balance\n')
    }

    // Deploy Verifier contract
    console.log('Deploying Verifier contract with arguments:', n, logN, numPublicInputs)
    const Verifier = await hre.ethers.getContractFactory('HonkVerifier')
    // Pass the constructor arguments
    const verifier = await Verifier.deploy() 

    console.log('Waiting for deployment...')
    await verifier.waitForDeployment()

    const verifierAddress = await verifier.getAddress()
    console.log('\n✅ Verifier deployed successfully!')
    console.log('Contract address:', verifierAddress)

    console.log('\n📝 Save this address to interact with your contract:')
    console.log(
        `CONTRACT_ADDRESS=${verifierAddress} npx hardhat run scripts/interact.js --network ${hre.network.name}`
    )
    
    // Optional: Log arguments for verification on Etherscan
    console.log('\nArguments for Etherscan verification:')
    console.log(JSON.stringify([n, logN, numPublicInputs]))
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error)
        process.exit(1)
    })
