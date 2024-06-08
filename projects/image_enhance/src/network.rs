use alumina::ops::activ::*;
use alumina::ops::basic::*;
use alumina::ops::conv::*;
use alumina::ops::loss::*;
use alumina::ops::reshape::*;
use alumina::ops::*;
use std::sync::Arc;

use alumina::graph::*;

const CHANNELS: usize = 3;

pub fn super_resolution_network(
    scale_factor: usize,
    training_params: Option<(f32, bool)>,
) -> Graph {
    let mut graph = Graph::new();

    let (input_node, output_node) = if training_params.is_some() {
        (
            graph.add_node(Node::new_shaped(CHANNELS, 2, "input")),
            graph.add_node(Node::new_shaped(CHANNELS, 2, "output")),
        )
    } else {
        (
            graph.add_input_node(Node::new_shaped(CHANNELS, 2, "input")),
            graph.add_output_node(Node::new_shaped(CHANNELS, 2, "output")),
        )
    };

    graph.add_operation(LinearInterp::new(
        &input_node,
        &output_node,
        &[scale_factor, scale_factor],
        "linear_interpolation",
    ));

    let mut operations: Vec<Box<dyn Operation>> = vec![];

    let conv_node = graph.add_node(Node::new_shaped(32, 2, "conv"));
    let activ_node = graph.add_node(Node::new_shaped(32, 2, "activ"));

    operations.push(Convolution::new(
        &input_node,
        &conv_node,
        &[5, 5],
        Padding::Same,
        "conv0",
        Convolution::init_msra(1.0),
    ));
    operations.push(Bias::new(
        &conv_node,
        ParamSharing::Spatial,
        "bias",
        init_fill(0.0),
    ));
    operations.push(BeLU::new(
        &conv_node,
        &activ_node,
        ParamSharing::Spatial,
        "activation",
        BeLU::init_porque_no_los_dos(),
    ));

    let expand_node = graph.add_node(Node::new_shaped(
        CHANNELS * scale_factor * scale_factor,
        2,
        "expand",
    ));
    operations.push(Bias::new(
        &expand_node,
        ParamSharing::Spatial,
        "expand_bias",
        init_fill(0.0),
    ));
    operations.push(Expand::new(
        &expand_node,
        &output_node,
        &[scale_factor, scale_factor],
        "expand",
    ));

    for _ in 0..1 {
        let num_channels = 32;
        let layer1_conv = graph.add_node(Node::new_shaped(num_channels, 2, "layer1_conv"));
        let layer1_activ = graph.add_node(Node::new_shaped(num_channels, 2, "layer1_activ"));
        let layer2_conv = graph.add_node(Node::new_shaped(num_channels, 2, "layer2_conv"));
        let layer2_activ = graph.add_node(Node::new_shaped(num_channels, 2, "layer2_activ"));
        let layer3_conv = graph.add_node(Node::new_shaped(num_channels, 2, "layer3_conv"));
        let layer3_activ = graph.add_node(Node::new_shaped(num_channels, 2, "layer3_activ"));

        operations.push(Bias::new(
            &layer1_conv,
            ParamSharing::Spatial,
            "layer1_bias",
            init_fill(0.0),
        ));
        operations.push(Bias::new(
            &layer2_conv,
            ParamSharing::Spatial,
            "layer2_bias",
            init_fill(0.0),
        ));
        operations.push(Bias::new(
            &layer3_conv,
            ParamSharing::Spatial,
            "layer3_bias",
            init_fill(0.0),
        ));

        operations.push(BeLU::new(
            &layer1_conv,
            &layer1_activ,
            ParamSharing::Spatial,
            "layer1_activation",
            BeLU::init_porque_no_los_dos(),
        ));
        operations.push(BeLU::new(
            &layer2_conv,
            &layer2_activ,
            ParamSharing::Spatial,
            "layer2_activation",
            BeLU::init_porque_no_los_dos(),
        ));
        operations.push(BeLU::new(
            &layer3_conv,
            &layer3_activ,
            ParamSharing::Spatial,
            "layer3_activation",
            BeLU::init_porque_no_los_dos(),
        ));

        operations.push(Convolution::new(
            &activ_node,
            &layer1_conv,
            &[5, 5],
            Padding::Same,
            "conv1",
            Convolution::init_msra(0.1),
        ));
        operations.push(Convolution::new(
            &activ_node,
            &layer2_conv,
            &[5, 5],
            Padding::Same,
            "conv2",
            Convolution::init_msra(0.1),
        ));
        operations.push(Convolution::new(
            &activ_node,
            &layer3_conv,
            &[5, 5],
            Padding::Same,
            "conv3",
            Convolution::init_msra(0.1),
        ));

        operations.push(Convolution::new(
            &layer1_activ,
            &layer2_conv,
            &[3, 3],
            Padding::Same,
            "conv4",
            Convolution::init_msra(0.1),
        ));
        operations.push(Convolution::new(
            &layer1_activ,
            &layer3_conv,
            &[3, 3],
            Padding::Same,
            "conv5",
            Convolution::init_msra(0.1),
        ));
        operations.push(Convolution::new(
            &layer1_activ,
            &expand_node,
            &[3, 3],
            Padding::Same,
            "conv6",
            Convolution::init_msra(0.1),
        ));

        operations.push(Convolution::new(
            &layer2_activ,
            &layer3_conv,
            &[3, 3],
            Padding::Same,
            "conv7",
            Convolution::init_msra(0.1),
        ));
        operations.push(Convolution::new(
            &layer2_activ,
            &expand_node,
            &[3, 3],
            Padding::Same,
            "conv8",
            Convolution::init_msra(0.1),
        ));

        operations.push(Convolution::new(
            &layer3_activ,
            &expand_node,
            &[3, 3],
            Padding::Same,
            "conv9",
            Convolution::init_msra(0.1),
        ));
    }

    let operation_indices = graph.add_operations(operations);

    if let Some((regularisation_factor, use_linear_loss)) = training_params {
        if regularisation_factor != 0.0 {
            for operation_id in &operation_indices {
                if operation_id.num_params == 0 {
                    continue;
                }
                graph.add_secondary_operation(
                    L2Regularisation::new(operation_id, regularisation_factor, "L2_regularisation"),
                    operation_id,
                );
            }
        }

        let input_high_res = graph.add_input_node(Node::new_shaped(CHANNELS, 2, "input_high_res"));
        let input_high_res_linear =
            graph.add_node(Node::new_shaped(CHANNELS, 2, "input_high_res_linear"));
        let input_pool_node = graph.add_node(Node::new_shaped(CHANNELS, 2, "input_pool"));
        graph.add_operation(SrgbToLinear::new(
            &input_high_res,
            &input_high_res_linear,
            "srgb_to_linear",
        ));
        graph.add_operation(Pooling::new(
            &input_high_res_linear,
            &input_pool_node,
            &[scale_factor, scale_factor],
            "input_pooling",
        ));
        graph.add_operation(LinearToSrgb::new(
            &input_pool_node,
            &input_node,
            "linear_to_srgb",
        ));

        if use_linear_loss {
            let output_linear = graph.add_node(Node::new_shaped(CHANNELS, 2, "output_linear"));
            graph.add_operation(SrgbToLinear::new(
                &output_node,
                &output_linear,
                "srgb_to_linear_output",
            ));
            graph.add_operation(MseLoss::new(
                &output_linear,
                &input_high_res_linear,
                1.0,
                "mse_loss",
            ));
        } else {
            graph.add_operation(MseLoss::new(&output_node, &input_high_res, 1.0, "mse_loss"));
        }

        graph.add_operation(ShapeConstraint::new(
            &input_high_res,
            &output_node,
            &[Arc::new(|d| d), Arc::new(|d| d)],
            "output_shape_constraint",
        ));
    } else {
        graph.add_operation(ShapeConstraint::new(
            &input_node,
            &output_node,
            &[
                Arc::new(move |d| d * scale_factor),
                Arc::new(move |d| d * scale_factor),
            ],
            "output_shape_constraint",
        ));
    }

    graph
}

