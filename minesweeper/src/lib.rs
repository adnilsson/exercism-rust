pub fn annotate(minefield: &[&str]) -> Vec<String> {
    todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    // Plan1:
    // 1. iterate over rows
    //  1.1 if element is a  mine, mark or increment the counter for  the previous and next elements in the row (may be oob)
    // 2. repeat for columns intead of rows

    // Plan2:
    // 1. write a function to compute all surronding coordinates given the coordinate of a mine
    // 2. iterate over the boardl
    //  2.1 if the element is a mine, compute surrounding coordinates and push to a vector
    // 3. filter out oob coordinates, count duplicates
    // 4. fill out result board
}
