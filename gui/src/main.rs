fn main() {
  // define the gtk application with a unique name and default parameters
  let application = gtk::Application::new("The.name.goes.here", Default::default())
      .expect("Initialization failed");

  // this registers a closure (executing our setup_gui function) 
  //that has to be run on a `activate` event, triggered when the UI is loaded
  application.connect_activate(move |app| {
      setup_gui(app);
  });

  application.run();
}

fn setup_gui(app: &Application){
  
  // we bake our glade file into the application code itself
  let glade_src = include_str!("main.glade");

  // this builder provides access to all components of the defined ui
  let builder = Builder::new_from_string(glade_src);

  // glade allows us to get UI elements by id but we need to specify the type
  let window: Window = builder.get_object("wnd_main").expect("Couldn't get window");
  window.set_title("Memegen");
  window.set_application(app);

  // the save button, saves the resulting image
  let btn_save: Button = builder.get_object("btn_save").expect("Couldn't get btn_save");
  
  // connect to the clicked event, providing an action handler in the closure
  btn_save.connect_clicked( 
    clone!( // clone the references with a macro
      lines, background_location => move |_| {
        // invoke our handler with mutable references
        handle_save(background_location.borrow_mut(),lines.borrow_mut())
    })
  );
  window.show_all();
}
