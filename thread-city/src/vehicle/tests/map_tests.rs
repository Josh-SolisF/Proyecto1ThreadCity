#[cfg(test)]
mod complex_routing {
    use mypthreads::mythread::mymutex::MyMutex;
    use mypthreads::mythread::mythread::ThreadId;
    use crate::cityblock::Block;
    use crate::cityblock::bridge::BridgeBlock;
    use crate::cityblock::bridge::control::Control;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::dock::DockBlock;
    use crate::cityblock::map::Map;
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::shopblock::ShopBlock;
    use crate::cityblock::water::WaterBlock;
    use crate::vehicle::car::Car;
    use crate::vehicle::ship::Ship;
    use crate::vehicle::vehicle::Vehicle;

    #[test]
    fn test_create_road_vehicle_multi_option() {
        let mut custom = Vec::new();
        let arr0: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(0)),
            Box::new(RoadBlock::new(1)),
            Box::new(RoadBlock::new(2)),
            Box::new(RoadBlock::new(3)),
        ];
        custom.push(arr0);
        let arr1: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(4)),
            Box::new(WaterBlock::new(5)),
            Box::new(WaterBlock::new(6)),
            Box::new(RoadBlock::new(7)),
        ];
        custom.push(arr1);
        let arr2: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(8)),
            Box::new(RoadBlock::new(9)),
            Box::new(RoadBlock::new(10)),
            Box::new(RoadBlock::new(11)),
        ];
        custom.push(arr2);
        let arr3: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(12)),
            Box::new(WaterBlock::new(13)),
            Box::new(WaterBlock::new(14)),
            Box::new(ShopBlock::new(15, Vec::new())),
        ];
        custom.push(arr3);

        let tid: ThreadId = 0;
        let mut carr = Car::new(Coord::new(0, 3), Coord::new(3, 3));
        let map = Map::build_custom(custom);
        carr.initialize(&map, tid);

        let expected =
            vec![Coord::new(0,3), Coord::new(0,2),
                 Coord::new(1,2), Coord::new(2,2), Coord::new(3,2),
                 Coord::new(3,3)];

        assert!(carr.base.path.is_some());
        assert_eq!(expected, carr.base.path.unwrap());
    }

    #[test]
    fn test_create_water_vehicle() {
        let mut custom = Vec::new();
        let arr0: Vec<Box<dyn Block>> = vec![
            Box::new(DockBlock::new(0)),
            Box::new(WaterBlock::new(1)),
            Box::new(WaterBlock::new(2)),
            Box::new(WaterBlock::new(3)),
        ];
        custom.push(arr0);
        let arr1: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(4)),
            Box::new(RoadBlock::new(5)),
            Box::new(RoadBlock::new(6)),
            Box::new(WaterBlock::new(7)),
        ];
        custom.push(arr1);
        let arr2: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(8)),
            Box::new(WaterBlock::new(9)),
            Box::new(WaterBlock::new(10)),
            Box::new(WaterBlock::new(11)),
        ];
        custom.push(arr2);
        let arr3: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(12)),
            Box::new(WaterBlock::new(13)),
            Box::new(RoadBlock::new(14)),
            Box::new(RoadBlock::new(15)),
        ];
        custom.push(arr3);

        let tid: ThreadId = 0;
        let mut boat = Ship::new(Coord::new(1, 3), Coord::new(0, 0), 1);

        let map = Map::build_custom(custom);
        boat.initialize(&map, tid);
        let expected =
                        vec![Coord::new(1,3),
                            Coord::new(1,2), Coord::new(2,2), Coord::new(3,2),
                            Coord::new(3,1),
                            Coord::new(3,0), Coord::new(2,0), Coord::new(1,0), Coord::new(0,0),];

        assert!(boat.base.path.is_some());
        assert_eq!(expected, boat.base.path.unwrap());
    }

    #[test]
    fn test_bridge_intersection() {
        let mut custom = Vec::new();
        let arr0: Vec<Box<dyn Block>> = vec![
            Box::new(RoadBlock::new(0)),
            Box::new(RoadBlock::new(1)),
            Box::new(WaterBlock::new(2)),
        ];
        custom.push(arr0);

        let mutex= MyMutex::new();
        let control= Control::without_traffic(false);
        let arr1: Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(3)),
            Box::new(BridgeBlock::new(4, control, mutex)),
            Box::new(WaterBlock::new(5)),
        ];
        custom.push(arr1);

        let arr2: Vec<Box<dyn Block>> = vec![
            Box::new(ShopBlock::new(6, Vec::new())),
            Box::new(RoadBlock::new(7)),
            Box::new(ShopBlock::new(8, Vec::new())),
        ];
        custom.push(arr2);
        let map = Map::build_custom(custom);

        let car_tid: ThreadId = 0;
        let mut carr = Car::new(Coord::new(1, 0), Coord::new(1, 2));
        carr.initialize(&map, car_tid);

        let boat_tid: ThreadId = 1;
        let mut boat = Ship::new(Coord::new(2, 1), Coord::new(0, 1), 1);
        boat.initialize(&map, boat_tid);

        let boat_expected_path =
            vec![Coord::new(2,1), Coord::new(1,1), Coord::new(0,1)];
        assert!(boat.base.path.is_some());
        assert_eq!(boat_expected_path, boat.base.path.unwrap());

        let car_expected_path = vec![
            Coord::new(1,0), Coord::new(1,1), Coord::new(1,2)];

        assert!(carr.base.path.is_some());
        assert_eq!(car_expected_path, carr.base.path.unwrap());
    }

    #[test]
    fn test_bridge_limitation() {
        let mut custom = Vec::new();

        let mutex= MyMutex::new();
        let bad_control= Control::without_traffic(true);
        let arr0: Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(0)),
            Box::new(BridgeBlock::new(1, bad_control, mutex)),
            Box::new(WaterBlock::new(2)),
        ];
        custom.push(arr0);

        let mutex2= MyMutex::new();
        let good_control= Control::without_traffic(false);
        let arr1: Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(3)),
            Box::new(BridgeBlock::new(4, good_control, mutex2)),
            Box::new(WaterBlock::new(5)),
        ];
        custom.push(arr1);

        let arr2: Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(6)),
            Box::new(WaterBlock::new(7)),
            Box::new(WaterBlock::new(8)),
        ];
        custom.push(arr2);
        let map = Map::build_custom(custom);

        let tid: ThreadId = 1;
        let mut boat = Ship::new(Coord::new(2, 0), Coord::new(0, 0), 1);
        boat.initialize(&map, tid);

        let expected = vec![
            Coord::new(2, 0),
            Coord::new(2,1), Coord::new(1,1), Coord::new(0,1),
            Coord::new(0, 0)];
        assert!(boat.base.path.is_some());
        assert_eq!(expected, boat.base.path.unwrap());
    }
}