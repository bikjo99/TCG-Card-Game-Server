use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::game_card_passive_skill_service_impl::GameCardPassiveSkillServiceImpl;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_unit::controller::game_card_unit_controller::GameCardUnitController;
use crate::game_card_unit::controller::request_form::attack_game_main_character_request_form::AttackGameMainCharacterRequestForm;
use crate::game_card_unit::controller::request_form::attack_unit_request_form::AttackUnitRequestForm;
use crate::game_card_unit::controller::request_form::deploy_unit_request_form::DeployUnitRequestForm;
use crate::game_card_unit::controller::response_form::attack_game_main_character_response_form::AttackGameMainCharacterResponseForm;
use crate::game_card_unit::controller::response_form::attack_unit_response_form::AttackUnitResponseForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_card_unit::service::game_card_unit_service::GameCardUnitService;

use crate::game_card_unit::service::game_card_unit_service_impl::GameCardUnitServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service::GameFieldUnitActionPossibilityValidatorService;
use crate::game_field_unit_action_possibility_validator::service::game_field_unit_action_possibility_validator_service_impl::GameFieldUnitActionPossibilityValidatorServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;
use crate::game_main_character::service::response::apply_damage_to_main_character_response::ApplyDamageToMainCharacterResponse;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardUnitControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>> ,
}

impl GameCardUnitControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_card_passive_skill_service: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               game_field_unit_action_possibility_validator_service: Arc<AsyncMutex<GameFieldUnitActionPossibilityValidatorServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,) -> Self {

        GameCardUnitControllerImpl {
            game_hand_service,
            battle_room_service,
            game_card_unit_service,
            game_field_unit_service,
            redis_in_memory_service,
            game_tomb_service,
            notify_player_action_service,
            game_card_passive_skill_service,
            game_protocol_validation_service,
            game_field_unit_action_possibility_validator_service,
            game_main_character_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardUnitControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardUnitControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardUnitControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardUnitServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameCardPassiveSkillServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            GameFieldUnitActionPossibilityValidatorServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }
}

