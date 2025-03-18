use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DependencyInfo {
    name: String,
    req: String,
    source: String,
    kind: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    name: String,
    version: String,
    id: String,
}

const MODELS_REPO: &str = "https://github.com/tracel-ai/models.git";

// Patch resnet code (remove pretrained feature code)
const PATCH: &str = r#"diff --git a/resnet-burn/resnet/src/resnet.rs b/resnet-burn/resnet/src/resnet.rs
index e7f8787..3967049 100644
--- a/resnet-burn/resnet/src/resnet.rs
+++ b/resnet-burn/resnet/src/resnet.rs
@@ -12,13 +12,6 @@ use burn::{
 
 use super::block::{LayerBlock, LayerBlockConfig};
 
-#[cfg(feature = "pretrained")]
-use {
-    super::weights::{self, WeightsMeta},
-    burn::record::{FullPrecisionSettings, Recorder, RecorderError},
-    burn_import::pytorch::{LoadArgs, PyTorchFileRecorder},
-};
-
 // ResNet residual layer block configs
 const RESNET18_BLOCKS: [usize; 4] = [2, 2, 2, 2];
 const RESNET34_BLOCKS: [usize; 4] = [3, 4, 6, 3];
@@ -77,29 +70,6 @@ impl<B: Backend> ResNet<B> {
         ResNetConfig::new(RESNET18_BLOCKS, num_classes, 1).init(device)
     }

-    /// ResNet-18 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385)
-    /// with pre-trained weights.
-    ///
-    /// # Arguments
-    ///
-    /// * `weights`: Pre-trained weights to load.
-    /// * `device` - Device to create the module on.
-    ///
-    /// # Returns
-    ///
-    /// A ResNet-18 module with pre-trained weights.
-    #[cfg(feature = "pretrained")]
-    pub fn resnet18_pretrained(
-        weights: weights::ResNet18,
-        device: &Device<B>,
-    ) -> Result<Self, RecorderError> {
-        let weights = weights.weights();
-        let record = Self::load_weights_record(&weights, device)?;
-        let model = ResNet::<B>::resnet18(weights.num_classes, device).load_record(record);
-
-        Ok(model)
-    }
-
     /// ResNet-34 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385).
     ///
     /// # Arguments
@@ -114,29 +84,6 @@ impl<B: Backend> ResNet<B> {
         ResNetConfig::new(RESNET34_BLOCKS, num_classes, 1).init(device)
     }

-    /// ResNet-34 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385)
-    /// with pre-trained weights.
-    ///
-    /// # Arguments
-    ///
-    /// * `weights`: Pre-trained weights to load.
-    /// * `device` - Device to create the module on.
-    ///
-    /// # Returns
-    ///
-    /// A ResNet-34 module with pre-trained weights.
-    #[cfg(feature = "pretrained")]
-    pub fn resnet34_pretrained(
-        weights: weights::ResNet34,
-        device: &Device<B>,
-    ) -> Result<Self, RecorderError> {
-        let weights = weights.weights();
-        let record = Self::load_weights_record(&weights, device)?;
-        let model = ResNet::<B>::resnet34(weights.num_classes, device).load_record(record);
-
-        Ok(model)
-    }
-
     /// ResNet-50 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385).
     ///
     /// # Arguments
@@ -151,29 +98,6 @@ impl<B: Backend> ResNet<B> {
         ResNetConfig::new(RESNET50_BLOCKS, num_classes, 4).init(device)
     }

-    /// ResNet-50 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385)
-    /// with pre-trained weights.
-    ///
-    /// # Arguments
-    ///
-    /// * `weights`: Pre-trained weights to load.
-    /// * `device` - Device to create the module on.
-    ///
-    /// # Returns
-    ///
-    /// A ResNet-50 module with pre-trained weights.
-    #[cfg(feature = "pretrained")]
-    pub fn resnet50_pretrained(
-        weights: weights::ResNet50,
-        device: &Device<B>,
-    ) -> Result<Self, RecorderError> {
-        let weights = weights.weights();
-        let record = Self::load_weights_record(&weights, device)?;
-        let model = ResNet::<B>::resnet50(weights.num_classes, device).load_record(record);
-
-        Ok(model)
-    }
-
     /// ResNet-101 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385).
     ///
     /// # Arguments
@@ -188,29 +112,6 @@ impl<B: Backend> ResNet<B> {
         ResNetConfig::new(RESNET101_BLOCKS, num_classes, 4).init(device)
     }

-    /// ResNet-101 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385)
-    /// with pre-trained weights.
-    ///
-    /// # Arguments
-    ///
-    /// * `weights`: Pre-trained weights to load.
-    /// * `device` - Device to create the module on.
-    ///
-    /// # Returns
-    ///
-    /// A ResNet-101 module with pre-trained weights.
-    #[cfg(feature = "pretrained")]
-    pub fn resnet101_pretrained(
-        weights: weights::ResNet101,
-        device: &Device<B>,
-    ) -> Result<Self, RecorderError> {
-        let weights = weights.weights();
-        let record = Self::load_weights_record(&weights, device)?;
-        let model = ResNet::<B>::resnet101(weights.num_classes, device).load_record(record);
-
-        Ok(model)
-    }
-
     /// ResNet-152 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385).
     ///
     /// # Arguments
@@ -225,29 +126,6 @@ impl<B: Backend> ResNet<B> {
         ResNetConfig::new(RESNET152_BLOCKS, num_classes, 4).init(device)
     }

-    /// ResNet-152 from [`Deep Residual Learning for Image Recognition`](https://arxiv.org/abs/1512.03385)
-    /// with pre-trained weights.
-    ///
-    /// # Arguments
-    ///
-    /// * `weights`: Pre-trained weights to load.
-    /// * `device` - Device to create the module on.
-    ///
-    /// # Returns
-    ///
-    /// A ResNet-152 module with pre-trained weights.
-    #[cfg(feature = "pretrained")]
-    pub fn resnet152_pretrained(
-        weights: weights::ResNet152,
-        device: &Device<B>,
-    ) -> Result<Self, RecorderError> {
-        let weights = weights.weights();
-        let record = Self::load_weights_record(&weights, device)?;
-        let model = ResNet::<B>::resnet152(weights.num_classes, device).load_record(record);
-
-        Ok(model)
-    }
-
     /// Re-initialize the last layer with the specified number of output classes.
     pub fn with_classes(mut self, num_classes: usize) -> Self {
         let [d_input, _d_output] = self.fc.weight.dims();
@@ -256,32 +134,6 @@ impl<B: Backend> ResNet<B> {
     }
 }

-#[cfg(feature = "pretrained")]
-impl<B: Backend> ResNet<B> {
-    /// Load specified pre-trained PyTorch weights as a record.
-    fn load_weights_record(
-        weights: &weights::Weights,
-        device: &Device<B>,
-    ) -> Result<ResNetRecord<B>, RecorderError> {
-        // Download torch weights
-        let torch_weights = weights.download().map_err(|err| {
-            RecorderError::Unknown(format!("Could not download weights.\nError: {err}"))
-        })?;
-
-        // Load weights from torch state_dict
-        let load_args = LoadArgs::new(torch_weights)
-            // Map *.downsample.0.* -> *.downsample.conv.*
-            .with_key_remap("(.+)\\.downsample\\.0\\.(.+)", "$1.downsample.conv.$2")
-            // Map *.downsample.1.* -> *.downsample.bn.*
-            .with_key_remap("(.+)\\.downsample\\.1\\.(.+)", "$1.downsample.bn.$2")
-            // Map layer[i].[j].* -> layer[i].blocks.[j].*
-            .with_key_remap("(layer[1-4])\\.([0-9]+)\\.(.+)", "$1.blocks.$2.$3");
-        let record = PyTorchFileRecorder::<FullPrecisionSettings>::new().load(load_args, device)?;
-
-        Ok(record)
-    }
-}
-
 /// [ResNet](ResNet) configuration.
 struct ResNetConfig {
     conv1: Conv2dConfig,
"#;

fn run<F>(name: &str, mut configure: F)
where
    F: FnMut(&mut Command) -> &mut Command,
{
    let mut command = Command::new(name);
    let configured = configure(&mut command);
    println!("Executing {:?}", configured);
    if !configured.status().unwrap().success() {
        panic!("failed to execute {:?}", configured);
    }
    println!("Command {:?} finished successfully", configured);
}

fn clone_resnet_source() {
    let models_dir = std::env::temp_dir().join("models");
    let models_dir = models_dir.as_path();
    // Checkout ResNet code from models repo
    let models_dir = Path::new(models_dir);
    if !models_dir.join(".git").exists() {
        run("git", |command| {
            command
                .arg("clone")
                .arg("--depth=1")
                .arg("--no-checkout")
                .arg(MODELS_REPO)
                .arg(models_dir)
        });

        run("git", |command| {
            command
                .current_dir(models_dir)
                .arg("sparse-checkout")
                .arg("set")
                .arg("resnet-burn")
        });

        run("git", |command| {
            command.current_dir(models_dir).arg("checkout")
        });

        let patch_file = models_dir.join("benchmark.patch");

        fs::write(&patch_file, PATCH).expect("should write to file successfully");

        // Apply patch
        run("git", |command| {
            command
                .current_dir(models_dir)
                .arg("apply")
                .arg(patch_file.to_str().unwrap())
        });
    }

    // Copy contents to output dir
    let out_dir = env::var("OUT_DIR").unwrap();
    let source_path = models_dir.join("resnet-burn").join("resnet").join("src");
    let dest_path = Path::new(&out_dir);

    if let Ok(source_path) = fs::read_dir(source_path) {
        for file in source_path {
            let source_file = file.unwrap().path();
            let dest_file = dest_path.join(source_file.file_name().unwrap());
            fs::copy(source_file, dest_file).expect("should copy file successfully");
        }
    }

    // Delete cloned repository contents
    fs::remove_dir_all(models_dir.join(".git")).unwrap();
    fs::remove_dir_all(models_dir).unwrap();
}

fn capture_packages_info() {
    let package_name = env!("CARGO_PKG_NAME");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");

    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Invalid JSON from cargo metadata");

    // Start generating Rust code
    let mut code = String::new();
    code.push_str("use phf::phf_map;\n\n");

    // Define the PackageInfo struct
    code.push_str("#[derive(Debug)]\n");
    code.push_str("pub struct PackageInfo {\n");
    code.push_str("    pub version: &'static str,\n");
    code.push_str("    pub source: &'static str,\n");
    code.push_str("}\n\n");

    // Extract dependencies, versions, and sources
    let dependencies = metadata
        .get("packages")
        .and_then(|p| p.as_array())
        .expect("Should parse dependencies");

    // Find the direct dependencies of this package
    let mut direct_dependencies: Vec<DependencyInfo> = dependencies
        .iter()
        .find(|p| p.get("name").and_then(|v| v.as_str()) == Some(&package_name))
        .and_then(|p| {
            p.get("dependencies").map(|deps| {
                serde_json::from_value(deps.clone()).expect("Should parse direct dependencies")
            })
        })
        .expect("Should find direct dependencies");
    // println!("cargo::warning={direct_dependencies:?}");

    let packages: HashMap<String, PackageInfo> = serde_json::from_value::<Vec<PackageInfo>>(
        metadata
            .get("packages")
            .expect("Should parse packages")
            .clone(),
    )
    .unwrap()
    .into_iter()
    .map(|pkg| (pkg.name.clone(), pkg))
    .collect();

    let mut deps_str =
        String::from("pub static DEPENDENCIES: phf::Map<&'static str, PackageInfo> = phf_map! {\n");
    let mut deps_dev_str = String::from(
        "pub static DEPENDENCIES_DEV: phf::Map<&'static str, PackageInfo> = phf_map! {\n",
    );
    let mut deps_build_str = String::from(
        "pub static DEPENDENCIES_BUILD: phf::Map<&'static str, PackageInfo> = phf_map! {\n",
    );
    // println!("cargo::warning={direct_dependencies:?}");
    direct_dependencies.iter_mut().for_each(|dep| {
        if let Some(pkg) = packages.get(&dep.name) {
            // Overwrite dependency info with actual package info
            dep.source = pkg.id.clone();
            dep.req = pkg.version.clone();

            // Cannot easily threshold based on git revision (commit hash), so
            // we only check for version 0.17.0 to set `burn_version_lt_0170`
            // since it is the lowest comparison point at this point
            if dep.name == "burn" {
                let pkg_version =
                    semver::Version::parse(&pkg.version).expect("Invalid version format");
                if pkg_version < semver::Version::new(0, 17, 0) {
                    println!("cargo:rustc-cfg=burn_version_lt_0170");
                }
            }
        }

        let pkg_info = format!(
            "    \"{}\" => PackageInfo {{ version: \"{}\", source: \"{}\" }},\n",
            dep.name, dep.req, dep.source
        );

        match &dep.kind {
            Some(kind) => {
                if kind == "dev" {
                    deps_dev_str.push_str(&pkg_info);
                } else if kind == "build" {
                    deps_build_str.push_str(&pkg_info);
                } else {
                    unreachable!("Unexpexted dependency kind {kind}")
                }
            }
            None => deps_str.push_str(&pkg_info),
        }
    });
    deps_str.push_str("};\n");
    deps_dev_str.push_str("};\n");
    deps_build_str.push_str("};\n");
    // println!("cargo::warning={direct_dependencies:?}");

    code.push_str(&format!("{deps_str}\n{deps_dev_str}\n{deps_build_str}"));

    // Write the generated code to `OUT_DIR`
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("metadata.rs");
    fs::write(dest_path, code).expect("Failed to write metadata.rs");
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(burn_version_lt_0170)");

    // For the ResNet benchmark we need to clone the source since we want it to use the selected burn version or revision
    clone_resnet_source();

    // Capture the burn version used
    capture_packages_info();
}
