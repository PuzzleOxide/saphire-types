use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum SetVariable {
    SettoString { variable_to_set: Variable, string_to_set_to: Vec<AnyType> },
    SetParticleEffectType {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        type_: Text,
    },
    SetItemEnchants {},
    PurgeMatchingVariables { name_to_match: Vec<Text> },
    ShiftLocationonAllAxes {
        variable_to_set: Variable,
        location_to_change: Option<Location>,
        x_change: Option<Number>,
        y_change: Option<Number>,
        z_change: Option<Number>,
    },
    GetParticleEffectMaterial { variable_to_set: Variable, effect_to_get: Particle },
    SetParticleEffectSpread {
        variable_to_set: Variable,
        effect_to_change: Option<Particle>,
        horizontal_spread: Number,
        vertical_spread: Number,
    },
    SettoAbsoluteValue { variable_to_set: Variable, number_input: Option<Number> },
    AppendValuetoList { list_to_append_to: Variable, values_to_append: Vec<AnyType> },
    SettoRemainder { variable_to_set: Variable, dividend: Number, divisor: Number },
    ShiftLocationonVector {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        shift_vector: Vector,
        shift_distance: Option<Number>,
    },
    GetItemAttribute { variable_to_set: Variable, item: Item },
    ClearDictionary { dictionary_to_clear: Variable },
    AddNumbers { variable_to_set: Variable, numbers_to_add: Vec<Number> },
    ShiftLocationRotation {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        rotation_amount: Number,
    },
    SubtractNumbers { variable_to_set: Variable, numbers_to_subtract: Vec<Number> },
    GetItemName { variable_to_set: Variable, item_to_get_name_of: Item },
    GetItemRarity { variable_to_set: Variable, item: Item },
    MultiplyVector {
        variable_to_set: Variable,
        vector_to_multiply: Option<Vector>,
        multiplier: Number,
    },
    DivideNumbers { variable_to_set: Variable, numbers_to_divide: Vec<Number> },
    GetSignText {},
    SettoBitwiseOperation {
        variable_to_set: Variable,
        operand_1: Number,
        operand_2: Option<Number>,
    },
    GetLecternPage { variable_to_set: Variable, lectern_location: Location },
    ParseX {},
    ShiftLocationonAxis {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        shift_distance: Number,
    },
    ParseY {},
    SettoVectorBetweenLocations {
        variable_to_set: Variable,
        start_location: Location,
        end_location: Location,
    },
    ParseZ {},
    GetVectorComponent { variable_to_set: Variable, vector_to_get: Vector },
    SettoValueEq { variable_to_set: Variable, value: AnyType },
    RmText {},
    AddItemAttribute {
        variable_to_set: Variable,
        item: Option<Item>,
        modifier_amount: Number,
    },
    SettoCenterLocation {
        variable_to_set: Variable,
        locations_to_center: Vec<Location>,
    },
    AlignLocation { variable_to_set: Variable, location_to_align: Option<Location> },
    GetSoundVolume { variable_to_set: Variable, sound_to_get_volume_of: Sound },
    SettoRandomNumber {
        variable_to_set: Variable,
        minimum_number: Number,
        maximum_number: Number,
    },
    GetContainerName { variable_to_set: Variable, container_location: Location },
    RaycastfromLocation {
        variable_to_set: Variable,
        ray_origin: Location,
        ray_distance: Number,
    },
    RotateVectorAroundVector {
        variable_to_set: Variable,
        vector_to_rotate: Option<Vector>,
        axis_vector: Vector,
        angle: Number,
    },
    GetParticleEffectMotion { variable_to_set: Variable, effect_to_get: Particle },
    SetParticleMotion {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        particle_motion: Option<Vector>,
        motion_variation_: Option<Number>,
    },
    SettoAverageNumber { variable_to_set: Variable, numbers_to_average: Vec<Number> },
    WrapNumberOld {},
    SetY {},
    SetMapTexture {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        image_url: Text,
    },
    GetBlockData { variable_to_set: Variable, block_location: Location, tag_name: Text },
    SetX {},
    SortDictionary { variable_to_set: Variable, dictionary_to_sort: Option<Dict> },
    GetLecternBook { variable_to_set: Variable, lectern_location: Location },
    GetCustomSoundKey { variable_to_set: Variable, sound_to_get_key_of: Sound },
    SettoCrossProduct { variable_to_set: Variable, vector_1: Vector, vector_2: Vector },
    MultiplyNumbers { variable_to_set: Variable, numbers_to_multiply: Vec<Number> },
    GetParticleRoll { variable_to_set: Variable, effect_to_get: Option<Particle> },
    ParseYaw {},
    SettoDotProduct { variable_to_set: Variable, vector_1: Vector, vector_2: Vector },
    SetZ {},
    SetArmorTrim { variable_to_set: Variable, item_to_change: Option<Item> },
    PopListValue {
        variable_to_set: Variable,
        list_to_get_value_of: List,
        index: Option<Number>,
    },
    SettoMinimumNumber { variable_to_set: Variable, number_set: Vec<Number> },
    GetPotionEffectType { variable_to_set: Variable, potion_to_get: Potion },
    SetItemName {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        name: Vec<MiniMessage>,
    },
    GetListLength { variable_to_set: Variable, list_to_measure: List },
    SettoSine { variable_to_set: Variable, number_input: Number },
    SettoDirectionName { variable_to_set: Variable, direction: Vector },
    RepeatString {
        variable_to_set: Variable,
        string_to_repeat: Text,
        times_to_repeat: Number,
    },
    GetItemLore {},
    JoinString {
        variable_to_set: Variable,
        strings_to_join: List,
        joining_string: Option<Text>,
        final_joining_string: Option<Text>,
    },
    ReverseList { variable_to_set: Variable, list_to_reverse: Option<List> },
    RemoveDuplicateListElements {
        variable_to_set: Variable,
        list_to_deduplicate: Option<List>,
    },
    CreateDictionary {
        variable_to_set: Variable,
        key_list: Option<List>,
        value_list: Option<List>,
    },
    RoundNumber {},
    FaceLocation {
        variable_to_set: Variable,
        location_to_change: Option<Location>,
        target_location: Location,
    },
    GetItemLoreLine {},
    SetVectorLength {
        variable_to_set: Variable,
        vector_to_change: Option<Vector>,
        length: Option<Number>,
    },
    SetPotionEffectDuration {
        variable_to_set: Variable,
        potion_to_change: Option<Potion>,
        duration_ticks: Number,
    },
    SplitString {
        variable_to_set: Variable,
        string_to_split: Text,
        splitter_string: Option<Text>,
    },
    SettoNormallyDistributedRandomNumber {
        variable_to_set: Variable,
        mean_midpoint: Number,
        standard_deviation: Number,
    },
    SetPotionEffectType {
        variable_to_set: Variable,
        potion_to_change: Option<Potion>,
        type_: Text,
    },
    AlignVector { variable_to_set: Variable, vector_to_align: Option<Vector> },
    SetItemDurability {
        variable_to_set: Variable,
        item: Option<Item>,
        item_durability: Number,
    },
    SetItemBreakability { variable_to_set: Variable, item: Option<Item> },
    GetSignTextN { variable_to_set: Variable, location: Location },
    RaycastEntity {},
    SetDictionaryValue { dictionary_to_add_to: Variable, key: Text, value: AnyType },
    SetAllLocationCoordinates {
        variable_to_set: Variable,
        location_to_change: Option<Location>,
        new_x: Option<Number>,
        new_y: Option<Number>,
        new_z: Option<Number>,
        new_pitch: Option<Number>,
        new_yaw: Option<Number>,
    },
    SettoRGBColor {
        variable_to_set: Variable,
        red_c07c2557: Number,
        green_c07c2557: Number,
        blue_c07c2557: Either<Number, List>,
    },
    SetBreakableBlocks {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        breakable_blocks: Vec<Block>,
    },
    SettoHSLColor {
        variable_to_set: Variable,
        hue_color_circle_c07c3607: Number,
        saturation_c07c1007: Option<Number>,
        lightness_c07c1007: Option<Either<Number, List>>,
    },
    GetDirection {},
    GetItemLoreN { variable_to_set: Variable, item_to_get_lore_from: Item },
    RemoveListValueatIndex { list_to_change: Variable, index_to_remove: Vec<Number> },
    SettoLogarithm {
        variable_to_set: Variable,
        number_input: Option<Number>,
        base: Number,
    },
    SetItemCustomTag {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        tag_name: Text,
        tag_value: Either<Number, Text>,
    },
    TrimString {
        variable_to_set: Variable,
        string_to_trim: Option<Text>,
        start_character_position: Number,
        end_character_position: Option<Number>,
    },
    ParseMiniMessageExpression { variable_to_set: Variable, string_to_parse: Text },
    GetItemStackSize { variable_to_set: Variable, item_to_get_stack: Item },
    SetPotionEffectAmplifier {
        variable_to_set: Variable,
        potion_to_change: Option<Potion>,
        amplifier: Number,
    },
    GetBreakableBlocks { variable_to_set: Variable, item: Item },
    RotateVectorAroundAxis {
        variable_to_set: Variable,
        vector_to_rotate: Option<Vector>,
        angle: Number,
    },
    GetItemNameN {},
    GetItemDurability { variable_to_set: Variable, item: Item },
    ShiftLocationinDirection {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        shift_distance: Option<Number>,
    },
    WrapNumber {
        variable_to_set: Variable,
        number_to_wrap: Option<Number>,
        lower_bound_inclusive: Number,
        upper_bound_exclusive: Number,
    },
    ReplaceString {
        variable: Variable,
        string_to_change: Text,
        string_part_to_replace: Text,
        replacement: Text,
    },
    SetCompassLodestoneLocation {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        lodestone_location: Location,
    },
    FlattenList { variable_to_set: Variable, list_to_flatten: Option<List> },
    GetPotionEffectAmplifier { variable_to_set: Variable, potion_to_get: Potion },
    GetParticleEffectAmount { variable_to_set: Variable, effect_to_get: Particle },
    GetDictionarySize { variable_to_set: Variable, dictionary_to: Dict },
    SetItemStackSize {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        stack_size: Number,
    },
    SubtractVectors { variable_to_set: Variable, vectors_to_subtract: Vec<Vector> },
    SetStringCase { variable_to_set: Variable, string_to_change: Option<Text> },
    SetParticleEffectColor {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        color_hexadecimal: Text,
        color_variation_: Option<Number>,
    },
    GetLightLevel { variable_to_set: Variable, light_location: Location },
    GetBookText { variable_to_set: Variable, book: Item, page_number: Option<Number> },
    GetDictionaryValues { variable_to_set: Variable, dictionary_to: Dict },
    SettoVector {
        variable_to_set: Variable,
        x_component: Number,
        y_component: Number,
        z_component: Number,
    },
    SettoDistance {
        variable_to_set: Variable,
        location_1: Location,
        location_2: Location,
    },
    SetItemLore {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        lore: Vec<Either<MiniMessage, MiniMessage>>,
        line_number: Number,
    },
    SettoRoot {
        variable_to_set: Variable,
        number_input: Option<Number>,
        root_index: Option<Number>,
    },
    SetParticleEffectAmount {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        particle_amount: Number,
    },
    AddItemEnchantment {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        enchantment_name: Text,
        enchantment_level: Number,
    },
    GetItemMaterial { variable_to_set: Variable, item_to_get_material_of: Item },
    GetLocationDirection { variable_to_set: Variable, location_to_get: Location },
    GetLoreLine {
        variable_to_set: Variable,
        item_to_get_lore_from: Item,
        lore_line_to_get: Number,
    },
    GetParticleEffectType { variable_to_set: Variable, effect_to_get: Particle },
    RemoveString {
        variable: Variable,
        string_to_change: Option<Text>,
        string_to_remove: Vec<Text>,
    },
    GetAllBlockData { variable_to_set: Variable, block_location: Location },
    SettoMaximumNumber { variable_to_set: Variable, number_set: Vec<Number> },
    GetDictionaryKeys { variable_to_set: Variable, dictionary_to: Dict },
    TrimStyledTextContent {
        variable_to_set: Variable,
        text_to_trim: Option<MiniMessage>,
        start_character_position: Number,
        end_character_position: Option<Number>,
    },
    SetParticleEffectMaterial {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        particle_material: Item,
    },
    GetLocationCoordinate { variable_to_set: Variable, location_to_get: Location },
    RemoveItemCustomTag {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        tag_name: Text,
    },
    SetParticleEffectSize {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        particle_size: Number,
        size_variation_: Option<Number>,
    },
    GetPotionEffectDuration { variable_to_set: Variable, potion_to_get: Potion },
    SettoRandomLocation {
        variable_to_set: Variable,
        location_1: Location,
        location_2: Location,
    },
    SetSoundType {
        variable_to_set: Variable,
        sound_to_change: Option<Sound>,
        sound_name_eg_rabbit_eat: Text,
    },
    GetCompassLodestoneLocation {
        variable_to_set: Variable,
        compass_to_get_lodestone: Item,
    },
    ShiftDirection {},
    GetContainerNameN {},
    GetParticleEffectSpread { variable_to_set: Variable, effect_to_get: Particle },
    ReflectVector {
        variable_to_set: Variable,
        vector_to_reflect: Option<Vector>,
        surface_vector: Vector,
    },
    GetHeadOwner { variable_to_set: Variable, head_to_get_owner_of: Item },
    GetItemEnchants {},
    AppendDictionary { dictionary_to: Variable, dictionary: Dict },
    GetItemMaximumStackSize {
        variable_to_set: Variable,
        item_to_get_maximum_stack: Item,
    },
    GetColorChannels { variable_to_set: Variable, color_hexadecimal: Text },
    SetLocationDirection {
        variable_to_set: Variable,
        location_to_change: Option<Location>,
        direction: Vector,
    },
    SetListValue { list_to_change: Variable, index: Number, value_to_set: AnyType },
    SetItemEnchantments {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        enchantments: Dict,
    },
    SetBookText {
        variable_to_set: Variable,
        book: Option<Item>,
        pages: Vec<Either<MiniMessage, MiniMessage>>,
        page_number: Number,
    },
    SettoRandomValue { variable_to_set: Variable, value_set: Vec<AnyType> },
    SetItemMaterial {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        material: Text,
    },
    GetSoundType { variable_to_set: Variable, sound_to_get_type__of: Sound },
    GetListValue {
        variable_to_set: Variable,
        list_to_get_value_of: List,
        index: Number,
    },
    SettoTangent { variable_to_set: Variable, number_input: Number },
    GetVoronoiNoise {
        variable_to_set: Variable,
        noise_location: Location,
        cell_frequency: Option<Number>,
        cell_scatter: Option<Number>,
        generation_seed: Option<Number>,
    },
    SetDirection {},
    SettoHSBColor {
        variable_to_set: Variable,
        hue_color_circle_c07c3607: Number,
        saturation_c07c1007: Option<Number>,
        brightness_c07c1007: Option<Either<Number, List>>,
    },
    IncrementNumberEq { variable: Variable, numbers_to: Vec<Number> },
    GetSoundVariant { variable_to_set: Variable, sound_to_get_variant_of: Sound },
    GetItemColor { variable: Variable, item_to_get_color_of: Item },
    ClearFormatting { variable_to_set: Variable, text_to_change: Option<MiniMessage> },
    InsertListValue {
        list_to_change: Variable,
        index: Number,
        value_to_insert: AnyType,
    },
    SetSoundVolume {
        variable_to_set: Variable,
        sound_to_change: Option<Sound>,
        volume: Number,
    },
    SetLocationCoordinate {
        variable_to_set: Variable,
        location_to_change: Option<Location>,
        coordinate: Number,
    },
    AddVectors { variable_to_set: Variable, vectors_to_add: Vec<Vector> },
    SetPitch {},
    RaycastBlock {},
    GetItemEnchantments {
        variable_to_set: Variable,
        item_to_get_enchantments_from: Item,
    },
    SetHeadTexture {
        variable_to_set: Variable,
        player_head: Option<Item>,
        owner_name_uuid_or: Text,
    },
    GetPerlinNoise {
        variable_to_set: Variable,
        noise_location: Location,
        frequency_scale: Option<Number>,
        octaves_perlin_layers: Option<Number>,
        octave_frequency_gain: Option<Number>,
        octave_amplitude_gain: Option<Number>,
        generation_seed: Option<Number>,
    },
    GetWorleyNoise {
        variable_to_set: Variable,
        noise_location: Location,
        cell_frequency: Option<Number>,
        cell_scatter: Option<Number>,
        generation_seed: Option<Number>,
    },
    SetItemColor {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        color_hexadecimal: Text,
    },
    GetParticleEffectColor { variable_to_set: Variable, effect_to_get: Particle },
    SetSoundPitch {
        variable_to_set: Variable,
        sound_to_change: Option<Sound>,
        pitch: Either<Number, Text>,
    },
    RoundNumberN {
        variable_to_set: Variable,
        number_to_round: Option<Number>,
        round_multiple: Option<Number>,
    },
    GetPlaceableBlocks { variable_to_set: Variable, item: Item },
    SortList { variable_to_set: Variable, list_to_sort: Option<List> },
    SetCustomSoundKey {
        variable_to_set: Variable,
        sound_to_change: Option<Sound>,
        sound_key: Option<Text>,
    },
    RemoveDictionaryEntry {
        dictionary_to_change: Variable,
        key_to_remove: Text,
        expected_values: Vec<AnyType>,
    },
    FormatTimestamp {
        variable_to_set: Variable,
        time_to_format: Number,
        custom_format: Option<Text>,
    },
    SetItemVisibilityFlags { variable_to_set: Variable, item: Option<Item> },
    GetStringLength { variable_to_set: Variable, string_to_measure: Text },
    GetItemPotionEffects { variable_to_set: Variable, item_to_get_effects_from: Item },
    GetMiniMessageExpression { variable_to_set: Variable, text_to_read: MiniMessage },
    SetYaw {},
    SetItemPotionEffects {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        item_effects: Vec<Potion>,
    },
    DecrementNumberEq { variable: Variable, numbers_to: Vec<Number> },
    GetItemCustomTag {
        variable_to_set: Variable,
        item_to_get_tag_of: Item,
        tag_name: Text,
    },
    CreateList { variable_to_set: Variable, value_list: Vec<AnyType> },
    AppendListtoList { list_to_append_to: Variable, lists_to_append: Vec<List> },
    GetContainerContents { variable_to_set: Variable, container_location: Location },
    ShiftLocationTowardLocation {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        target_location: Location,
        shift_distance: Option<Number>,
    },
    TrimList {
        variable_to_set: Variable,
        list_to_trim: Option<List>,
        start_index: Number,
        end_index: Option<Number>,
    },
    SettoCosine { variable_to_set: Variable, number_input: Number },
    SetVectorComponent {
        variable_to_set: Variable,
        vector_to_change: Option<Vector>,
        component: Number,
    },
    ParseNumberfromString { variable_to_set: Variable, string_to_convert: Option<Text> },
    SettoExponential {
        variable_to_set: Variable,
        number_input: Option<Number>,
        exponent: Option<Number>,
    },
    ShiftAllDirs {},
    GetListIndexofValue {
        variable_to_set: Variable,
        list_to_search_in: List,
        value_to_search: AnyType,
    },
    RemoveItemEnchantment {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        enchantment_name: Text,
    },
    GetBookTextN {},
    SetParticleRoll {
        variable_to_set: Variable,
        effect_to: Option<Particle>,
        particle_roll: Number,
    },
    SetSoundVariant {
        variable_to_set: Variable,
        sound_to_change: Option<Sound>,
        variant_id_eg_break1: Option<Text>,
    },
    ShiftLocation {},
    RandomizeList { variable_to_set: Variable, list_to_randomize: Option<List> },
    ClampNumber {
        variable_to_set: Variable,
        number_to_clamp: Option<Number>,
        minimum: Number,
        maximum: Number,
    },
    Round {},
    GetSoundPitch { variable_to_set: Variable, sound_to_get_pitch_or: Sound },
    TranslateColors {},
    GetBlockGrowth { variable_to_set: Variable, block_location: Location },
    GetAllCustomItemTags { variable_to_set: Variable, item_to_get_tags_from: Item },
    RemoveListValue { list_to_change: Variable, values_to: Vec<AnyType> },
    ShiftLocationinAllDirections {
        variable_to_set: Variable,
        location_to_shift: Option<Location>,
        forwards_change: Option<Number>,
        upwards_change: Option<Number>,
        sideways_change_l__r: Option<Number>,
    },
    SetPlaceableBlocks {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        placeable_blocks: Vec<Block>,
    },
    GetBlockMaterial { variable_to_set: Variable, block_location: Location },
    ParsePitch {},
    GetDictionaryValue { variable_to_set: Variable, dictionary_to: Dict, key: Text },
    GetContainerLock { variable_to_set: Variable, container_location: Location },
    GetBlockPower { variable_to_set: Variable, block_location: Location },
    GetVectorLength { variable_to_set: Variable, vector_to_get: Vector },
    SetCustomModelData {
        variable_to_set: Variable,
        item_to_change: Option<Item>,
        model_value: Number,
    },
    SetCoords {},
    GetParticleEffectSize { variable_to_set: Variable, effect_to_get: Option<Particle> },
}
impl SetVariable {
    pub fn compile(&self) -> Value {
        match self {
            SetVariable::SettoString { variable_to_set, string_to_set_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_set_to.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("String".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectType { variable_to_set, effect_to, type_ } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to.json(), type_.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemEnchants {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemEnchants".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::PurgeMatchingVariables { name_to_match } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![name_to_match.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PurgeVars".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationonAllAxes {
                variable_to_set,
                location_to_change,
                x_change,
                y_change,
                z_change,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), x_change
                        .json(), y_change.json(), z_change.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftAllAxes".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectMaterial {
                variable_to_set,
                effect_to_get,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleMat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectSpread {
                variable_to_set,
                effect_to_change,
                horizontal_spread,
                vertical_spread,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to_change.json(),
                        horizontal_spread.json(), vertical_spread.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleSprd".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoAbsoluteValue { variable_to_set, number_input } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AbsoluteValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AppendValuetoList { list_to_append_to, values_to_append } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_append_to.json(), values_to_append.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AppendValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRemainder { variable_to_set, dividend, divisor } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dividend.json(), divisor.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("%".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationonVector {
                variable_to_set,
                location_to_shift,
                shift_vector,
                shift_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_vector
                        .json(), shift_distance.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftOnVector".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemAttribute { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemAttribute".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ClearDictionary { dictionary_to_clear } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![dictionary_to_clear.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearDict".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AddNumbers { variable_to_set, numbers_to_add } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_add.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("+".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationRotation {
                variable_to_set,
                location_to_shift,
                rotation_amount,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), rotation_amount
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftRotation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SubtractNumbers { variable_to_set, numbers_to_subtract } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_subtract.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("-".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemName { variable_to_set, item_to_get_name_of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_name_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetItemName ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemRarity { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemRarity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::MultiplyVector {
                variable_to_set,
                vector_to_multiply,
                multiplier,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_multiply.json(), multiplier
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MultiplyVector".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::DivideNumbers { variable_to_set, numbers_to_divide } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_divide.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("/".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSignText {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetSignText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoBitwiseOperation {
                variable_to_set,
                operand_1,
                operand_2,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), operand_1.json(), operand_2.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Bitwise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLecternPage { variable_to_set, lectern_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), lectern_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetLecternPage".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseX {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseX".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationonAxis {
                variable_to_set,
                location_to_shift,
                shift_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_distance
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftOnAxis".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseY {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseY".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoVectorBetweenLocations {
                variable_to_set,
                start_location,
                end_location,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), start_location.json(), end_location
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VectorBetween".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseZ {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseZ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetVectorComponent { variable_to_set, vector_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetVectorComp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoValueEq { variable_to_set, value } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), value.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RmText {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RmText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AddItemAttribute { variable_to_set, item, modifier_amount } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json(), modifier_amount.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AddItemAttribute".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoCenterLocation {
                variable_to_set,
                locations_to_center,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), locations_to_center.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCenterLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AlignLocation { variable_to_set, location_to_align } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_to_align.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AlignLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSoundVolume { variable_to_set, sound_to_get_volume_of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_volume_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetSoundVolume".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRandomNumber {
                variable_to_set,
                minimum_number,
                maximum_number,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), minimum_number.json(), maximum_number
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetContainerName { variable_to_set, container_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), container_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ContainerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RaycastfromLocation {
                variable_to_set,
                ray_origin,
                ray_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), ray_origin.json(), ray_distance.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Raycast".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RotateVectorAroundVector {
                variable_to_set,
                vector_to_rotate,
                axis_vector,
                angle,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_rotate.json(), axis_vector
                        .json(), angle.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RotateAroundVec".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectMotion { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleMotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleMotion {
                variable_to_set,
                effect_to,
                particle_motion,
                motion_variation_,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to.json(), particle_motion.json(),
                        motion_variation_.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleMotion".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoAverageNumber { variable_to_set, numbers_to_average } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_average.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Average".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::WrapNumberOld {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WrapNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetY {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetY".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetMapTexture {
                variable_to_set,
                item_to_change,
                image_url,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json(), image_url.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetMapTexture".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBlockData { variable_to_set, block_location, tag_name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json(), tag_name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetBlockData".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetX {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetX".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SortDictionary { variable_to_set, dictionary_to_sort } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to_sort.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SortDict".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLecternBook { variable_to_set, lectern_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), lectern_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetLecternBook".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetCustomSoundKey { variable_to_set, sound_to_get_key_of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_key_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCustomSound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoCrossProduct { variable_to_set, vector_1, vector_2 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_1.json(), vector_2.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CrossProduct".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::MultiplyNumbers { variable_to_set, numbers_to_multiply } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_multiply.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("x".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleRoll { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleRoll".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseYaw {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseYaw".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoDotProduct { variable_to_set, vector_1, vector_2 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_1.json(), vector_2.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DotProduct".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetZ {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetZ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetArmorTrim { variable_to_set, item_to_change } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetArmorTrim".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::PopListValue {
                variable_to_set,
                list_to_get_value_of,
                index,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), list_to_get_value_of.json(), index.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PopListValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoMinimumNumber { variable_to_set, number_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), number_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MinNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetPotionEffectType { variable_to_set, potion_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), potion_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetPotionType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemName { variable_to_set, item_to_change, name } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json(), name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetListLength { variable_to_set, list_to_measure } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_measure.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ListLength".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoSine { variable_to_set, number_input } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Sine".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoDirectionName { variable_to_set, direction } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), direction.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DirectionName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RepeatString {
                variable_to_set,
                string_to_repeat,
                times_to_repeat,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), string_to_repeat.json(), times_to_repeat
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RepeatString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemLore {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemLore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::JoinString {
                variable_to_set,
                strings_to_join,
                joining_string,
                final_joining_string,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), strings_to_join.json(), joining_string
                        .json(), final_joining_string.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("JoinString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ReverseList { variable_to_set, list_to_reverse } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_reverse.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReverseList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveDuplicateListElements {
                variable_to_set,
                list_to_deduplicate,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_deduplicate.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("DedupList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::CreateDictionary { variable_to_set, key_list, value_list } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), key_list.json(), value_list.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreateDict".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RoundNumber {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RoundNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::FaceLocation {
                variable_to_set,
                location_to_change,
                target_location,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(),
                        target_location.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FaceLocation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemLoreLine {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemLoreLine".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetVectorLength {
                variable_to_set,
                vector_to_change,
                length,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_change.json(), length.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVectorLength".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetPotionEffectDuration {
                variable_to_set,
                potion_to_change,
                duration_ticks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), potion_to_change.json(), duration_ticks
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPotionDur".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SplitString {
                variable_to_set,
                string_to_split,
                splitter_string,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), string_to_split.json(), splitter_string
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SplitString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoNormallyDistributedRandomNumber {
                variable_to_set,
                mean_midpoint,
                standard_deviation,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), mean_midpoint.json(), standard_deviation
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("NormalRandom".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetPotionEffectType {
                variable_to_set,
                potion_to_change,
                type_,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), potion_to_change.json(), type_.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPotionType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AlignVector { variable_to_set, vector_to_align } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_align.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AlignVector".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemDurability {
                variable_to_set,
                item,
                item_durability,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json(), item_durability.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemDura".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemBreakability { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBreakability".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSignTextN { variable_to_set, location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), location.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetSignText ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RaycastEntity {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RaycastEntity".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetDictionaryValue { dictionary_to_add_to, key, value } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![dictionary_to_add_to.json(), key.json(), value.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDictValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetAllLocationCoordinates {
                variable_to_set,
                location_to_change,
                new_x,
                new_y,
                new_z,
                new_pitch,
                new_yaw,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), new_x.json(),
                        new_y.json(), new_z.json(), new_pitch.json(), new_yaw.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetAllCoords".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRGBColor {
                variable_to_set,
                red_c07c2557,
                green_c07c2557,
                blue_c07c2557,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), red_c07c2557.json(), green_c07c2557
                        .json(), blue_c07c2557.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RGBColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetBreakableBlocks {
                variable_to_set,
                item_to_change,
                breakable_blocks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), breakable_blocks
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCanDestroy".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoHSLColor {
                variable_to_set,
                hue_color_circle_c07c3607,
                saturation_c07c1007,
                lightness_c07c1007,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), hue_color_circle_c07c3607.json(),
                        saturation_c07c1007.json(), lightness_c07c1007.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HSLColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetDirection {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetDirection ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemLoreN { variable_to_set, item_to_get_lore_from } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_lore_from.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetItemLore ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveListValueatIndex { list_to_change, index_to_remove } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_change.json(), index_to_remove.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveListIndex".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoLogarithm { variable_to_set, number_input, base } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json(), base.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Logarithm".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemCustomTag {
                variable_to_set,
                item_to_change,
                tag_name,
                tag_value,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), tag_name.json(),
                        tag_value.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::TrimString {
                variable_to_set,
                string_to_trim,
                start_character_position,
                end_character_position,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), string_to_trim.json(),
                        start_character_position.json(), end_character_position.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TrimString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseMiniMessageExpression {
                variable_to_set,
                string_to_parse,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_parse.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseMiniMessageExpr".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemStackSize { variable_to_set, item_to_get_stack } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_stack.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemAmount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetPotionEffectAmplifier {
                variable_to_set,
                potion_to_change,
                amplifier,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), potion_to_change.json(), amplifier.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPotionAmp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBreakableBlocks { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCanDestroy".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RotateVectorAroundAxis {
                variable_to_set,
                vector_to_rotate,
                angle,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_rotate.json(), angle.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RotateAroundAxis".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemNameN {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemDurability { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemDura".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationinDirection {
                variable_to_set,
                location_to_shift,
                shift_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_distance
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftInDirection".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::WrapNumber {
                variable_to_set,
                number_to_wrap,
                lower_bound_inclusive,
                upper_bound_exclusive,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), number_to_wrap.json(),
                        lower_bound_inclusive.json(), upper_bound_exclusive.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WrapNum".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ReplaceString {
                variable,
                string_to_change,
                string_part_to_replace,
                replacement,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable.json(), string_to_change.json(), string_part_to_replace
                        .json(), replacement.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReplaceString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetCompassLodestoneLocation {
                variable_to_set,
                item_to_change,
                lodestone_location,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), lodestone_location
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetLodestoneLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::FlattenList { variable_to_set, list_to_flatten } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_flatten.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FlattenList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetPotionEffectAmplifier { variable_to_set, potion_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), potion_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetPotionAmp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectAmount { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleAmount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetDictionarySize { variable_to_set, dictionary_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetDictSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemStackSize {
                variable_to_set,
                item_to_change,
                stack_size,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), stack_size.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemAmount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SubtractVectors { variable_to_set, vectors_to_subtract } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vectors_to_subtract.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SubtractVectors".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetStringCase { variable_to_set, string_to_change } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_change.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCase".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectColor {
                variable_to_set,
                effect_to,
                color_hexadecimal,
                color_variation_,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to.json(), color_hexadecimal
                        .json(), color_variation_.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLightLevel { variable_to_set, light_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), light_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetLight".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBookText { variable_to_set, book, page_number } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), book.json(), page_number.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetBookText ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetDictionaryValues { variable_to_set, dictionary_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetDictValues".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoVector {
                variable_to_set,
                x_component,
                y_component,
                z_component,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), x_component.json(), y_component.json(),
                        z_component.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Vector".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoDistance { variable_to_set, location_1, location_2 } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_1.json(), location_2.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Distance".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemLore {
                variable_to_set,
                item_to_change,
                lore,
                line_number,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), lore.json(),
                        line_number.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemLore".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRoot { variable_to_set, number_input, root_index } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json(), root_index.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Root".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectAmount {
                variable_to_set,
                effect_to,
                particle_amount,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to.json(), particle_amount.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleAmount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AddItemEnchantment {
                variable_to_set,
                item_to_change,
                enchantment_name,
                enchantment_level,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), enchantment_name
                        .json(), enchantment_level.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AddItemEnchant".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemMaterial {
                variable_to_set,
                item_to_get_material_of,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_material_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLocationDirection { variable_to_set, location_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetDirection".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLoreLine {
                variable_to_set,
                item_to_get_lore_from,
                lore_line_to_get,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_get_lore_from.json(),
                        lore_line_to_get.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetLoreLine".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectType { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveString {
                variable,
                string_to_change,
                string_to_remove,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable.json(), string_to_change.json(), string_to_remove.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveString".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetAllBlockData { variable_to_set, block_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetAllBlockData".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoMaximumNumber { variable_to_set, number_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), number_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("MaxNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetDictionaryKeys { variable_to_set, dictionary_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetDictKeys".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::TrimStyledTextContent {
                variable_to_set,
                text_to_trim,
                start_character_position,
                end_character_position,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), text_to_trim.json(),
                        start_character_position.json(), end_character_position.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TrimStyledText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectMaterial {
                variable_to_set,
                effect_to,
                particle_material,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to.json(), particle_material
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleMat".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetLocationCoordinate { variable_to_set, location_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCoord".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveItemCustomTag {
                variable_to_set,
                item_to_change,
                tag_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json(), tag_name.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveItemTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleEffectSize {
                variable_to_set,
                effect_to,
                particle_size,
                size_variation_,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), effect_to.json(), particle_size.json(),
                        size_variation_.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetPotionEffectDuration { variable_to_set, potion_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), potion_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetPotionDur".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRandomLocation {
                variable_to_set,
                location_1,
                location_2,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_1.json(), location_2.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetSoundType {
                variable_to_set,
                sound_to_change,
                sound_name_eg_rabbit_eat,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), sound_to_change.json(),
                        sound_name_eg_rabbit_eat.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSoundType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetCompassLodestoneLocation {
                variable_to_set,
                compass_to_get_lodestone,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), compass_to_get_lodestone.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetLodestoneLoc".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftDirection {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftDirection".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetContainerNameN {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetContainerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectSpread { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleSprd".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ReflectVector {
                variable_to_set,
                vector_to_reflect,
                surface_vector,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_reflect.json(), surface_vector
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ReflectVector".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetHeadOwner { variable_to_set, head_to_get_owner_of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), head_to_get_owner_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetHeadOwner".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemEnchants {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemEnchants".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AppendDictionary { dictionary_to, dictionary } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![dictionary_to.json(), dictionary.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AppendDict".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemMaximumStackSize {
                variable_to_set,
                item_to_get_maximum_stack,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_maximum_stack.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetMaxItemAmount".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetColorChannels { variable_to_set, color_hexadecimal } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), color_hexadecimal.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetColorChannels".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetLocationDirection {
                variable_to_set,
                location_to_change,
                direction,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), direction
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetDirection ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetListValue { list_to_change, index, value_to_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_change.json(), index.json(), value_to_set.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetListValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemEnchantments {
                variable_to_set,
                item_to_change,
                enchantments,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), enchantments
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" SetItemEnchants ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetBookText { variable_to_set, book, pages, page_number } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), book.json(), pages.json(), page_number
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBookText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoRandomValue { variable_to_set, value_set } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), value_set.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemMaterial {
                variable_to_set,
                item_to_change,
                material,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json(), material.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSoundType { variable_to_set, sound_to_get_type__of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_type__of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetSoundType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetListValue {
                variable_to_set,
                list_to_get_value_of,
                index,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), list_to_get_value_of.json(), index.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetListValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoTangent { variable_to_set, number_input } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Tangent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetVoronoiNoise {
                variable_to_set,
                noise_location,
                cell_frequency,
                cell_scatter,
                generation_seed,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), cell_frequency
                        .json(), cell_scatter.json(), generation_seed.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("VoronoiNoise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetDirection {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetDirection".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoHSBColor {
                variable_to_set,
                hue_color_circle_c07c3607,
                saturation_c07c1007,
                brightness_c07c1007,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), hue_color_circle_c07c3607.json(),
                        saturation_c07c1007.json(), brightness_c07c1007.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("HSBColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::IncrementNumberEq { variable, numbers_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable.json(), numbers_to.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("+=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSoundVariant {
                variable_to_set,
                sound_to_get_variant_of,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_variant_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetSoundVariant".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemColor { variable, item_to_get_color_of } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable.json(), item_to_get_color_of.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ClearFormatting { variable_to_set, text_to_change } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), text_to_change.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClearFormatting".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::InsertListValue { list_to_change, index, value_to_insert } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_change.json(), index.json(), value_to_insert.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("InsertListValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetSoundVolume { variable_to_set, sound_to_change, volume } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_change.json(), volume.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSoundVolume".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetLocationCoordinate {
                variable_to_set,
                location_to_change,
                coordinate,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), coordinate
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCoord".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AddVectors { variable_to_set, vectors_to_add } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vectors_to_add.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AddVectors".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetPitch {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPitch".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RaycastBlock {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RaycastBlock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemEnchantments {
                variable_to_set,
                item_to_get_enchantments_from,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_enchantments_from.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" GetItemEnchants ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetHeadTexture {
                variable_to_set,
                player_head,
                owner_name_uuid_or,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), player_head.json(), owner_name_uuid_or
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetHeadTexture".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetPerlinNoise {
                variable_to_set,
                noise_location,
                frequency_scale,
                octaves_perlin_layers,
                octave_frequency_gain,
                octave_amplitude_gain,
                generation_seed,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), frequency_scale
                        .json(), octaves_perlin_layers.json(), octave_frequency_gain
                        .json(), octave_amplitude_gain.json(), generation_seed.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("PerlinNoise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetWorleyNoise {
                variable_to_set,
                noise_location,
                cell_frequency,
                cell_scatter,
                generation_seed,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), cell_frequency
                        .json(), cell_scatter.json(), generation_seed.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("WorleyNoise".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemColor {
                variable_to_set,
                item_to_change,
                color_hexadecimal,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), color_hexadecimal
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectColor { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleColor".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetSoundPitch { variable_to_set, sound_to_change, pitch } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_change.json(), pitch.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSoundPitch".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RoundNumberN {
                variable_to_set,
                number_to_round,
                round_multiple,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), number_to_round.json(), round_multiple
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String(" RoundNumber ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetPlaceableBlocks { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCanPlaceOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SortList { variable_to_set, list_to_sort } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_sort.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SortList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetCustomSoundKey {
                variable_to_set,
                sound_to_change,
                sound_key,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), sound_to_change.json(), sound_key.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCustomSound".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveDictionaryEntry {
                dictionary_to_change,
                key_to_remove,
                expected_values,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        dictionary_to_change.json(), key_to_remove.json(),
                        expected_values.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveDictEntry".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::FormatTimestamp {
                variable_to_set,
                time_to_format,
                custom_format,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), time_to_format.json(), custom_format
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("FormatTime".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemVisibilityFlags { variable_to_set, item } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), item.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemFlags".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetStringLength { variable_to_set, string_to_measure } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_measure.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("StringLength".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemPotionEffects {
                variable_to_set,
                item_to_get_effects_from,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_effects_from.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemEffects".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetMiniMessageExpression { variable_to_set, text_to_read } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), text_to_read.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetMiniMessageExpr".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetYaw {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetYaw".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetItemPotionEffects {
                variable_to_set,
                item_to_change,
                item_effects,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), item_effects
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemEffects".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::DecrementNumberEq { variable, numbers_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable.json(), numbers_to.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("-=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemCustomTag {
                variable_to_set,
                item_to_get_tag_of,
                tag_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_get_tag_of.json(), tag_name
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemTag".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::CreateList { variable_to_set, value_list } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![variable_to_set.json(), value_list.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("CreateList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AppendListtoList { list_to_append_to, lists_to_append } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![list_to_append_to.json(), lists_to_append.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("AppendList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetContainerContents {
                variable_to_set,
                container_location,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), container_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetContainerItems".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationTowardLocation {
                variable_to_set,
                location_to_shift,
                target_location,
                shift_distance,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), target_location
                        .json(), shift_distance.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftToward".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::TrimList {
                variable_to_set,
                list_to_trim,
                start_index,
                end_index,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), list_to_trim.json(), start_index.json(),
                        end_index.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TrimList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoCosine { variable_to_set, number_input } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Cosine".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetVectorComponent {
                variable_to_set,
                vector_to_change,
                component,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_change.json(), component.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetVectorComp".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParseNumberfromString {
                variable_to_set,
                string_to_convert,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_convert.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SettoExponential {
                variable_to_set,
                number_input,
                exponent,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json(), exponent.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Exponent".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftAllDirs {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftAllDirs".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetListIndexofValue {
                variable_to_set,
                list_to_search_in,
                value_to_search,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), list_to_search_in.json(), value_to_search
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetValueIndex".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveItemEnchantment {
                variable_to_set,
                item_to_change,
                enchantment_name,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), enchantment_name
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemItemEnchant".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBookTextN {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetBookText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetParticleRoll {
                variable_to_set,
                effect_to,
                particle_roll,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to.json(), particle_roll.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetParticleRoll".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetSoundVariant {
                variable_to_set,
                sound_to_change,
                variant_id_eg_break1,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), sound_to_change.json(),
                        variant_id_eg_break1.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetSoundVariant".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocation {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftLocation".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RandomizeList { variable_to_set, list_to_randomize } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_randomize.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RandomizeList".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ClampNumber {
                variable_to_set,
                number_to_clamp,
                minimum,
                maximum,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), number_to_clamp.json(), minimum.json(),
                        maximum.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ClampNumber".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::Round {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Round".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSoundPitch { variable_to_set, sound_to_get_pitch_or } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_pitch_or.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetSoundPitch".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::TranslateColors {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TranslateColors".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBlockGrowth { variable_to_set, block_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetBlockGrowth".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetAllCustomItemTags {
                variable_to_set,
                item_to_get_tags_from,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_tags_from.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetAllItemTags".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RemoveListValue { list_to_change, values_to } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![list_to_change.json(), values_to.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RemoveListValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ShiftLocationinAllDirections {
                variable_to_set,
                location_to_shift,
                forwards_change,
                upwards_change,
                sideways_change_l__r,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), forwards_change
                        .json(), upwards_change.json(), sideways_change_l__r.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ShiftAllDirections".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetPlaceableBlocks {
                variable_to_set,
                item_to_change,
                placeable_blocks,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), placeable_blocks
                        .json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCanPlaceOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBlockMaterial { variable_to_set, block_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetBlockType".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::ParsePitch {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParsePitch".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetDictionaryValue { variable_to_set, dictionary_to, key } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to.json(), key.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetDictValue".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetContainerLock { variable_to_set, container_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), container_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ContainerLock".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBlockPower { variable_to_set, block_location } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetBlockPower".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetVectorLength { variable_to_set, vector_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetVectorLength".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetCustomModelData {
                variable_to_set,
                item_to_change,
                model_value,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), model_value.json()
                    ],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetModelData".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetCoords {} => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetCoords".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectSize { variable_to_set, effect_to_get } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetParticleSize".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
        }
    }
}
