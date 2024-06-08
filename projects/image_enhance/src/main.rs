mod network;

use std::cmp;
use std::fs::*;
use std::io::{stdout, Read, Write};
use std::path::Path;

use alumina::supplier::*;
use bytevec::{ByteDecodable, ByteEncodable};
use clap::{Parser, Subcommand};

use alumina::graph::*;
use alumina::opt::adam::Adam;
use alumina::opt::*;
use alumina::shape::*;
use image::GenericImage;
use imagefolder::data_to_img;
use imagefolder::*;
use network::*;

const IMAGENET_PARAMS: &[u8] = include_bytes!("res/imagenet.rsr");
const IMAGENETLINEAR_PARAMS: &[u8] = include_bytes!("res/imagenetlinear.rsr");
const ANIME_PARAMS: &[u8] = include_bytes!("res/anime.rsr");

const FACTOR: usize = 3;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.1",
    about = "A convolutional neural network trained to upscale images"
)]
struct Args {
    #[clap(help = "Sets the input image to upscale", required = true, index = 1)]
    input_file: String,

    #[clap(
        help = "Sets the output file to write/overwrite (.png recommended)",
        required = true,
        index = 2
    )]
    output_file: String,

    #[clap(
        short,
        long,
        value_name = "PARAMETERS",
        help = "Sets which built-in parameters to use with the neural net"
    )]
    parameters: Option<String>,

    #[clap(
        short,
        long,
        conflicts_with = "parameters",
        value_name = "PARAMETER_FILE",
        help = "Sets a custom parameter file (.rsr) to use with the neural net"
    )]
    custom: Option<String>,

    #[clap(short, long, conflicts_with_all = ["parameters", "custom"], help = "Perform downscaling rather than upscaling")]
    downsample: bool,

    #[command(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Train {
        #[clap(
            required = true,
            index = 1,
            help = "Learned network parameters will be (over)written to this parameter file (.rsr)"
        )]
        parameter_file: String,

        #[clap(
            required = true,
            index = 2,
            help = "Images from this folder(or sub-folders) will be used for training"
        )]
        training_folder: String,

        #[clap(
            short,
            long,
            help = "Apply MSE loss to a linearised RGB output rather than sRGB values"
        )]
        linear_loss: bool,

        #[clap(
            short,
            long,
            help = "Recurse into subfolders of training and validation folders looking for files"
        )]
        recurse_subfolders: bool,

        #[clap(
            short,
            long,
            value_name = "START_PARAMETERS",
            help = "Start training from known parameters loaded from this .rsr file rather than random initialisation"
        )]
        start_parameters: Option<String>,

        #[clap(
            short,
            long,
            value_name = "VALIDATION_FOLDER",
            help = "Images from this folder(or sub-folders) will be used to evaluate training progress"
        )]
        validation_folder: Option<String>,

        #[clap(
            short,
            long,
            requires = "validation_folder",
            value_name = "N",
            help = "Set upper limit on number of images used for each validation pass"
        )]
        val_max: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();

    if let Some(SubCommands::Train {
        parameter_file,
        training_folder,
        linear_loss,
        recurse_subfolders,
        start_parameters,
        validation_folder,
        val_max,
    }) = args.subcommand
    {
        train(
            parameter_file,
            training_folder,
            linear_loss,
            recurse_subfolders,
            start_parameters,
            validation_folder,
            val_max,
        );
    } else {
        upscale(&args);
    }
}

