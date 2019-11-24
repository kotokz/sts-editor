#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GameObj {
    act_num: i64,
    ai_seed_count: i64,
    ascension_level: i64,
    blight_counters: Vec<::serde_json::Value>,
    blights: Vec<::serde_json::Value>,
    blue: i64,
    boss: String,
    boss_list: Vec<String>,
    boss_relics: Vec<String>,
    bottled_lightning: String,
    bottled_lightning_misc: i64,
    bottled_lightning_upgrade: i64,
    card_random_seed_count: i64,
    card_random_seed_randomizer: i64,
    card_seed_count: i64,
    pub cards: Vec<Card>,
    champions: i64,
    chose_neow_reward: bool,
    combo: bool,
    common_relics: Vec<String>,
    current_health: i64,
    current_room: String,
    custom_mods: Vec<::serde_json::Value>,
    daily_date: i64,
    daily_mods: Vec<::serde_json::Value>,
    elite_monster_list: Vec<String>,
    elites1_killed: i64,
    elites2_killed: i64,
    elites3_killed: i64,
    endless_increments: Vec<::serde_json::Value>,
    event_chances: Vec<f64>,
    event_list: Vec<String>,
    event_seed_count: i64,
    floor_num: i64,
    pub gold: i64,
    pub gold_gained: i64,
    green: i64,
    hand_size: i64,
    has_emerald_key: bool,
    has_ruby_key: bool,
    has_sapphire_key: bool,
    is_ascension_mode: bool,
    is_daily: bool,
    is_endless_mode: bool,
    is_final_act_on: bool,
    is_trial: bool,
    level_name: String,
    max_health: i64,
    max_orbs: i64,
    merchant_seed_count: i64,
    metric_boss_relics: Vec<MetricBossRelic>,
    metric_build_version: String,
    metric_campfire_choices: Vec<MetricCampfireChoice>,
    metric_campfire_meditates: i64,
    metric_campfire_rested: i64,
    metric_campfire_rituals: i64,
    metric_campfire_upgraded: i64,
    metric_card_choices: Vec<MetricCardChoice>,
    metric_current_hp_per_floor: Vec<i64>,
    metric_damage_taken: Vec<MetricDamageTaken>,
    metric_event_choices: Vec<MetricEventChoice>,
    metric_floor_reached: i64,
    metric_gold_per_floor: Vec<i64>,
    metric_item_purchase_floors: Vec<i64>,
    metric_items_purchased: Vec<String>,
    metric_items_purged: Vec<String>,
    metric_items_purged_floors: Vec<i64>,
    metric_max_hp_per_floor: Vec<i64>,
    metric_path_per_floor: Vec<Option<String>>,
    metric_path_taken: Vec<String>,
    metric_playtime: i64,
    metric_potions_floor_spawned: Vec<i64>,
    metric_potions_floor_usage: Vec<i64>,
    metric_potions_obtained: Vec<MetricPotionsObtained>,
    metric_purchased_purges: i64,
    metric_relics_obtained: Vec<MetricRelicsObtained>,
    metric_seed_played: String,
    monster_list: Vec<String>,
    monster_seed_count: i64,
    monsters_killed: i64,
    mugged: bool,
    mystery_machine: i64,
    name: String,
    neow_bonus: String,
    neow_cost: String,
    obtained_cards: ObtainedCards,
    one_time_event_list: Vec<String>,
    overkill: bool,
    path_x: Vec<i64>,
    path_y: Vec<i64>,
    perfect: i64,
    play_time: i64,
    post_combat: bool,
    potion_chance: i64,
    potion_seed_count: i64,
    potion_slots: i64,
    potions: Vec<String>,
    #[serde(rename = "purgeCost")]
    purge_cost: i64,
    rare_relics: Vec<String>,
    pub red: i64,
    relic_counters: Vec<i64>,
    relic_seed_count: i64,
    pub relics: Vec<String>,
    room_x: i64,
    room_y: i64,
    save_date: i64,
    seed: i64,
    seed_set: bool,
    shop_relics: Vec<String>,
    shuffle_seed_count: i64,
    smoked: bool,
    special_seed: i64,
    spirit_count: i64,
    treasure_seed_count: i64,
    uncommon_relics: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Card {
    pub id: String,
    misc: i64,
    pub upgrades: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricBossRelic {
    not_picked: Vec<String>,
    picked: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricCampfireChoice {
    data: Option<String>,
    floor: f64,
    key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricCardChoice {
    floor: f64,
    not_picked: Vec<String>,
    picked: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricDamageTaken {
    damage: f64,
    enemies: String,
    floor: f64,
    turns: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricEventChoice {
    cards_upgraded: Option<Vec<String>>,
    damage_healed: i64,
    damage_taken: i64,
    event_name: String,
    floor: i64,
    gold_gain: i64,
    gold_loss: i64,
    max_hp_gain: i64,
    max_hp_loss: i64,
    player_choice: String,
    #[serde(default)]
    cards_removed: Vec<String>,
    #[serde(default)]
    cards_obtained: Vec<String>,
    #[serde(default)]
    cards_transformed: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricPotionsObtained {
    floor: i64,
    key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct MetricRelicsObtained {
    floor: i64,
    key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct ObtainedCards {
    #[serde(rename = "Adaptation")]
    adaptation: i64,
    #[serde(rename = "DevaForm")]
    deva_form: i64,
    #[serde(rename = "EmptyBody")]
    empty_body: i64,
    #[serde(rename = "Fasting2")]
    fasting2: i64,
    #[serde(rename = "JustLucky")]
    just_lucky: i64,
    #[serde(rename = "LikeWater")]
    like_water: i64,
    #[serde(rename = "MasterReality")]
    master_reality: i64,
    #[serde(rename = "Prostrate")]
    prostrate: i64,
    #[serde(rename = "Protect")]
    protect: i64,
    #[serde(rename = "Ragnarok")]
    ragnarok: i64,
    #[serde(rename = "Tantrum")]
    tantrum: i64,
    #[serde(rename = "ThirdEye")]
    third_eye: i64,
    #[serde(rename = "Wallop")]
    wallop: i64,
    #[serde(rename = "WheelKick")]
    wheel_kick: i64,
}