pub fn bilinear_network(scale_factor: usize) -> Graph {
    let mut graph = Graph::new();
    let input_node = graph.add_input_node(Node::new_shaped(CHANNELS, 2, "input"));
    let linear_node = graph.add_node(Node::new_shaped(CHANNELS, 2, "linear"));
    let upscale_node = graph.add_node(Node::new_shaped(CHANNELS, 2, "upscale"));
    let output_node = graph.add_output_node(Node::new_shaped(CHANNELS, 2, "output"));

    graph.add_operation(SrgbToLinear::new(
        &input_node,
        &linear_node,
        "srgb_to_linear",
    ));
    graph.add_operation(LinearInterp::new(
        &linear_node,
        &upscale_node,
        &[scale_factor, scale_factor],
        "linear_interpolation",
    ));
    graph.add_operation(LinearToSrgb::new(
        &upscale_node,
        &output_node,
        "linear_to_srgb",
    ));
    graph.add_operation(ShapeConstraint::new(
        &input_node,
        &upscale_node,
        &[
            Arc::new(move |d| d * scale_factor),
            Arc::new(move |d| d * scale_factor),
        ],
        "shape_constraint",
    ));

    graph
}

pub fn downsample_network(scale_factor: usize) -> Graph {
    let mut graph = Graph::new();

    let input_high_res = graph.add_input_node(Node::new_shaped(CHANNELS, 2, "input_high_res"));
    let input_high_res_linear =
        graph.add_node(Node::new_shaped(CHANNELS, 2, "input_high_res_linear"));
    let input_pool_node = graph.add_node(Node::new_shaped(CHANNELS, 2, "input_pool"));
    let output_node = graph.add_output_node(Node::new_shaped(CHANNELS, 2, "output"));

    graph.add_operation(SrgbToLinear::new(
        &input_high_res,
        &input_high_res_linear,
        "srgb_to_linear",
    ));
    graph.add_operation(Pooling::new(
        &input_high_res_linear,
        &input_pool_node,
        &[scale_factor, scale_factor],
        "input_pooling",
    ));
    graph.add_operation(LinearToSrgb::new(
        &input_pool_node,
        &output_node,
        "linear_to_srgb",
    ));

    graph
}
