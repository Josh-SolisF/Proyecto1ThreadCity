use std::cell::RefCell;
use std::rc::Rc;
use glib::Continue;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::cairo;
use glib::source::timeout_add_local;
use mypthreads::mythread::mymutex::MyMutex;
use crate::cityblock::{map, Block};
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
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;
use gtk::cairo::Operator;
use crate::city::simulation_controller::SimulationController;
use crate::cityblock::block_type::BlockType::NuclearPlant;
use crate::cityblock::nuclearplant::plant_status::PlantStatus::Boom;
use crate::vehicle::vehicle_type::VehicleType;

//  UI Hooks: cómo la GUI consulta
#[derive(Clone)]
pub struct UiHooks {
    world_size: Rc<dyn Fn() -> (i16, i16)>,
    block_type_at: Rc<dyn Fn(Coord) -> Option<BlockType>>,
    is_occupied: Rc<dyn Fn(Coord) -> bool>,
    plant_status_at: Rc<RefCell<dyn FnMut(Coord) -> Option<PlantStatus>>>, 
    tick: Rc<RefCell<dyn FnMut()>>,
    vehicles_at: Rc<dyn Fn() -> Vec<(usize, Coord, VehicleType)>>, // 👈 nuevo hook

}


fn color_for_block(bt: &BlockType) -> (f64, f64, f64) {
    use crate::cityblock::block_type::BlockType::*;
    match bt {
        Road          => (0.00, 0.00, 0.00), // negro
        Bridge        => (0.60, 0.80, 1.00), // (azulado) para distinguir de carretera
        Shops          => (1.00, 0.55, 0.00), // naranja (tipo #FF8C00)
        Dock          => (0.59, 0.29, 0.00), // marrón (tipo #964B00)
        Water         => (0.00, 0.50, 1.00), // azul (tipo #0080FF)
        NuclearPlant  => (0.00, 0.70, 0.20), // verde 
    }
}
fn color_for_vehicle(vtype: &VehicleType) -> (f64, f64, f64) {
    use VehicleType::*;
    match vtype {
        CarE => (0.95, 0.20, 0.20),   // rojo
        AmbulanceE => (1.00, 0.75, 0.00),   // amarillo
        TruckE => (0.20, 0.60, 1.00), // azul
        ShipE => (0.00, 0.85, 0.30),  // verde
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
    let alloc = area.allocation();
    let w_px = alloc.width() as f64;
    let h_px = alloc.height() as f64;

    // Dimensiones del mundo (en celdas) como enteros
    let (w_i16, h_i16) = (hooks.world_size)();
    let w = (w_i16.max(0) as usize).max(1);
    let h = (h_i16.max(0) as usize).max(1);

    // Tamaño de celda cuadrada
    let cell_w = (w_px / (w as f64)).floor();
    let cell_h = (h_px / (h as f64)).floor();
    let cell = cell_w.min(cell_h).max(1.0);

    // Offsets para centrar el grid
    let ox = (w_px - cell * (w as f64)).max(0.0) / 2.0;
    let oy = (h_px - cell * (h as f64)).max(0.0) / 2.0;

    // Fondo
    cr.set_source_rgb(0.12, 0.12, 0.12);
    cr.paint().unwrap();

    // Único borrow del closure
    let mut plant_status = hooks.plant_status_at.borrow_mut();

    // Dibujo de celdas
    for y in 0..h {
        for x in 0..w {
            let coord = Coord::new(x as i16, y as i16);

            // Esquina superior-izquierda de la celda en píxeles
            let x_px = ox + (x as f64) * cell;
            let y_px = oy + (y as f64) * cell;

            cr.save().unwrap();              // aísla estado por celda
            cr.set_operator(Operator::Over);
            cr.set_line_width(1.0);

            // Relleno según tipo (o gris si None)
            if let Some(bt) = (hooks.block_type_at)(coord) {
                if bt == NuclearPlant &&  plant_status(coord) == Option::from(Boom) {
                    cr.set_source_rgb(0.25, 0.25, 0.25);
                } else {
                    let (r, g, b) = color_for_block(&bt);
                    cr.set_source_rgb(r, g, b);
                }
            } else {
                cr.set_source_rgb(0.25, 0.25, 0.25);
            }

            // Relleno dejando 1px de "rejilla" visual
            cr.rectangle(x_px, y_px, cell - 1.0, cell - 1.0);
            cr.fill().unwrap();

            // Borde por estado de planta (si aplica)
            if let Some(ps) = (plant_status)(coord) {
                let (br, bg, bb) = border_for_plant_status(ps);
                cr.set_source_rgb(br, bg, bb);
                cr.set_line_width((cell * 0.12).clamp(1.0, 3.0));
                cr.rectangle(x_px + 1.0, y_px + 1.0, cell - 3.0, cell - 3.0);
                cr.stroke().unwrap();
            }

            // Overlay círculo rojo si está ocupado
            if (hooks.is_occupied)(coord) {
                cr.set_source_rgb(0.95, 0.20, 0.20);
                let cx = x_px + cell * 0.5;
                let cy = y_px + cell * 0.5;
                let r  = (cell * 0.25).max(3.0);
                cr.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
                cr.fill().unwrap();
            }

            cr.restore().unwrap();
        }
    }
    let vehicles = (hooks.vehicles_at)();

    // Agrupar por coordenada (para saber cuántos hay por celda)
    use std::collections::HashMap;
    let mut grouped: HashMap<(i16, i16), Vec<(usize, VehicleType)>> = HashMap::new();

    for (tid, coord, vtype) in vehicles {
        grouped
            .entry((coord.x, coord.y))
            .or_default()
            .push((tid, vtype));
    }

    // Dibujo de cada grupo de vehículos
    for ((x, y), list) in grouped {
        let x_px = ox + (x as f64) * cell;
        let y_px = oy + (y as f64) * cell;

        // desplazamientos relativos (por cuadrante)
        let offsets = [
            (-0.25, -0.25), // 0: superior izquierda
            (0.25, -0.25),  // 1: superior derecha
            (-0.25, 0.25),  // 2: inferior izquierda
            (0.25, 0.25),   // 3: inferior derecha
        ];

        for (i, (tid, vtype)) in list.iter().enumerate() {
            if i >= 4 {
                break; // máximo 4 por celda
            }

            let (dx, dy) = offsets[i];
            let (r, g, b) = color_for_vehicle(vtype);
            cr.set_source_rgb(r, g, b);

            // posición del vehículo dentro de la celda
            let cx = x_px + cell * (0.5 + dx * 0.5);
            let cy = y_px + cell * (0.5 + dy * 0.5);
            let radius = (cell * 0.15).max(2.0);

            cr.arc(cx, cy, radius, 0.0, std::f64::consts::TAU);
            cr.fill().unwrap();

            // tid en el centro del círculo
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            cr.set_font_size((cell * 0.25).clamp(6.0, 12.0));

            let text = tid.to_string();
            let extents = cr.text_extents(&text).unwrap();
            cr.move_to(cx - extents.width() / 2.0, cy + extents.height() / 2.0);
            cr.show_text(&text).unwrap();
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
    timeout_add_local(std::time::Duration::from_millis(1000), move || {
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


    //Controlador de simulación (map, tráfico, plantas, etc.)
    let sim: Rc<RefCell<SimulationController>> =
        Rc::new(RefCell::new(SimulationController::new()));

    // Ocupación visible para la GUI (se actualiza en cada tick)
    let occupancy = Rc::new(RefCell::new(HashSet::<(i16, i16)>::new()));


    let world_size = {
        let sim = Rc::clone(&sim);
        Rc::new(move || -> (i16, i16) {
            //SimulationController (inmutable)
            let sim_borrow = sim.borrow();
            // Map (inmutable) depende del anterior
            let map_borrow = sim_borrow.map.borrow();
            (map_borrow.width_cells(), map_borrow.height_cells())
            // Ambos borrows salen de scope al final
        })
    };

    let block_type_at = {
        let sim = Rc::clone(&sim);
        Rc::new(move |coord: Coord| -> Option<BlockType> {
            let sim_borrow = sim.borrow();
            let map_borrow = sim_borrow.map.borrow();
            map_borrow.block_type_at(coord)
        })
    };

    let vehicles_at = {
        let sim = Rc::clone(&sim);
        Rc::new(move || -> Vec<(usize, Coord, VehicleType)> {
            let sb = sim.borrow();
            sb.traffic.vehicles
                .iter()
                .map(|v| (*v.0 as usize, v.1.base().current_position, *v.1.get_type()))
                .collect()
        })
    };

    //Hashmap de ocupado
    let is_occupied = {
        let occupancy = Rc::clone(&occupancy);
        Rc::new(move |coord: Coord| -> bool {
            let occ = occupancy.borrow();
            occ.contains(&(coord.x, coord.y))
        })
    };

    //Estado de planta nuclear
    let plant_status_at = {
        let sim = Rc::clone(&sim);
        Rc::new(RefCell::new(move |coord: Coord| -> Option<PlantStatus> {
            let sim_borrow = sim.borrow();
            let mut map_borrow = sim_borrow.map.borrow_mut();
            map_borrow.try_plant_status_at(coord)
        }))
    };

    //Tick: avanza 1 frame y reconstruye la ocupación desde TrafficHandler::occupied_coords()
    let tick = {
        let sim = Rc::clone(&sim);
        let occ_rc = Rc::clone(&occupancy);

        Rc::new(RefCell::new(move || {
            // Avanza 1 frame
            {
                let mut sb = sim.borrow_mut();
                sb.advance_time(1);
            }

            // Lee posiciones actuales
            let coords = {
                let sb = sim.borrow();
                sb.traffic.occupied_coords()
            };

            // Actualiza ocupación
            let mut occ = occ_rc.borrow_mut();
            occ.clear();
            for c in coords {
                occ.insert((c.x, c.y));
            }
        }))
    };

    UiHooks {
        world_size,
        block_type_at,
        is_occupied,
        plant_status_at,
        tick,
        vehicles_at
    }
}
