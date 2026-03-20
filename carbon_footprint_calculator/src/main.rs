mod emission_factors {
    // --- Transport ---
    pub const PETROL_KG_PER_LITRE: f64 = 2.31;
    pub const DIESEL_KG_PER_LITRE: f64 = 2.68;
    pub const ELECTRIC_KG_PER_LITRE: f64 = 0.0;

    pub const FLIGHT_SHORT_HAUL_KG_PER_KM: f64 = 0.255;
    pub const FLIGHT_LONG_HAUL_KG_PER_KM: f64 = 0.195;
    pub const PUBLIC_TRANSPORT_KG_PER_KM: f64 = 0.089;

    // --- Energy ---
    pub const ELECTRICITY_GRID_PER_KWH: f64 = 0.233;
    pub const NATURAL_GAS_KG_PER_M3: f64 = 2.02;
    pub const FUEL_OIL_KG_PER_LITRE: f64 = 2.96;
    pub const PROPANE_KG_PER_KG: f64 = 2.98;

    // --- Diet (kg CO₂e per serving) ---
    pub const BEEF_KG_PER_SERVING: f64 = 3.3;
    pub const LAMB_KG_PER_SERVING: f64 = 2.4;
    pub const POULTRY_KG_PER_SERVING: f64 = 0.7;
    pub const PORK_KG_PER_SERVING: f64 = 0.7;
    pub const DAIRY_KG_PER_SERVING: f64 = 0.4;
}

#[derive(Debug, Clone)]
enum Activity {
    Travel(TravelDetails),
    HomeEnergy(EnergyDetails),
    Diet(DietDetails),
    ConsumerGoods(GoodsDetails),
}

#[derive(Debug, Clone)]
struct TravelDetails {
    distance_km: f64,
    mode: TransportMode,
}

#[derive(Debug, Clone)]
enum TransportMode {
    Car {
        fuel_type: FuelType,
        consumption_l_per_100km: f64,
    },
    Flight {
        is_long_haul: bool,
    },
    PublicTransport,
}

#[derive(Debug, Clone)]
enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

#[derive(Debug, Clone)]
struct EnergyDetails {
    electricity: Option<ElectricityUsage>,
    heating: Option<HeatingUsage>,
}

#[derive(Debug, Clone)]
struct ElectricityUsage {
    kwh: f64,
    is_renewable: bool,
}

#[derive(Debug, Clone)]
enum HeatingUsage {
    NaturalGas { units_m3: f64 },
    FuelOil { liters: f64 },
    Propane { kg: f64 },
    ElectricHeat,
}

#[derive(Debug, Clone)]
struct DietDetails {
    meat_servings: Vec<MeatData>, // empty = no meat
    food_waste_kg: f64,
    local_food_ratio: f64,
}

#[derive(Debug, Clone)]
enum ProteinSource {
    Beef,
    Lamb,
    Poultry,
    Pork,
    Dairy,
}

#[derive(Debug, Clone)]
struct MeatData {
    category: ProteinSource,
    servings_per_week: f64,
}

#[derive(Debug, Clone)]
struct GoodsDetails {
    purchases: Vec<Purchase>,
    waste: Vec<WasteEntry>, // multiple waste streams
    digital_usage_gb: f64,
}

#[derive(Debug, Clone)]
enum Purchase {
    Clothing { pieces: u32, is_fast_fashion: bool },
    Electronics { device_type: DeviceType },
    HouseholdItem,
}

#[derive(Debug, Clone)]
enum DeviceType {
    SmartPhone,
    Laptop,
    LargeAppliance,
}

#[derive(Debug, Clone)]
enum WasteType {
    Landfill,
    Recycle,
    Compost,
}

#[derive(Debug, Clone)]
struct WasteEntry {
    category: WasteType,
    weight_kg: f64,
}

fn calculate_travel_co2e(details: &TravelDetails) -> f64 {
    match &details.mode {
        TransportMode::Car { fuel_type, consumption_l_per_100km} => {
            // How many did the trip use?
            let litres = (details.distance_km / 100.0 ) * consumption_l_per_100km;

            // which emission factor applies to this fuel?
            let kg_per_litre = match fuel_type {
                FuelType::Petrol   => emission_factors::PETROL_KG_PER_LITRE,
                FuelType::Diesel   => emission_factors::DIESEL_KG_PER_LITRE,
                FuelType::Electric => emission_factors::ELECTRIC_KG_PER_KWH,
            };

            litres * kg_per_litre
        },

        TransportMode::Flight { is_long_haul } => {
            let factor = if *is_long_haul {
                emission_factors::FLIGHT_LONG_HAUL_KG_PER_KM
            } else {
                emission_factors::FLIGHT_SHORT_HAUL_KG_PER_KM
            };
            details.distance_km * factor
        },

        TransportMode::PublicTransport => {
            details.distance_km * emission_factors::PUBLIC_TRANSPORT_KG_PER_KM
        },
    }
}

fn main() {
    println!("Hello, world!");
}
