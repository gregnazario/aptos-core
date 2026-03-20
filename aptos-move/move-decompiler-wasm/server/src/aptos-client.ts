/**
 * Aptos blockchain client for fetching on-chain modules
 */

interface ModuleABI {
  address: string;
  name: string;
  friends?: string[];
  exposed_functions?: Array<{
    name: string;
    visibility: string;
    is_entry: boolean;
    generic_type_params: any[];
    params: string[];
    return: string[];
  }>;
  structs?: Array<{
    name: string;
    is_native: boolean;
    abilities: string[];
    generic_type_params: any[];
    fields: any[];
  }>;
}

interface AccountModule {
  bytecode: string;
  abi: ModuleABI;
  name: string;
  address: string;
}

export class AptosClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, ''); // Remove trailing slash
  }

  /**
   * Fetch all modules for an account
   */
  async getAccountModules(address: string): Promise<AccountModule[]> {
    const normalizedAddress = this.normalizeAddress(address);
    const url = `${this.baseUrl}/accounts/${normalizedAddress}/modules`;

    console.log(`Fetching modules from: ${url}`);

    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to fetch modules: ${response.status} ${error}`);
    }

    const modules = await response.json();

    return modules.map((module: any) => ({
      bytecode: module.bytecode,
      abi: module.abi,
      name: module.abi.name,
      address: module.abi.address,
    }));
  }

  /**
   * Fetch a specific module
   */
  async getAccountModule(address: string, moduleName: string): Promise<AccountModule> {
    const normalizedAddress = this.normalizeAddress(address);
    const url = `${this.baseUrl}/accounts/${normalizedAddress}/module/${moduleName}`;

    console.log(`Fetching module from: ${url}`);

    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Failed to fetch module: ${response.status} ${error}`);
    }

    const module = await response.json();

    return {
      bytecode: module.bytecode,
      abi: module.abi,
      name: module.abi.name,
      address: module.abi.address,
    };
  }

  /**
   * Normalize address format (add 0x prefix, pad to 64 chars)
   */
  private normalizeAddress(address: string): string {
    let addr = address.toLowerCase();

    // Remove 0x prefix if present
    if (addr.startsWith('0x')) {
      addr = addr.slice(2);
    }

    // Pad to 64 characters
    addr = addr.padStart(64, '0');

    return '0x' + addr;
  }

  /**
   * Check if address exists
   */
  async accountExists(address: string): Promise<boolean> {
    try {
      const normalizedAddress = this.normalizeAddress(address);
      const url = `${this.baseUrl}/accounts/${normalizedAddress}`;

      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      return response.ok;
    } catch {
      return false;
    }
  }
}
