#[cfg(test)]
mod basics {
    use crate::cityblock::Block;
    use crate::cityblock::block_type::BlockType::{Dock, Road, Shops, Water};
    use crate::cityblock::dock::DockBlock;
    use crate::cityblock::transport_policy::TransportPolicy::{Car, NoVehicles, Ship};
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::shopblock::shop::Shop;
    use crate::cityblock::shopblock::ShopBlock;
    use crate::cityblock::water::WaterBlock;

    #[test]
    fn test_create_road_block() {
        let id: usize = 1;
        let road = RoadBlock::new(id);

        assert_eq!(&id, road.get_id());
        assert_eq!(&Road, road.get_type());
        assert_eq!(&Car, road.get_policy())
    }

    #[test]
    fn test_create_shop_block() {
        let id: usize = 3;
        let mut shops: Vec<Shop> = Vec::new();
        shops.insert(0, Shop::new("Lolitas".parse().unwrap()));
        shops.insert(1, Shop::new("Super Gloria".parse().unwrap()));
        shops.insert(2, Shop::new("Ferretería Ocre".parse().unwrap()));
        let shops_copy = shops.clone();
        let shop_block = ShopBlock::new(id, shops);

        assert_eq!(&id, shop_block.get_id());
        assert_eq!(&Shops, shop_block.get_type());
        assert_eq!(&NoVehicles, shop_block.get_policy());
        assert_eq!(shops_copy.get(2), shop_block.get_shops().get(2));
    }

    #[test]
    fn test_create_water_block() {
        let id: usize = 1;
        let water = WaterBlock::new(id);

        assert_eq!(&id, water.get_id());
        assert_eq!(&Ship, water.get_policy());
        assert_eq!(&Water, water.get_type());
    }

    #[test]
    fn test_create_dock_block() {
        let id: usize = 1;
        let dock = DockBlock::new(id);

        assert_eq!(&id, dock.get_id());
        assert_eq!(&Dock, dock.get_type());
        assert_eq!(&Ship, dock.get_policy());
    }
}

#[cfg(test)]
mod nuclear {
    use std::rc::Rc;
    use crate::cityblock::map::Map;
    use crate::cityblock::Block;
    use crate::cityblock::block_type::BlockType::NuclearPlant;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::nuclearplant::NuclearPlantBlock;
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::transport_policy::TransportPolicy::NoVehicles;
    use crate::cityblock::water::WaterBlock;

    #[test]
    fn test_create_nuclear() {
        let id: usize = 33;

        let mut custom: Vec<Vec<Box<dyn Block>>> = Vec::new();
        let mut arr1: Vec<Box<dyn Block>> =
            vec![Box::new(RoadBlock::new(1)), Box::new(RoadBlock::new(2))];
        let mut arr2: Vec<Box<dyn Block>> =
            vec![Box::new(RoadBlock::new(id)), Box::new(WaterBlock::new(3))];

        custom.push(arr1);
        custom.push(arr2);
        let mut city_map = Rc::new(Map::build_custom(custom));

        let plant_coord = Coord::new(1, 0);
        let dl_policy: usize = 35;
        let update_interval: usize = 33;

        let mut plant = NuclearPlantBlock::new(id, dl_policy, update_interval);
        city_map.grid[plant_coord.y as usize][plant_coord.x as usize] = Box::new(plant);

        city_map.grid[plant_coord.y as usize][plant_coord.x as usize].;
        let test = city_map.grid[plant_coord.x as usize][plant_coord.y as usize].get_type();
        plant.attach_map(city_map.clone());
        assert_eq!(test, &NuclearPlant);
    }

    #[test]
    #[should_panic(expected = "Nuclear plant: can´t override")]
    fn test_panic_on_create_nuclear_by_id_mismatch() {
        todo!()
    }
}