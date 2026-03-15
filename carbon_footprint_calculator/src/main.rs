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
    Car { fuel_type: FuelType, consumption_l_per_100km: f64 },
    Flight { is_long_haul: bool },
    PublicTransport,
}

#[derive(Debug, Clone)]
enum FuelType { Petrol, Diesel, Electric }

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
    meat_servings: Vec<MeatData>,  // empty = no meat
    food_waste_kg: f64,
    local_food_ratio: f64,
}

#[derive(Debug, Clone)]
enum ProteinSource { Beef, Lamb, Poultry, Pork, Dairy }

#[derive(Debug, Clone)]
struct MeatData {
    category: ProteinSource,
    servings_per_week: f64,
}

#[derive(Debug, Clone)]
struct GoodsDetails {
    purchases: Vec<Purchase>,
    waste: Vec<WasteEntry>,       // multiple waste streams
    digital_usage_gb: f64,
}

#[derive(Debug, Clone)]
enum Purchase {
    Clothing { pieces: u32, is_fast_fashion: bool },
    Electronics { device_type: DeviceType },  // typo fixed
    HouseholdItem,
}

#[derive(Debug, Clone)]
enum DeviceType { SmartPhone, Laptop, LargeAppliance }

#[derive(Debug, Clone)]
enum WasteType { Landfill, Recycle, Compost }

#[derive(Debug, Clone)]
struct WasteEntry {
    category: WasteType,
    weight_kg: f64,
}

fn main() {
    println!("Hello, world!");
}
