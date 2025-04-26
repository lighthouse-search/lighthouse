declare function list(id: string[] | undefined, filter: any): Promise<any>;
declare function update(actions: object): Promise<any>;
declare const user_rating: {
    list: typeof list;
    update: typeof update;
};
export default user_rating;
