// agvz_implement_a_min.rs

use std::collections::HashMap;

mod ar_module {
    pub trait ARModule {
        fn init(&mut self);
        fn update(&mut self);
        fn render(&self);
    }
}

mod vr_module {
    pub trait VRModule {
        fn init(&mut self);
        fn update(&mut self);
        fn render(&self);
    }
}

struct ModuleDashboard {
    ar_modules: HashMap<String, Box<dyn ar_module::ARModule>>,
    vr_modules: HashMap<String, Box<dyn vr_module::VRModule>>,
}

impl ModuleDashboard {
    fn new() -> Self {
        ModuleDashboard {
            ar_modules: HashMap::new(),
            vr_modules: HashMap::new(),
        }
    }

    fn add_ar_module(&mut self, name: String, module: Box<dyn ar_module::ARModule>) {
        self.ar_modules.insert(name, module);
    }

    fn add_vr_module(&mut self, name: String, module: Box<dyn vr_module::VRModule>) {
        self.vr_modules.insert(name, module);
    }

    fn init(&mut self) {
        for module in self.ar_modules.values_mut() {
            module.init();
        }
        for module in self.vr_modules.values_mut() {
            module.init();
        }
    }

    fn update(&mut self) {
        for module in self.ar_modules.values_mut() {
            module.update();
        }
        for module in self.vr_modules.values_mut() {
            module.update();
        }
    }

    fn render(&self) {
        for module in self.ar_modules.values() {
            module.render();
        }
        for module in self.vr_modules.values() {
            module.render();
        }
    }
}

fn main() {
    let mut dashboard = ModuleDashboard::new();

    // Example AR module implementation
    struct ARModuleExample {
        data: i32,
    }

    impl ar_module::ARModule for ARModuleExample {
        fn init(&mut self) {
            println!("AR Module Example initialized");
        }
        fn update(&mut self) {
            self.data += 1;
            println!("AR Module Example updated: {}", self.data);
        }
        fn render(&self) {
            println!("AR Module Example rendered");
        }
    }

    let ar_module_example = Box::new(ARModuleExample { data: 0 });
    dashboard.add_ar_module("example".to_string(), ar_module_example);

    dashboard.init();
    dashboard.update();
    dashboard.render();
}