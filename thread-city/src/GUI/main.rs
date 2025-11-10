// src/main.rs (o src/bin/gtk_view.rs)
use std::cell::RefCell;
use std::rc::Rc;
use glib::Continue;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::cairo;
use glib::source::timeout_add_local;
use mypthreads::mythread::mymutex::MyMutex;
use crate::cityblock::Block;
use crate::cityblock::coord::Coord;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::bridge::BridgeBlock;
use crate::cityblock::bridge::control::Control;
use crate::cityblock::dock::DockBlock;
use crate::cityblock::map::Map;
use crate::cityblock::nuclearplant::NuclearPlantBlock;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;
use crate::cityblock::road::RoadBlock;
use crate::cityblock::shopblock::shop::Shop;
use crate::cityblock::shopblock::ShopBlock;
use crate::cityblock::water::WaterBlock;

//  UI Hooks: cómo la GUI consulta 
#[derive(Clone)]
pub struct UiHooks {
    world_size: Rc<dyn Fn() -> (i16, i16)>,
    block_type_at: Rc<dyn Fn(Coord) -> Option<BlockType>>,
    is_occupied: Rc<dyn Fn(Coord) -> bool>,
    plant_status_at: Rc<RefCell<dyn FnMut(Coord) -> Option<PlantStatus>>>, 
    tick: Rc<RefCell<dyn FnMut()>>, // avanza 1 frame
}


fn color_for_block(bt: &BlockType) -> (f64, f64, f64) {
    use crate::cityblock::block_type::BlockType::*;
    match bt {
        Road          => (0.00, 0.00, 0.00), // negro
        Bridge        => (0.60, 0.80, 1.00), // (azulado) para distinguir de carretera
        Shop          => (1.00, 0.55, 0.00), // naranja (tipo #FF8C00)
        Dock          => (0.59, 0.29, 0.00), // marrón (tipo #964B00)
        Water         => (0.00, 0.50, 1.00), // azul (tipo #0080FF)
        NuclearPlant  => (0.00, 0.70, 0.20), // verde 
        _             => (0.85, 0.85, 0.85), // por defecto gris claro
    }
}



fn border_for_plant_status(ps: PlantStatus) -> (f64, f64, f64) {
    use crate::cityblock::nuclearplant::plant_status::PlantStatus::*;
    match ps {
        Ok => (0.0, 0.6, 0.0),
        AtRisk => (0.95, 0.75, 0.20),
        Critical => (0.95, 0.15, 0.15),
        Boom => (0.15, 0.15, 0.15),
    }
}

fn draw_world(area: &DrawingArea, cr: &cairo::Context, hooks: &UiHooks) {

    // Geometría del área en píxeles lógicos (GTK4 usa logical coords).
    // allocation() da el rectángulo asignado al widget.

    let alloc = area.allocation();
    let w_px = alloc.width() as f64;
    let h_px = alloc.height() as f64;
//Dimensiones del mundo (en celdas).
    let (w_cells, h_cells) = (hooks.world_size)();
    let w_cells = w_cells.max(1) as f64;
    let h_cells = h_cells.max(1) as f64;
//tamaño de celda: intenta que todas las celdas quepan y sean cuadradas.
    let cell_w = (w_px / w_cells).floor();
    let cell_h = (h_px / h_cells).floor();
    let cell = cell_w.min(cell_h).max(1.0);
    //Offsets para centrar el grid dentro del área
    let ox = (w_px - (cell * w_cells)).max(0.0) / 2.0;
    let oy = (h_px - (cell * h_cells)).max(0.0) / 2.0;

    // Fondo
    cr.set_source_rgb(0.12, 0.12, 0.12);
    cr.paint().unwrap();

    for y in 0..(h_cells as i16) {
        for x in 0..(w_cells as i16) {
            let coord = Coord::new(x, y);
            // ordenadas en píxeles para la esquina sup-izq de la celda.
            let x_px = ox + (x as f64) * cell;
            let y_px = oy + (y as f64) * cell;

            // // Si no hay bloque, usamos un gris medio
            if let Some(bt) = (hooks.block_type_at)(coord) {
                let (r, g, b) = color_for_block(&bt);
                cr.set_source_rgb(r, g, b);
            } else {
                cr.set_source_rgb(0.25, 0.25, 0.25);
            }
            //quede una "rejilla" sutil entre celdas y no se vean pegadas
            cr.rectangle(x_px, y_px, cell - 1.0, cell - 1.0);
            cr.fill().unwrap();

            // Borde por estado de planta (si aplica)
            if let Some(mut get_ps) = Some(hooks.plant_status_at.clone()) {
                if let Some(ps) = (get_ps.borrow_mut())(coord) {
                    let (br, bg, bb) = border_for_plant_status(ps);
                    cr.set_source_rgb(br, bg, bb);
                    cr.set_line_width(3.0);
                    cr.rectangle(x_px + 1.0, y_px + 1.0, cell - 3.0, cell - 3.0);
                    cr.stroke().unwrap();
                }
            }

            // Overlay si está ocupado: círculo rojo
            if (hooks.is_occupied)(coord) {
                cr.set_source_rgb(0.95, 0.20, 0.20);
                let cx = x_px + cell * 0.5;
                let cy = y_px + cell * 0.5;
                let r = (cell * 0.25).max(3.0);
                cr.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
                cr.fill().unwrap();
            }
        }
    }
}

pub(crate) fn build_ui(app: &Application, hooks: UiHooks) {
    let win = ApplicationWindow::builder()
        .application(app)
        .title("City Traffic (GTK)")
        .default_width(900)
        .default_height(900)
        .build();

    let area = DrawingArea::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let hooks_for_draw = hooks.clone();
    area.set_draw_func(move |area, cr, _, _| {
        draw_world(area, cr, &hooks_for_draw);
    });

    win.set_child(Some(&area));
    win.show();

    // Timer de frames (33ms ~ 30 FPS). En cada tick, avanza 1 frame y repinta.
    let area_weak = area.downgrade();
    let tick_cb = hooks.tick.clone();
    timeout_add_local(std::time::Duration::from_millis(33), move || {
        (tick_cb.borrow_mut())(); // avanza tu simulación 1 frame
        if let Some(area) = area_weak.upgrade() {
            area.queue_draw();
        }
        Continue(true)
    });
}

