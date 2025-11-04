#[cfg(test)]
mod create_vehicle {
    use mypthreads::mythread::mythread::ThreadId;
    use crate::city::supply_kind::SupplyKind::NuclearMaterial;
    use crate::cityblock::Block;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::map::Map;
    use crate::cityblock::nuclearplant::supply_spec::SupplySpec;
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::water::WaterBlock;
    use crate::vehicle::ambulance::Ambulance;
    use crate::vehicle::car::Car;
    use crate::vehicle::cargotruck::CargoTruck;
    use crate::vehicle::ship::Ship;
    use crate::vehicle::vehicle::Vehicle;
    use crate::vehicle::vehicle_type::VehicleType::{AmbulanceE, CarE, ShipE, TruckE};

    fn basic_road() -> Vec<Box<dyn Block>> {
        let arr1:Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(0)),
            Box::new(RoadBlock::new(1)),
            Box::new(RoadBlock::new(2)),
        ];
        arr1
    }
    fn basic_river() -> Vec<Box<dyn Block>> {
        let arr1:Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(0)),
            Box::new(WaterBlock::new(1)),
            Box::new(WaterBlock::new(2)),

        ];
        arr1
    }
    #[test]
    fn test_create_car() {
        let tid: ThreadId = 0;
        let mut carrr = Car::new(Coord::new(0,0), Coord::new(2,0));

        let mut custom = Vec::new();

        custom.push(basic_road());
        let map = Map::build_custom(custom);
        carrr.initialize(&map, tid);
        
        let expected = vec![Coord::new(0,0), Coord::new(1,0), Coord::new(2,0)];
        assert!(carrr.base.path.is_some());
        assert_eq!(expected, carrr.base.path.unwrap());
        assert_eq!(CarE, carrr.base.vehicle_type);
    }

    #[test]
    fn test_create_ambulance() {
        let tid: ThreadId = 0;
        let mut ambbbb = Ambulance::new(Coord::new(0, 0), Coord::new(2, 0));

        let mut custom = Vec::new();
        custom.push(basic_road());
        let map = Map::build_custom(custom);
        ambbbb.initialize(&map, tid);

        let expected = vec![Coord::new(0,0), Coord::new(1,0), Coord::new(2,0)];
        assert!(ambbbb.base.path.is_some());
        assert_eq!(expected, ambbbb.base.path.unwrap());
        assert_eq!(AmbulanceE, ambbbb.base.vehicle_type);
    }

    #[test]
    fn test_create_ship() {
        let tid: ThreadId = 0;
        let mut boat = Ship::new(Coord::new(0, 0), Coord::new(2, 0), 1);

        let mut custom = Vec::new();
        custom.push(basic_river());
        let map = Map::build_custom(custom);
        boat.initialize(&map, tid);

        let expected = vec![Coord::new(0,0), Coord::new(1,0), Coord::new(2,0)];
        assert!(boat.base.path.is_some());
        assert_eq!(expected, boat.base.path.unwrap());
        assert_eq!(ShipE, boat.base.vehicle_type);
    }

    #[test]
    fn test_create_truck() {
        let tid: ThreadId = 0;
        let supply = SupplySpec { kind: NuclearMaterial, dead_line: 20, time_passed_ms: 0};
        let mut brrrum = CargoTruck::new(Coord::new(0, 0), Coord::new(2, 0),
                                         1, supply);

        let mut custom = Vec::new();
        custom.push(basic_road());
        let map = Map::build_custom(custom);
        brrrum.initialize(&map, tid);

        let expected = vec![Coord::new(0,0), Coord::new(1,0), Coord::new(2,0)];
        assert!(brrrum.base.path.is_some());
        assert_eq!(expected, brrrum.base.path.unwrap());
        assert_eq!(TruckE, brrrum.base.vehicle_type);
        assert_eq!(NuclearMaterial, brrrum.cargo.kind);
    }
}