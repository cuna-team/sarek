use {
    log::{
        info
    },
    sarek::{
        Context,
        DataSourceListExt,
        Model,
        Trainer,
        layers::{
            *
        }
    },
    std::{
        env,
        thread
    }
};

mod data;

fn main() {
    if env::var( "RUST_LOG" ).is_err() {
        env::set_var( "RUST_LOG", "sarek=info,cifar10=info" );
    }

    env_logger::init();

    let training_data = thread::spawn( || data::load_training_data() );
    let test_data = thread::spawn( || data::load_test_data() );

    let ctx = Context::new().unwrap();

    let training_data = training_data.join().unwrap();
    let input_shape = training_data.input_list().data_sources().next().unwrap().shape();
    let model = Model::new_sequential( input_shape, (
        LayerConvolution::new( 4, (3, 3) ),
        LayerActivation::new(),
        LayerMaxPooling::new( (2, 2) ),

        LayerDense::new( 32 ),
        LayerActivation::new(),
        LayerDense::new( 32 ),
        LayerActivation::new(),
        LayerDense::new( 32 ),
        LayerActivation::new(),

        LayerDense::new( 10 ),
        LayerSoftmax::new(),
        LayerIntoCategory::new()
    ));

    let mut instance = Trainer::new( &ctx, model, training_data ).unwrap();

    let test_data = test_data.join().unwrap();
    let loss = instance.test( &test_data );
    info!( "Initial accuracy on the test set: {:.02}% (loss = {})", loss.accuracy().unwrap() * 100.0, loss.get() );

    for _ in 0..10 {
        instance.train();

        let loss = instance.test( &test_data );
        info!( "Accuracy on the test set: {:.02}% (loss = {})", loss.accuracy().unwrap() * 100.0, loss.get() );
    }
}