fn main() {
    let app = Application::builder()
        .application_id("com.joshuaS.citygtk")
        .build();

    app.connect_activate(|app| {
        let hooks = make_hooks_from_world();
        build_ui(app, hooks);
    });

    app.run();
}



pub(crate) fn make_hooks_from_world() -> UiHooks {
    use std::collections::{HashMap, HashSet};
    use std::cell::RefCell;
    use std::rc::Rc;


    use crate::cityblock::block_type::BlockType;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::nuclearplant::plant_status::PlantStatus;


    // Hay que reemplazar con el mapa de verdad


    let map: Rc<RefCell<Map>> = Rc::new(RefCell::new(map_25x25_with_all_blocks()));
    let occupancy = Rc::new(RefCell::new(HashSet::<(i16,i16)>::new())); // usa Coord si implementa Hash/Eq

    let world_size = {
        let map = Rc::clone(&map);
        Rc::new(move || -> (i16, i16) {
            let m = map.borrow();
            (m.width_cells(), m.height_cells())
        })
    };

    let block_type_at = {
        let map = Rc::clone(&map);
        Rc::new(move |coord: Coord| -> Option<BlockType> {
            map.borrow().block_type_at(coord)
        })
    };

    let is_occupied = {
        let occupancy = Rc::clone(&occupancy);
        Rc::new(move |coord: Coord| -> bool {
            // Se usa la tupla para que sea más sencillo, pero si no también podemos usar el hashmap:
            occupancy.borrow().contains(&(coord.x, coord.y))
        })
    };

    let plant_status_at = {
        let map = Rc::clone(&map);
        Rc::new(RefCell::new(move |coord: Coord| -> Option<PlantStatus> {

            map.borrow_mut().try_plant_status_at(coord)
        }))
    };

    let tick = {
        let map = Rc::clone(&map);
        let occupancy = Rc::clone(&occupancy);
        Rc::new(RefCell::new(move || {
            // Aquí iría toda la logica de la ciudad, basicametne cada tick
            let mut occ = occupancy.borrow_mut();
            occ.clear();
            // Marca, (0,0) como ocupado (usa tu handler real)
            occ.insert((0,0));
        }))
    };

    UiHooks {
        world_size,
        block_type_at,
        is_occupied,
        plant_status_at,
        tick,
    }
}
pub fn map_25x25_with_all_blocks() -> Map {
    let mut grid: Vec<Vec<Box<dyn Block>>> = Vec::with_capacity(25);

    // y = 0
    grid.push(vec![
        Box::new(RoadBlock::new(0)),
        Box::new(RoadBlock::new(1)),
        Box::new(RoadBlock::new(2)),
        Box::new(RoadBlock::new(3)),
        Box::new(RoadBlock::new(4)),
        Box::new(RoadBlock::new(5)),
        Box::new(RoadBlock::new(6)),
        Box::new(RoadBlock::new(7)),
        Box::new(RoadBlock::new(8)),
        Box::new(RoadBlock::new(9)),
        Box::new(ShopBlock::new(10,vec![Shop::new("TacoBell #1".parse().unwrap()), Shop::new("TacoBell #2".parse().unwrap())])),
        Box::new(ShopBlock::new(11,vec![Shop::new("KFC #1".parse().unwrap()), Shop::new("KFC #2".parse().unwrap())])),
        Box::new(ShopBlock::new(12,vec![Shop::new("BurgerKing #1".parse().unwrap()), Shop::new("BurgerKing #2".parse().unwrap())])),
        Box::new(ShopBlock::new(13,vec![Shop::new("McDonald #1".parse().unwrap()), Shop::new("McDonald #2".parse().unwrap())])),
        Box::new(RoadBlock::new(14)),
        Box::new(RoadBlock::new(15)),
        Box::new(RoadBlock::new(16)),
        Box::new(RoadBlock::new(17)),
        Box::new(ShopBlock::new(18, vec![Shop::new("Pizzeria Don Juan #1".parse().unwrap())])),
        Box::new(ShopBlock::new(19,vec![Shop::new("Barberia #1".parse().unwrap()), Shop::new("Barberia #2".parse().unwrap()), Shop::new("Barberia #3".parse().unwrap())])),
        Box::new(RoadBlock::new(20)),
        Box::new(RoadBlock::new(21)),
        Box::new(WaterBlock::new(22)),
        Box::new(WaterBlock::new(23)),
        Box::new(WaterBlock::new(24)),
        Box::new(WaterBlock::new(25)),
    ]);

    // y = 1
    grid.push(vec![
        Box::new(RoadBlock::new(26)),
        Box::new(ShopBlock::new(27,vec![Shop::new("NovaCinemas #1".parse().unwrap())])),
        Box::new(RoadBlock::new(28)),
        Box::new(ShopBlock::new(29,vec![Shop::new("Cinemark #1".parse().unwrap())])),
        Box::new(RoadBlock::new(30)),
        Box::new(ShopBlock::new(31,vec![Shop::new("Pollo Granjero #1".parse().unwrap()), Shop::new("Pollo Granjero #2".parse().unwrap())])),
        Box::new(ShopBlock::new(32,vec![Shop::new("Pizza hut #1".parse().unwrap()), Shop::new("Pizza Hut #2".parse().unwrap())])),
        Box::new(ShopBlock::new(33,vec![Shop::new("Dominoes #1".parse().unwrap()), Shop::new("Dominoes #1".parse().unwrap()),Shop::new("Dominoes #3".parse().unwrap())])),
        Box::new(RoadBlock::new(34)),
        Box::new(RoadBlock::new(35)),
        Box::new(RoadBlock::new(36)),
        Box::new(RoadBlock::new(37)),
        Box::new(ShopBlock::new(38,vec![Shop::new("Ready Pizza #1".parse().unwrap())])),
        Box::new(ShopBlock::new(39,vec![Shop::new("Cesaers Pizza #1".parse().unwrap())])),
        Box::new(ShopBlock::new(40,vec![Shop::new("Papas John #1".parse().unwrap())])),
        Box::new(ShopBlock::new(41,vec![Shop::new(" Pizza a la leña #1".parse().unwrap())])),
        Box::new(RoadBlock::new(42)),
        Box::new(ShopBlock::new(43,vec![Shop::new("Barberia Manolo".parse().unwrap()), Shop::new("Barberia Paco".parse().unwrap()), Shop::new("Barberia #3".parse().unwrap())])),
        Box::new(ShopBlock::new(44, vec![Shop::new("POPS".parse().unwrap())])),
        Box::new(RoadBlock::new(45)),
        Box::new(RoadBlock::new(46)),
        Box::new(WaterBlock::new(47)),
        Box::new(WaterBlock::new(48)),
        Box::new(WaterBlock::new(49)),
        Box::new(WaterBlock::new(50)),
    ]);
    // y=2
    grid.push(vec![
        Box::new(RoadBlock::new(51)),
        Box::new(RoadBlock::new(52)),
        Box::new(RoadBlock::new(53)),
        Box::new(RoadBlock::new(54)),
        Box::new(RoadBlock::new(55)),
        Box::new(RoadBlock::new(56)),
        Box::new(ShopBlock::new(57,vec![Shop::new("iCon #1".parse().unwrap()), Shop::new("TacoBell #2".parse().unwrap())])),
        Box::new(RoadBlock::new(58)),
        Box::new(RoadBlock::new(59)),
        Box::new(RoadBlock::new(60)),
        Box::new(ShopBlock::new(61,vec![Shop::new("Huawei #1".parse().unwrap()), Shop::new("KFC #2".parse().unwrap())])),
        Box::new(RoadBlock::new(62)),
        Box::new(RoadBlock::new(63)),
        Box::new(RoadBlock::new(64)),
        Box::new(RoadBlock::new(65)),
        Box::new(RoadBlock::new(66)),
        Box::new(RoadBlock::new(67)),
        Box::new(RoadBlock::new(68)),
        Box::new(ShopBlock::new(69, vec![Shop::new("Samsung #1".parse().unwrap())])),
        Box::new(ShopBlock::new(70,vec![Shop::new("Xiaomi #1".parse().unwrap()), Shop::new("Xiaomi #2".parse().unwrap()), Shop::new("Xiaomi #3".parse().unwrap())])),
        Box::new(RoadBlock::new(71)),
        Box::new(WaterBlock::new(72)),
        Box::new(WaterBlock::new(73)),
        Box::new(WaterBlock::new(74)),
        Box::new(WaterBlock::new(75)),
    ]);
    // y = 3
    grid.push(vec![
        Box::new(RoadBlock::new(76)),
        Box::new(ShopBlock::new(77,vec![Shop::new("La estacion #1".parse().unwrap())])),
        Box::new(RoadBlock::new(78)),
        Box::new(ShopBlock::new(79,vec![Shop::new("Pinitos #1".parse().unwrap())])),
        Box::new(ShopBlock::new(80,vec![Shop::new("Dos Pinos #1".parse().unwrap())])),
        Box::new(RoadBlock::new(81)),
        Box::new(RoadBlock::new(82)),
        Box::new(RoadBlock::new(83)),
        Box::new(ShopBlock::new(84,vec![Shop::new("Municipalidad #1".parse().unwrap()), Shop::new("Ebais #2".parse().unwrap())])),
        Box::new(RoadBlock::new(85)),
        Box::new(RoadBlock::new(86)),
        Box::new(RoadBlock::new(87)),
        Box::new(RoadBlock::new(88)),
        Box::new(ShopBlock::new(89,vec![Shop::new("Walmart #1".parse().unwrap())])),
        Box::new(ShopBlock::new(90,vec![Shop::new("Megasuper #1".parse().unwrap())])),
        Box::new(ShopBlock::new(91,vec![Shop::new("HiperMas #1".parse().unwrap())])),
        Box::new(RoadBlock::new(92)),
        Box::new(ShopBlock::new(93,vec![Shop::new("Farmavalue #1".parse().unwrap())])),
        Box::new(ShopBlock::new(94,vec![Shop::new("La confianza #1".parse().unwrap())])),
        Box::new(ShopBlock::new(95,vec![Shop::new("Dunkin #1".parse().unwrap())])),
        Box::new(RoadBlock::new(96)),
        Box::new(WaterBlock::new(97)),
        Box::new(WaterBlock::new(98)),
        Box::new(WaterBlock::new(99)),
        Box::new(WaterBlock::new(100)),
    ]);

    // y = 4
    grid.push(vec![
        Box::new(RoadBlock::new(101)),
        Box::new(ShopBlock::new(102,vec![Shop::new("Super baterias #1".parse().unwrap())])),
        Box::new(RoadBlock::new(103)),
        Box::new(RoadBlock::new(104)),
        Box::new(RoadBlock::new(105)),
        Box::new(RoadBlock::new(106)),
        Box::new(ShopBlock::new(107,vec![Shop::new("Hamburguesas pirata #1".parse().unwrap())])),
        Box::new(RoadBlock::new(108)),
        Box::new(RoadBlock::new(109)),
        Box::new(RoadBlock::new(110)),
        Box::new(ShopBlock::new(111,vec![Shop::new("Space pizza #1".parse().unwrap()), Shop::new("Space pizza #2".parse().unwrap())])),
        Box::new(RoadBlock::new(112)),
        Box::new(ShopBlock::new(113,vec![Shop::new("Video #1".parse().unwrap())])),
        Box::new(ShopBlock::new(114,vec![Shop::new("Iglesia #1".parse().unwrap())])),
        Box::new(RoadBlock::new(115)),
        Box::new(RoadBlock::new(116)),
        Box::new(RoadBlock::new(117)),
        Box::new(RoadBlock::new(118)),
        Box::new(RoadBlock::new(119)),
        Box::new(RoadBlock::new(120)),
        Box::new(RoadBlock::new(121)),
        Box::new(WaterBlock::new(122)),
        Box::new(WaterBlock::new(123)),
        Box::new(WaterBlock::new(124)),
        Box::new(WaterBlock::new(125)),
    ]);


    // y = 5
    grid.push(vec![
        Box::new(RoadBlock::new(126)),
        Box::new(ShopBlock::new(127,vec![Shop::new("La casona #1".parse().unwrap())])),
        Box::new(ShopBlock::new(128,vec![Shop::new("Vainilla #1".parse().unwrap())])),
        Box::new(ShopBlock::new(129,vec![Shop::new("BAC #1".parse().unwrap()), Shop::new("BAC #2".parse().unwrap())])),

        Box::new(RoadBlock::new(130)),
        Box::new(ShopBlock::new(131,vec![Shop::new("BCR #1".parse().unwrap())])),
        Box::new(ShopBlock::new(132,vec![Shop::new("Ciudad manga #1".parse().unwrap())])),
        Box::new(RoadBlock::new(133)),
        Box::new(ShopBlock::new(134,vec![Shop::new("Agonia #1".parse().unwrap())])),

        Box::new(RoadBlock::new(135)),
        Box::new(RoadBlock::new(136)),
        Box::new(RoadBlock::new(137)),
        Box::new(RoadBlock::new(138)),
        Box::new(RoadBlock::new(139)),
        Box::new(RoadBlock::new(140)),

        Box::new(RoadBlock::new(141)),
        Box::new(RoadBlock::new(142)),
        Box::new(RoadBlock::new(143)),
        Box::new(RoadBlock::new(144)),
        Box::new(ShopBlock::new(145,vec![Shop::new("Carnes castillo #1".parse().unwrap())])),
        Box::new(RoadBlock::new(146)),

        Box::new(WaterBlock::new(147)),
        Box::new(WaterBlock::new(148)),
        Box::new(WaterBlock::new(149)),
        Box::new(WaterBlock::new(150)),
    ]);



    // y = 6
    grid.push(vec![
        Box::new(RoadBlock::new(151)),
        Box::new(RoadBlock::new(152)),
        Box::new(RoadBlock::new(153)),
        Box::new(RoadBlock::new(154)),
        Box::new(RoadBlock::new(155)),

        Box::new(ShopBlock::new(156,vec![Shop::new("Negocio #1".parse().unwrap())])),
        Box::new(ShopBlock::new(157,vec![Shop::new("Arenas #1".parse().unwrap())])),

        Box::new(RoadBlock::new(158)),
        Box::new(ShopBlock::new(159,vec![Shop::new("Vertigo #1".parse().unwrap())])),
        Box::new(RoadBlock::new(160)),
        Box::new(RoadBlock::new(161)),
        Box::new(ShopBlock::new(162,vec![Shop::new("Yamaha #1".parse().unwrap())])),

        Box::new(RoadBlock::new(163)),
        Box::new(RoadBlock::new(164)),
        Box::new(RoadBlock::new(165)),
        Box::new(RoadBlock::new(166)),
        Box::new(RoadBlock::new(167)),
        Box::new(ShopBlock::new(168,vec![Shop::new("GEF #1".parse().unwrap())])),

        Box::new(RoadBlock::new(169)),
        Box::new(RoadBlock::new(170)),
        Box::new(RoadBlock::new(171)),

        Box::new(WaterBlock::new(172)),
        Box::new(WaterBlock::new(173)),
        Box::new(WaterBlock::new(174)),
        Box::new(WaterBlock::new(175)),
    ]);


    // y = 7
    grid.push(vec![
        Box::new(RoadBlock::new(176)),
        Box::new(ShopBlock::new(177,vec![Shop::new("Super #1".parse().unwrap())])),
        Box::new(RoadBlock::new(178)),
        Box::new(ShopBlock::new(179,vec![Shop::new("Super Mario #1".parse().unwrap())])),

        Box::new(RoadBlock::new(180)),
        Box::new(RoadBlock::new(181)),
        Box::new(RoadBlock::new(182)),
        Box::new(RoadBlock::new(183)),
        Box::new(RoadBlock::new(184)),
        Box::new(RoadBlock::new(185)),
        Box::new(RoadBlock::new(186)),

        Box::new(ShopBlock::new(187,vec![Shop::new("Arenas #1".parse().unwrap())])),

        Box::new(RoadBlock::new(188)),
        Box::new(ShopBlock::new(189,vec![Shop::new("Vertigo #1".parse().unwrap())])),
        Box::new(RoadBlock::new(190)),
        Box::new(RoadBlock::new(191)),
        Box::new(RoadBlock::new(192)),
        Box::new(RoadBlock::new(193)),
        Box::new(RoadBlock::new(194)),
        Box::new(RoadBlock::new(195)),

        Box::new(WaterBlock::new(196)),
        Box::new(WaterBlock::new(197)),
        Box::new(WaterBlock::new(198)),
        Box::new(WaterBlock::new(199)),
        Box::new(WaterBlock::new(200)),
    ]);
    // y = 8
    grid.push(vec![
        Box::new(RoadBlock::new(201)),
        Box::new(ShopBlock::new(202,vec![Shop::new("Super #1".parse().unwrap())])),
        Box::new(RoadBlock::new(203)),
        Box::new(RoadBlock::new(204)),
        Box::new(RoadBlock::new(205)),

        Box::new(ShopBlock::new(206,vec![Shop::new("Super  María #1".parse().unwrap())])),
        Box::new(ShopBlock::new(207,vec![Shop::new("Redentorista #1".parse().unwrap())])),

        Box::new(RoadBlock::new(208)),
        Box::new(ShopBlock::new(209,vec![Shop::new("Colegio #1".parse().unwrap())])),
        Box::new(ShopBlock::new(210,vec![Shop::new("Llobet #1".parse().unwrap())])),

        Box::new(RoadBlock::new(211)),
        Box::new(ShopBlock::new(212,vec![Shop::new("Kawe #1".parse().unwrap())])),

        Box::new(RoadBlock::new(213)),
        Box::new(RoadBlock::new(214)),
        Box::new(RoadBlock::new(215)),
        Box::new(ShopBlock::new(216,vec![Shop::new("Siete fuegos #1".parse().unwrap())])),
        Box::new(ShopBlock::new(217,vec![Shop::new("Super la fortuna #1".parse().unwrap())])),

        Box::new(RoadBlock::new(218)),
        Box::new(RoadBlock::new(219)),

        Box::new(WaterBlock::new(220)),
        Box::new(WaterBlock::new(221)),
        Box::new(WaterBlock::new(222)),
        Box::new(WaterBlock::new(223)),
        Box::new(WaterBlock::new(224)),
        Box::new(WaterBlock::new(225)),
    ]);



    // y = 9
    grid.push(vec![
        Box::new(RoadBlock::new(226)),
        Box::new(RoadBlock::new(227)),
        Box::new(RoadBlock::new(228)),
        Box::new(ShopBlock::new(229,vec![Shop::new("Starbucks #1".parse().unwrap())])),
        Box::new(RoadBlock::new(230)),
        Box::new(RoadBlock::new(231)),
        Box::new(RoadBlock::new(232)),
        Box::new(RoadBlock::new(233)),
        Box::new(RoadBlock::new(234)),
        Box::new(RoadBlock::new(235)),
        Box::new(RoadBlock::new(236)),
        Box::new(RoadBlock::new(237)),
        Box::new(RoadBlock::new(238)),
        Box::new(RoadBlock::new(239)),
        Box::new(RoadBlock::new(240)),
        Box::new(RoadBlock::new(241)),
        Box::new(RoadBlock::new(242)),
        Box::new(RoadBlock::new(243)),

        Box::new(WaterBlock::new(244)),
        Box::new(WaterBlock::new(245)),
        Box::new(WaterBlock::new(246)),
        Box::new(WaterBlock::new(247)),
        Box::new(WaterBlock::new(248)),
        Box::new(WaterBlock::new(249)),
        Box::new(WaterBlock::new(250)),

    ]);

    // y = 10
    grid.push(vec![

        Box::new(WaterBlock::new(251)),
        Box::new(BridgeBlock::new(252, Control::with_traffic(8, 10), MyMutex::new())),
        Box::new(WaterBlock::new(253)),
        Box::new(WaterBlock::new(254)),
        Box::new(WaterBlock::new(255)),
        Box::new(BridgeBlock::new(256,Control::without_traffic(true),MyMutex::new())),

        Box::new(WaterBlock::new(257)),
        Box::new(WaterBlock::new(258)),
        Box::new(WaterBlock::new(259)),
        Box::new(WaterBlock::new(260)),
        Box::new(WaterBlock::new(261)),
        Box::new(BridgeBlock::new(262,Control::without_traffic(false),MyMutex::new())),

        Box::new(WaterBlock::new(263)),
        Box::new(WaterBlock::new(264)),
        Box::new(WaterBlock::new(265)),
        Box::new(WaterBlock::new(266)),
        Box::new(WaterBlock::new(267)),
        Box::new(WaterBlock::new(268)),
        Box::new(WaterBlock::new(269)),
        Box::new(WaterBlock::new(270)),
        Box::new(WaterBlock::new(271)),
        Box::new(WaterBlock::new(272)),
        Box::new(WaterBlock::new(273)),
        Box::new(WaterBlock::new(274)),
        Box::new(WaterBlock::new(275)),




    ]);

    // y = 12
    grid.push(vec![

        Box::new(RoadBlock::new(276)),
        Box::new(RoadBlock::new(277)),
        Box::new(RoadBlock::new(278)),

        Box::new(WaterBlock::new(279)),
        Box::new(WaterBlock::new(280)),

        Box::new(RoadBlock::new(281)),
        Box::new(RoadBlock::new(282)),
        Box::new(RoadBlock::new(283)),

        Box::new(WaterBlock::new(284)),
        Box::new(DockBlock::new(285)),
        Box::new(WaterBlock::new(286)),

        Box::new(RoadBlock::new(287)),
        Box::new(RoadBlock::new(288)),
        Box::new(RoadBlock::new(289)),


        Box::new(WaterBlock::new(290)),
        Box::new(WaterBlock::new(291)),
        Box::new(WaterBlock::new(292)),
        Box::new(WaterBlock::new(293)),
        Box::new(WaterBlock::new(294)),
        Box::new(WaterBlock::new(295)),
        Box::new(WaterBlock::new(296)),
        Box::new(WaterBlock::new(297)),
        Box::new(WaterBlock::new(298)),
        Box::new(WaterBlock::new(299)),
        Box::new(WaterBlock::new(300)),
    ]);


    // y = 14
    grid.push(vec![

        Box::new(RoadBlock::new(301)),
        Box::new(ShopBlock::new(302,vec![Shop::new("Pinturas sur #1".parse().unwrap())])),

        Box::new(RoadBlock::new(303)),
        Box::new(RoadBlock::new(304)),
        Box::new(RoadBlock::new(305)),
        Box::new(RoadBlock::new(306)),
        Box::new(RoadBlock::new(307)),
        Box::new(RoadBlock::new(308)),
        Box::new(RoadBlock::new(309)),
        Box::new(RoadBlock::new(310)),

        Box::new(ShopBlock::new(311,vec![Shop::new("King Chicken #1".parse().unwrap())])),
        Box::new(RoadBlock::new(312)),
        Box::new(RoadBlock::new(313)),
        Box::new(RoadBlock::new(314)),
        Box::new(RoadBlock::new(315)),
        Box::new(RoadBlock::new(316)),
        Box::new(RoadBlock::new(317)),
        Box::new(RoadBlock::new(318)),

        Box::new(WaterBlock::new(319)),
        Box::new(WaterBlock::new(320)),
        Box::new(WaterBlock::new(321)),
        Box::new(WaterBlock::new(322)),
        Box::new(WaterBlock::new(323)),
        Box::new(WaterBlock::new(324)),
        Box::new(WaterBlock::new(325)),
    ]);
    // y = 15
    grid.push(vec![

        Box::new(RoadBlock::new(326)),
        Box::new(ShopBlock::new(327,vec![Shop::new("Arbys #1".parse().unwrap())])),

        Box::new(RoadBlock::new(328)),
        Box::new(RoadBlock::new(329)),
        Box::new(ShopBlock::new(330,vec![Shop::new("Pira #1".parse().unwrap())])),

        Box::new(RoadBlock::new(331)),
        Box::new(RoadBlock::new(332)),
        Box::new(RoadBlock::new(333)),


        Box::new(ShopBlock::new(334,vec![Shop::new("HP #1".parse().unwrap())])),
        Box::new(RoadBlock::new(335)),
        Box::new(RoadBlock::new(336)),

        Box::new(ShopBlock::new(337,vec![Shop::new("Radioshack #1".parse().unwrap())])),
        Box::new(ShopBlock::new(338,vec![Shop::new("Liberia Internacional #1".parse().unwrap())])),
        Box::new(ShopBlock::new(339,vec![Shop::new("PHP #1".parse().unwrap())])),
        Box::new(ShopBlock::new(340,vec![Shop::new("Intel #1".parse().unwrap())])),

        Box::new(RoadBlock::new(341)),
        Box::new(RoadBlock::new(342)),
        Box::new(RoadBlock::new(343)),
        Box::new(RoadBlock::new(344)),

        Box::new(WaterBlock::new(345)),
        Box::new(WaterBlock::new(346)),
        Box::new(WaterBlock::new(347)),
        Box::new(WaterBlock::new(348)),
        Box::new(WaterBlock::new(349)),
        Box::new(WaterBlock::new(350)),
    ]);
    // y = 16
    grid.push(vec![

        Box::new(RoadBlock::new(351)),
        Box::new(RoadBlock::new(352)),
        Box::new(RoadBlock::new(353)),
        Box::new(RoadBlock::new(354)),
        Box::new(RoadBlock::new(355)),
        Box::new(RoadBlock::new(356)),
        Box::new(RoadBlock::new(357)),
        Box::new(RoadBlock::new(358)),


        Box::new(ShopBlock::new(359,vec![Shop::new("Spoon #1".parse().unwrap())])),
        Box::new(ShopBlock::new(360,vec![Shop::new("Coqui #1".parse().unwrap())])),


        Box::new(RoadBlock::new(361)),
        Box::new(RoadBlock::new(362)),
        Box::new(RoadBlock::new(363)),
        Box::new(RoadBlock::new(364)),

        Box::new(ShopBlock::new(365,vec![Shop::new("CRGAMES #1".parse().unwrap())])),

        Box::new(RoadBlock::new(366)),
        Box::new(ShopBlock::new(367,vec![Shop::new("Labin #1".parse().unwrap())])),
        Box::new(RoadBlock::new(368)),
        Box::new(RoadBlock::new(369)),


        Box::new(WaterBlock::new(370)),
        Box::new(WaterBlock::new(371)),
        Box::new(WaterBlock::new(372)),
        Box::new(WaterBlock::new(373)),
        Box::new(WaterBlock::new(374)),
        Box::new(WaterBlock::new(375)),
    ]);


    // y = 17
    grid.push(vec![

        Box::new(RoadBlock::new(376)),
        Box::new(RoadBlock::new(377)),

        Box::new(ShopBlock::new(378,vec![Shop::new("BobaTeaTienda #1".parse().unwrap())])),

        Box::new(RoadBlock::new(379)),
        Box::new(RoadBlock::new(380)),

        Box::new(ShopBlock::new(381,vec![Shop::new("SkinCareTienda #1".parse().unwrap())])),

        Box::new(RoadBlock::new(382)),
        Box::new(RoadBlock::new(383)),
        Box::new(RoadBlock::new(384)),
        Box::new(RoadBlock::new(385)),
        Box::new(RoadBlock::new(386)),
        Box::new(RoadBlock::new(387)),

        Box::new(ShopBlock::new(388,vec![Shop::new("Spoon #1".parse().unwrap())])),

        Box::new(RoadBlock::new(389)),
        Box::new(RoadBlock::new(390)),
        Box::new(RoadBlock::new(391)),

        Box::new(ShopBlock::new(392,vec![Shop::new("CRGAMES #1".parse().unwrap())])),

        Box::new(RoadBlock::new(393)),
        Box::new(RoadBlock::new(394)),

        Box::new(WaterBlock::new(395)),
        Box::new(WaterBlock::new(396)),
        Box::new(WaterBlock::new(397)),
        Box::new(WaterBlock::new(398)),
        Box::new(WaterBlock::new(399)),
        Box::new(WaterBlock::new(400)),
    ]);


    // y = 18
    grid.push(vec![

        Box::new(RoadBlock::new(401)),
        Box::new(RoadBlock::new(402)),

        Box::new(ShopBlock::new(403,vec![Shop::new("Miniso #1".parse().unwrap())])),
        Box::new(ShopBlock::new(404,vec![Shop::new("Oftamlogo #1".parse().unwrap())])),
        Box::new(ShopBlock::new(405,vec![Shop::new("Clinica #1".parse().unwrap())])),
        Box::new(ShopBlock::new(406,vec![Shop::new("Laboratorio #1".parse().unwrap())])),
        Box::new(ShopBlock::new(407,vec![Shop::new("Oculista #1".parse().unwrap())])),


        Box::new(RoadBlock::new(408)),
        Box::new(RoadBlock::new(409)),
        Box::new(RoadBlock::new(410)),
        Box::new(RoadBlock::new(411)),

        Box::new(ShopBlock::new(412,vec![Shop::new("Bambash #1".parse().unwrap())])),
        Box::new(ShopBlock::new(413,vec![Shop::new("Jugueton #1".parse().unwrap())])),
        Box::new(ShopBlock::new(414,vec![Shop::new("Toys #1".parse().unwrap())])),
        Box::new(ShopBlock::new(415,vec![Shop::new("Panini #1".parse().unwrap())])),
        Box::new(ShopBlock::new(416,vec![Shop::new("Pescaderia #1".parse().unwrap())])),
        Box::new(ShopBlock::new(417,vec![Shop::new("Verdureria #1".parse().unwrap())])),

        Box::new(RoadBlock::new(418)),
        Box::new(RoadBlock::new(419)),

        Box::new(WaterBlock::new(420)),
        Box::new(WaterBlock::new(421)),
        Box::new(WaterBlock::new(422)),
        Box::new(WaterBlock::new(423)),
        Box::new(WaterBlock::new(424)),
        Box::new(WaterBlock::new(425)),
    ]);

    // y = 19
    grid.push(vec![

        Box::new(RoadBlock::new(426)),
        Box::new(RoadBlock::new(427)),
        Box::new(RoadBlock::new(428)),

        Box::new(ShopBlock::new(429,vec![Shop::new("Tokyo #1".parse().unwrap())])),
        Box::new(RoadBlock::new(430)),
        Box::new(RoadBlock::new(431)),

        Box::new(ShopBlock::new(432,vec![Shop::new("Teriyaki #1".parse().unwrap())])),
        Box::new(RoadBlock::new(433)),
        Box::new(RoadBlock::new(434)),
        Box::new(RoadBlock::new(435)),
        Box::new(RoadBlock::new(436)),
        Box::new(RoadBlock::new(437)),
        Box::new(RoadBlock::new(438)),

        Box::new(ShopBlock::new(439,vec![Shop::new("Clinica #1".parse().unwrap())])),
        Box::new(NuclearPlantBlock::new(440, 120, 30)),
        Box::new(RoadBlock::new(441)),
        Box::new(RoadBlock::new(442)),
        Box::new(RoadBlock::new(443)),

        Box::new(ShopBlock::new(444,vec![Shop::new("Laboratorio #1".parse().unwrap())])),

        Box::new(WaterBlock::new(445)),
        Box::new(WaterBlock::new(446)),
        Box::new(WaterBlock::new(447)),
        Box::new(WaterBlock::new(448)),
        Box::new(WaterBlock::new(449)),
        Box::new(WaterBlock::new(450)),
    ]);

    // y = 20
    grid.push(vec![

        Box::new(ShopBlock::new(451,vec![Shop::new("UTN #1".parse().unwrap())])),
        Box::new(ShopBlock::new(452,vec![Shop::new("TEC #1".parse().unwrap())])),
        Box::new(ShopBlock::new(453,vec![Shop::new("UCR #1".parse().unwrap())])),
        Box::new(ShopBlock::new(454,vec![Shop::new("UNA #1".parse().unwrap())])),

        Box::new(RoadBlock::new(455)),
        Box::new(ShopBlock::new(456,vec![Shop::new("Fidelitas #1".parse().unwrap())])),
        Box::new(ShopBlock::new(457,vec![Shop::new("UIA #1".parse().unwrap())])),
        Box::new(ShopBlock::new(458,vec![Shop::new("UNED #1".parse().unwrap())])),
        Box::new(ShopBlock::new(459,vec![Shop::new("UH #1".parse().unwrap())])),
        Box::new(ShopBlock::new(460,vec![Shop::new("Latina #1".parse().unwrap())])),

        Box::new(RoadBlock::new(461)),
        Box::new(RoadBlock::new(462)),
        Box::new(RoadBlock::new(463)),

        Box::new(ShopBlock::new(464,vec![Shop::new("JBL #1".parse().unwrap())])),
        Box::new(ShopBlock::new(465,vec![Shop::new("Skullhead #1".parse().unwrap())])),
        Box::new(ShopBlock::new(466,vec![Shop::new("PLaystation #1".parse().unwrap())])),
        Box::new(ShopBlock::new(467,vec![Shop::new("Xbox #1".parse().unwrap())])),
        Box::new(ShopBlock::new(468,vec![Shop::new("Nintendo #1".parse().unwrap())])),
        Box::new(ShopBlock::new(469,vec![Shop::new("Team cherry tienda#1".parse().unwrap())])),

        Box::new(WaterBlock::new(470)),
        Box::new(WaterBlock::new(471)),
        Box::new(WaterBlock::new(472)),
        Box::new(WaterBlock::new(473)),
        Box::new(WaterBlock::new(474)),
        Box::new(WaterBlock::new(475)),


    ]);

    // y = 21
    grid.push(vec![

        Box::new(ShopBlock::new(476,vec![Shop::new("universidad Alajuela #1".parse().unwrap())])),
        Box::new(RoadBlock::new(477)),
        Box::new(RoadBlock::new(478)),
        Box::new(RoadBlock::new(479)),
        Box::new(RoadBlock::new(480)),
        Box::new(RoadBlock::new(481)),
        Box::new(RoadBlock::new(482)),
        Box::new(RoadBlock::new(483)),

        Box::new(ShopBlock::new(484,vec![Shop::new("Google #1".parse().unwrap())])),
        Box::new(RoadBlock::new(485)),
        Box::new(RoadBlock::new(486)),
        Box::new(RoadBlock::new(487)),
        Box::new(RoadBlock::new(488)),


        Box::new(ShopBlock::new(489,vec![Shop::new("Microsoft #1".parse().unwrap())])),
        Box::new(RoadBlock::new(490)),
        Box::new(RoadBlock::new(491)),
        Box::new(RoadBlock::new(492)),
        Box::new(RoadBlock::new(493)),

        Box::new(ShopBlock::new(494,vec![Shop::new("In and out #1".parse().unwrap())])),
        Box::new(RoadBlock::new(495)),

        Box::new(WaterBlock::new(496)),
        Box::new(WaterBlock::new(497)),
        Box::new(WaterBlock::new(498)),
        Box::new(WaterBlock::new(499)),
        Box::new(WaterBlock::new(500)),


    ]);

    // y = 22
    grid.push(vec![

        Box::new(ShopBlock::new(501,vec![Shop::new("Kojim Productions #1".parse().unwrap())])),
        Box::new(RoadBlock::new(502)),
        Box::new(ShopBlock::new(503,vec![Shop::new("Black&Decker #1".parse().unwrap())])),
        Box::new(ShopBlock::new(504,vec![Shop::new("Suli #1".parse().unwrap())])),

        Box::new(RoadBlock::new(505)),
        Box::new(RoadBlock::new(506)),
        Box::new(RoadBlock::new(507)),
        Box::new(RoadBlock::new(508)),

        Box::new(ShopBlock::new(509,vec![Shop::new("Pali #1".parse().unwrap())])),
        Box::new(ShopBlock::new(510,vec![Shop::new("MasxMenos #1".parse().unwrap())])),
        Box::new(ShopBlock::new(511,vec![Shop::new("Telenoticias #1".parse().unwrap())])),


        Box::new(RoadBlock::new(512)),
        Box::new(RoadBlock::new(513)),

        Box::new(ShopBlock::new(514,vec![Shop::new("Repretel #1".parse().unwrap())])),
        Box::new(RoadBlock::new(515)),
        Box::new(ShopBlock::new(516,vec![Shop::new("Televisa #1".parse().unwrap())])),

        Box::new(RoadBlock::new(517)),
        Box::new(RoadBlock::new(518)),
        Box::new(ShopBlock::new(519,vec![Shop::new("Cinepolis #1".parse().unwrap())])),
        Box::new(RoadBlock::new(520)),


        Box::new(WaterBlock::new(521)),
        Box::new(WaterBlock::new(522)),
        Box::new(WaterBlock::new(523)),
        Box::new(WaterBlock::new(524)),
        Box::new(WaterBlock::new(525)),


    ]);


    // y = 23
    grid.push(vec![

        Box::new(RoadBlock::new(526)),
        Box::new(RoadBlock::new(527)),

        Box::new(ShopBlock::new(528,vec![Shop::new("restaurante #1".parse().unwrap())])),
        Box::new(ShopBlock::new(529,vec![Shop::new("Salon de patines #1".parse().unwrap())])),

        Box::new(RoadBlock::new(530)),

        Box::new(ShopBlock::new(531,vec![Shop::new("Sardimar #1".parse().unwrap())])),
        Box::new(ShopBlock::new(532,vec![Shop::new("Tesoro del mar #1".parse().unwrap())])),

        Box::new(RoadBlock::new(533)),
        Box::new(RoadBlock::new(534)),
        Box::new(RoadBlock::new(535)),
        Box::new(RoadBlock::new(536)),
        Box::new(RoadBlock::new(537)),
        Box::new(RoadBlock::new(538)),
        Box::new(RoadBlock::new(539)),
        Box::new(RoadBlock::new(540)),

        Box::new(ShopBlock::new(541,vec![Shop::new("Fanta #1".parse().unwrap())])),
        Box::new(ShopBlock::new(542,vec![Shop::new("Coca cola #1".parse().unwrap())])),
        Box::new(RoadBlock::new(543)),
        Box::new(ShopBlock::new(544,vec![Shop::new("Pepsi #1".parse().unwrap())])),


        Box::new(RoadBlock::new(545)),
        Box::new(RoadBlock::new(546)),
        Box::new(RoadBlock::new(547)),

        Box::new(WaterBlock::new(548)),
        Box::new(WaterBlock::new(549)),
        Box::new(WaterBlock::new(550)),
    ]);
    // y = 24
    grid.push(vec![
        Box::new(NuclearPlantBlock::new(551, 120, 30)),
        Box::new(ShopBlock::new(552,vec![Shop::new("Tienda mercancía Chernobyl #1".parse().unwrap())])),

        Box::new(RoadBlock::new(553)),
        Box::new(RoadBlock::new(554)),
        Box::new(RoadBlock::new(555)),
        Box::new(RoadBlock::new(556)),
        Box::new(RoadBlock::new(557)),


        Box::new(ShopBlock::new(558,vec![Shop::new("Salon de patines #1".parse().unwrap())])),

        Box::new(RoadBlock::new(559)),
        Box::new(RoadBlock::new(560)),
        Box::new(RoadBlock::new(561)),
        Box::new(RoadBlock::new(562)),
        Box::new(RoadBlock::new(563)),
        Box::new(RoadBlock::new(564)),
        Box::new(RoadBlock::new(565)),

        Box::new(ShopBlock::new(566,vec![Shop::new("Sardimar #1".parse().unwrap())])),
        Box::new(RoadBlock::new(567)),
        Box::new(RoadBlock::new(568)),
        Box::new(RoadBlock::new(569)),
        Box::new(RoadBlock::new(570)),
        Box::new(RoadBlock::new(571)),
        Box::new(RoadBlock::new(572)),

        Box::new(ShopBlock::new(573,vec![Shop::new("Tesoro del mar #1".parse().unwrap())])),

        Box::new(WaterBlock::new(574)),
        Box::new(WaterBlock::new(575)),
    ]);

    //            Box::new(ShopBlock::new(330,vec![Shop::new("Cemaco #1".parse().unwrap())])),

    Map::build_custom(grid)

}
