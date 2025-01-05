function commas_to_number(number) {
    if (!number) { return null; }
    return number.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

export { commas_to_number }