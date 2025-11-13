#[cfg(test)]
mod tests {
    use mypthreads::mythread::mymutex::MyMutex;
    use crate::cityblock::Block;
    use crate::cityblock::bridge::BridgeBlock;
    use crate::cityblock::bridge::control::Control;
    use crate::cityblock::dock::DockBlock;
    use crate::cityblock::map::Map;
    use crate::cityblock::nuclearplant::NuclearPlantBlock;
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::shopblock::ShopBlock;
    use crate::cityblock::water::WaterBlock;

    #[test]
    fn test_map_custom_creation_with_all_blocks() {
        let mut custom = Vec::new();
        let arr1:Vec<Box<dyn Block>> = vec![
            Box::new(WaterBlock::new(0)),
            Box::new(RoadBlock::new(1)),
            Box::new(DockBlock::new(2)),
        ];
        custom.push(arr1);
        let shops = Vec::new();
        let control = Control::without_traffic(false);
        let arr2:Vec<Box<dyn Block>> = vec![
            Box::new(BridgeBlock::new(3, control, MyMutex::new())),
            Box::new(ShopBlock::new(4, shops)),
            Box::new(NuclearPlantBlock::new(5, 12, 15)),
        ];
        custom.push(arr2);
        let mut map = Map::build_custom(custom);

        assert_eq!(3, map.width);
        assert_eq!(2, map.height);
        assert!(
            map.grid[0][0].as_any().downcast_ref::<WaterBlock>().is_some()
        );
        assert!(
            map.grid[0][1].as_any().downcast_ref::<RoadBlock>().is_some()
        );
        assert!(
            map.grid[0][2].as_any().downcast_ref::<DockBlock>().is_some()
        );
        assert!(
            map.grid[1][0].as_any().downcast_ref::<BridgeBlock>().is_some()
        );
        assert!(
            map.grid[1][1].as_any().downcast_ref::<ShopBlock>().is_some()
        );
        assert!(
            map.grid[1][2].as_any().downcast_ref::<NuclearPlantBlock>().is_some()
        );
        assert!(
            map.grid[1][2].as_any().downcast_ref::<WaterBlock>().is_none()
        );
        println!("El mapa de 2 filas y 3 columnas se genero correctamente, además todos \
        los bloques pueden ser reconocidos como su tipo especifico");
    }
}