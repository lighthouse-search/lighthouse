declare function list(): Promise<any>;
declare function update(data: object): Promise<any>;
declare const job: {
    list: typeof list;
    update: typeof update;
};
export default job;
