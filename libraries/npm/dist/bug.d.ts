declare function list(id?: string[]): Promise<any>;
declare const bug: {
    list: typeof list;
};
export default bug;
