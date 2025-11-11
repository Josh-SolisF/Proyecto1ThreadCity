use mypthreads::mythread::mymutex::MyMutex;
use crate::cityblock::Block;
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::map::Map;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::shopblock::ShopBlock;

/// 1x4: Road - Road - Road - Shop

pub fn build_column_map_4x1() -> Map {
    // 4 filas, 1 columna ⇒ grid.len() = 4 (height), grid[0].len() = 1 (width)
    let mut grid: Vec<Vec<Box<dyn Block>>> = Vec::new();
    grid.push(vec![ Box::new(RoadBlock::new(0)) ]); // (x=0, y=0)
    grid.push(vec![ Box::new(RoadBlock::new(1)) ]); // (0,1)
    grid.push(vec![ Box::new(RoadBlock::new(2)) ]); // (0,2)
    grid.push(vec![ Box::new(ShopBlock::new(3, Vec::new())) ]); // (0,3)
    Map::build_custom(grid)
}


/// 1x3: Road - Bridge - Road  (útil para probar NextIsBridge)

pub fn build_column_map_with_bridge() -> Map {
    let mut grid: Vec<Vec<Box<dyn Block>>> = Vec::new();
    grid.push(vec![ Box::new(RoadBlock::new(10)) ]); // (0,0)
    let control = Control::without_traffic(false);   // policy: AnyVehicle (cars allowed)
    grid.push(vec![ Box::new(BridgeBlock::new(11, control, MyMutex::new())) ]); // (0,1)
    grid.push(vec![ Box::new(RoadBlock::new(12)) ]); // (0,2)
    Map::build_custom(grid)
}


#[cfg(test)]
mod nuclear_plant_tests {
    use super::*;
    use mypthreads::mythread::mythread::ThreadId;

    use crate::city::supply_kind::SupplyKind;
    use crate::cityblock::nuclearplant::plant_status::PlantStatus;
    use crate::cityblock::nuclearplant::NuclearPlantBlock;
    use crate::vehicle::cargotruck::CargoTruck;
    use crate::vehicle::vehicle::{Vehicle, MoveIntent};

    // Helpers del usuario (ajusta el path si están en otro módulo)
    use crate::vehicle::tests::traffic_tests::c;

    // Pequeño helper para avanzar N frames (cada frame = advance_time(1))
    fn advance_frames(plant: &mut NuclearPlantBlock, frames: usize) {
        for _ in 0..frames {
            let _ = plant.advance_time(1);
        }
    }

    
    #[test]
    fn plant_advances_state_every_interval() {
        let mut plant = NuclearPlantBlock::new(/*id*/ 1, /*deadline*/ 100, /*update_interval_ms*/ 30);

        assert_eq!(plant.plant_status, PlantStatus::Ok);

        // Ok -> AtRisk
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // AtRisk -> Critical
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::Critical);

        // Critical -> Boom
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::Boom);

        // En Boom permanece inerte
        advance_frames(&mut plant, 300);
        assert_eq!(plant.plant_status, PlantStatus::Boom);
    }

    #[test]
    fn plant_enqueues_requirements_when_entering_at_risk_once() {
        let mut plant = NuclearPlantBlock::new(1, 100, 30);

        // Entra a AtRisk
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // Debe haber 2 requerimientos: NuclearMaterial y Water
        assert_eq!(plant.requires.len(), 2);
        assert!(plant.requires.iter().any(|r| r.kind == SupplyKind::NuclearMaterial));
        assert!(plant.requires.iter().any(|r| r.kind == SupplyKind::Water));

        // No debe duplicar requerimientos en frames subsiguientes
        advance_frames(&mut plant, 10);
        assert_eq!(plant.requires.len(), 2);
    }

    #[test]
    fn reaching_boom_clears_requirements_and_scheduled() {
        let mut plant = NuclearPlantBlock::new(1, 100, 30);

        // Ok -> AtRisk (crea requerimientos)
        advance_frames(&mut plant, 30);
        assert_eq!(plant.requires.len(), 2);

        // AtRisk -> Critical
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::Critical);

        // Critical -> Boom (aquí se debe limpiar todo)
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::Boom);
        assert!(plant.requires.is_empty(), "Al llegar a Boom se limpian los requerimientos");
        assert!(plant.scheduled_kinds.is_empty(), "Al llegar a Boom se limpia la lista de programados");
    }

}