using StellarDotnetSdk.Accounts;
using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using dotenv.net;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");
    // var soroban_server = new Server("https://soroban-testnet.stellar.org/");

    async static Task Main(string[] args)
    {
        DotEnv.Load();
        if(args.Length == 0 || args.Length > 1 || (args[0] != "testnet" && args[0] != "publicnet")){
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Please provide a network: testnet or publicnet");
            return;
        }
        var network = args[0];
        switch (network) {
            case "testnet":
                Console.WriteLine("Using testnet");
                Network.UseTestNetwork();
                break;
            case "mainnet":
                Console.WriteLine("Using mainnet");
                Network.UsePublicNetwork();
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Mainnet is not yet supported");
                return;
        }
        Console.ResetColor();

        var keypair = KeyPair.Random();
        Console.WriteLine("Generated public key: " + keypair.AccountId);

        var server = new Server("https://horizon-testnet.stellar.org");
        var friendbot = server.TestNetFriendBot;

        try {
            var response = await friendbot.FundAccount(keypair.AccountId).Execute();
            Console.ForegroundColor = ConsoleColor.Green;
            Console.WriteLine(response.Hash);
            Console.ResetColor();
        } catch (Exception ex) {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.Error.WriteLine("Error while funding account: " + ex.Message);
            Console.ResetColor();
        }

        //View account balances
        var account = await server.Accounts.Account(keypair.AccountId);
        if(account.Balances[0].BalanceString != "10000.0000000"){
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Account not funded");
            Console.ResetColor();
        } else {
            Console.ForegroundColor = ConsoleColor.Green;
            Console.WriteLine("Ok.");
            Console.ResetColor();
        }

        string? secretKey = Environment.GetEnvironmentVariable("SOROSWAP_MINT_SECRET_KEY");
        if (string.IsNullOrEmpty(secretKey))
        {
            Console.WriteLine("Error: La clave secreta no está configurada.");
            return;
        }
        var soroban_server = new SorobanServer("https://soroban-testnet.stellar.org/");
        var soroswap_admin = KeyPair.FromSecretSeed(secretKey);
        var soroswap_admin_account = await server.Accounts.Account(soroswap_admin.AccountId);
        var USDC_address = new SCContractId("CARDT45FED2I3FKESPMHDFV3ZMR6VH5ZHCFIOPH6TPSU35GPB6QBBCSU");

        var mint_args = new StellarDotnetSdk.Soroban.SCVal[] {
            new StellarDotnetSdk.Soroban.SCAccountId(account.AccountId),
            new StellarDotnetSdk.Soroban.SCInt128("100000000000"),
        };
        var mint_symbol = new StellarDotnetSdk.Soroban.SCSymbol("mint");

        var mintUSDC = new InvokeContractOperation(USDC_address, mint_symbol, mint_args);
        var mintTransaction = new TransactionBuilder(soroswap_admin_account)
            .AddOperation(mintUSDC)
            .SetFee(10000)
            .Build();   

        var simulated_mint_transaction = await soroban_server.SimulateTransaction(mintTransaction);

        var transaction_data = simulated_mint_transaction.SorobanTransactionData;
        var authorization_data = simulated_mint_transaction.SorobanAuthorization;
        var resource_fee  = simulated_mint_transaction.MinResourceFee;

        if (transaction_data != null && authorization_data != null && resource_fee != null)
        {
            mintTransaction.SetSorobanTransactionData(transaction_data);
            mintTransaction.SetSorobanAuthorization(authorization_data);
            mintTransaction.AddResourceFee(resource_fee.Value + 100000);
        }
        mintTransaction.Sign(soroswap_admin);
        
        var mintResult = await soroban_server.SendTransaction(mintTransaction);

        Console.WriteLine($"Mint Transaction Status: {mintResult.Status}");
        ;

        while (true)
        {
            var tx_status = await soroban_server.GetTransaction(mintResult.Hash);


            if (tx_status.Status.ToString() == "SUCCESS")
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine($"Mint Transaction Status: {tx_status.Status}");
                Console.WriteLine($"Mint Transaction hash: {mintResult.Hash}");
                break;
            }
            else if (tx_status.Status.ToString() == "FAILED" || tx_status.Status.ToString() == "ERROR")
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine($"Mint Transaction Status: {tx_status.Status}");
                Console.WriteLine($"Mint Transaction hash: {mintResult.Hash}");
                break;
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Yellow;
                Console.WriteLine($"Mint Transaction Status: {tx_status.Status}, retrying in 20ms");
                await Task.Delay(20);
            }
        }
        Console.ResetColor();

        var get_balance_args = new StellarDotnetSdk.Soroban.SCVal[] {
            new StellarDotnetSdk.Soroban.SCAccountId(keypair.AccountId),
        };
        var balance_symbol = new StellarDotnetSdk.Soroban.SCSymbol("balance");

        var invokeContractOperation = new InvokeContractOperation(USDC_address, balance_symbol, get_balance_args);
        var transaction = new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();

        var simulatedTransactionResult = await soroban_server.SimulateTransaction(transaction);
        if (simulatedTransactionResult.Results != null)
        {
            foreach (var result in simulatedTransactionResult.Results)
            {
                Console.WriteLine($"🟢Result: {result.Xdr}");
            }
        }

        string? xdrString = simulatedTransactionResult.Results?[0].Xdr;
        if (xdrString == null){

            Console.WriteLine("XDR string is null.");
            return;
        }

        var result_xdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(result_xdr);
        Console.WriteLine($"Balance result: {xdr.I128.Lo.InnerValue}");
        return;
    }
} 