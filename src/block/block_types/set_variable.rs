use either::Either;
use serde_json::Value;
use crate::types::*;
use crate::block::block_types::subactions::*;
pub enum SetVariable {
    SettoString {
        variable_to_set: VariableLiteral,
        string_to_set_to: Vec<AnyType>,
        text_value_merging_tag: TextValueMergingSettoString,
    },
    SetParticleEffectType {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        type_: Text,
    },
    SetItemEnchants {},
    PurgeMatchingVariables {
        name_to_match: Vec<Text>,
        match_requirement_tag: MatchRequirementPurgeMatchingVariables,
        ignore_case_tag: IgnoreCasePurgeMatchingVariables,
    },
    ShiftLocationonAllAxes {
        variable_to_set: VariableLiteral,
        location_to_change: Option<Location>,
        x_change: Option<Number>,
        y_change: Option<Number>,
        z_change: Option<Number>,
    },
    GetParticleEffectMaterial {
        variable_to_set: VariableLiteral,
        effect_to_get: Particle,
    },
    SetParticleEffectSpread {
        variable_to_set: VariableLiteral,
        effect_to_change: Option<Particle>,
        horizontal_spread: Number,
        vertical_spread: Number,
    },
    SettoAbsoluteValue {
        variable_to_set: VariableLiteral,
        number_input: Option<Number>,
    },
    AppendValuetoList {
        list_to_append_to: VariableLiteral,
        values_to_append: Vec<AnyType>,
    },
    SettoRemainder {
        variable_to_set: VariableLiteral,
        dividend: Number,
        divisor: Number,
        remainder_mode_tag: RemainderModeSettoRemainder,
    },
    ShiftLocationonVector {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        shift_vector: Vector,
        shift_distance: Option<Number>,
        add_location_rotation_tag: AddLocationRotationShiftLocationonVector,
    },
    GetItemAttribute {
        variable_to_set: VariableLiteral,
        item: Item,
        attribute_tag: AttributeGetItemAttribute,
        active_equipment_slot_tag: ActiveEquipmentSlotGetItemAttribute,
    },
    ClearDictionary { dictionary_to_clear: VariableLiteral },
    AddNumbers { variable_to_set: VariableLiteral, numbers_to_add: Vec<Number> },
    ShiftLocationRotation {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        rotation_amount: Number,
        rotation_axis_tag: RotationAxisShiftLocationRotation,
    },
    SubtractNumbers {
        variable_to_set: VariableLiteral,
        numbers_to_subtract: Vec<Number>,
    },
    GetItemName { variable_to_set: VariableLiteral, item_to_get_name_of: Item },
    GetItemRarity { variable_to_set: VariableLiteral, item: Item },
    MultiplyVector {
        variable_to_set: VariableLiteral,
        vector_to_multiply: Option<Vector>,
        multiplier: Number,
    },
    DivideNumbers {
        variable_to_set: VariableLiteral,
        numbers_to_divide: Vec<Number>,
        division_mode_tag: DivisionModeDivideNumbers,
    },
    GetSignText { sign_line_tag: SignLineGetSignText },
    SettoBitwiseOperation {
        variable_to_set: VariableLiteral,
        operand_1: Number,
        operand_2: Option<Number>,
        operator_tag: OperatorSettoBitwiseOperation,
    },
    GetLecternPage { variable_to_set: VariableLiteral, lectern_location: Location },
    ParseX {},
    ShiftLocationonAxis {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        shift_distance: Number,
        coordinate_tag: CoordinateShiftLocationonAxis,
    },
    ParseY {},
    SettoVectorBetweenLocations {
        variable_to_set: VariableLiteral,
        start_location: Location,
        end_location: Location,
    },
    ParseZ {},
    GetVectorComponent {
        variable_to_set: VariableLiteral,
        vector_to_get: Vector,
        component_tag: ComponentGetVectorComponent,
    },
    SettoValueEq { variable_to_set: VariableLiteral, value: AnyType },
    RmText { regular_expressions_tag: RegularExpressionsRmText },
    AddItemAttribute {
        variable_to_set: VariableLiteral,
        item: Option<Item>,
        modifier_amount: Number,
        attribute_tag: AttributeAddItemAttribute,
        operation_tag: OperationAddItemAttribute,
        active_equipment_slot_tag: ActiveEquipmentSlotAddItemAttribute,
    },
    SettoCenterLocation {
        variable_to_set: VariableLiteral,
        locations_to_center: Vec<Location>,
    },
    AlignLocation {
        variable_to_set: VariableLiteral,
        location_to_align: Option<Location>,
        alignment_mode_tag: AlignmentModeAlignLocation,
        coordinates_tag: CoordinatesAlignLocation,
        rotation_tag: RotationAlignLocation,
    },
    GetSoundVolume { variable_to_set: VariableLiteral, sound_to_get_volume_of: Sound },
    SettoRandomNumber {
        variable_to_set: VariableLiteral,
        minimum_number: Number,
        maximum_number: Number,
        rounding_mode_tag: RoundingModeSettoRandomNumber,
    },
    GetContainerName { variable_to_set: VariableLiteral, container_location: Location },
    RaycastfromLocation {
        variable_to_set: VariableLiteral,
        ray_origin: Location,
        ray_distance: Number,
        entity_collision_tag: EntityCollisionRaycastfromLocation,
        block_collision_tag: BlockCollisionRaycastfromLocation,
    },
    RotateVectorAroundVector {
        variable_to_set: VariableLiteral,
        vector_to_rotate: Option<Vector>,
        axis_vector: Vector,
        angle: Number,
        angle_units_tag: AngleUnitsRotateVectorAroundVector,
    },
    GetParticleEffectMotion {
        variable_to_set: VariableLiteral,
        effect_to_get: Particle,
    },
    SetParticleMotion {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        particle_motion: Option<Vector>,
        motion_variation_: Option<Number>,
    },
    SettoAverageNumber {
        variable_to_set: VariableLiteral,
        numbers_to_average: Vec<Number>,
    },
    WrapNumberOld {},
    SetY {},
    SetMapTexture {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        image_url: Text,
    },
    GetBlockData {
        variable_to_set: VariableLiteral,
        block_location: Location,
        tag_name: Text,
    },
    SetX {},
    SortDictionary {
        variable_to_set: VariableLiteral,
        dictionary_to_sort: Option<Dict>,
        sorting_type__tag: SortingTypeSortDictionary,
        sorting_order_tag: SortingOrderSortDictionary,
    },
    GetLecternBook { variable_to_set: VariableLiteral, lectern_location: Location },
    GetCustomSoundKey { variable_to_set: VariableLiteral, sound_to_get_key_of: Sound },
    SettoCrossProduct {
        variable_to_set: VariableLiteral,
        vector_1: Vector,
        vector_2: Vector,
    },
    MultiplyNumbers {
        variable_to_set: VariableLiteral,
        numbers_to_multiply: Vec<Number>,
    },
    GetParticleRoll {
        variable_to_set: VariableLiteral,
        effect_to_get: Option<Particle>,
    },
    ParseYaw {},
    SettoDotProduct {
        variable_to_set: VariableLiteral,
        vector_1: Vector,
        vector_2: Vector,
    },
    SetZ {},
    SetArmorTrim {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        trim_pattern_tag: TrimPatternSetArmorTrim,
        trim_material_tag: TrimMaterialSetArmorTrim,
    },
    PopListValue {
        variable_to_set: VariableLiteral,
        list_to_get_value_of: List,
        index: Option<Number>,
    },
    SettoMinimumNumber { variable_to_set: VariableLiteral, number_set: Vec<Number> },
    GetPotionEffectType { variable_to_set: VariableLiteral, potion_to_get: Potion },
    SetItemName {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        name: Vec<MiniMessage>,
    },
    GetListLength { variable_to_set: VariableLiteral, list_to_measure: List },
    SettoSine {
        variable_to_set: VariableLiteral,
        number_input: Number,
        sine_variant_tag: SineVariantSettoSine,
        input_tag: InputSettoSine,
    },
    SettoDirectionName { variable_to_set: VariableLiteral, direction: Vector },
    RepeatString {
        variable_to_set: VariableLiteral,
        string_to_repeat: Text,
        times_to_repeat: Number,
    },
    GetItemLore {},
    JoinString {
        variable_to_set: VariableLiteral,
        strings_to_join: List,
        joining_string: Option<Text>,
        final_joining_string: Option<Text>,
    },
    ReverseList { variable_to_set: VariableLiteral, list_to_reverse: Option<List> },
    RemoveDuplicateListElements {
        variable_to_set: VariableLiteral,
        list_to_deduplicate: Option<List>,
    },
    CreateDictionary {
        variable_to_set: VariableLiteral,
        key_list: Option<List>,
        value_list: Option<List>,
    },
    RoundNumber { round_mode_tag: RoundModeRoundNumber },
    FaceLocation {
        variable_to_set: VariableLiteral,
        location_to_change: Option<Location>,
        target_location: Location,
        face_direction_tag: FaceDirectionFaceLocation,
    },
    GetItemLoreLine {},
    SetVectorLength {
        variable_to_set: VariableLiteral,
        vector_to_change: Option<Vector>,
        length: Option<Number>,
    },
    SetPotionEffectDuration {
        variable_to_set: VariableLiteral,
        potion_to_change: Option<Potion>,
        duration_ticks: Number,
    },
    SplitString {
        variable_to_set: VariableLiteral,
        string_to_split: Text,
        splitter_string: Option<Text>,
    },
    SettoNormallyDistributedRandomNumber {
        variable_to_set: VariableLiteral,
        mean_midpoint: Number,
        standard_deviation: Number,
        distribution_tag: DistributionSettoNormallyDistributedRandomNumber,
    },
    SetPotionEffectType {
        variable_to_set: VariableLiteral,
        potion_to_change: Option<Potion>,
        type_: Text,
    },
    AlignVector { variable_to_set: VariableLiteral, vector_to_align: Option<Vector> },
    SetItemDurability {
        variable_to_set: VariableLiteral,
        item: Option<Item>,
        item_durability: Number,
        durability_type__tag: DurabilityTypeSetItemDurability,
    },
    SetItemBreakability {
        variable_to_set: VariableLiteral,
        item: Option<Item>,
        breakability_tag: BreakabilitySetItemBreakability,
    },
    GetSignTextN {
        variable_to_set: VariableLiteral,
        location: Location,
        sign_line_tag: SignLineGetSignTextN,
        sign_side_tag: SignSideGetSignTextN,
    },
    RaycastEntity {},
    SetDictionaryValue {
        dictionary_to_add_to: VariableLiteral,
        key: Text,
        value: AnyType,
    },
    SetAllLocationCoordinates {
        variable_to_set: VariableLiteral,
        location_to_change: Option<Location>,
        new_x: Option<Number>,
        new_y: Option<Number>,
        new_z: Option<Number>,
        new_pitch: Option<Number>,
        new_yaw: Option<Number>,
        coordinate_type__tag: CoordinateTypeSetAllLocationCoordinates,
    },
    SettoRGBColor {
        variable_to_set: VariableLiteral,
        red_c07c2557: Number,
        green_c07c2557: Number,
        blue_c07c2557: Either<Number, List>,
    },
    SetBreakableBlocks {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        breakable_blocks: Vec<Block>,
    },
    SettoHSLColor {
        variable_to_set: VariableLiteral,
        hue_color_circle_c07c3607: Number,
        saturation_c07c1007: Option<Number>,
        lightness_c07c1007: Option<Either<Number, List>>,
    },
    GetDirection { return_type__tag: ReturnTypeGetDirection },
    GetItemLoreN { variable_to_set: VariableLiteral, item_to_get_lore_from: Item },
    RemoveListValueatIndex {
        list_to_change: VariableLiteral,
        index_to_remove: Vec<Number>,
    },
    SettoLogarithm {
        variable_to_set: VariableLiteral,
        number_input: Option<Number>,
        base: Number,
    },
    SetItemCustomTag {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        tag_name: Text,
        tag_value: Either<Number, Text>,
    },
    TrimString {
        variable_to_set: VariableLiteral,
        string_to_trim: Option<Text>,
        start_character_position: Number,
        end_character_position: Option<Number>,
    },
    ParseMiniMessageExpression {
        variable_to_set: VariableLiteral,
        string_to_parse: Text,
        allowed_tags_tag: AllowedTagsParseMiniMessageExpression,
        parse_legacy_color_codes_tag: ParseLegacyColorCodesParseMiniMessageExpression,
    },
    GetItemStackSize { variable_to_set: VariableLiteral, item_to_get_stack: Item },
    SetPotionEffectAmplifier {
        variable_to_set: VariableLiteral,
        potion_to_change: Option<Potion>,
        amplifier: Number,
    },
    GetBreakableBlocks { variable_to_set: VariableLiteral, item: Item },
    RotateVectorAroundAxis {
        variable_to_set: VariableLiteral,
        vector_to_rotate: Option<Vector>,
        angle: Number,
        axis_tag: AxisRotateVectorAroundAxis,
        angle_units_tag: AngleUnitsRotateVectorAroundAxis,
    },
    GetItemNameN {},
    GetItemDurability {
        variable_to_set: VariableLiteral,
        item: Item,
        durability_type__tag: DurabilityTypeGetItemDurability,
    },
    ShiftLocationinDirection {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        shift_distance: Option<Number>,
        direction_tag: DirectionShiftLocationinDirection,
    },
    WrapNumber {
        variable_to_set: VariableLiteral,
        number_to_wrap: Option<Number>,
        lower_bound_inclusive: Number,
        upper_bound_exclusive: Number,
    },
    ReplaceString {
        variable: VariableLiteral,
        string_to_change: Text,
        string_part_to_replace: Text,
        replacement: Text,
        regular_expressions_tag: RegularExpressionsReplaceString,
        replacement_type__tag: ReplacementTypeReplaceString,
    },
    SetCompassLodestoneLocation {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        lodestone_location: Location,
        require_lodestone_at_location_tag: RequireLodestoneatLocationSetCompassLodestoneLocation,
    },
    FlattenList { variable_to_set: VariableLiteral, list_to_flatten: Option<List> },
    GetPotionEffectAmplifier { variable_to_set: VariableLiteral, potion_to_get: Potion },
    GetParticleEffectAmount {
        variable_to_set: VariableLiteral,
        effect_to_get: Particle,
    },
    GetDictionarySize { variable_to_set: VariableLiteral, dictionary_to: Dict },
    SetItemStackSize {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        stack_size: Number,
    },
    SubtractVectors {
        variable_to_set: VariableLiteral,
        vectors_to_subtract: Vec<Vector>,
    },
    SetStringCase {
        variable_to_set: VariableLiteral,
        string_to_change: Option<Text>,
        capitalization_type__tag: CapitalizationTypeSetStringCase,
    },
    SetParticleEffectColor {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        color_hexadecimal: Text,
        color_variation_: Option<Number>,
    },
    GetLightLevel {
        variable_to_set: VariableLiteral,
        light_location: Location,
        light_type__tag: LightTypeGetLightLevel,
    },
    GetBookText {
        variable_to_set: VariableLiteral,
        book: Item,
        page_number: Option<Number>,
    },
    GetDictionaryValues { variable_to_set: VariableLiteral, dictionary_to: Dict },
    SettoVector {
        variable_to_set: VariableLiteral,
        x_component: Number,
        y_component: Number,
        z_component: Number,
    },
    SettoDistance {
        variable_to_set: VariableLiteral,
        location_1: Location,
        location_2: Location,
        distance_type__tag: DistanceTypeSettoDistance,
    },
    SetItemLore {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        lore: Vec<Either<MiniMessage, MiniMessage>>,
        line_number: Number,
    },
    SettoRoot {
        variable_to_set: VariableLiteral,
        number_input: Option<Number>,
        root_index: Option<Number>,
    },
    SetParticleEffectAmount {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        particle_amount: Number,
    },
    AddItemEnchantment {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        enchantment_name: Text,
        enchantment_level: Number,
    },
    GetItemMaterial {
        variable_to_set: VariableLiteral,
        item_to_get_material_of: Item,
        return_value_type__tag: ReturnValueTypeGetItemMaterial,
    },
    GetLocationDirection { variable_to_set: VariableLiteral, location_to_get: Location },
    GetLoreLine {
        variable_to_set: VariableLiteral,
        item_to_get_lore_from: Item,
        lore_line_to_get: Number,
    },
    GetParticleEffectType { variable_to_set: VariableLiteral, effect_to_get: Particle },
    RemoveString {
        variable: VariableLiteral,
        string_to_change: Option<Text>,
        string_to_remove: Vec<Text>,
        regular_expressions_tag: RegularExpressionsRemoveString,
    },
    GetAllBlockData {
        variable_to_set: VariableLiteral,
        block_location: Location,
        hide_default_tag: HideDefaultGetAllBlockData,
    },
    SettoMaximumNumber { variable_to_set: VariableLiteral, number_set: Vec<Number> },
    GetDictionaryKeys { variable_to_set: VariableLiteral, dictionary_to: Dict },
    TrimStyledTextContent {
        variable_to_set: VariableLiteral,
        text_to_trim: Option<MiniMessage>,
        start_character_position: Number,
        end_character_position: Option<Number>,
    },
    SetParticleEffectMaterial {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        particle_material: Item,
    },
    GetLocationCoordinate {
        variable_to_set: VariableLiteral,
        location_to_get: Location,
        coordinate_type__tag: CoordinateTypeGetLocationCoordinate,
        coordinate_tag: CoordinateGetLocationCoordinate,
    },
    RemoveItemCustomTag {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        tag_name: Text,
    },
    SetParticleEffectSize {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        particle_size: Number,
        size_variation_: Option<Number>,
    },
    GetPotionEffectDuration { variable_to_set: VariableLiteral, potion_to_get: Potion },
    SettoRandomLocation {
        variable_to_set: VariableLiteral,
        location_1: Location,
        location_2: Location,
    },
    SetSoundType {
        variable_to_set: VariableLiteral,
        sound_to_change: Option<Sound>,
        sound_name_eg_rabbit_eat: Text,
    },
    GetCompassLodestoneLocation {
        variable_to_set: VariableLiteral,
        compass_to_get_lodestone: Item,
    },
    ShiftDirection { direction_tag: DirectionShiftDirection },
    GetContainerNameN {},
    GetParticleEffectSpread {
        variable_to_set: VariableLiteral,
        effect_to_get: Particle,
        spread_tag: SpreadGetParticleEffectSpread,
    },
    ReflectVector {
        variable_to_set: VariableLiteral,
        vector_to_reflect: Option<Vector>,
        surface_vector: Vector,
    },
    GetHeadOwner {
        variable_to_set: VariableLiteral,
        head_to_get_owner_of: Item,
        text_value_tag: TextValueGetHeadOwner,
    },
    GetItemEnchants {},
    AppendDictionary { dictionary_to: VariableLiteral, dictionary: Dict },
    GetItemMaximumStackSize {
        variable_to_set: VariableLiteral,
        item_to_get_maximum_stack: Item,
    },
    GetColorChannels {
        variable_to_set: VariableLiteral,
        color_hexadecimal: Text,
        color_channels_tag: ColorChannelsGetColorChannels,
    },
    SetLocationDirection {
        variable_to_set: VariableLiteral,
        location_to_change: Option<Location>,
        direction: Vector,
    },
    SetListValue {
        list_to_change: VariableLiteral,
        index: Number,
        value_to_set: AnyType,
    },
    SetItemEnchantments {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        enchantments: Dict,
    },
    SetBookText {
        variable_to_set: VariableLiteral,
        book: Option<Item>,
        pages: Vec<Either<MiniMessage, MiniMessage>>,
        page_number: Number,
    },
    SettoRandomValue { variable_to_set: VariableLiteral, value_set: Vec<AnyType> },
    SetItemMaterial {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        material: Text,
    },
    GetSoundType { variable_to_set: VariableLiteral, sound_to_get_type__of: Sound },
    GetListValue {
        variable_to_set: VariableLiteral,
        list_to_get_value_of: List,
        index: Number,
    },
    SettoTangent {
        variable_to_set: VariableLiteral,
        number_input: Number,
        tangent_variant_tag: TangentVariantSettoTangent,
        input_tag: InputSettoTangent,
    },
    GetVoronoiNoise {
        variable_to_set: VariableLiteral,
        noise_location: Location,
        cell_frequency: Option<Number>,
        cell_scatter: Option<Number>,
        generation_seed: Option<Number>,
        cell_edge_type__tag: CellEdgeTypeGetVoronoiNoise,
    },
    SetDirection { face_direction_tag: FaceDirectionSetDirection },
    SettoHSBColor {
        variable_to_set: VariableLiteral,
        hue_color_circle_c07c3607: Number,
        saturation_c07c1007: Option<Number>,
        brightness_c07c1007: Option<Either<Number, List>>,
    },
    IncrementNumberEq { variable: VariableLiteral, numbers_to: Vec<Number> },
    GetSoundVariant { variable_to_set: VariableLiteral, sound_to_get_variant_of: Sound },
    GetItemColor { variable: VariableLiteral, item_to_get_color_of: Item },
    ClearFormatting {
        variable_to_set: VariableLiteral,
        text_to_change: Option<MiniMessage>,
    },
    InsertListValue {
        list_to_change: VariableLiteral,
        index: Number,
        value_to_insert: AnyType,
    },
    SetSoundVolume {
        variable_to_set: VariableLiteral,
        sound_to_change: Option<Sound>,
        volume: Number,
    },
    SetLocationCoordinate {
        variable_to_set: VariableLiteral,
        location_to_change: Option<Location>,
        coordinate: Number,
        coordinate_type__tag: CoordinateTypeSetLocationCoordinate,
        coordinate_tag: CoordinateSetLocationCoordinate,
    },
    AddVectors { variable_to_set: VariableLiteral, vectors_to_add: Vec<Vector> },
    SetPitch {},
    RaycastBlock {
        ignore_passable_blocks_tag: IgnorePassableBlocksRaycastBlock,
        fluid_collision_tag: FluidCollisionRaycastBlock,
    },
    GetItemEnchantments {
        variable_to_set: VariableLiteral,
        item_to_get_enchantments_from: Item,
    },
    SetHeadTexture {
        variable_to_set: VariableLiteral,
        player_head: Option<Item>,
        owner_name_uuid_or: Text,
    },
    GetPerlinNoise {
        variable_to_set: VariableLiteral,
        noise_location: Location,
        frequency_scale: Option<Number>,
        octaves_perlin_layers: Option<Number>,
        octave_frequency_gain: Option<Number>,
        octave_amplitude_gain: Option<Number>,
        generation_seed: Option<Number>,
        fractal_type__tag: FractalTypeGetPerlinNoise,
    },
    GetWorleyNoise {
        variable_to_set: VariableLiteral,
        noise_location: Location,
        cell_frequency: Option<Number>,
        cell_scatter: Option<Number>,
        generation_seed: Option<Number>,
        cell_edge_type__tag: CellEdgeTypeGetWorleyNoise,
        distance_calculation_tag: DistanceCalculationGetWorleyNoise,
    },
    SetItemColor {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        color_hexadecimal: Text,
    },
    GetParticleEffectColor { variable_to_set: VariableLiteral, effect_to_get: Particle },
    SetSoundPitch {
        variable_to_set: VariableLiteral,
        sound_to_change: Option<Sound>,
        pitch: Either<Number, Text>,
    },
    RoundNumberN {
        variable_to_set: VariableLiteral,
        number_to_round: Option<Number>,
        round_multiple: Option<Number>,
        round_mode_tag: RoundModeRoundNumberN,
    },
    GetPlaceableBlocks { variable_to_set: VariableLiteral, item: Item },
    SortList {
        variable_to_set: VariableLiteral,
        list_to_sort: Option<List>,
        sort_order_tag: SortOrderSortList,
    },
    SetCustomSoundKey {
        variable_to_set: VariableLiteral,
        sound_to_change: Option<Sound>,
        sound_key: Option<Text>,
    },
    RemoveDictionaryEntry {
        dictionary_to_change: VariableLiteral,
        key_to_remove: Text,
        expected_values: Vec<AnyType>,
    },
    FormatTimestamp {
        variable_to_set: VariableLiteral,
        time_to_format: Number,
        custom_format: Option<Text>,
        format_tag: FormatFormatTimestamp,
    },
    SetItemVisibilityFlags {
        variable_to_set: VariableLiteral,
        item: Option<Item>,
        hide_armor_trim_tag: HideArmorTrimSetItemVisibilityFlags,
        hide_color_tag: HideColorSetItemVisibilityFlags,
        hide_enchantments_tag: HideEnchantmentsSetItemVisibilityFlags,
        hide_attributes_tag: HideAttributesSetItemVisibilityFlags,
        hide_unbreakable_tag: HideUnbreakableSetItemVisibilityFlags,
        hide_can_destroy_tag: HideCanDestroySetItemVisibilityFlags,
        hide_can_place_on_tag: HideCanPlaceOnSetItemVisibilityFlags,
        hide_potion_effects_tag: HidePotionEffectsSetItemVisibilityFlags,
    },
    GetStringLength { variable_to_set: VariableLiteral, string_to_measure: Text },
    GetItemPotionEffects {
        variable_to_set: VariableLiteral,
        item_to_get_effects_from: Item,
    },
    GetMiniMessageExpression {
        variable_to_set: VariableLiteral,
        text_to_read: MiniMessage,
    },
    SetYaw {},
    SetItemPotionEffects {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        item_effects: Vec<Potion>,
    },
    DecrementNumberEq { variable: VariableLiteral, numbers_to: Vec<Number> },
    GetItemCustomTag {
        variable_to_set: VariableLiteral,
        item_to_get_tag_of: Item,
        tag_name: Text,
    },
    CreateList { variable_to_set: VariableLiteral, value_list: Vec<AnyType> },
    AppendListtoList { list_to_append_to: VariableLiteral, lists_to_append: Vec<List> },
    GetContainerContents {
        variable_to_set: VariableLiteral,
        container_location: Location,
        ignore_empty_slots_tag: IgnoreEmptySlotsGetContainerContents,
    },
    ShiftLocationTowardLocation {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        target_location: Location,
        shift_distance: Option<Number>,
    },
    TrimList {
        variable_to_set: VariableLiteral,
        list_to_trim: Option<List>,
        start_index: Number,
        end_index: Option<Number>,
    },
    SettoCosine {
        variable_to_set: VariableLiteral,
        number_input: Number,
        cosine_variant_tag: CosineVariantSettoCosine,
        input_tag: InputSettoCosine,
    },
    SetVectorComponent {
        variable_to_set: VariableLiteral,
        vector_to_change: Option<Vector>,
        component: Number,
        component_tag: ComponentSetVectorComponent,
    },
    ParseNumberfromString {
        variable_to_set: VariableLiteral,
        string_to_convert: Option<Text>,
    },
    SettoExponential {
        variable_to_set: VariableLiteral,
        number_input: Option<Number>,
        exponent: Option<Number>,
    },
    ShiftAllDirs { ignore_pitch_tag: IgnorePitchShiftAllDirs },
    GetListIndexofValue {
        variable_to_set: VariableLiteral,
        list_to_search_in: List,
        value_to_search: AnyType,
        search_order_tag: SearchOrderGetListIndexofValue,
    },
    RemoveItemEnchantment {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        enchantment_name: Text,
    },
    GetBookTextN {},
    SetParticleRoll {
        variable_to_set: VariableLiteral,
        effect_to: Option<Particle>,
        particle_roll: Number,
    },
    SetSoundVariant {
        variable_to_set: VariableLiteral,
        sound_to_change: Option<Sound>,
        variant_id_eg_break1: Option<Text>,
    },
    ShiftLocation { shift_direction_tag: ShiftDirectionShiftLocation },
    RandomizeList { variable_to_set: VariableLiteral, list_to_randomize: Option<List> },
    ClampNumber {
        variable_to_set: VariableLiteral,
        number_to_clamp: Option<Number>,
        minimum: Number,
        maximum: Number,
    },
    Round { round_mode_tag: RoundModeRound },
    GetSoundPitch {
        variable_to_set: VariableLiteral,
        sound_to_get_pitch_or: Sound,
        return_value_type__tag: ReturnValueTypeGetSoundPitch,
    },
    TranslateColors { translation_type__tag: TranslationTypeTranslateColors },
    GetBlockGrowth {
        variable_to_set: VariableLiteral,
        block_location: Location,
        growth_unit_tag: GrowthUnitGetBlockGrowth,
    },
    GetAllCustomItemTags {
        variable_to_set: VariableLiteral,
        item_to_get_tags_from: Item,
    },
    RemoveListValue { list_to_change: VariableLiteral, values_to: Vec<AnyType> },
    ShiftLocationinAllDirections {
        variable_to_set: VariableLiteral,
        location_to_shift: Option<Location>,
        forwards_change: Option<Number>,
        upwards_change: Option<Number>,
        sideways_change_l__r: Option<Number>,
    },
    SetPlaceableBlocks {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        placeable_blocks: Vec<Block>,
    },
    GetBlockMaterial {
        variable_to_set: VariableLiteral,
        block_location: Location,
        return_value_type__tag: ReturnValueTypeGetBlockMaterial,
    },
    ParsePitch {},
    GetDictionaryValue {
        variable_to_set: VariableLiteral,
        dictionary_to: Dict,
        key: Text,
    },
    GetContainerLock { variable_to_set: VariableLiteral, container_location: Location },
    GetBlockPower { variable_to_set: VariableLiteral, block_location: Location },
    GetVectorLength {
        variable_to_set: VariableLiteral,
        vector_to_get: Vector,
        length_type__tag: LengthTypeGetVectorLength,
    },
    SetCustomModelData {
        variable_to_set: VariableLiteral,
        item_to_change: Option<Item>,
        model_value: Number,
    },
    SetCoords {},
    GetParticleEffectSize {
        variable_to_set: VariableLiteral,
        effect_to_get: Option<Particle>,
    },
}
impl SetVariable {
    pub fn compile(&self) -> Value {
        match self {
            SetVariable::SettoString {
                variable_to_set,
                string_to_set_to,
                text_value_merging_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_set_to.json()],
                    vec![text_value_merging_tag.json()],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetItemEnchants".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::PurgeMatchingVariables {
                name_to_match,
                match_requirement_tag,
                ignore_case_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![name_to_match.json()],
                    vec![match_requirement_tag.json(), ignore_case_tag.json()],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SettoRemainder {
                variable_to_set,
                dividend,
                divisor,
                remainder_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dividend.json(), divisor.json()],
                    vec![remainder_mode_tag.json()],
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
                add_location_rotation_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_vector
                        .json(), shift_distance.json()
                    ],
                    vec![add_location_rotation_tag.json()],
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
            SetVariable::GetItemAttribute {
                variable_to_set,
                item,
                attribute_tag,
                active_equipment_slot_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![attribute_tag.json(), active_equipment_slot_tag.json()],
                );
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
                let item_args = compile(vec![dictionary_to_clear.json()], vec![]);
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
                    vec![],
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
                rotation_axis_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), rotation_amount
                        .json()
                    ],
                    vec![rotation_axis_tag.json()],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![],
                );
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
                    vec![],
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
            SetVariable::DivideNumbers {
                variable_to_set,
                numbers_to_divide,
                division_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), numbers_to_divide.json()],
                    vec![division_mode_tag.json()],
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
            SetVariable::GetSignText { sign_line_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![sign_line_tag.json()]);
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
                operator_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), operand_1.json(), operand_2.json()],
                    vec![operator_tag.json()],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                coordinate_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_distance
                        .json()
                    ],
                    vec![coordinate_tag.json()],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("ParseZ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetVectorComponent {
                variable_to_set,
                vector_to_get,
                component_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_get.json()],
                    vec![component_tag.json()],
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
                let item_args = compile(
                    vec![variable_to_set.json(), value.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("=".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RmText { regular_expressions_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![regular_expressions_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("RmText".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::AddItemAttribute {
                variable_to_set,
                item,
                modifier_amount,
                attribute_tag,
                operation_tag,
                active_equipment_slot_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json(), modifier_amount.json()],
                    vec![
                        attribute_tag.json(), operation_tag.json(),
                        active_equipment_slot_tag.json()
                    ],
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
                    vec![],
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
            SetVariable::AlignLocation {
                variable_to_set,
                location_to_align,
                alignment_mode_tag,
                coordinates_tag,
                rotation_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_to_align.json()],
                    vec![
                        alignment_mode_tag.json(), coordinates_tag.json(), rotation_tag
                        .json()
                    ],
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
                    vec![],
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
                rounding_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), minimum_number.json(), maximum_number
                        .json()
                    ],
                    vec![rounding_mode_tag.json()],
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
                    vec![],
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
                entity_collision_tag,
                block_collision_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), ray_origin.json(), ray_distance.json()],
                    vec![entity_collision_tag.json(), block_collision_tag.json()],
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
                angle_units_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_rotate.json(), axis_vector
                        .json(), angle.json()
                    ],
                    vec![angle_units_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(vec![], vec![]);
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
                    vec![
                        variable_to_set.json(), item_to_change.json(), image_url.json()
                    ],
                    vec![],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetX".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SortDictionary {
                variable_to_set,
                dictionary_to_sort,
                sorting_type__tag,
                sorting_order_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), dictionary_to_sort.json()],
                    vec![sorting_type__tag.json(), sorting_order_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetZ".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SetArmorTrim {
                variable_to_set,
                item_to_change,
                trim_pattern_tag,
                trim_material_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_change.json()],
                    vec![trim_pattern_tag.json(), trim_material_tag.json()],
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
                    vec![],
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
                let item_args = compile(
                    vec![variable_to_set.json(), number_set.json()],
                    vec![],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SettoSine {
                variable_to_set,
                number_input,
                sine_variant_tag,
                input_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                    vec![sine_variant_tag.json(), input_tag.json()],
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
                let item_args = compile(
                    vec![variable_to_set.json(), direction.json()],
                    vec![],
                );
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::RoundNumber { round_mode_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![round_mode_tag.json()]);
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
                face_direction_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(),
                        target_location.json()
                    ],
                    vec![face_direction_tag.json()],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                distribution_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), mean_midpoint.json(), standard_deviation
                        .json()
                    ],
                    vec![distribution_tag.json()],
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
                    vec![],
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
                    vec![],
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
                durability_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json(), item_durability.json()],
                    vec![durability_type__tag.json()],
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
            SetVariable::SetItemBreakability {
                variable_to_set,
                item,
                breakability_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![breakability_tag.json()],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetBreakability".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSignTextN {
                variable_to_set,
                location,
                sign_line_tag,
                sign_side_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location.json()],
                    vec![sign_line_tag.json(), sign_side_tag.json()],
                );
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                coordinate_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), new_x.json(),
                        new_y.json(), new_z.json(), new_pitch.json(), new_yaw.json()
                    ],
                    vec![coordinate_type__tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::GetDirection { return_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![return_type__tag.json()]);
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                allowed_tags_tag,
                parse_legacy_color_codes_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_parse.json()],
                    vec![allowed_tags_tag.json(), parse_legacy_color_codes_tag.json()],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![],
                );
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
                axis_tag,
                angle_units_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_rotate.json(), angle.json()],
                    vec![axis_tag.json(), angle_units_tag.json()],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetItemName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetItemDurability {
                variable_to_set,
                item,
                durability_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![durability_type__tag.json()],
                );
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
                direction_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_shift.json(), shift_distance
                        .json()
                    ],
                    vec![direction_tag.json()],
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
                    vec![],
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
                regular_expressions_tag,
                replacement_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable.json(), string_to_change.json(), string_part_to_replace
                        .json(), replacement.json()
                    ],
                    vec![regular_expressions_tag.json(), replacement_type__tag.json()],
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
                require_lodestone_at_location_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), item_to_change.json(), lodestone_location
                        .json()
                    ],
                    vec![require_lodestone_at_location_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SetStringCase {
                variable_to_set,
                string_to_change,
                capitalization_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), string_to_change.json()],
                    vec![capitalization_type__tag.json()],
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
                    vec![],
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
            SetVariable::GetLightLevel {
                variable_to_set,
                light_location,
                light_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), light_location.json()],
                    vec![light_type__tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SettoDistance {
                variable_to_set,
                location_1,
                location_2,
                distance_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_1.json(), location_2.json()],
                    vec![distance_type__tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                return_value_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item_to_get_material_of.json()],
                    vec![return_value_type__tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                regular_expressions_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable.json(), string_to_change.json(), string_to_remove.json()
                    ],
                    vec![regular_expressions_tag.json()],
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
            SetVariable::GetAllBlockData {
                variable_to_set,
                block_location,
                hide_default_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                    vec![hide_default_tag.json()],
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
                let item_args = compile(
                    vec![variable_to_set.json(), number_set.json()],
                    vec![],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::GetLocationCoordinate {
                variable_to_set,
                location_to_get,
                coordinate_type__tag,
                coordinate_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), location_to_get.json()],
                    vec![coordinate_type__tag.json(), coordinate_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::ShiftDirection { direction_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![direction_tag.json()]);
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetContainerName".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetParticleEffectSpread {
                variable_to_set,
                effect_to_get,
                spread_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), effect_to_get.json()],
                    vec![spread_tag.json()],
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
                    vec![],
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
            SetVariable::GetHeadOwner {
                variable_to_set,
                head_to_get_owner_of,
                text_value_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), head_to_get_owner_of.json()],
                    vec![text_value_tag.json()],
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
                let item_args = compile(vec![], vec![]);
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
                let item_args = compile(
                    vec![dictionary_to.json(), dictionary.json()],
                    vec![],
                );
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
                    vec![],
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
            SetVariable::GetColorChannels {
                variable_to_set,
                color_hexadecimal,
                color_channels_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), color_hexadecimal.json()],
                    vec![color_channels_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(
                    vec![variable_to_set.json(), value_set.json()],
                    vec![],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SettoTangent {
                variable_to_set,
                number_input,
                tangent_variant_tag,
                input_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                    vec![tangent_variant_tag.json(), input_tag.json()],
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
                cell_edge_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), cell_frequency
                        .json(), cell_scatter.json(), generation_seed.json()
                    ],
                    vec![cell_edge_type__tag.json()],
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
            SetVariable::SetDirection { face_direction_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![face_direction_tag.json()]);
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
                    vec![],
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
                let item_args = compile(
                    vec![variable.json(), numbers_to.json()],
                    vec![],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                coordinate_type__tag,
                coordinate_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), location_to_change.json(), coordinate
                        .json()
                    ],
                    vec![coordinate_type__tag.json(), coordinate_tag.json()],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("SetPitch".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::RaycastBlock {
                ignore_passable_blocks_tag,
                fluid_collision_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![],
                    vec![ignore_passable_blocks_tag.json(), fluid_collision_tag.json()],
                );
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
                    vec![],
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
                    vec![],
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
                fractal_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), frequency_scale
                        .json(), octaves_perlin_layers.json(), octave_frequency_gain
                        .json(), octave_amplitude_gain.json(), generation_seed.json()
                    ],
                    vec![fractal_type__tag.json()],
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
                cell_edge_type__tag,
                distance_calculation_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), noise_location.json(), cell_frequency
                        .json(), cell_scatter.json(), generation_seed.json()
                    ],
                    vec![cell_edge_type__tag.json(), distance_calculation_tag.json()],
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                round_mode_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), number_to_round.json(), round_multiple
                        .json()
                    ],
                    vec![round_mode_tag.json()],
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
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![],
                );
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("GetCanPlaceOn".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::SortList { variable_to_set, list_to_sort, sort_order_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), list_to_sort.json()],
                    vec![sort_order_tag.json()],
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
                    vec![],
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
                    vec![],
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
                format_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), time_to_format.json(), custom_format
                        .json()
                    ],
                    vec![format_tag.json()],
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
            SetVariable::SetItemVisibilityFlags {
                variable_to_set,
                item,
                hide_armor_trim_tag,
                hide_color_tag,
                hide_enchantments_tag,
                hide_attributes_tag,
                hide_unbreakable_tag,
                hide_can_destroy_tag,
                hide_can_place_on_tag,
                hide_potion_effects_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), item.json()],
                    vec![
                        hide_armor_trim_tag.json(), hide_color_tag.json(),
                        hide_enchantments_tag.json(), hide_attributes_tag.json(),
                        hide_unbreakable_tag.json(), hide_can_destroy_tag.json(),
                        hide_can_place_on_tag.json(), hide_potion_effects_tag.json()
                    ],
                );
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                let item_args = compile(
                    vec![variable.json(), numbers_to.json()],
                    vec![],
                );
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
                    vec![],
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
                let item_args = compile(
                    vec![variable_to_set.json(), value_list.json()],
                    vec![],
                );
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
                    vec![],
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
                ignore_empty_slots_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), container_location.json()],
                    vec![ignore_empty_slots_tag.json()],
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
                    vec![],
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
                    vec![],
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
            SetVariable::SettoCosine {
                variable_to_set,
                number_input,
                cosine_variant_tag,
                input_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), number_input.json()],
                    vec![cosine_variant_tag.json(), input_tag.json()],
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
                component_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), vector_to_change.json(), component.json()
                    ],
                    vec![component_tag.json()],
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
                    vec![],
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
                    vec![],
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
            SetVariable::ShiftAllDirs { ignore_pitch_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![ignore_pitch_tag.json()]);
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
                search_order_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![
                        variable_to_set.json(), list_to_search_in.json(), value_to_search
                        .json()
                    ],
                    vec![search_order_tag.json()],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                    vec![],
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
            SetVariable::ShiftLocation { shift_direction_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![shift_direction_tag.json()]);
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
                    vec![],
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
                    vec![],
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
            SetVariable::Round { round_mode_tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![round_mode_tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("Round".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetSoundPitch {
                variable_to_set,
                sound_to_get_pitch_or,
                return_value_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), sound_to_get_pitch_or.json()],
                    vec![return_value_type__tag.json()],
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
            SetVariable::TranslateColors { translation_type__tag } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(vec![], vec![translation_type__tag.json()]);
                let mut args = serde_json::Map::new();
                args.insert("items".to_string(), serde_json::Value::Array(item_args));
                map.insert(
                    "action".to_string(),
                    serde_json::Value::String("TranslateColors".to_string()),
                );
                map.insert("args".to_string(), serde_json::Value::Object(args));
                serde_json::Value::Object(map)
            }
            SetVariable::GetBlockGrowth {
                variable_to_set,
                block_location,
                growth_unit_tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                    vec![growth_unit_tag.json()],
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
                    vec![],
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
                let item_args = compile(
                    vec![list_to_change.json(), values_to.json()],
                    vec![],
                );
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
                    vec![],
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
                    vec![],
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
            SetVariable::GetBlockMaterial {
                variable_to_set,
                block_location,
                return_value_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), block_location.json()],
                    vec![return_value_type__tag.json()],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
                    vec![],
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
                    vec![],
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
            SetVariable::GetVectorLength {
                variable_to_set,
                vector_to_get,
                length_type__tag,
            } => {
                let mut map = serde_json::Map::new();
                let item_args = compile(
                    vec![variable_to_set.json(), vector_to_get.json()],
                    vec![length_type__tag.json()],
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
                    vec![],
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
                let item_args = compile(vec![], vec![]);
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
                    vec![],
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
#[derive(Debug, Clone)]
pub enum TextValueMergingSettoString {
    Addspaces,
    Nospaces,
}
impl TextValueMergingSettoString {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueMergingSettoString::Addspaces => {
                    Value::String("Add spaces".to_string())
                }
                TextValueMergingSettoString::Nospaces => {
                    Value::String("No spaces".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value Merging".to_string()));
        data.insert("action".to_string(), Value::String("String".to_string()));
        data.insert("block".to_string(), Value::String("String".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueMergingSettoString {
    fn default() -> Self {
        Self::Nospaces
    }
}
#[derive(Debug, Clone)]
pub enum MatchRequirementPurgeMatchingVariables {
    Entirename,
    Fullwordsinname,
    Anypartofname,
}
impl MatchRequirementPurgeMatchingVariables {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                MatchRequirementPurgeMatchingVariables::Entirename => {
                    Value::String("Entire name".to_string())
                }
                MatchRequirementPurgeMatchingVariables::Fullwordsinname => {
                    Value::String("Full word(s) in name".to_string())
                }
                MatchRequirementPurgeMatchingVariables::Anypartofname => {
                    Value::String("Any part of name".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Match Requirement".to_string()));
        data.insert("action".to_string(), Value::String("PurgeVars".to_string()));
        data.insert("block".to_string(), Value::String("PurgeVars".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for MatchRequirementPurgeMatchingVariables {
    fn default() -> Self {
        Self::Fullwordsinname
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreCasePurgeMatchingVariables {
    True,
    False,
}
impl IgnoreCasePurgeMatchingVariables {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreCasePurgeMatchingVariables::True => {
                    Value::String("True".to_string())
                }
                IgnoreCasePurgeMatchingVariables::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Case".to_string()));
        data.insert("action".to_string(), Value::String("PurgeVars".to_string()));
        data.insert("block".to_string(), Value::String("PurgeVars".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreCasePurgeMatchingVariables {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum RemainderModeSettoRemainder {
    Remainder,
    Modulo,
}
impl RemainderModeSettoRemainder {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RemainderModeSettoRemainder::Remainder => {
                    Value::String("Remainder".to_string())
                }
                RemainderModeSettoRemainder::Modulo => {
                    Value::String("Modulo".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Remainder Mode".to_string()));
        data.insert("action".to_string(), Value::String("%".to_string()));
        data.insert("block".to_string(), Value::String("%".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RemainderModeSettoRemainder {
    fn default() -> Self {
        Self::Remainder
    }
}
#[derive(Debug, Clone)]
pub enum AddLocationRotationShiftLocationonVector {
    True,
    False,
}
impl AddLocationRotationShiftLocationonVector {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AddLocationRotationShiftLocationonVector::True => {
                    Value::String("True".to_string())
                }
                AddLocationRotationShiftLocationonVector::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Add Location Rotation".to_string()),
        );
        data.insert("action".to_string(), Value::String("ShiftOnVector".to_string()));
        data.insert("block".to_string(), Value::String("ShiftOnVector".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AddLocationRotationShiftLocationonVector {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum AttributeGetItemAttribute {
    Armor,
    Armortoughness,
    Attackdamage,
    Attackspeed,
    Maximumhealth,
    Knockbackresistance,
    Movementspeed,
    Followrange,
}
impl AttributeGetItemAttribute {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AttributeGetItemAttribute::Armor => Value::String("Armor".to_string()),
                AttributeGetItemAttribute::Armortoughness => {
                    Value::String("Armor toughness".to_string())
                }
                AttributeGetItemAttribute::Attackdamage => {
                    Value::String("Attack damage".to_string())
                }
                AttributeGetItemAttribute::Attackspeed => {
                    Value::String("Attack speed".to_string())
                }
                AttributeGetItemAttribute::Maximumhealth => {
                    Value::String("Maximum health".to_string())
                }
                AttributeGetItemAttribute::Knockbackresistance => {
                    Value::String("Knockback resistance".to_string())
                }
                AttributeGetItemAttribute::Movementspeed => {
                    Value::String("Movement speed".to_string())
                }
                AttributeGetItemAttribute::Followrange => {
                    Value::String("Follow range".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Attribute".to_string()));
        data.insert("action".to_string(), Value::String("GetItemAttribute".to_string()));
        data.insert("block".to_string(), Value::String("GetItemAttribute".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AttributeGetItemAttribute {
    fn default() -> Self {
        Self::Armor
    }
}
#[derive(Debug, Clone)]
pub enum ActiveEquipmentSlotGetItemAttribute {
    Any,
    Mainhand,
    Offhand,
    Head,
    Body,
    Legs,
    Feet,
}
impl ActiveEquipmentSlotGetItemAttribute {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ActiveEquipmentSlotGetItemAttribute::Any => {
                    Value::String("Any".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Mainhand => {
                    Value::String("Main hand".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Offhand => {
                    Value::String("Off hand".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Head => {
                    Value::String("Head".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Body => {
                    Value::String("Body".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Legs => {
                    Value::String("Legs".to_string())
                }
                ActiveEquipmentSlotGetItemAttribute::Feet => {
                    Value::String("Feet".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Active Equipment Slot".to_string()),
        );
        data.insert("action".to_string(), Value::String("GetItemAttribute".to_string()));
        data.insert("block".to_string(), Value::String("GetItemAttribute".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ActiveEquipmentSlotGetItemAttribute {
    fn default() -> Self {
        Self::Any
    }
}
#[derive(Debug, Clone)]
pub enum RotationAxisShiftLocationRotation {
    Pitch,
    Yaw,
}
impl RotationAxisShiftLocationRotation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RotationAxisShiftLocationRotation::Pitch => {
                    Value::String("Pitch".to_string())
                }
                RotationAxisShiftLocationRotation::Yaw => {
                    Value::String("Yaw".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Rotation Axis".to_string()));
        data.insert("action".to_string(), Value::String("ShiftRotation".to_string()));
        data.insert("block".to_string(), Value::String("ShiftRotation".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RotationAxisShiftLocationRotation {
    fn default() -> Self {
        Self::Pitch
    }
}
#[derive(Debug, Clone)]
pub enum DivisionModeDivideNumbers {
    Default,
    Floorresult,
}
impl DivisionModeDivideNumbers {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DivisionModeDivideNumbers::Default => {
                    Value::String("Default".to_string())
                }
                DivisionModeDivideNumbers::Floorresult => {
                    Value::String("Floor result".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Division Mode".to_string()));
        data.insert("action".to_string(), Value::String("/".to_string()));
        data.insert("block".to_string(), Value::String("/".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DivisionModeDivideNumbers {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Debug, Clone)]
pub enum SignLineGetSignText {
    One,
    Two,
    Three,
    Four,
    Alllines,
}
impl SignLineGetSignText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignLineGetSignText::One => Value::String("1".to_string()),
                SignLineGetSignText::Two => Value::String("2".to_string()),
                SignLineGetSignText::Three => Value::String("3".to_string()),
                SignLineGetSignText::Four => Value::String("4".to_string()),
                SignLineGetSignText::Alllines => Value::String("All lines".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Line".to_string()));
        data.insert("action".to_string(), Value::String("GetSignText".to_string()));
        data.insert("block".to_string(), Value::String("GetSignText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignLineGetSignText {
    fn default() -> Self {
        Self::One
    }
}
#[derive(Debug, Clone)]
pub enum OperatorSettoBitwiseOperation {
    BitwiseOr,
    BitwiseAnd,
    BitwiseNotno2nd,
    BitwiseXor,
    LessThanLessThan,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
}
impl OperatorSettoBitwiseOperation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OperatorSettoBitwiseOperation::BitwiseOr => {
                    Value::String("|".to_string())
                }
                OperatorSettoBitwiseOperation::BitwiseAnd => {
                    Value::String("&".to_string())
                }
                OperatorSettoBitwiseOperation::BitwiseNotno2nd => {
                    Value::String("~".to_string())
                }
                OperatorSettoBitwiseOperation::BitwiseXor => {
                    Value::String("^".to_string())
                }
                OperatorSettoBitwiseOperation::LessThanLessThan => {
                    Value::String("<<".to_string())
                }
                OperatorSettoBitwiseOperation::GreaterThanGreaterThan => {
                    Value::String(">>".to_string())
                }
                OperatorSettoBitwiseOperation::GreaterThanGreaterThanGreaterThan => {
                    Value::String(">>>".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Operator".to_string()));
        data.insert("action".to_string(), Value::String("Bitwise".to_string()));
        data.insert("block".to_string(), Value::String("Bitwise".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OperatorSettoBitwiseOperation {
    fn default() -> Self {
        Self::BitwiseOr
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateShiftLocationonAxis {
    X,
    Y,
    Z,
}
impl CoordinateShiftLocationonAxis {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateShiftLocationonAxis::X => Value::String("X".to_string()),
                CoordinateShiftLocationonAxis::Y => Value::String("Y".to_string()),
                CoordinateShiftLocationonAxis::Z => Value::String("Z".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate".to_string()));
        data.insert("action".to_string(), Value::String("ShiftOnAxis".to_string()));
        data.insert("block".to_string(), Value::String("ShiftOnAxis".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateShiftLocationonAxis {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum ComponentGetVectorComponent {
    X,
    Y,
    Z,
}
impl ComponentGetVectorComponent {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ComponentGetVectorComponent::X => Value::String("X".to_string()),
                ComponentGetVectorComponent::Y => Value::String("Y".to_string()),
                ComponentGetVectorComponent::Z => Value::String("Z".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Component".to_string()));
        data.insert("action".to_string(), Value::String("GetVectorComp".to_string()));
        data.insert("block".to_string(), Value::String("GetVectorComp".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ComponentGetVectorComponent {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum RegularExpressionsRmText {
    Enable,
    Disable,
}
impl RegularExpressionsRmText {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RegularExpressionsRmText::Enable => Value::String("Enable".to_string()),
                RegularExpressionsRmText::Disable => Value::String("Disable".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Regular Expressions".to_string()));
        data.insert("action".to_string(), Value::String("RmText".to_string()));
        data.insert("block".to_string(), Value::String("RmText".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RegularExpressionsRmText {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum AttributeAddItemAttribute {
    Armor,
    Armortoughness,
    Attackdamage,
    Attackspeed,
    Maximumhealth,
    Knockbackresistance,
    Movementspeed,
    Followrange,
}
impl AttributeAddItemAttribute {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AttributeAddItemAttribute::Armor => Value::String("Armor".to_string()),
                AttributeAddItemAttribute::Armortoughness => {
                    Value::String("Armor toughness".to_string())
                }
                AttributeAddItemAttribute::Attackdamage => {
                    Value::String("Attack damage".to_string())
                }
                AttributeAddItemAttribute::Attackspeed => {
                    Value::String("Attack speed".to_string())
                }
                AttributeAddItemAttribute::Maximumhealth => {
                    Value::String("Maximum health".to_string())
                }
                AttributeAddItemAttribute::Knockbackresistance => {
                    Value::String("Knockback resistance".to_string())
                }
                AttributeAddItemAttribute::Movementspeed => {
                    Value::String("Movement speed".to_string())
                }
                AttributeAddItemAttribute::Followrange => {
                    Value::String("Follow range".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Attribute".to_string()));
        data.insert("action".to_string(), Value::String("AddItemAttribute".to_string()));
        data.insert("block".to_string(), Value::String("AddItemAttribute".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AttributeAddItemAttribute {
    fn default() -> Self {
        Self::Armor
    }
}
#[derive(Debug, Clone)]
pub enum OperationAddItemAttribute {
    Addnumber,
    Addpercentagetobase,
    Multiplymodifierbypercentage,
}
impl OperationAddItemAttribute {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                OperationAddItemAttribute::Addnumber => {
                    Value::String("Add number".to_string())
                }
                OperationAddItemAttribute::Addpercentagetobase => {
                    Value::String("Add percentage to base".to_string())
                }
                OperationAddItemAttribute::Multiplymodifierbypercentage => {
                    Value::String("Multiply modifier by percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Operation".to_string()));
        data.insert("action".to_string(), Value::String("AddItemAttribute".to_string()));
        data.insert("block".to_string(), Value::String("AddItemAttribute".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for OperationAddItemAttribute {
    fn default() -> Self {
        Self::Addnumber
    }
}
#[derive(Debug, Clone)]
pub enum ActiveEquipmentSlotAddItemAttribute {
    Any,
    Mainhand,
    Offhand,
    Head,
    Body,
    Legs,
    Feet,
}
impl ActiveEquipmentSlotAddItemAttribute {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ActiveEquipmentSlotAddItemAttribute::Any => {
                    Value::String("Any".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Mainhand => {
                    Value::String("Main hand".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Offhand => {
                    Value::String("Off hand".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Head => {
                    Value::String("Head".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Body => {
                    Value::String("Body".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Legs => {
                    Value::String("Legs".to_string())
                }
                ActiveEquipmentSlotAddItemAttribute::Feet => {
                    Value::String("Feet".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Active Equipment Slot".to_string()),
        );
        data.insert("action".to_string(), Value::String("AddItemAttribute".to_string()));
        data.insert("block".to_string(), Value::String("AddItemAttribute".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ActiveEquipmentSlotAddItemAttribute {
    fn default() -> Self {
        Self::Mainhand
    }
}
#[derive(Debug, Clone)]
pub enum AlignmentModeAlignLocation {
    Blockcenter,
    Lowerblockcorner,
}
impl AlignmentModeAlignLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AlignmentModeAlignLocation::Blockcenter => {
                    Value::String("Block center".to_string())
                }
                AlignmentModeAlignLocation::Lowerblockcorner => {
                    Value::String("Lower block corner".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Alignment Mode".to_string()));
        data.insert("action".to_string(), Value::String("AlignLoc".to_string()));
        data.insert("block".to_string(), Value::String("AlignLoc".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AlignmentModeAlignLocation {
    fn default() -> Self {
        Self::Blockcenter
    }
}
#[derive(Debug, Clone)]
pub enum CoordinatesAlignLocation {
    Allcoordinates,
    XandZ,
    OnlyY,
}
impl CoordinatesAlignLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinatesAlignLocation::Allcoordinates => {
                    Value::String("All coordinates".to_string())
                }
                CoordinatesAlignLocation::XandZ => Value::String("X and Z".to_string()),
                CoordinatesAlignLocation::OnlyY => Value::String("Only Y".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinates".to_string()));
        data.insert("action".to_string(), Value::String("AlignLoc".to_string()));
        data.insert("block".to_string(), Value::String("AlignLoc".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinatesAlignLocation {
    fn default() -> Self {
        Self::Allcoordinates
    }
}
#[derive(Debug, Clone)]
pub enum RotationAlignLocation {
    Keeprotation,
    Removerotation,
}
impl RotationAlignLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RotationAlignLocation::Keeprotation => {
                    Value::String("Keep rotation".to_string())
                }
                RotationAlignLocation::Removerotation => {
                    Value::String("Remove rotation".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Rotation".to_string()));
        data.insert("action".to_string(), Value::String("AlignLoc".to_string()));
        data.insert("block".to_string(), Value::String("AlignLoc".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RotationAlignLocation {
    fn default() -> Self {
        Self::Keeprotation
    }
}
#[derive(Debug, Clone)]
pub enum RoundingModeSettoRandomNumber {
    Wholenumber,
    Decimalnumber,
}
impl RoundingModeSettoRandomNumber {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RoundingModeSettoRandomNumber::Wholenumber => {
                    Value::String("Whole number".to_string())
                }
                RoundingModeSettoRandomNumber::Decimalnumber => {
                    Value::String("Decimal number".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Rounding Mode".to_string()));
        data.insert("action".to_string(), Value::String("RandomNumber".to_string()));
        data.insert("block".to_string(), Value::String("RandomNumber".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RoundingModeSettoRandomNumber {
    fn default() -> Self {
        Self::Wholenumber
    }
}
#[derive(Debug, Clone)]
pub enum EntityCollisionRaycastfromLocation {
    True,
    False,
}
impl EntityCollisionRaycastfromLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                EntityCollisionRaycastfromLocation::True => {
                    Value::String("True".to_string())
                }
                EntityCollisionRaycastfromLocation::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Entity Collision".to_string()));
        data.insert("action".to_string(), Value::String("Raycast".to_string()));
        data.insert("block".to_string(), Value::String("Raycast".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for EntityCollisionRaycastfromLocation {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum BlockCollisionRaycastfromLocation {
    Allblocks,
    Nonfluidblocks,
    Solidblocks,
    None,
}
impl BlockCollisionRaycastfromLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BlockCollisionRaycastfromLocation::Allblocks => {
                    Value::String("All blocks".to_string())
                }
                BlockCollisionRaycastfromLocation::Nonfluidblocks => {
                    Value::String("Non-fluid blocks".to_string())
                }
                BlockCollisionRaycastfromLocation::Solidblocks => {
                    Value::String("Solid blocks".to_string())
                }
                BlockCollisionRaycastfromLocation::None => {
                    Value::String("None".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Block Collision".to_string()));
        data.insert("action".to_string(), Value::String("Raycast".to_string()));
        data.insert("block".to_string(), Value::String("Raycast".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BlockCollisionRaycastfromLocation {
    fn default() -> Self {
        Self::Allblocks
    }
}
#[derive(Debug, Clone)]
pub enum AngleUnitsRotateVectorAroundVector {
    Degrees,
    Radians,
}
impl AngleUnitsRotateVectorAroundVector {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AngleUnitsRotateVectorAroundVector::Degrees => {
                    Value::String("Degrees".to_string())
                }
                AngleUnitsRotateVectorAroundVector::Radians => {
                    Value::String("Radians".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Angle Units".to_string()));
        data.insert("action".to_string(), Value::String("RotateAroundVec".to_string()));
        data.insert("block".to_string(), Value::String("RotateAroundVec".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AngleUnitsRotateVectorAroundVector {
    fn default() -> Self {
        Self::Degrees
    }
}
#[derive(Debug, Clone)]
pub enum SortingTypeSortDictionary {
    SortbyKey,
    SortbyValue,
}
impl SortingTypeSortDictionary {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SortingTypeSortDictionary::SortbyKey => {
                    Value::String("Sort by Key".to_string())
                }
                SortingTypeSortDictionary::SortbyValue => {
                    Value::String("Sort by Value".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sorting Type".to_string()));
        data.insert("action".to_string(), Value::String("SortDict".to_string()));
        data.insert("block".to_string(), Value::String("SortDict".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SortingTypeSortDictionary {
    fn default() -> Self {
        Self::SortbyKey
    }
}
#[derive(Debug, Clone)]
pub enum SortingOrderSortDictionary {
    Ascending,
    Descending,
}
impl SortingOrderSortDictionary {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SortingOrderSortDictionary::Ascending => {
                    Value::String("Ascending".to_string())
                }
                SortingOrderSortDictionary::Descending => {
                    Value::String("Descending".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sorting Order".to_string()));
        data.insert("action".to_string(), Value::String("SortDict".to_string()));
        data.insert("block".to_string(), Value::String("SortDict".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SortingOrderSortDictionary {
    fn default() -> Self {
        Self::Ascending
    }
}
#[derive(Debug, Clone)]
pub enum TrimPatternSetArmorTrim {
    None,
    Coast,
    Dune,
    Eye,
    Rib,
    Sentry,
    Snout,
    Spire,
    Tide,
    Vex,
    Ward,
    Wayfinder,
    Shaper,
    Silence,
    Raiser,
    Host,
    Wild,
}
impl TrimPatternSetArmorTrim {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TrimPatternSetArmorTrim::None => Value::String("None".to_string()),
                TrimPatternSetArmorTrim::Coast => Value::String("Coast".to_string()),
                TrimPatternSetArmorTrim::Dune => Value::String("Dune".to_string()),
                TrimPatternSetArmorTrim::Eye => Value::String("Eye".to_string()),
                TrimPatternSetArmorTrim::Rib => Value::String("Rib".to_string()),
                TrimPatternSetArmorTrim::Sentry => Value::String("Sentry".to_string()),
                TrimPatternSetArmorTrim::Snout => Value::String("Snout".to_string()),
                TrimPatternSetArmorTrim::Spire => Value::String("Spire".to_string()),
                TrimPatternSetArmorTrim::Tide => Value::String("Tide".to_string()),
                TrimPatternSetArmorTrim::Vex => Value::String("Vex".to_string()),
                TrimPatternSetArmorTrim::Ward => Value::String("Ward".to_string()),
                TrimPatternSetArmorTrim::Wayfinder => {
                    Value::String("Wayfinder".to_string())
                }
                TrimPatternSetArmorTrim::Shaper => Value::String("Shaper".to_string()),
                TrimPatternSetArmorTrim::Silence => Value::String("Silence".to_string()),
                TrimPatternSetArmorTrim::Raiser => Value::String("Raiser".to_string()),
                TrimPatternSetArmorTrim::Host => Value::String("Host".to_string()),
                TrimPatternSetArmorTrim::Wild => Value::String("Wild".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Trim Pattern".to_string()));
        data.insert("action".to_string(), Value::String("SetArmorTrim".to_string()));
        data.insert("block".to_string(), Value::String("SetArmorTrim".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TrimPatternSetArmorTrim {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone)]
pub enum TrimMaterialSetArmorTrim {
    Amethyst,
    Copper,
    Diamond,
    Emerald,
    Gold,
    Iron,
    LapisLazuli,
    Netherite,
    Quartz,
    Redstone,
}
impl TrimMaterialSetArmorTrim {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TrimMaterialSetArmorTrim::Amethyst => {
                    Value::String("Amethyst".to_string())
                }
                TrimMaterialSetArmorTrim::Copper => Value::String("Copper".to_string()),
                TrimMaterialSetArmorTrim::Diamond => Value::String("Diamond".to_string()),
                TrimMaterialSetArmorTrim::Emerald => Value::String("Emerald".to_string()),
                TrimMaterialSetArmorTrim::Gold => Value::String("Gold".to_string()),
                TrimMaterialSetArmorTrim::Iron => Value::String("Iron".to_string()),
                TrimMaterialSetArmorTrim::LapisLazuli => {
                    Value::String("Lapis Lazuli".to_string())
                }
                TrimMaterialSetArmorTrim::Netherite => {
                    Value::String("Netherite".to_string())
                }
                TrimMaterialSetArmorTrim::Quartz => Value::String("Quartz".to_string()),
                TrimMaterialSetArmorTrim::Redstone => {
                    Value::String("Redstone".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Trim Material".to_string()));
        data.insert("action".to_string(), Value::String("SetArmorTrim".to_string()));
        data.insert("block".to_string(), Value::String("SetArmorTrim".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TrimMaterialSetArmorTrim {
    fn default() -> Self {
        Self::Amethyst
    }
}
#[derive(Debug, Clone)]
pub enum SineVariantSettoSine {
    Sine,
    Inversesinearcsine,
    Hyperbolicsine,
}
impl SineVariantSettoSine {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SineVariantSettoSine::Sine => Value::String("Sine".to_string()),
                SineVariantSettoSine::Inversesinearcsine => {
                    Value::String("Inverse sine (arcsine)".to_string())
                }
                SineVariantSettoSine::Hyperbolicsine => {
                    Value::String("Hyperbolic sine".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Sine Variant".to_string()));
        data.insert("action".to_string(), Value::String("Sine".to_string()));
        data.insert("block".to_string(), Value::String("Sine".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SineVariantSettoSine {
    fn default() -> Self {
        Self::Sine
    }
}
#[derive(Debug, Clone)]
pub enum InputSettoSine {
    Degrees,
    Radians,
}
impl InputSettoSine {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InputSettoSine::Degrees => Value::String("Degrees".to_string()),
                InputSettoSine::Radians => Value::String("Radians".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Input".to_string()));
        data.insert("action".to_string(), Value::String("Sine".to_string()));
        data.insert("block".to_string(), Value::String("Sine".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InputSettoSine {
    fn default() -> Self {
        Self::Degrees
    }
}
#[derive(Debug, Clone)]
pub enum RoundModeRoundNumber {
    Floor,
    Nearest,
    Ceiling,
}
impl RoundModeRoundNumber {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RoundModeRoundNumber::Floor => Value::String("Floor".to_string()),
                RoundModeRoundNumber::Nearest => Value::String("Nearest".to_string()),
                RoundModeRoundNumber::Ceiling => Value::String("Ceiling".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Round Mode".to_string()));
        data.insert("action".to_string(), Value::String("RoundNumber".to_string()));
        data.insert("block".to_string(), Value::String("RoundNumber".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RoundModeRoundNumber {
    fn default() -> Self {
        Self::Nearest
    }
}
#[derive(Debug, Clone)]
pub enum FaceDirectionFaceLocation {
    Towardlocation,
    Awayfromlocation,
}
impl FaceDirectionFaceLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FaceDirectionFaceLocation::Towardlocation => {
                    Value::String("Toward location".to_string())
                }
                FaceDirectionFaceLocation::Awayfromlocation => {
                    Value::String("Away from location".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Face Direction".to_string()));
        data.insert("action".to_string(), Value::String("FaceLocation".to_string()));
        data.insert("block".to_string(), Value::String("FaceLocation".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FaceDirectionFaceLocation {
    fn default() -> Self {
        Self::Towardlocation
    }
}
#[derive(Debug, Clone)]
pub enum DistributionSettoNormallyDistributedRandomNumber {
    Normal,
    Foldednormal,
}
impl DistributionSettoNormallyDistributedRandomNumber {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DistributionSettoNormallyDistributedRandomNumber::Normal => {
                    Value::String("Normal".to_string())
                }
                DistributionSettoNormallyDistributedRandomNumber::Foldednormal => {
                    Value::String("Folded normal".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Distribution".to_string()));
        data.insert("action".to_string(), Value::String("NormalRandom".to_string()));
        data.insert("block".to_string(), Value::String("NormalRandom".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DistributionSettoNormallyDistributedRandomNumber {
    fn default() -> Self {
        Self::Normal
    }
}
#[derive(Debug, Clone)]
pub enum DurabilityTypeSetItemDurability {
    SetDamage,
    SetDamagePercentage,
    SetRemaining,
    SetRemainingPercentage,
}
impl DurabilityTypeSetItemDurability {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DurabilityTypeSetItemDurability::SetDamage => {
                    Value::String("Set Damage".to_string())
                }
                DurabilityTypeSetItemDurability::SetDamagePercentage => {
                    Value::String("Set Damage Percentage".to_string())
                }
                DurabilityTypeSetItemDurability::SetRemaining => {
                    Value::String("Set Remaining".to_string())
                }
                DurabilityTypeSetItemDurability::SetRemainingPercentage => {
                    Value::String("Set Remaining Percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Durability Type".to_string()));
        data.insert("action".to_string(), Value::String("SetItemDura".to_string()));
        data.insert("block".to_string(), Value::String("SetItemDura".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DurabilityTypeSetItemDurability {
    fn default() -> Self {
        Self::SetDamage
    }
}
#[derive(Debug, Clone)]
pub enum BreakabilitySetItemBreakability {
    Breakable,
    Unbreakable,
}
impl BreakabilitySetItemBreakability {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                BreakabilitySetItemBreakability::Breakable => {
                    Value::String("Breakable".to_string())
                }
                BreakabilitySetItemBreakability::Unbreakable => {
                    Value::String("Unbreakable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Breakability".to_string()));
        data.insert("action".to_string(), Value::String("SetBreakability".to_string()));
        data.insert("block".to_string(), Value::String("SetBreakability".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for BreakabilitySetItemBreakability {
    fn default() -> Self {
        Self::Unbreakable
    }
}
#[derive(Debug, Clone)]
pub enum SignLineGetSignTextN {
    One,
    Two,
    Three,
    Four,
    Alllines,
}
impl SignLineGetSignTextN {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignLineGetSignTextN::One => Value::String("1".to_string()),
                SignLineGetSignTextN::Two => Value::String("2".to_string()),
                SignLineGetSignTextN::Three => Value::String("3".to_string()),
                SignLineGetSignTextN::Four => Value::String("4".to_string()),
                SignLineGetSignTextN::Alllines => Value::String("All lines".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Line".to_string()));
        data.insert("action".to_string(), Value::String(" GetSignText ".to_string()));
        data.insert("block".to_string(), Value::String(" GetSignText ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignLineGetSignTextN {
    fn default() -> Self {
        Self::One
    }
}
#[derive(Debug, Clone)]
pub enum SignSideGetSignTextN {
    Front,
    Back,
}
impl SignSideGetSignTextN {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SignSideGetSignTextN::Front => Value::String("Front".to_string()),
                SignSideGetSignTextN::Back => Value::String("Back".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sign Side".to_string()));
        data.insert("action".to_string(), Value::String(" GetSignText ".to_string()));
        data.insert("block".to_string(), Value::String(" GetSignText ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SignSideGetSignTextN {
    fn default() -> Self {
        Self::Front
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateTypeSetAllLocationCoordinates {
    Plotcoordinate,
    Worldcoordinate,
}
impl CoordinateTypeSetAllLocationCoordinates {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateTypeSetAllLocationCoordinates::Plotcoordinate => {
                    Value::String("Plot coordinate".to_string())
                }
                CoordinateTypeSetAllLocationCoordinates::Worldcoordinate => {
                    Value::String("World coordinate".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate Type".to_string()));
        data.insert("action".to_string(), Value::String("SetAllCoords".to_string()));
        data.insert("block".to_string(), Value::String("SetAllCoords".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateTypeSetAllLocationCoordinates {
    fn default() -> Self {
        Self::Plotcoordinate
    }
}
#[derive(Debug, Clone)]
pub enum ReturnTypeGetDirection {
    TextThreeD,
    TextTwoD,
    Vector,
}
impl ReturnTypeGetDirection {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReturnTypeGetDirection::TextThreeD => {
                    Value::String("Text (3D)".to_string())
                }
                ReturnTypeGetDirection::TextTwoD => {
                    Value::String("Text (2D)".to_string())
                }
                ReturnTypeGetDirection::Vector => Value::String("Vector".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Return Type".to_string()));
        data.insert("action".to_string(), Value::String(" GetDirection ".to_string()));
        data.insert("block".to_string(), Value::String(" GetDirection ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReturnTypeGetDirection {
    fn default() -> Self {
        Self::TextThreeD
    }
}
#[derive(Debug, Clone)]
pub enum AllowedTagsParseMiniMessageExpression {
    StyleOnly,
    Dynamic,
    Full,
}
impl AllowedTagsParseMiniMessageExpression {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AllowedTagsParseMiniMessageExpression::StyleOnly => {
                    Value::String("Style Only".to_string())
                }
                AllowedTagsParseMiniMessageExpression::Dynamic => {
                    Value::String("Dynamic".to_string())
                }
                AllowedTagsParseMiniMessageExpression::Full => {
                    Value::String("Full".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Allowed Tags".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("ParseMiniMessageExpr".to_string()),
        );
        data.insert(
            "block".to_string(),
            Value::String("ParseMiniMessageExpr".to_string()),
        );
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AllowedTagsParseMiniMessageExpression {
    fn default() -> Self {
        Self::StyleOnly
    }
}
#[derive(Debug, Clone)]
pub enum ParseLegacyColorCodesParseMiniMessageExpression {
    True,
    False,
}
impl ParseLegacyColorCodesParseMiniMessageExpression {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ParseLegacyColorCodesParseMiniMessageExpression::True => {
                    Value::String("True".to_string())
                }
                ParseLegacyColorCodesParseMiniMessageExpression::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Parse Legacy Color Codes".to_string()),
        );
        data.insert(
            "action".to_string(),
            Value::String("ParseMiniMessageExpr".to_string()),
        );
        data.insert(
            "block".to_string(),
            Value::String("ParseMiniMessageExpr".to_string()),
        );
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ParseLegacyColorCodesParseMiniMessageExpression {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum AxisRotateVectorAroundAxis {
    X,
    Y,
    Z,
}
impl AxisRotateVectorAroundAxis {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AxisRotateVectorAroundAxis::X => Value::String("X".to_string()),
                AxisRotateVectorAroundAxis::Y => Value::String("Y".to_string()),
                AxisRotateVectorAroundAxis::Z => Value::String("Z".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Axis".to_string()));
        data.insert("action".to_string(), Value::String("RotateAroundAxis".to_string()));
        data.insert("block".to_string(), Value::String("RotateAroundAxis".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AxisRotateVectorAroundAxis {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum AngleUnitsRotateVectorAroundAxis {
    Degrees,
    Radians,
}
impl AngleUnitsRotateVectorAroundAxis {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                AngleUnitsRotateVectorAroundAxis::Degrees => {
                    Value::String("Degrees".to_string())
                }
                AngleUnitsRotateVectorAroundAxis::Radians => {
                    Value::String("Radians".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Angle Units".to_string()));
        data.insert("action".to_string(), Value::String("RotateAroundAxis".to_string()));
        data.insert("block".to_string(), Value::String("RotateAroundAxis".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for AngleUnitsRotateVectorAroundAxis {
    fn default() -> Self {
        Self::Degrees
    }
}
#[derive(Debug, Clone)]
pub enum DurabilityTypeGetItemDurability {
    GetDamage,
    GetDamagePercentage,
    GetRemaining,
    GetRemainingPercentage,
    GetMaximum,
}
impl DurabilityTypeGetItemDurability {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DurabilityTypeGetItemDurability::GetDamage => {
                    Value::String("Get Damage".to_string())
                }
                DurabilityTypeGetItemDurability::GetDamagePercentage => {
                    Value::String("Get Damage Percentage".to_string())
                }
                DurabilityTypeGetItemDurability::GetRemaining => {
                    Value::String("Get Remaining".to_string())
                }
                DurabilityTypeGetItemDurability::GetRemainingPercentage => {
                    Value::String("Get Remaining Percentage".to_string())
                }
                DurabilityTypeGetItemDurability::GetMaximum => {
                    Value::String("Get Maximum".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Durability Type".to_string()));
        data.insert("action".to_string(), Value::String("GetItemDura".to_string()));
        data.insert("block".to_string(), Value::String("GetItemDura".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DurabilityTypeGetItemDurability {
    fn default() -> Self {
        Self::GetDamage
    }
}
#[derive(Debug, Clone)]
pub enum DirectionShiftLocationinDirection {
    Forward,
    Upward,
    Sideways,
}
impl DirectionShiftLocationinDirection {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DirectionShiftLocationinDirection::Forward => {
                    Value::String("Forward".to_string())
                }
                DirectionShiftLocationinDirection::Upward => {
                    Value::String("Upward".to_string())
                }
                DirectionShiftLocationinDirection::Sideways => {
                    Value::String("Sideways".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Direction".to_string()));
        data.insert("action".to_string(), Value::String("ShiftInDirection".to_string()));
        data.insert("block".to_string(), Value::String("ShiftInDirection".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DirectionShiftLocationinDirection {
    fn default() -> Self {
        Self::Forward
    }
}
#[derive(Debug, Clone)]
pub enum RegularExpressionsReplaceString {
    Enable,
    Disable,
}
impl RegularExpressionsReplaceString {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RegularExpressionsReplaceString::Enable => {
                    Value::String("Enable".to_string())
                }
                RegularExpressionsReplaceString::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Regular Expressions".to_string()));
        data.insert("action".to_string(), Value::String("ReplaceString".to_string()));
        data.insert("block".to_string(), Value::String("ReplaceString".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RegularExpressionsReplaceString {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum ReplacementTypeReplaceString {
    Firstoccurrence,
    Alloccurrences,
}
impl ReplacementTypeReplaceString {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReplacementTypeReplaceString::Firstoccurrence => {
                    Value::String("First occurrence".to_string())
                }
                ReplacementTypeReplaceString::Alloccurrences => {
                    Value::String("All occurrences".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Replacement Type".to_string()));
        data.insert("action".to_string(), Value::String("ReplaceString".to_string()));
        data.insert("block".to_string(), Value::String("ReplaceString".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReplacementTypeReplaceString {
    fn default() -> Self {
        Self::Alloccurrences
    }
}
#[derive(Debug, Clone)]
pub enum RequireLodestoneatLocationSetCompassLodestoneLocation {
    True,
    False,
}
impl RequireLodestoneatLocationSetCompassLodestoneLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RequireLodestoneatLocationSetCompassLodestoneLocation::True => {
                    Value::String("True".to_string())
                }
                RequireLodestoneatLocationSetCompassLodestoneLocation::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Require Lodestone at Location".to_string()),
        );
        data.insert("action".to_string(), Value::String("SetLodestoneLoc".to_string()));
        data.insert("block".to_string(), Value::String("SetLodestoneLoc".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RequireLodestoneatLocationSetCompassLodestoneLocation {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum CapitalizationTypeSetStringCase {
    UPPERCASE,
    lowercase,
    ProperCase,
    iNVERTCASE,
    RAnDoMcASe,
}
impl CapitalizationTypeSetStringCase {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CapitalizationTypeSetStringCase::UPPERCASE => {
                    Value::String("UPPERCASE".to_string())
                }
                CapitalizationTypeSetStringCase::lowercase => {
                    Value::String("lowercase".to_string())
                }
                CapitalizationTypeSetStringCase::ProperCase => {
                    Value::String("Proper Case".to_string())
                }
                CapitalizationTypeSetStringCase::iNVERTCASE => {
                    Value::String("iNVERT CASE".to_string())
                }
                CapitalizationTypeSetStringCase::RAnDoMcASe => {
                    Value::String("RAnDoM cASe".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Capitalization Type".to_string()));
        data.insert("action".to_string(), Value::String("SetCase".to_string()));
        data.insert("block".to_string(), Value::String("SetCase".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CapitalizationTypeSetStringCase {
    fn default() -> Self {
        Self::UPPERCASE
    }
}
#[derive(Debug, Clone)]
pub enum LightTypeGetLightLevel {
    Combinedlight,
    Skylight,
    Blocklight,
}
impl LightTypeGetLightLevel {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LightTypeGetLightLevel::Combinedlight => {
                    Value::String("Combined light".to_string())
                }
                LightTypeGetLightLevel::Skylight => {
                    Value::String("Sky light".to_string())
                }
                LightTypeGetLightLevel::Blocklight => {
                    Value::String("Block light".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Light Type".to_string()));
        data.insert("action".to_string(), Value::String("GetLight".to_string()));
        data.insert("block".to_string(), Value::String("GetLight".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LightTypeGetLightLevel {
    fn default() -> Self {
        Self::Combinedlight
    }
}
#[derive(Debug, Clone)]
pub enum DistanceTypeSettoDistance {
    DistanceTwoDXZ,
    DistanceThreeDXYZ,
    AltitudeY,
}
impl DistanceTypeSettoDistance {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DistanceTypeSettoDistance::DistanceTwoDXZ => {
                    Value::String("Distance 2D (X/Z)".to_string())
                }
                DistanceTypeSettoDistance::DistanceThreeDXYZ => {
                    Value::String("Distance 3D (X/Y/Z)".to_string())
                }
                DistanceTypeSettoDistance::AltitudeY => {
                    Value::String("Altitude (Y)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Distance Type".to_string()));
        data.insert("action".to_string(), Value::String("Distance".to_string()));
        data.insert("block".to_string(), Value::String("Distance".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DistanceTypeSettoDistance {
    fn default() -> Self {
        Self::DistanceThreeDXYZ
    }
}
#[derive(Debug, Clone)]
pub enum ReturnValueTypeGetItemMaterial {
    ItemIDgolden_apple,
    ItemNameGoldenApple,
    Item,
}
impl ReturnValueTypeGetItemMaterial {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReturnValueTypeGetItemMaterial::ItemIDgolden_apple => {
                    Value::String("Item ID (golden_apple)".to_string())
                }
                ReturnValueTypeGetItemMaterial::ItemNameGoldenApple => {
                    Value::String("Item Name (Golden Apple)".to_string())
                }
                ReturnValueTypeGetItemMaterial::Item => Value::String("Item".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Return Value Type".to_string()));
        data.insert("action".to_string(), Value::String("GetItemType".to_string()));
        data.insert("block".to_string(), Value::String("GetItemType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReturnValueTypeGetItemMaterial {
    fn default() -> Self {
        Self::ItemIDgolden_apple
    }
}
#[derive(Debug, Clone)]
pub enum RegularExpressionsRemoveString {
    Enable,
    Disable,
}
impl RegularExpressionsRemoveString {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RegularExpressionsRemoveString::Enable => {
                    Value::String("Enable".to_string())
                }
                RegularExpressionsRemoveString::Disable => {
                    Value::String("Disable".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Regular Expressions".to_string()));
        data.insert("action".to_string(), Value::String("RemoveString".to_string()));
        data.insert("block".to_string(), Value::String("RemoveString".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RegularExpressionsRemoveString {
    fn default() -> Self {
        Self::Disable
    }
}
#[derive(Debug, Clone)]
pub enum HideDefaultGetAllBlockData {
    True,
    False,
}
impl HideDefaultGetAllBlockData {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideDefaultGetAllBlockData::True => Value::String("True".to_string()),
                HideDefaultGetAllBlockData::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Default".to_string()));
        data.insert("action".to_string(), Value::String("GetAllBlockData".to_string()));
        data.insert("block".to_string(), Value::String("GetAllBlockData".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideDefaultGetAllBlockData {
    fn default() -> Self {
        Self::True
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateTypeGetLocationCoordinate {
    Plotcoordinate,
    Worldcoordinate,
}
impl CoordinateTypeGetLocationCoordinate {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateTypeGetLocationCoordinate::Plotcoordinate => {
                    Value::String("Plot coordinate".to_string())
                }
                CoordinateTypeGetLocationCoordinate::Worldcoordinate => {
                    Value::String("World coordinate".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate Type".to_string()));
        data.insert("action".to_string(), Value::String("GetCoord".to_string()));
        data.insert("block".to_string(), Value::String("GetCoord".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateTypeGetLocationCoordinate {
    fn default() -> Self {
        Self::Plotcoordinate
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateGetLocationCoordinate {
    X,
    Y,
    Z,
    Pitch,
    Yaw,
}
impl CoordinateGetLocationCoordinate {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateGetLocationCoordinate::X => Value::String("X".to_string()),
                CoordinateGetLocationCoordinate::Y => Value::String("Y".to_string()),
                CoordinateGetLocationCoordinate::Z => Value::String("Z".to_string()),
                CoordinateGetLocationCoordinate::Pitch => {
                    Value::String("Pitch".to_string())
                }
                CoordinateGetLocationCoordinate::Yaw => Value::String("Yaw".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate".to_string()));
        data.insert("action".to_string(), Value::String("GetCoord".to_string()));
        data.insert("block".to_string(), Value::String("GetCoord".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateGetLocationCoordinate {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum DirectionShiftDirection {
    ForwardsTwoDXZ,
    ForwardsThreeDXYZ,
    SidewaysLR,
}
impl DirectionShiftDirection {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DirectionShiftDirection::ForwardsTwoDXZ => {
                    Value::String("Forwards 2D (X/Z)".to_string())
                }
                DirectionShiftDirection::ForwardsThreeDXYZ => {
                    Value::String("Forwards 3D (X/Y/Z)".to_string())
                }
                DirectionShiftDirection::SidewaysLR => {
                    Value::String("Sideways (-L / +R)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Direction".to_string()));
        data.insert("action".to_string(), Value::String("ShiftDirection".to_string()));
        data.insert("block".to_string(), Value::String("ShiftDirection".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DirectionShiftDirection {
    fn default() -> Self {
        Self::ForwardsThreeDXYZ
    }
}
#[derive(Debug, Clone)]
pub enum SpreadGetParticleEffectSpread {
    Horizontal,
    Vertical,
}
impl SpreadGetParticleEffectSpread {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SpreadGetParticleEffectSpread::Horizontal => {
                    Value::String("Horizontal".to_string())
                }
                SpreadGetParticleEffectSpread::Vertical => {
                    Value::String("Vertical".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Spread".to_string()));
        data.insert("action".to_string(), Value::String("GetParticleSprd".to_string()));
        data.insert("block".to_string(), Value::String("GetParticleSprd".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SpreadGetParticleEffectSpread {
    fn default() -> Self {
        Self::Horizontal
    }
}
#[derive(Debug, Clone)]
pub enum TextValueGetHeadOwner {
    OwnerName,
    OwnerUUID,
}
impl TextValueGetHeadOwner {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TextValueGetHeadOwner::OwnerName => {
                    Value::String("Owner Name".to_string())
                }
                TextValueGetHeadOwner::OwnerUUID => {
                    Value::String("Owner UUID".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Text Value".to_string()));
        data.insert("action".to_string(), Value::String("GetHeadOwner".to_string()));
        data.insert("block".to_string(), Value::String("GetHeadOwner".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TextValueGetHeadOwner {
    fn default() -> Self {
        Self::OwnerName
    }
}
#[derive(Debug, Clone)]
pub enum ColorChannelsGetColorChannels {
    RGB,
    HSB,
    HSL,
}
impl ColorChannelsGetColorChannels {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ColorChannelsGetColorChannels::RGB => Value::String("RGB".to_string()),
                ColorChannelsGetColorChannels::HSB => Value::String("HSB".to_string()),
                ColorChannelsGetColorChannels::HSL => Value::String("HSL".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Color Channels".to_string()));
        data.insert("action".to_string(), Value::String("GetColorChannels".to_string()));
        data.insert("block".to_string(), Value::String("GetColorChannels".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ColorChannelsGetColorChannels {
    fn default() -> Self {
        Self::RGB
    }
}
#[derive(Debug, Clone)]
pub enum TangentVariantSettoTangent {
    Tangent,
    Inversetangentarctangent,
    Hyperbolictangent,
}
impl TangentVariantSettoTangent {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TangentVariantSettoTangent::Tangent => {
                    Value::String("Tangent".to_string())
                }
                TangentVariantSettoTangent::Inversetangentarctangent => {
                    Value::String("Inverse tangent (arctangent)".to_string())
                }
                TangentVariantSettoTangent::Hyperbolictangent => {
                    Value::String("Hyperbolic tangent".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Tangent Variant".to_string()));
        data.insert("action".to_string(), Value::String("Tangent".to_string()));
        data.insert("block".to_string(), Value::String("Tangent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TangentVariantSettoTangent {
    fn default() -> Self {
        Self::Tangent
    }
}
#[derive(Debug, Clone)]
pub enum InputSettoTangent {
    Degrees,
    Radians,
}
impl InputSettoTangent {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InputSettoTangent::Degrees => Value::String("Degrees".to_string()),
                InputSettoTangent::Radians => Value::String("Radians".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Input".to_string()));
        data.insert("action".to_string(), Value::String("Tangent".to_string()));
        data.insert("block".to_string(), Value::String("Tangent".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InputSettoTangent {
    fn default() -> Self {
        Self::Degrees
    }
}
#[derive(Debug, Clone)]
pub enum CellEdgeTypeGetVoronoiNoise {
    Euclidean,
    Manhattan,
    Natural,
}
impl CellEdgeTypeGetVoronoiNoise {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CellEdgeTypeGetVoronoiNoise::Euclidean => {
                    Value::String("Euclidean".to_string())
                }
                CellEdgeTypeGetVoronoiNoise::Manhattan => {
                    Value::String("Manhattan".to_string())
                }
                CellEdgeTypeGetVoronoiNoise::Natural => {
                    Value::String("Natural".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Cell Edge Type".to_string()));
        data.insert("action".to_string(), Value::String("VoronoiNoise".to_string()));
        data.insert("block".to_string(), Value::String("VoronoiNoise".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CellEdgeTypeGetVoronoiNoise {
    fn default() -> Self {
        Self::Euclidean
    }
}
#[derive(Debug, Clone)]
pub enum FaceDirectionSetDirection {
    Towardsdirection,
    Towardsoppositedirection,
}
impl FaceDirectionSetDirection {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FaceDirectionSetDirection::Towardsdirection => {
                    Value::String("Towards direction".to_string())
                }
                FaceDirectionSetDirection::Towardsoppositedirection => {
                    Value::String("Towards opposite direction".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Face Direction".to_string()));
        data.insert("action".to_string(), Value::String("SetDirection".to_string()));
        data.insert("block".to_string(), Value::String("SetDirection".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FaceDirectionSetDirection {
    fn default() -> Self {
        Self::Towardsdirection
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateTypeSetLocationCoordinate {
    Plotcoordinate,
    Worldcoordinate,
}
impl CoordinateTypeSetLocationCoordinate {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateTypeSetLocationCoordinate::Plotcoordinate => {
                    Value::String("Plot coordinate".to_string())
                }
                CoordinateTypeSetLocationCoordinate::Worldcoordinate => {
                    Value::String("World coordinate".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate Type".to_string()));
        data.insert("action".to_string(), Value::String("SetCoord".to_string()));
        data.insert("block".to_string(), Value::String("SetCoord".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateTypeSetLocationCoordinate {
    fn default() -> Self {
        Self::Plotcoordinate
    }
}
#[derive(Debug, Clone)]
pub enum CoordinateSetLocationCoordinate {
    X,
    Y,
    Z,
    Pitch,
    Yaw,
}
impl CoordinateSetLocationCoordinate {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CoordinateSetLocationCoordinate::X => Value::String("X".to_string()),
                CoordinateSetLocationCoordinate::Y => Value::String("Y".to_string()),
                CoordinateSetLocationCoordinate::Z => Value::String("Z".to_string()),
                CoordinateSetLocationCoordinate::Pitch => {
                    Value::String("Pitch".to_string())
                }
                CoordinateSetLocationCoordinate::Yaw => Value::String("Yaw".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Coordinate".to_string()));
        data.insert("action".to_string(), Value::String("SetCoord".to_string()));
        data.insert("block".to_string(), Value::String("SetCoord".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CoordinateSetLocationCoordinate {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum IgnorePassableBlocksRaycastBlock {
    True,
    False,
}
impl IgnorePassableBlocksRaycastBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnorePassableBlocksRaycastBlock::True => {
                    Value::String("True".to_string())
                }
                IgnorePassableBlocksRaycastBlock::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Ignore Passable Blocks".to_string()),
        );
        data.insert("action".to_string(), Value::String("RaycastBlock".to_string()));
        data.insert("block".to_string(), Value::String("RaycastBlock".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnorePassableBlocksRaycastBlock {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum FluidCollisionRaycastBlock {
    Ignorefluids,
    Detectfluids,
    Sourceblocksonly,
}
impl FluidCollisionRaycastBlock {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FluidCollisionRaycastBlock::Ignorefluids => {
                    Value::String("Ignore fluids".to_string())
                }
                FluidCollisionRaycastBlock::Detectfluids => {
                    Value::String("Detect fluids".to_string())
                }
                FluidCollisionRaycastBlock::Sourceblocksonly => {
                    Value::String("Source blocks only".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Fluid Collision".to_string()));
        data.insert("action".to_string(), Value::String("RaycastBlock".to_string()));
        data.insert("block".to_string(), Value::String("RaycastBlock".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FluidCollisionRaycastBlock {
    fn default() -> Self {
        Self::Ignorefluids
    }
}
#[derive(Debug, Clone)]
pub enum FractalTypeGetPerlinNoise {
    Brownian,
    BillowDarkedges,
    RigidLightedges,
}
impl FractalTypeGetPerlinNoise {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FractalTypeGetPerlinNoise::Brownian => {
                    Value::String("Brownian".to_string())
                }
                FractalTypeGetPerlinNoise::BillowDarkedges => {
                    Value::String("Billow (Dark edges)".to_string())
                }
                FractalTypeGetPerlinNoise::RigidLightedges => {
                    Value::String("Rigid (Light edges)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Fractal Type".to_string()));
        data.insert("action".to_string(), Value::String("PerlinNoise".to_string()));
        data.insert("block".to_string(), Value::String("PerlinNoise".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FractalTypeGetPerlinNoise {
    fn default() -> Self {
        Self::Brownian
    }
}
#[derive(Debug, Clone)]
pub enum CellEdgeTypeGetWorleyNoise {
    Euclidean,
    Manhattan,
    Natural,
}
impl CellEdgeTypeGetWorleyNoise {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CellEdgeTypeGetWorleyNoise::Euclidean => {
                    Value::String("Euclidean".to_string())
                }
                CellEdgeTypeGetWorleyNoise::Manhattan => {
                    Value::String("Manhattan".to_string())
                }
                CellEdgeTypeGetWorleyNoise::Natural => {
                    Value::String("Natural".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Cell Edge Type".to_string()));
        data.insert("action".to_string(), Value::String("WorleyNoise".to_string()));
        data.insert("block".to_string(), Value::String("WorleyNoise".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CellEdgeTypeGetWorleyNoise {
    fn default() -> Self {
        Self::Euclidean
    }
}
#[derive(Debug, Clone)]
pub enum DistanceCalculationGetWorleyNoise {
    Primary,
    Secondary,
    Additive,
    Subtractive,
    Multiplicative,
    Divisive,
}
impl DistanceCalculationGetWorleyNoise {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                DistanceCalculationGetWorleyNoise::Primary => {
                    Value::String("Primary".to_string())
                }
                DistanceCalculationGetWorleyNoise::Secondary => {
                    Value::String("Secondary".to_string())
                }
                DistanceCalculationGetWorleyNoise::Additive => {
                    Value::String("Additive".to_string())
                }
                DistanceCalculationGetWorleyNoise::Subtractive => {
                    Value::String("Subtractive".to_string())
                }
                DistanceCalculationGetWorleyNoise::Multiplicative => {
                    Value::String("Multiplicative".to_string())
                }
                DistanceCalculationGetWorleyNoise::Divisive => {
                    Value::String("Divisive".to_string())
                }
            },
        );
        data.insert(
            "tag".to_string(),
            Value::String("Distance Calculation".to_string()),
        );
        data.insert("action".to_string(), Value::String("WorleyNoise".to_string()));
        data.insert("block".to_string(), Value::String("WorleyNoise".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for DistanceCalculationGetWorleyNoise {
    fn default() -> Self {
        Self::Primary
    }
}
#[derive(Debug, Clone)]
pub enum RoundModeRoundNumberN {
    Floor,
    Nearest,
    Ceiling,
}
impl RoundModeRoundNumberN {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RoundModeRoundNumberN::Floor => Value::String("Floor".to_string()),
                RoundModeRoundNumberN::Nearest => Value::String("Nearest".to_string()),
                RoundModeRoundNumberN::Ceiling => Value::String("Ceiling".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Round Mode".to_string()));
        data.insert("action".to_string(), Value::String(" RoundNumber ".to_string()));
        data.insert("block".to_string(), Value::String(" RoundNumber ".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RoundModeRoundNumberN {
    fn default() -> Self {
        Self::Nearest
    }
}
#[derive(Debug, Clone)]
pub enum SortOrderSortList {
    Ascending,
    Descending,
}
impl SortOrderSortList {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SortOrderSortList::Ascending => Value::String("Ascending".to_string()),
                SortOrderSortList::Descending => Value::String("Descending".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Sort Order".to_string()));
        data.insert("action".to_string(), Value::String("SortList".to_string()));
        data.insert("block".to_string(), Value::String("SortList".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SortOrderSortList {
    fn default() -> Self {
        Self::Ascending
    }
}
#[derive(Debug, Clone)]
pub enum FormatFormatTimestamp {
    Custom,
    TwoZeroTwoZeroZeroEightOneSevenOneSevenTwoZeroFiveFour,
    TwoZeroTwoZeroZeroEightOneSeven,
    MonAugustOneSeven,
    Monday,
    OneSevenTwoZeroFiveFour,
    FiveTwoZeroPM,
    OneSevenhTwoZeromFiveFours,
    FiveFourTwoTwoNineseconds,
}
impl FormatFormatTimestamp {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                FormatFormatTimestamp::Custom => Value::String("Custom".to_string()),
                FormatFormatTimestamp::TwoZeroTwoZeroZeroEightOneSevenOneSevenTwoZeroFiveFour => {
                    Value::String("2020/08/17 17:20:54".to_string())
                }
                FormatFormatTimestamp::TwoZeroTwoZeroZeroEightOneSeven => {
                    Value::String("2020/08/17".to_string())
                }
                FormatFormatTimestamp::MonAugustOneSeven => {
                    Value::String("Mon, August 17".to_string())
                }
                FormatFormatTimestamp::Monday => Value::String("Monday".to_string()),
                FormatFormatTimestamp::OneSevenTwoZeroFiveFour => {
                    Value::String("17:20:54".to_string())
                }
                FormatFormatTimestamp::FiveTwoZeroPM => {
                    Value::String("5:20 PM".to_string())
                }
                FormatFormatTimestamp::OneSevenhTwoZeromFiveFours => {
                    Value::String("17h20m54s".to_string())
                }
                FormatFormatTimestamp::FiveFourTwoTwoNineseconds => {
                    Value::String("54.229 seconds".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Format".to_string()));
        data.insert("action".to_string(), Value::String("FormatTime".to_string()));
        data.insert("block".to_string(), Value::String("FormatTime".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for FormatFormatTimestamp {
    fn default() -> Self {
        Self::TwoZeroTwoZeroZeroEightOneSevenOneSevenTwoZeroFiveFour
    }
}
#[derive(Debug, Clone)]
pub enum HideArmorTrimSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideArmorTrimSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideArmorTrimSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideArmorTrimSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideArmorTrimSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Armor Trim".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideArmorTrimSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideColorSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideColorSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideColorSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideColorSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideColorSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Color".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideColorSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideEnchantmentsSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideEnchantmentsSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideEnchantmentsSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideEnchantmentsSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideEnchantmentsSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Enchantments".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideEnchantmentsSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideAttributesSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideAttributesSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideAttributesSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideAttributesSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideAttributesSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Attributes".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideAttributesSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideUnbreakableSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideUnbreakableSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideUnbreakableSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideUnbreakableSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideUnbreakableSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Unbreakable".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideUnbreakableSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideCanDestroySetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideCanDestroySetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideCanDestroySetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideCanDestroySetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideCanDestroySetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Can Destroy".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideCanDestroySetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HideCanPlaceOnSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HideCanPlaceOnSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HideCanPlaceOnSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HideCanPlaceOnSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HideCanPlaceOnSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Can Place On".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HideCanPlaceOnSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum HidePotionEffectsSetItemVisibilityFlags {
    True,
    False,
    NoChange,
}
impl HidePotionEffectsSetItemVisibilityFlags {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                HidePotionEffectsSetItemVisibilityFlags::True => {
                    Value::String("True".to_string())
                }
                HidePotionEffectsSetItemVisibilityFlags::False => {
                    Value::String("False".to_string())
                }
                HidePotionEffectsSetItemVisibilityFlags::NoChange => {
                    Value::String("No Change".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Hide Potion Effects".to_string()));
        data.insert("action".to_string(), Value::String("SetItemFlags".to_string()));
        data.insert("block".to_string(), Value::String("SetItemFlags".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for HidePotionEffectsSetItemVisibilityFlags {
    fn default() -> Self {
        Self::NoChange
    }
}
#[derive(Debug, Clone)]
pub enum IgnoreEmptySlotsGetContainerContents {
    True,
    False,
}
impl IgnoreEmptySlotsGetContainerContents {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnoreEmptySlotsGetContainerContents::True => {
                    Value::String("True".to_string())
                }
                IgnoreEmptySlotsGetContainerContents::False => {
                    Value::String("False".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Empty Slots".to_string()));
        data.insert(
            "action".to_string(),
            Value::String("GetContainerItems".to_string()),
        );
        data.insert("block".to_string(), Value::String("GetContainerItems".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnoreEmptySlotsGetContainerContents {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum CosineVariantSettoCosine {
    Cosine,
    Inversecosinearccosine,
    Hyperboliccosine,
}
impl CosineVariantSettoCosine {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                CosineVariantSettoCosine::Cosine => Value::String("Cosine".to_string()),
                CosineVariantSettoCosine::Inversecosinearccosine => {
                    Value::String("Inverse cosine (arccosine)".to_string())
                }
                CosineVariantSettoCosine::Hyperboliccosine => {
                    Value::String("Hyperbolic cosine".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Cosine Variant".to_string()));
        data.insert("action".to_string(), Value::String("Cosine".to_string()));
        data.insert("block".to_string(), Value::String("Cosine".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for CosineVariantSettoCosine {
    fn default() -> Self {
        Self::Cosine
    }
}
#[derive(Debug, Clone)]
pub enum InputSettoCosine {
    Degrees,
    Radians,
}
impl InputSettoCosine {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                InputSettoCosine::Degrees => Value::String("Degrees".to_string()),
                InputSettoCosine::Radians => Value::String("Radians".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Input".to_string()));
        data.insert("action".to_string(), Value::String("Cosine".to_string()));
        data.insert("block".to_string(), Value::String("Cosine".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for InputSettoCosine {
    fn default() -> Self {
        Self::Degrees
    }
}
#[derive(Debug, Clone)]
pub enum ComponentSetVectorComponent {
    X,
    Y,
    Z,
}
impl ComponentSetVectorComponent {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ComponentSetVectorComponent::X => Value::String("X".to_string()),
                ComponentSetVectorComponent::Y => Value::String("Y".to_string()),
                ComponentSetVectorComponent::Z => Value::String("Z".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Component".to_string()));
        data.insert("action".to_string(), Value::String("SetVectorComp".to_string()));
        data.insert("block".to_string(), Value::String("SetVectorComp".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ComponentSetVectorComponent {
    fn default() -> Self {
        Self::X
    }
}
#[derive(Debug, Clone)]
pub enum IgnorePitchShiftAllDirs {
    True,
    False,
}
impl IgnorePitchShiftAllDirs {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                IgnorePitchShiftAllDirs::True => Value::String("True".to_string()),
                IgnorePitchShiftAllDirs::False => Value::String("False".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Ignore Pitch".to_string()));
        data.insert("action".to_string(), Value::String("ShiftAllDirs".to_string()));
        data.insert("block".to_string(), Value::String("ShiftAllDirs".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for IgnorePitchShiftAllDirs {
    fn default() -> Self {
        Self::False
    }
}
#[derive(Debug, Clone)]
pub enum SearchOrderGetListIndexofValue {
    Ascendingfirstindex,
    Descendinglastindex,
}
impl SearchOrderGetListIndexofValue {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                SearchOrderGetListIndexofValue::Ascendingfirstindex => {
                    Value::String("Ascending (first index)".to_string())
                }
                SearchOrderGetListIndexofValue::Descendinglastindex => {
                    Value::String("Descending (last index)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Search Order".to_string()));
        data.insert("action".to_string(), Value::String("GetValueIndex".to_string()));
        data.insert("block".to_string(), Value::String("GetValueIndex".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for SearchOrderGetListIndexofValue {
    fn default() -> Self {
        Self::Ascendingfirstindex
    }
}
#[derive(Debug, Clone)]
pub enum ShiftDirectionShiftLocation {
    UpwardsDownwards,
    ForwardsBackwards,
    RightLeft,
}
impl ShiftDirectionShiftLocation {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ShiftDirectionShiftLocation::UpwardsDownwards => {
                    Value::String("(+) Upwards / (-) Downwards".to_string())
                }
                ShiftDirectionShiftLocation::ForwardsBackwards => {
                    Value::String("(+) Forwards / (-) Backwards".to_string())
                }
                ShiftDirectionShiftLocation::RightLeft => {
                    Value::String("(+) Right / (-) Left".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Shift Direction".to_string()));
        data.insert("action".to_string(), Value::String("ShiftLocation".to_string()));
        data.insert("block".to_string(), Value::String("ShiftLocation".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ShiftDirectionShiftLocation {
    fn default() -> Self {
        Self::ForwardsBackwards
    }
}
#[derive(Debug, Clone)]
pub enum RoundModeRound {
    Floor,
    Nearest,
    Ceiling,
}
impl RoundModeRound {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                RoundModeRound::Floor => Value::String("Floor".to_string()),
                RoundModeRound::Nearest => Value::String("Nearest".to_string()),
                RoundModeRound::Ceiling => Value::String("Ceiling".to_string()),
            },
        );
        data.insert("tag".to_string(), Value::String("Round Mode".to_string()));
        data.insert("action".to_string(), Value::String("Round".to_string()));
        data.insert("block".to_string(), Value::String("Round".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for RoundModeRound {
    fn default() -> Self {
        Self::Nearest
    }
}
#[derive(Debug, Clone)]
pub enum ReturnValueTypeGetSoundPitch {
    Pitchnumber,
    Notetext,
}
impl ReturnValueTypeGetSoundPitch {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReturnValueTypeGetSoundPitch::Pitchnumber => {
                    Value::String("Pitch (number)".to_string())
                }
                ReturnValueTypeGetSoundPitch::Notetext => {
                    Value::String("Note (text)".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Return Value Type".to_string()));
        data.insert("action".to_string(), Value::String("GetSoundPitch".to_string()));
        data.insert("block".to_string(), Value::String("GetSoundPitch".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReturnValueTypeGetSoundPitch {
    fn default() -> Self {
        Self::Pitchnumber
    }
}
#[derive(Debug, Clone)]
pub enum TranslationTypeTranslateColors {
    Fromhextocolor,
    Fromtocolor,
    Fromcolorto,
    Stripcolor,
}
impl TranslationTypeTranslateColors {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                TranslationTypeTranslateColors::Fromhextocolor => {
                    Value::String("From hex to color".to_string())
                }
                TranslationTypeTranslateColors::Fromtocolor => {
                    Value::String("From & to color".to_string())
                }
                TranslationTypeTranslateColors::Fromcolorto => {
                    Value::String("From color to &".to_string())
                }
                TranslationTypeTranslateColors::Stripcolor => {
                    Value::String("Strip color".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Translation Type".to_string()));
        data.insert("action".to_string(), Value::String("TranslateColors".to_string()));
        data.insert("block".to_string(), Value::String("TranslateColors".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for TranslationTypeTranslateColors {
    fn default() -> Self {
        Self::Fromtocolor
    }
}
#[derive(Debug, Clone)]
pub enum GrowthUnitGetBlockGrowth {
    Growthstagenumber,
    Growthpercentage,
}
impl GrowthUnitGetBlockGrowth {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                GrowthUnitGetBlockGrowth::Growthstagenumber => {
                    Value::String("Growth stage number".to_string())
                }
                GrowthUnitGetBlockGrowth::Growthpercentage => {
                    Value::String("Growth percentage".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Growth Unit".to_string()));
        data.insert("action".to_string(), Value::String("GetBlockGrowth".to_string()));
        data.insert("block".to_string(), Value::String("GetBlockGrowth".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for GrowthUnitGetBlockGrowth {
    fn default() -> Self {
        Self::Growthstagenumber
    }
}
#[derive(Debug, Clone)]
pub enum ReturnValueTypeGetBlockMaterial {
    BlockIDoak_log,
    BlocknameOakLog,
    Item,
}
impl ReturnValueTypeGetBlockMaterial {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                ReturnValueTypeGetBlockMaterial::BlockIDoak_log => {
                    Value::String("Block ID (oak_log)".to_string())
                }
                ReturnValueTypeGetBlockMaterial::BlocknameOakLog => {
                    Value::String("Block name (Oak Log)".to_string())
                }
                ReturnValueTypeGetBlockMaterial::Item => {
                    Value::String("Item".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Return Value Type".to_string()));
        data.insert("action".to_string(), Value::String("GetBlockType".to_string()));
        data.insert("block".to_string(), Value::String("GetBlockType".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for ReturnValueTypeGetBlockMaterial {
    fn default() -> Self {
        Self::BlockIDoak_log
    }
}
#[derive(Debug, Clone)]
pub enum LengthTypeGetVectorLength {
    Length,
    LengthSquared,
}
impl LengthTypeGetVectorLength {
    pub fn json(&self) -> serde_json::Map<String, Value> {
        let mut map = serde_json::Map::new();
        let mut data = serde_json::Map::new();
        data.insert(
            "option".to_string(),
            match self {
                LengthTypeGetVectorLength::Length => Value::String("Length".to_string()),
                LengthTypeGetVectorLength::LengthSquared => {
                    Value::String("Length Squared".to_string())
                }
            },
        );
        data.insert("tag".to_string(), Value::String("Length Type".to_string()));
        data.insert("action".to_string(), Value::String("GetVectorLength".to_string()));
        data.insert("block".to_string(), Value::String("GetVectorLength".to_string()));
        map.insert("data".to_string(), Value::Object(data));
        map.insert("id".to_string(), Value::String("bl_tag".to_string()));
        map
    }
}
impl Default for LengthTypeGetVectorLength {
    fn default() -> Self {
        Self::Length
    }
}
