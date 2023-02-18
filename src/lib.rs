use ethers::prelude::abigen;

// This works
abigen!(Seaport11, "src/abi_1_1.json");

// But these don't!
abigen!(Seaport12, "src/abi_1_2.json");
abigen!(Seaport13, "src/abi_1_3.json");
