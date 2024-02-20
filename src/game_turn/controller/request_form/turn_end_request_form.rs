use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_field_unit::service::request::apply_status_effect_damage_iteratively_request::ApplyStatusEffectDamageIterativelyRequest;
use crate::game_round::service::request::next_game_turn_request::NextGameRoundRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;

#[derive(Debug)]
pub struct TurnEndRequestForm {
    session_id: String,

}

impl TurnEndRequestForm {
    pub fn new(session_id: String,) -> Self {
        TurnEndRequestForm {
            session_id,

        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_apply_status_effect_damage_iteratively_request(&self, account_unique_id: i32) -> ApplyStatusEffectDamageIterativelyRequest {
        ApplyStatusEffectDamageIterativelyRequest::new(account_unique_id)
    }

    pub fn to_next_turn_request(&self, account_unique_id: i32) -> NextTurnRequest {
        NextTurnRequest::new(account_unique_id)
    }

    pub fn to_next_round_request(&self) -> NextGameRoundRequest {
        NextGameRoundRequest::new(self.session_id.clone())
    }

    pub fn to_draw_cards_from_deck_request(&self, opponent_account_unique_id: i32) -> DrawCardsFromDeckRequest {
        DrawCardsFromDeckRequest::new(opponent_account_unique_id, 1)
    }

    pub fn to_add_field_energy_request(&self, opponent_account_unique_id: i32) -> AddFieldEnergyWithAmountRequest {
        AddFieldEnergyWithAmountRequest::new(opponent_account_unique_id, 1)
    }
}