enum Activity{
    Travel(TravelDetails),
    HomeEnergy(EnergyDetails),
    Diet(DietDetails),
    ConsumerGoods(GoodsDetails),
}

struct TravelDetails {
    distance_km: f64,
    mode: TransportMode,
}

enum TransportMode {
    Car {fuel_type: FuelType, consumption_l_per_100km: f64 },
    Flight { is_long_haul: bool },
    PublicTransport,
}

enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

struct EnergyDetails {
    electricity: Option<ElectricityUsage>,
    heating: Option<HeatingUsage>,
}

struct ElectricityUsage {
    kwh: f64,
    is_renewable: bool,
}

enum HeatingUsage {
    NaturalGas {units_m3: f64},
    FuelOil {liters: f64},
    Propane {kg: f64},
    ElectricHeat, 
}

struct DietDetails {
    meat_consumption: Option<MeatData>,
    food_waste_kg: f64,
    local_food_ratio: f64, // A value between 0.0 and 1.0 (0% to 100%)
}

enum MeatType {
    Beef,
    Lamb,
    Poultry,
    Pork,
    Dairy,
}

struct MeatData {
    category: MeatType,
    servings_per_week: f64,
}

struct GoodsDetails {
    purchases: Vec<Purchase>, // a lisst of multiple items bought
    waste: WasteData,
    digital_usage_gb: f64, // internet data usage in Gigabytes

}

enum Purchase {
    Clothing { pieces: u32, is_fast_fashion: bool},
    Electronincs { device_type: DeviceType },
    HouseholdItem,
}

enum DeviceType {
    SmartPhone,
    Laptop,
    LargeAppliance,
}

enum WasteType {
    Landfill,
    Recycle,
    Compost,
}

struct WasteData {
    category: WasteType,
    weight_kg: f64,
}

fn main() {
    println!("Hello, world!");
}
