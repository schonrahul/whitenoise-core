use std::collections::{HashMap, HashSet, VecDeque};
use crate::proto;
use ndarray::prelude::*;
use crate::utilities::serial::{Value, serialize_value, parse_value};
use crate::utilities::serial::ArrayND;


// equivalent to proto Release
pub type GraphEvaluation = HashMap<u32, Value>;

// arguments to a node prior to evaluation
pub type NodeArguments<'a> = HashMap<String, &'a Value>;

pub fn get_arguments<'a>(component: &proto::Component, graph_evaluation: &'a GraphEvaluation) -> NodeArguments<'a> {
    let mut arguments = NodeArguments::new();
    component.arguments.iter().for_each(|(field_id, field)| {
        let evaluation: &'a Value = graph_evaluation.get(&field).unwrap();
        arguments.insert(field_id.to_owned(), evaluation);
    });
    arguments
}


pub fn get_f64(arguments: &NodeArguments, column: &str) -> f64 {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(if *x.first().unwrap() { 1. } else { 0. }),
            ArrayND::I64(x) => Ok(f64::from(*x.first().unwrap() as i32)),
            ArrayND::F64(x) => Ok(x.first().unwrap().to_owned()),
            _ => Err(column.to_string() + " must be numeric")
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_array_f64(arguments: &NodeArguments, column: &str) -> ArrayD<f64> {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(x.mapv(|v| if v { 1. } else { 0. })),
            ArrayND::I64(x) => Ok(x.mapv(|v| f64::from(v as i32))),
            ArrayND::F64(x) => Ok(x.to_owned()),
            _ => Err(column.to_string() + " must be numeric")
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_i64(arguments: &NodeArguments, column: &str) -> i64 {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(if *x.first().unwrap() { 1 } else { 0 }),
            ArrayND::I64(x) => Ok(x.first().unwrap().to_owned()),
            _ => Err(column.to_string() + " must be integer"),
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_array_i64(arguments: &NodeArguments, column: &str) -> ArrayD<i64> {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(x.mapv(|v| if v { 1 } else { 0 })),
            ArrayND::I64(x) => Ok(x.to_owned()),
            _ => Err(column.to_string() + " must be integer")
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_str(arguments: &NodeArguments, column: &str) -> String {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Str(x) => Ok(x.first().unwrap().to_owned()),
            _ => Err(column.to_string() + " must be string"),
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_array_str(arguments: &NodeArguments, column: &str) -> ArrayD<String> {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Str(x) => Ok(x.to_owned()),
            _ => Err(column.to_string() + " must be string"),
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_bool(arguments: &NodeArguments, column: &str) -> bool {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(x.first().unwrap().to_owned()),
            _ => Err(column.to_string() + " must be boolean"),
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn get_array_bool(arguments: &NodeArguments, column: &str) -> ArrayD<bool> {
    match arguments.get(column).unwrap() {
        Value::ArrayND(array) => match array {
            ArrayND::Bool(x) => Ok(x.to_owned()),
            _ => Err(column.to_string() + " must be boolean"),
        },
        _ => Err("getters may only be called on an ArrayND".to_string())
    }.unwrap()
}

pub fn release_to_evaluations(release: &proto::Release) -> Result<GraphEvaluation, String> {
    let mut evaluations = GraphEvaluation::new();

    for (node_id, node_release) in &release.values {
        evaluations.insert(*node_id, parse_value(&node_release.value.to_owned().unwrap()).unwrap());
    }
    Ok(evaluations)
}

pub fn evaluations_to_release(evaluations: &GraphEvaluation) -> Result<proto::Release, String> {
    let mut releases: HashMap<u32, proto::ReleaseNode> = HashMap::new();
    for (node_id, node_eval) in evaluations {
        if let Ok(array_serialized) = serialize_value(node_eval) {
            releases.insert(*node_id, proto::ReleaseNode {
                value: Some(array_serialized),
                privacy_usage: None,
            });
        }
    }
    Ok(proto::Release {
        values: releases
    })
}