#[cfg(test)]
mod basics {
    use crate::cityblock::Block;
    use crate::cityblock::block_type::BlockType::{Dock, NuclearPlant, Road, Shops, Water};
    use crate::cityblock::dock::DockBlock;
    use crate::cityblock::nuclearplant::NuclearPlantBlock;
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

    #[test]
    fn test_create_nuclear_block() {
        let id: usize = 1;
        let dl_policy: usize = 35;
        let update_interval: usize = 33;
        let plant = NuclearPlantBlock::new(id, dl_policy, update_interval);
        assert_eq!(plant.get_type(), &NuclearPlant);
        assert_eq!(plant.get_id(), &id);
        assert_eq!(plant.get_policy(), &NoVehicles);
    }
}

#[cfg(test)]
mod bridge {
    use mypthreads::mythread::mymutex::MyMutex;
    use mypthreads::mythread::mypthread::MyPThread;
    use crate::cityblock::Block;
    use crate::cityblock::block_type::BlockType::Bridge;
    use crate::cityblock::bridge::BridgeBlock;
    use crate::cityblock::bridge::control::Control;
    use crate::cityblock::transport_policy::TransportPolicy::{AnyVehicle, NoVehicles};

    #[test]
    fn test_create_control_with_traffic() {
        let in_int: usize = 10;
        let out_int: usize = 12;
        let control = Control::with_traffic(in_int, out_int);

        assert_eq!(false, control.has_yield);
        assert_eq!(in_int, control.in_traffic_light.unwrap().update_interval_ms);
        assert_eq!(out_int, control.out_traffic_light.unwrap().update_interval_ms);
    }
    #[test]
    fn test_create_control_without_traffic_nor_yield() {
        let control = Control::without_traffic(false);

        assert_eq!(false, control.has_yield);
        assert_eq!(None, control.in_traffic_light);
        assert_eq!(None, control.out_traffic_light);
    }
    #[test]
    fn test_create_control_with_yield() {
        let control = Control::without_traffic(true);
        assert_eq!(true, control.has_yield);
        assert_eq!(None, control.in_traffic_light);
        assert_eq!(None, control.out_traffic_light);

    }
    
    #[test]
    fn test_create_bridge() {
        let id = 1;
        let ctrl = Control::without_traffic(false);
        let mut mutex = MyMutex::new();
        unsafe {
            MyPThread::new().my_mutex_init(&mut mutex as *mut MyMutex, std::ptr::null());
        }
        
        let mut bridge = BridgeBlock::new(id, ctrl, mutex);
        
        assert_eq!(&id, bridge.get_id());
        assert_eq!(&Bridge, bridge.get_type());
        assert_eq!(&AnyVehicle, bridge.get_policy());
        assert!(bridge.mutex.is_some());
        
        if let Some(mut mute) = bridge.return_mutex(){
            unsafe { MyPThread::new().my_mutex_destroy(&mut mute as *mut MyMutex) ; }
        } else { panic!("Bridge isn't returning the mutex") };
        
        assert!(bridge.mutex.is_none());
    }
}