// src/main.rs (o src/bin/gtk_view.rs)
use std::cell::RefCell;
use std::rc::Rc;
use glib::Continue;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::cairo;
use glib::source::timeout_add_local;

use crate::cityblock::coord::Coord;
use crate::cityblock::block_type::BlockType;
use crate::cityblock::nuclearplant::plant_status::PlantStatus;


// ---- UI Hooks: cómo la GUI consulta tu mundo ----
#[derive(Clone)]
pub struct UiHooks {
    world_size: Rc<dyn Fn() -> (i16, i16)>,
    block_type_at: Rc<dyn Fn(Coord) -> Option<BlockType>>,
    is_occupied: Rc<dyn Fn(Coord) -> bool>,
    plant_status_at: Rc<RefCell<dyn FnMut(Coord) -> Option<PlantStatus>>>, // necesita &mut Map p/ downcast
    tick: Rc<RefCell<dyn FnMut()>>, // avanza 1 frame
}

fn color_for_block(bt: &BlockType) -> (f64, f64, f64) {
    use crate::cityblock::block_type::BlockType::*;
    match bt {
        Road => (0.80, 0.80, 0.80),
        Bridge => (0.60, 0.80, 1.00),
        Shop => (0.93, 0.90, 0.60),
        Dock => (0.60, 0.60, 0.90),
        Water => (0.45, 0.70, 1.00),
        NuclearPlant => (0.92, 0.65, 0.65),
        _ => (0.85, 0.85, 0.85),
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

    use crate::cityblock::map::Map;
    use crate::cityblock::block_type::BlockType;
    use crate::cityblock::coord::Coord;
    use crate::cityblock::nuclearplant::plant_status::PlantStatus;


    // Hay que reemplazar con el mapa de verdad!)
    let map = Rc::new(RefCell::new(dummy_map_3x3())); // <-- reemplaza con tu mapa real
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

// --------- DUMMY MAP
fn dummy_map_3x3() -> crate::cityblock::map::Map {
    use crate::cityblock::Block;
    use crate::cityblock::road::RoadBlock;
    use crate::cityblock::nuclearplant::NuclearPlantBlock;
    use crate::cityblock::transport_policy::TransportPolicy::NoVehicles;
    use crate::cityblock::block_type::BlockType::{Road, NuclearPlant};

    // construye 3x3 de Roads, con una NuclearPlant en (1,1)
    let mut grid: Vec<Vec<Box<dyn Block>>> = vec![];
    for y in 0..3 {
        let mut row: Vec<Box<dyn Block>> = vec![];
        for x in 0..3 {
            if x == 1 && y == 1 {
                // crea una NuclearPlant con update_interval_ms=30
                let plant = NuclearPlantBlock::new(/*id*/ 100 + y as usize * 3 + x as usize,100,30);
                row.push(Box::new(plant));
            } else {row.push(Box::new(RoadBlock::new(/*id*/ 100 + y as usize * 3 + x as usize)));
            }
        }
        grid.push(row);
    }
    crate::cityblock::map::Map::build_custom(grid)
}