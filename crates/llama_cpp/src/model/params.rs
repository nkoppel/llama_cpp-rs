//! Implements [`LlamaParams`]

use std::ptr;

use llama_cpp_sys::{
    llama_model_default_params, llama_model_params, llama_split_mode,
    llama_split_mode_LLAMA_SPLIT_LAYER, llama_split_mode_LLAMA_SPLIT_NONE,
    llama_split_mode_LLAMA_SPLIT_ROW,
};

/// Parameters for llama.
pub struct LlamaParams {
    /// Number of layers to store in VRAM.
    ///
    /// If this number is bigger than the amount of model layers, all layers are loaded to VRAM.
    pub n_gpu_layers: u32,

    /// How to split the model across multiple GPUs
    pub split_mode: SplitMode,

    /// The GPU that is used for scratch and small tensors
    pub main_gpu: u32,

    /// How to split layers across multiple GPUs (size: LLAMA_MAX_DEVICES)
    //const float * tensor_split, TODO

    /// Called with a progress value between 0 and 1, pass NULL to disable
    //llama_progress_callback progress_callback, TODO

    /// Context pointer passed to the progress callback
    //void * progress_callback_user_data, TODO

    /// Override key-value pairs of the model meta data
    //const struct llama_model_kv_override * kv_overrides, TODO

    /// Only load the vocabulary, no weights
    pub vocab_only: bool,

    /// Use mmap if possible
    pub use_mmap: bool,

    /// Force system to keep model in RAM
    pub use_mlock: bool,
}

/// A policy to split the model across multiple GPUs
#[non_exhaustive]
pub enum SplitMode {
    /// Single GPU.
    ///
    /// Equivalent to [`llama_split_mode_LLAMA_SPLIT_NONE`]
    None,

    /// Split layers and KV across GPUs
    ///
    /// Equivalent to [`llama_split_mode_LLAMA_SPLIT_LAYER`]
    Layer,

    /// Split rows across GPUs
    ///
    /// Equivalent to [`llama_split_mode_LLAMA_SPLIT_ROW`]
    Row,
}

impl From<SplitMode> for llama_split_mode {
    fn from(value: SplitMode) -> Self {
        match value {
            SplitMode::None => llama_split_mode_LLAMA_SPLIT_NONE,
            SplitMode::Layer => llama_split_mode_LLAMA_SPLIT_LAYER,
            SplitMode::Row => llama_split_mode_LLAMA_SPLIT_ROW,
        }
    }
}

impl From<llama_split_mode> for SplitMode {
    fn from(value: llama_split_mode) -> Self {
        #![allow(non_upper_case_globals)]
        match value {
            llama_split_mode_LLAMA_SPLIT_NONE => SplitMode::None,
            llama_split_mode_LLAMA_SPLIT_LAYER => SplitMode::Layer,
            llama_split_mode_LLAMA_SPLIT_ROW => SplitMode::Row,
            _ => unimplemented!(),
        }
    }
}

impl Default for LlamaParams {
    fn default() -> Self {
        // SAFETY: Stack constructor, always safe
        let c_params = unsafe { llama_model_default_params() };

        Self {
            n_gpu_layers: c_params.n_gpu_layers as u32,
            split_mode: c_params.split_mode.into(),
            main_gpu: c_params.main_gpu as u32,
            vocab_only: c_params.vocab_only,
            use_mmap: c_params.use_mmap,
            use_mlock: c_params.use_mlock,
        }
    }
}

impl From<LlamaParams> for llama_model_params {
    fn from(value: LlamaParams) -> Self {
        llama_model_params {
            n_gpu_layers: value.n_gpu_layers as i32,
            split_mode: value.split_mode.into(),
            main_gpu: value.main_gpu as i32,
            tensor_split: ptr::null_mut(),
            progress_callback: None,
            progress_callback_user_data: ptr::null_mut(),
            kv_overrides: ptr::null_mut(),
            vocab_only: value.vocab_only,
            use_mmap: value.use_mmap,
            use_mlock: value.use_mlock,
        }
    }
}
