mod emission_factors {
    // --- Transport ---
    pub const PETROL_KG_PER_LITRE: f64 = 2.31;
    pub const DIESEL_KG_PER_LITRE: f64 = 2.68;
    pub const ELECTRIC_KG_PER_KWH: f64 = 0.0;

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
    pub const FOOD_WASTE_KG_PER_KG: f64 = 0.5;
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
        TransportMode::Car {
            fuel_type,
            consumption_l_per_100km,
        } => {
            // How many did the trip use?
            let litres = (details.distance_km / 100.0) * consumption_l_per_100km;

            // which emission factor applies to this fuel?
            let kg_per_litre = match fuel_type {
                FuelType::Petrol => emission_factors::PETROL_KG_PER_LITRE,
                FuelType::Diesel => emission_factors::DIESEL_KG_PER_LITRE,
                FuelType::Electric => emission_factors::ELECTRIC_KG_PER_KWH,
            };

            litres * kg_per_litre
        }

        TransportMode::Flight { is_long_haul } => {
            let factor = if *is_long_haul {
                emission_factors::FLIGHT_LONG_HAUL_KG_PER_KM
            } else {
                emission_factors::FLIGHT_SHORT_HAUL_KG_PER_KM
            };
            details.distance_km * factor
        }

        TransportMode::PublicTransport => {
            details.distance_km * emission_factors::PUBLIC_TRANSPORT_KG_PER_KM
        }
    }
}

fn calculate_energy_co2e(details: &EnergyDetails) -> f64 {
    let electricity_co2e = match &details.electricity {
        Some(usage) => {
            if usage.is_renewable {
                0.0 // renewable elctricity = no grid emissions
            } else {
                usage.kwh * emission_factors::ELECTRICITY_GRID_PER_KWH
            }
        }

        None => 0.0, // no electricity data provided.
    };

    let heating_co2e = match &details.heating {
        Some(heating) => {
            match heating {
                HeatingUsage::NaturalGas { units_m3 } => {
                    units_m3 * emission_factors::NATURAL_GAS_KG_PER_M3
                }
                HeatingUsage::FuelOil { liters } => {
                    liters * emission_factors::FUEL_OIL_KG_PER_LITRE
                }
                HeatingUsage::Propane { kg } => kg * emission_factors::PROPANE_KG_PER_KG,
                HeatingUsage::ElectricHeat => {
                    // electric heating — already counted in electricity above
                    0.0
                }
            }
        }

        None => 0.0, // no heating data
    };

    electricity_co2e + heating_co2e
}

fn calculate_diet_co2e(details: &DietDetails) -> f64 {
    // --- meat and dairy servings ---

    let meat_co2e: f64 = details
        .meat_servings
        .iter()
        .map(|item| {
            let factor = match item.category {
                ProteinSource::Beef => emission_factors::BEEF_KG_PER_SERVING,
                ProteinSource::Lamb => emission_factors::LAMB_KG_PER_SERVING,
                ProteinSource::Poultry => emission_factors::POULTRY_KG_PER_SERVING,
                ProteinSource::Pork => emission_factors::PORK_KG_PER_SERVING,
                ProteinSource::Dairy => emission_factors::DAIRY_KG_PER_SERVING,
            };
            item.servings_per_week * factor
        })
        .sum();

    // --- food waste ---
    let waste_co2e = details.food_waste_kg * emission_factors::FOOD_WASTE_KG_PER_KG;

    // --- apply local food reduction ---
    let local_reduction = 1.0 - (details.local_food_ratio * 0.1);

    // --- Add it all up and apply the reduction ---
    (meat_co2e + waste_co2e) * local_reduction
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)] // "only compile this when running tests"
mod tests {
    use super::*; // "bring everything from the parent module in"

    #[test] // "this function is a test"
    fn test_petrol_car_100km() {
        let details = TravelDetails {
            distance_km: 100.0,
            mode: TransportMode::Car {
                fuel_type: FuelType::Petrol,
                consumption_l_per_100km: 8.0,
            },
        };

        // 100km at 8L/100km = 8 litres
        // 8 litres × 2.31 kg/L = 18.48 kg CO₂e
        let result = calculate_travel_co2e(&details);
        let expected = 18.48;

        // abs() is absolute value — we check they're within 0.01 of each other
        // because floating point math isn't perfectly precise
        assert!(
            (result - expected).abs() < 0.01,
            "Expected {}, got {}",
            expected,
            result
        );
    }

    #[test]
    fn test_electricity_only() {
        let details = EnergyDetails {
            electricity: Some(ElectricityUsage {
                kwh: 100.0,
                is_renewable: false,
            }),
            heating: None, // ← None here
        };
        let result = calculate_energy_co2e(&details);
        assert!(
            (result - 23.3).abs() < 0.01,
            "Expected 23.3, got {}",
            result
        );
    }

    #[test]
    fn test_natural_gas_only() {
        let details = EnergyDetails {
            electricity: None,
            heating: Some(HeatingUsage::NaturalGas { units_m3: 50.0 }),
        };
        let result = calculate_energy_co2e(&details);
        assert!(
            (result - 101.0).abs() < 0.01,
            "Expected 101.0, got {}",
            result
        );
    }

    #[test]
    fn test_beef_only() {
        let details = DietDetails {
            meat_servings: vec![MeatData {
                category: ProteinSource::Beef,
                servings_per_week: 2.0,
            }],
            food_waste_kg: 0.0,
            local_food_ratio: 0.0,
        };
        let result = calculate_diet_co2e(&details);
        // 2.0 servings × 3.3 kg/serving = 6.6
        assert!((result - 6.6).abs() < 0.01, "Expected 6.6, got {}", result);
    }

    #[test]
    fn test_poultry_with_waste() {
        let details = DietDetails {
            meat_servings: vec![MeatData {
                category: ProteinSource::Poultry,
                servings_per_week: 4.0,
            }],
            food_waste_kg: 10.0,
            local_food_ratio: 0.0,
        };
        let result = calculate_diet_co2e(&details);
        // meat:  4.0 × 0.7 = 2.8
        // waste: 10.0 × 0.5 = 5.0
        // total: 7.8
        assert!((result - 7.8).abs() < 0.01, "Expected 7.8, got {}", result);
    }
}
