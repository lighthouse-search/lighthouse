declare function list(id?: string[]): Promise<any>;
declare const error: {
    list: typeof list;
};
export default error;
