pub trait GameDeckRepository {
    fn create_game_deck_object(&mut self, account_unique_id: i32) -> bool;
}