#[async_trait]
impl GameCardUnitController for GameCardUnitControllerImpl {
    async fn request_to_deploy_unit(&self, deploy_unit_request_form: DeployUnitRequestForm) -> DeployUnitResponseForm {
        println!("GameCardUnitControllerImpl: request_to_deploy_unit()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(deploy_unit_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return DeployUnitResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_id_string = deploy_unit_request_form.get_unit_id();
        let unit_card_id = unit_id_string.parse::<i32>().unwrap();

        // 2. Game Protocol Validation Service 호출하여 Hand 에 있는지 확인하여 해킹 여부 검증
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let check_protocol_hacking_response =
            game_protocol_validation_service_guard.check_protocol_hacking(
                deploy_unit_request_form.to_check_protocol_hacking_request(account_unique_id,
                                                                           unit_card_id)).await;

        if !check_protocol_hacking_response.is_success() {
            println!("해킹범을 검거합니다!");
            return DeployUnitResponseForm::new(false)
        }

        // 3. Card Kinds Service 를 호출하여 실제 유닛 카드가 맞는지 확인
        let is_it_unit_response =
            game_protocol_validation_service_guard.is_it_unit_card(
                deploy_unit_request_form.to_is_it_unit_card_request(unit_card_id)).await;

        if !is_it_unit_response.is_success() {
            println!("유닛 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return DeployUnitResponseForm::new(false)
        }

        // 4. 신화 등급의 경우 라운드 체크하도록 함
        let can_use_card_response =
            game_protocol_validation_service_guard.can_use_card(
                deploy_unit_request_form.to_can_use_card_request(account_unique_id,
                                                                 unit_card_id)).await;

        if !can_use_card_response.is_success() {
            println!("신화 등급 카드는 5라운드부터 사용 가능합니다.");
            return DeployUnitResponseForm::new(false)
        }

        // 5. Hand Service 호출하여 카드 사용
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        let use_game_hand_unit_card_response =
            game_hand_service_guard.use_unit_card(
                deploy_unit_request_form.to_use_game_hand_unit_card_request(account_unique_id,
                                                                            unit_card_id)).await;

        let usage_hand_card_id = use_game_hand_unit_card_response.get_found_unit_card_id();

        let mut game_card_service_guard =
            self.game_card_unit_service.lock().await;

        let unit_card_info_response =
            game_card_service_guard.summary_unit_card(
                deploy_unit_request_form.to_summary_unit_card_info_request(unit_card_id)).await;

        // 5. Battle Field 에 유닛 배치
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let add_unit_to_game_field_response =
            game_field_unit_service_guard.add_unit_to_game_field(
                deploy_unit_request_form.to_add_unit_to_game_field_request(
                    account_unique_id,
                    usage_hand_card_id,
                    unit_card_info_response.get_unit_race(),
                    unit_card_info_response.get_unit_grade(),
                    unit_card_info_response.get_unit_attack_point(),
                    unit_card_info_response.get_unit_health_point(),
                    unit_card_info_response.get_unit_attack_required_energy(),
                    unit_card_info_response.has_first_passive_skill(),
                    unit_card_info_response.has_second_passive_skill(),
                    unit_card_info_response.has_third_passive_skill(),
                    unit_card_info_response.get_passive_status_list().clone())).await;

        if add_unit_to_game_field_response.get_placed_unit_index() == -1 {
            println!("필드에 유닛 배치 중 문제가 발생하였습니다.");
            return DeployUnitResponseForm::new(false)
        }

        // 6. 상대방의 고유 id 값을 확보
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let find_opponent_by_account_id_response =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                deploy_unit_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await;

        // 7. 유닛이 출격하자마자 발동하는 스킬이 있는지 확인
        let game_card_passive_skill_service_guard =
            self.game_card_passive_skill_service.lock().await;

        let passive_skill_response =
            game_card_passive_skill_service_guard.summary_passive_skill(
                deploy_unit_request_form.to_summary_passive_skill_request(usage_hand_card_id)).await;

        drop(game_card_passive_skill_service_guard);

        // TODO: 여기서도 Domain 분리를 고려하면 좋을텐데 우선은 배제합니다.
        if !passive_skill_response.is_empty() {

            // TODO: 상황에 따라 공격 / 버프 등등에 대한 고찰이 들어가면 더 좋았을 것임
            println!("처리 할 패시브 효과가 있습니다");

            game_field_unit_service_guard
                .apply_passive_skill_list(
                    deploy_unit_request_form.to_apply_passive_skill_list_request(
                        account_unique_id,
                        add_unit_to_game_field_response.get_placed_unit_index(),
                        find_opponent_by_account_id_response.get_opponent_unique_id(),
                        passive_skill_response.get_passive_skill_effect_list().clone())).await;
        }

        // 9. 상대방에게 당신이 무엇을 했는지 알려줘야 합니다
        let mut notify_player_action_service_guard =
            self.notify_player_action_service.lock().await;

        let notify_to_opponent_you_deploy_unit_response =
            notify_player_action_service_guard.notify_opponent_you_deploy_unit(
                deploy_unit_request_form.to_notify_to_opponent_what_you_do_request(
                    find_opponent_by_account_id_response.get_opponent_unique_id(),
                    usage_hand_card_id)).await;

        if !notify_to_opponent_you_deploy_unit_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return DeployUnitResponseForm::new(false)
        }

        DeployUnitResponseForm::new(true)
    }

    async fn request_to_attack_unit(
        &self, attack_unit_request_form: AttackUnitRequestForm) -> AttackUnitResponseForm {

        println!("GameCardUnitControllerImpl: request_to_attack_unit()");

        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(attack_unit_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return AttackUnitResponseForm::new(false)
        }

        // TODO: 프로토콜 검증 (지금 이거 신경 쓸 때가 아님)

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let attacker_unit_card_index_string = attack_unit_request_form.get_attacker_unit_index();
        let attacker_unit_card_index = attacker_unit_card_index_string.parse::<i32>().unwrap();

        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let attacker_unit_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                attack_unit_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_found_opponent_unit_id();

        let mut game_card_unit_service_guard =
            self.game_card_unit_service.lock().await;

        let attacker_unit_required_energy =
            game_card_unit_service_guard.summary_unit_card(
                attack_unit_request_form.to_summary_unit_card_info_request(
                    attacker_unit_id)).await.get_unit_attack_required_energy();

        drop(game_card_unit_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;

        let is_unit_basic_attack_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_unit_basic_attack_possible(
                attack_unit_request_form.to_is_unit_basic_attack_possible_request(
                    account_unique_id, attacker_unit_card_index, attacker_unit_required_energy)).await;

        if !is_unit_basic_attack_possible_response.is_possible() {
            return AttackUnitResponseForm::new(false)
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 유닛 인덱스에서 기본 공격력 정보 확보
        let find_attacker_unit_attack_point_response =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_unit_request_form
                    .to_acquire_unit_attack_point_request(
                        account_unique_id,
                        attacker_unit_card_index)).await;

        // extra effect 가지고 있는지 여부
        let attacker_unit_extra_effect_list =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attack_unit_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_extra_status_effect_list().clone();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attack_unit_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 피격 유닛이 기본 공격 면역을 가지고 있는지 확인
        let opponent_target_unit_card_index_string = attack_unit_request_form.get_target_unit_index();
        let opponent_target_unit_card_index = opponent_target_unit_card_index_string.parse::<i32>().unwrap();

        let opponent_target_unit_passive_status_list =
            game_field_unit_service_guard.acquire_unit_passive_status_list(
                attack_unit_request_form
                    .to_acquire_unit_passive_status_list_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_passive_status_effect_list().clone();

        if opponent_target_unit_passive_status_list.contains(&PassiveStatus::PhysicalImmunity) {
            println!("기본 공격 면역 패시브로 인해 공격을 가할 수 없습니다.");
            return AttackUnitResponseForm::new(false)
        }

        // 적 타겟 유닛을 효과를 가지고 공격
        let attack_opponent_target_unit_with_extra_effect_response =
            game_field_unit_service_guard.attack_target_unit_with_extra_effect(
                attack_unit_request_form
                    .to_attack_target_unit_with_extra_effect_request(
                        opponent_unique_id,
                        find_attacker_unit_attack_point_response.get_attack_point(),
                        &attacker_unit_extra_effect_list,
                        opponent_target_unit_card_index)).await;

        if !attack_opponent_target_unit_with_extra_effect_response.is_success() {
            println!("적 유닛 공격에 실패했습니다.");
            return AttackUnitResponseForm::new(false)
        }

        // 반격 이전 공격 유닛의 기본 지속 상태 확보
        let attacker_unit_passive_status_list =
            game_field_unit_service_guard.acquire_unit_passive_status_list(
                attack_unit_request_form
                    .to_acquire_unit_passive_status_list_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_passive_status_effect_list().clone();

        // 공격 유닛이 기본 공격 면역일 경우 반격 무효 처리
        if attacker_unit_passive_status_list.contains(&PassiveStatus::PhysicalImmunity) {
            println!("공격한 유닛이 기본 공격 면역이 존재하여 반격이 적용되지 않습니다.");

            // 피격 유닛이 죽었는지 판정
            let maybe_dead_opponent_unit_id =
                game_field_unit_service_guard.judge_death_of_unit(
                    attack_unit_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            opponent_target_unit_card_index)).await.get_dead_unit_id();

            // 죽은 경우 묘지에 추가
            let mut game_tomb_service_guard =
                self.game_tomb_service.lock().await;

            if maybe_dead_opponent_unit_id != -1 {
                println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

                game_tomb_service_guard.add_used_card_to_tomb(
                    attack_unit_request_form
                        .to_place_dead_unit_to_tomb_request(
                            opponent_unique_id,
                            maybe_dead_opponent_unit_id)).await;
            }

            drop(game_tomb_service_guard);

            // TODO: 여기서 걸리는 경우에도 Notify 할 수 있어야 함

            return AttackUnitResponseForm::new(true)
        }

        // 반격을 위해 피격 유닛의 공격력 확보
        let find_opponent_target_unit_attack_point_response =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_unit_request_form
                    .to_acquire_unit_attack_point_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await;

        // 피격 유닛이 extra effect 를 가지고 있는지 여부
        let opponent_target_unit_extra_effect_list =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attack_unit_request_form
                    .to_acquire_unit_extra_effect_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_extra_status_effect_list().clone();

        // 공격한 유닛에게 피격 유닛의 효과와 함께 반격 적용
        game_field_unit_service_guard.attack_target_unit_with_extra_effect(
            attack_unit_request_form
                .to_attack_target_unit_with_extra_effect_request(
                    account_unique_id,
                    find_opponent_target_unit_attack_point_response.get_attack_point(),
                    &opponent_target_unit_extra_effect_list,
                    attacker_unit_card_index)).await;

        // 액션 완료 설정
        game_field_unit_service_guard.execute_turn_action(
            attack_unit_request_form
                .to_execute_turn_action_request(
                    account_unique_id,
                    attacker_unit_card_index)).await;

        // 유닛들이 죽었는지 판정
        let maybe_dead_opponent_unit_id =
            game_field_unit_service_guard.judge_death_of_unit(
                attack_unit_request_form
                    .to_judge_death_of_unit_request(
                        opponent_unique_id,
                        opponent_target_unit_card_index)).await.get_dead_unit_id();


        // 죽은 유닛의 경우 묘지에 추가
        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        if maybe_dead_opponent_unit_id != -1 {
            println!("공격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                attack_unit_request_form
                    .to_place_dead_unit_to_tomb_request(
                        opponent_unique_id,
                        maybe_dead_opponent_unit_id)).await;
        }

        let maybe_dead_attacker_unit_id =
            game_field_unit_service_guard.judge_death_of_unit(
                attack_unit_request_form
                    .to_judge_death_of_unit_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_dead_unit_id();

        if maybe_dead_attacker_unit_id != -1 {
            println!("반격 당한 유닛이 사망했으므로 묘지로 이동합니다.");

            game_tomb_service_guard.add_used_card_to_tomb(
                attack_unit_request_form
                    .to_place_dead_unit_to_tomb_request(
                        account_unique_id,
                        maybe_dead_attacker_unit_id)).await;
        }

        drop(game_field_unit_service_guard);

        drop(game_tomb_service_guard);

        // TODO: 상대방 알림

        AttackUnitResponseForm::new(true)
    }

    async fn request_to_attack_game_main_character(
        &self, attack_game_main_character_request_form: AttackGameMainCharacterRequestForm) -> AttackGameMainCharacterResponseForm {
        println!("GameCardUnitControllerImpl: request_to_attack_game_main_character()");
        // 세션 아이디를 검증합니다.
        let account_unique_id =
            self.is_valid_session(attack_game_main_character_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return AttackGameMainCharacterResponseForm::new(false)
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attack_game_main_character_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttackGameMainCharacterResponseForm::new(false)
        }

        // Battle Field 에서 공격하는 유닛의 index 를 토대로 id 값 확보
        let attacker_unit_card_index_string = attack_game_main_character_request_form.get_attacker_unit_index();
        let attacker_unit_card_index = attacker_unit_card_index_string.parse::<i32>().unwrap();

        // 액션 가능한 턴인지 검증
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;


        let attacker_unit_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                attack_game_main_character_request_form
                    .to_find_unit_id_by_index_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_found_opponent_unit_id();

        let mut game_card_unit_service_guard =
            self.game_card_unit_service.lock().await;

        let attacker_unit_required_energy =
            game_card_unit_service_guard.summary_unit_card(
                attack_game_main_character_request_form.to_summary_unit_card_info_request(
                    attacker_unit_id)).await.get_unit_attack_required_energy();

        drop(game_card_unit_service_guard);

        let mut game_field_unit_action_possibility_validator_service_guard =
            self.game_field_unit_action_possibility_validator_service.lock().await;


        let is_unit_basic_attack_possible_response =
            game_field_unit_action_possibility_validator_service_guard.is_unit_basic_attack_possible(
                attack_game_main_character_request_form.to_is_unit_basic_attack_possible_request(
                    account_unique_id, attacker_unit_card_index, attacker_unit_required_energy)).await;

        if !is_unit_basic_attack_possible_response.is_possible() {
            return AttackGameMainCharacterResponseForm::new(false)
        }

        drop(game_field_unit_action_possibility_validator_service_guard);

        // 유닛 인덱스에서 기본 공격력 정보 확보
        let attacker_unit_attack_point =
            game_field_unit_service_guard.acquire_unit_attack_point(
                attack_game_main_character_request_form
                    .to_acquire_unit_attack_point_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_attack_point();

        // todo 메인캐릭터가 extra effect경우 추가해야한다
        // extra effect 가지고 있는지 여부
        let attacker_unit_extra_effect_list =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attack_game_main_character_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        attacker_unit_card_index)).await.get_extra_status_effect_list().clone();

        // 공격을 위해 상대방 고유값 획득
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attack_game_main_character_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

       let mut game_main_character_service_guard =
           self.game_main_character_service.lock().await;

       let is_apply_damage_to_main_character_response =
           game_main_character_service_guard.apply_damage_to_main_character(
               attack_game_main_character_request_form.to_apply_damage_to_main_character_request(
                   opponent_unique_id, attacker_unit_attack_point)).await;


       let is_check_main_character_of_account_unique_id_response =
           game_main_character_service_guard.check_main_character_of_account_unique_id(
               attack_game_main_character_request_form.to_check_main_character_of_account_unique_id_request(
                   account_unique_id, )).await;

       drop(game_main_character_service_guard);

       AttackGameMainCharacterResponseForm::new(true)
    }
}



