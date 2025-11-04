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
    fn spawn_trucks_creates_one_per_pending_kind_and_marks_scheduled() {
        let mut plant = NuclearPlantBlock::new(1, 100, 30);
        let plant_coord = c(0, 2);

        // Entra a AtRisk y crea requerimientos
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // Crea camiones; origen fijo por ahora (luego será aleatorio)
        let trucks = plant.spawn_trucks_for_pending_requirements(plant_coord, |_req| c(0, 0));
        assert_eq!(trucks.len(), 2, "Debe crear 2 camiones (uno por cada requerimiento)");

        // scheduled_kinds debe registrar ambos tipos
        assert!(plant.scheduled_kinds.contains(&SupplyKind::NuclearMaterial));
        assert!(plant.scheduled_kinds.contains(&SupplyKind::Water));

        // Una segunda llamada no debe crear duplicados (ya están programados)
        let trucks2 = plant.spawn_trucks_for_pending_requirements(plant_coord, |_req| c(0, 0));
        assert!(trucks2.is_empty());
    }

    #[test]
    fn commit_delivery_requires_both_to_raise_state_from_at_risk() {
        let mut plant = NuclearPlantBlock::new(1, 100, 30);
        let plant_coord = c(0, 2);

        // Entra a AtRisk y genera pedidos
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // Crea ambos camiones
        let mut trucks = plant.spawn_trucks_for_pending_requirements(plant_coord, |_req| c(0, 0));
        assert_eq!(trucks.len(), 2);

        // Entrega solo uno
        let t0 = trucks.pop().unwrap();
        plant.commit_delivery(&t0);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk, "Con una sola entrega no debe subir de estado");
        assert_eq!(plant.requires.len(), 1, "Debe quedar un requerimiento pendiente");

        // Entrega el segundo
        let t1 = trucks.pop().unwrap();
        plant.commit_delivery(&t1);
        assert_eq!(plant.plant_status, PlantStatus::Ok, "Con ambas entregas debe subir a Ok");
        assert!(plant.requires.is_empty());
    }

    #[test]
    fn commit_delivery_from_critical_raises_to_at_risk_not_ok() {
        let mut plant = NuclearPlantBlock::new(1, 100, 30);
        let plant_coord = c(0, 2);

        // Ok -> AtRisk (crea requerimientos)
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // AtRisk -> Critical (sin entregar, los requerimentos siguen)
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::Critical);
        assert_eq!(plant.requires.len(), 2);

        // Crea camiones ahora (si no estaban programados ya)
        let mut trucks = plant.spawn_trucks_for_pending_requirements(plant_coord, |_req| c(0, 0));

        // Entrega ambos
        for t in &trucks {
            plant.commit_delivery(t);
        }

        // Debe subir de Critical -> AtRisk (no salta a Ok)
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);
        assert!(plant.requires.is_empty());
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

    #[test]
    fn truck_planned_by_plant_marks_bridge_as_special() {
        // Mapa: Road -> Bridge -> Road  (0,0) -> (0,1) -> (0,2)
        let map = build_column_map_with_bridge();
        let plant_coord = c(0, 2);

        let mut plant = NuclearPlantBlock::new(1, 100, 30);
        advance_frames(&mut plant, 30);
        assert_eq!(plant.plant_status, PlantStatus::AtRisk);

        // Que la planta cree camiones; tomamos uno
        let mut trucks = plant.spawn_trucks_for_pending_requirements(plant_coord, |_req| c(0, 0));
        assert!(!trucks.is_empty());
        let mut truck = trucks.remove(0);

        // Inicializamos y verificamos el hook de puente
        let tid: ThreadId = 42;
        truck.initialize(&map, tid);

        match truck.base().plan_next(&map) {
            MoveIntent::NextIsBridge { coord } => assert_eq!(coord, c(0, 1)),
            other => panic!("Esperaba NextIsBridge((0,1)), obtuve {:?}", other),
        }
    }
}