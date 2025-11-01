#[cfg(test)]
mod complex_routing {
    use mypthreads::mythread::mythread::ThreadId;
    use crate::cityblock::Block;
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
        let mut carr = Car::new(Coord::new(0, 3), Coord::new(3, 3), 1);
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
}