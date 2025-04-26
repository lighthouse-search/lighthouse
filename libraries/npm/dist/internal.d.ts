declare function getFileBinary(file: any): Promise<unknown>;
declare function generatePublicPrivateKey(): Promise<{
    publicKeyNaked: string;
    privateKeyNaked: string;
}>;
export { getFileBinary, generatePublicPrivateKey };
