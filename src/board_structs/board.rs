pub trait Board {
    fn read_from_fen<T>(fen: String) -> T;

    fn get_valid_moves<T>();
}
