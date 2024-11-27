export interface UserProduct {
    ProductId: string;         // Unique identifier for the product
    Abbreviation: string;      // Abbreviation or short form of the product's name
    FullName: string;          // Full name of the product
    Description: string;       // Description of the product
    Icon: string;              // Icon representation of the product
    FrontendIp: string;        // IP address where the product's frontend is hosted
    FrontendPort: number;      // Port number where the product's frontend is served
}
