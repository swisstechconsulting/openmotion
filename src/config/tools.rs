enum Unit {
    Millimeter,
    Inch,
    Degrees,
    MillimetersPerMinute,
    RotationsPerMinute,
    MetersPerMinute,
}

#[derive(Debug)]
struct Dimention {
    value: u32,
    unit: Unit,
}

impl Dimention {
    fn angle(degrees: u8) -> Self {
        Dimention {
            value: degrees,
            unit: Unit.Degrees,
        }
    }

    fn length(value: u32, unit: Unit::Degrees) -> u8 {
        Dimention {
            value: degrees,
            unit: Unit.Degrees,
        }
    }

    fn millimeters(&self) -> u8 {
        self.value
    }

    // fn angle()
}

#[derive(Debug)]
struct AngleDimention {
    degrees: int32,
}

enum MillType {
    BallEnd,
    BullNoseEnd,
    EngraveChamfer,
    Dovetail,
    Face,
    FlatEnd,
    Form,
    Lollipop,
    Radius,
    Slot,
    Tapered,
    Thread,
}

enum ToolMaterial {
    Unspecified,
    HSS,
    TiCoated,
    Carbide,
    Ceramic,
}

enum HolderSize {
    CT30,
    CT40,
    CT45,
    CT50,

    BT30,
    BT35,
    BT40,
    BT45,
    BT50,

    HSK40A,
    HSK40E,
    HSK63A,
    HSK63F,
    HSK100A,
}

#[derive(Debug)]
struct ToolHolder {
    description: String,
    product_id: String,
    link: String,
    vendor: String,
    size: HolderSize,
}

#[derive(Debug)]
struct ShaftSlice {
    index: u32,
    upper_diameter: Dimention,
    lower_diameter: Dimention,
}

enum Coolant {
    Disabled,
    Flood,
    Mist,
    ThroughTool,
    Air,
    AirThroughTool,
    Suction,
    FloodAndMist,
    FloodAndThroughTool,
}

#[derive(Debug)]
struct Tool {
    // General
    description: String,
    vendor: String,
    product_id: String,
    product_link: String,

    // Cutter
    mill_type: MillType,
    num_flutes: u8,
    is_clockwise: bool,

    // Geometry
    diameter: Dimention,
    shaft_diameter: Dimention,
    overall_length: Dimention,
    length_below_holder: Dimention,
    shoulder_length: Dimention,
    flute_length: Dimention,
    corner_radius: length,
    taper_angle: AngleDimention,

    // Shaft
    shaft: Vec<ShaftSlice>,

    // Holder
    holder: ToolHolder,

    // Cutting Data
    /// Speed
    spindle_speed_rpm: u32,
    surface_speed_meters_min: u32,
    ramp_spindle_speed_rpm: u32,
    /// Feedrates
    cutting_feedrate_mm_min: u32,
    feed_per_tooth_mm: u32,
    lead_in_feedrate_mm_min: u32,
    lead_out_feedrate_mm_min: u32,
    transition_feedrate_mm_min: u32,
    ramp_feedrate: u32,
    /// Vertical Feedrates
    plunge_feedrate: u32,
    plunge_feed_per_revolution_mm: u32,
    coolant: Coolant,
}
