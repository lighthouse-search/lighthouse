function search_builder(query) {
    const params = new URLSearchParams({
        query: query
    });

    return `/query?${params.toString()}`;
}

export { search_builder };