fn upscale(args: &Args) {
    let (params, mut graph) = if let Some(file_str) = &args.custom {
        let mut param_file = File::open(Path::new(file_str)).expect("Error opening parameter file");
        let mut data = Vec::new();
        param_file
            .read_to_end(&mut data)
            .expect("Reading parameter file failed");
        print!("Upscaling using custom neural net parameters...");
        (
            <Vec<f32>>::decode::<u32>(&data).expect("ByteVec conversion failed"),
            super_resolution_network(FACTOR, None),
        )
    } else if args.downsample {
        print!("Downsampling using average pooling of linear RGB values...");
        (Vec::new(), downsample_network(FACTOR))
    } else {
        match args.parameters.as_deref() {
            Some("imagenet") | None => {
                print!("Upscaling using imagenet neural net parameters...");
                (
                    <Vec<f32>>::decode::<u32>(IMAGENET_PARAMS).expect("ByteVec conversion failed"),
                    super_resolution_network(FACTOR, None),
                )
            }
            Some("imagenetlinear") => {
                print!("Upscaling using linear loss imagenet neural net parameters...");
                (
                    <Vec<f32>>::decode::<u32>(IMAGENETLINEAR_PARAMS)
                        .expect("ByteVec conversion failed"),
                    super_resolution_network(FACTOR, None),
                )
            }
            Some("anime") => {
                print!("Upscaling using anime neural net parameters...");
                (
                    <Vec<f32>>::decode::<u32>(ANIME_PARAMS).expect("ByteVec conversion failed"),
                    super_resolution_network(FACTOR, None),
                )
            }
            Some("bilinear") => {
                print!("Upscaling using bilinear interpolation...");
                (Vec::new(), bilinear_network(FACTOR))
            }
            _ => unreachable!(),
        }
    };

    stdout().flush().ok();
    assert_eq!(params.len(), graph.num_params(), "Parameters selected do not have the size required by the neural net. Ensure that the same sample factor is used for upscaling and training");

    let input_image =
        image::open(Path::new(&args.input_file)).expect("Error opening input image file.");
    let out_path = Path::new(&args.output_file);

    let mut input = NodeData::new_blank(DataShape::new(
        CHANNELS,
        &[
            input_image.dimensions().0 as usize,
            input_image.dimensions().1 as usize,
        ],
        1,
    ));

    img_to_data(&mut input.values, &input_image);
    let output = graph.forward(1, vec![input], &params).remove(0);

    print!(" Writing file...");
    stdout().flush().ok();
    data_to_img(output)
        .to_rgba()
        .save(out_path)
        .expect("Could not write output file");

    println!(" Done");
}

fn train(
    parameter_file: String,
    training_folder: String,
    linear_loss: bool,
    recurse_subfolders: bool,
    start_parameters: Option<String>,
    validation_folder: Option<String>,
    val_max: Option<usize>,
) {
    let mut g = super_resolution_network(FACTOR, Some((1e-6, linear_loss)));
    let training_set = ImageFolderSupplier::<ShuffleRandom>::new(
        Path::new(&training_folder),
        recurse_subfolders,
        Cropping::Random {
            width: 192,
            height: 192,
        },
    );
    let mut training_set = Buffer::new(training_set, 128);

    let start_params = if let Some(param_str) = start_parameters {
        let mut param_file =
            File::open(Path::new(&param_str)).expect("Error opening start parameter file");
        let mut data = Vec::new();
        param_file
            .read_to_end(&mut data)
            .expect("Reading start parameter file failed");
        <Vec<f32>>::decode::<u32>(&data).expect("ByteVec conversion failed")
    } else {
        g.init_params()
    };

    let mut solver = Adam::new(&mut g)
        .batch_size(4)
        .beta1(0.95)
        .beta2(0.995)
        .epsilon(1e-7)
        .learning_rate(2e-3)
        .finish();

    let param_file_path = Path::new(&parameter_file).to_path_buf();

    solver.add_step_callback(move |data| {
        if data.step_count % 100 == 0 || data.step_count == 1 {
            let mut parameter_file =
                File::create(&param_file_path).expect("Could not make parameter file");
            let bytes = data
                .params
                .encode::<u32>()
                .expect("ByteVec conversion failed");
            parameter_file
                .write_all(&bytes)
                .expect("Could not save to parameter file");
        }
        CallbackSignal::Continue
    });

    if let Some(val_str) = validation_folder {
        let mut g2 = super_resolution_network(FACTOR, Some((0.0, linear_loss)));
        let validation_set = ImageFolderSupplier::<Sequential>::new(
            Path::new(&val_str),
            recurse_subfolders,
            Cropping::None,
        );

        let n = val_max.map_or_else(
            || validation_set.epoch_size(),
            |val_max| cmp::min(validation_set.epoch_size(), val_max),
        );
        let mut validation_set = Buffer::new(validation_set, n);

        solver.add_step_callback(move |data| {
            if data.step_count % 100 == 0 || data.step_count == 1 {
                let mut err_sum = 0.0;
                let mut pix_sum = 0.0;
                for _ in 0..n {
                    let (input, training_input) = validation_set.next_n(1);
                    let pixels = input[0].shape.flat_size_single() as f32;
                    let (batch_err, _, _) = g2.backprop(1, input, training_input, data.params);

                    pix_sum += pixels;
                    err_sum += batch_err * pixels;
                }
                let psnr = -10.0 * (err_sum / pix_sum).log10();
                println!("Validation PSNR:\t{}", psnr);
            }

            CallbackSignal::Continue
        });
    }

    solver.add_boxed_step_callback(max_evals(10_000_000));

    println!("Beginning Training");
    solver.optimise_from(&mut training_set, start_params);
    println!("Done");
}
