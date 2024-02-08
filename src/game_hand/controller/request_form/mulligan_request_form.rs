use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;
use crate::game_deck::service::request::game_deck_card_redraw_request::GameDeckCardRedrawRequest;
use crate::game_hand::service::request::put_cards_on_deck_request::PutCardsOnDeckRequest;
use crate::game_protocol_validation::service::request::check_cards_from_hand_request::CheckCardsFromHandRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;


#[derive(Debug)]
pub struct MulliganRequestForm {
    session_id: String,
    will_be_changed_card_list: Vec<String>,
}

impl MulliganRequestForm {
    pub fn new(session_id: String, will_be_changed_card_list: Vec<String>) -> Self {
        MulliganRequestForm {
            session_id,
            will_be_changed_card_list
        }
    }
    pub fn to_put_cards_on_deck_request(&self) -> PutCardsOnDeckRequest {
        PutCardsOnDeckRequest::new(
            self.session_id.clone(),
            self.will_be_changed_card_list.clone()
        )
    }
    pub fn to_shuffle_and_redraw_card_request(&self) -> GameDeckCardRedrawRequest {
        GameDeckCardRedrawRequest::new(
            self.session_id.clone(),
            self.will_be_changed_card_list.clone().len() as i32
        )
    }
    pub fn to_get_value_with_key_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(&self.session_id)
    }
    pub fn to_check_cards_from_hand_request(&self) -> CheckCardsFromHandRequest {
        let maybe_hand_card_list =
            VectorStringToVectorInteger::vector_string_to_vector_i32(self.will_be_changed_card_list.clone());

        CheckCardsFromHandRequest::new(&self.session_id, maybe_hand_card_list)
    }
}