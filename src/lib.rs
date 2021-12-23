use pyo3::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use visioncortex::PathSimplifyMode;
use vtracer::{convert_image_to_svg, ColorMode, Config, Hierarchical};

#[pyclass]
struct Vtracer {
    conf: Config,
}

#[pymethods]
impl Vtracer {
    #[new]
    fn new() -> Self {
        Vtracer {
            conf: Config::default(),
        }
    }

    #[setter]
    fn set_input_path(&mut self, path: &str) {
        self.conf.input_path = PathBuf::from(path);
    }

    #[getter]
    fn get_input_path(&self) -> PyResult<String> {
        let path = self.conf.input_path.to_path_buf();

        match path.into_os_string().into_string() {
            Ok(s) => Ok(s),
            Err(..) => Ok("".to_string()),
        }
    }

    #[setter]
    fn set_output_path(&mut self, path: &str) {
        self.conf.output_path = PathBuf::from(path);
    }

    #[getter]
    fn get_output_path(&self) -> PyResult<String> {
        let path = self.conf.output_path.to_path_buf();

        match path.into_os_string().into_string() {
            Ok(s) => Ok(s),
            Err(..) => Ok("".to_string()),
        }
    }

    #[setter]
    fn set_color_mode(&mut self, mode: &str) {
        self.conf.color_mode = match ColorMode::from_str(mode) {
            Ok(m) => m,
            _ => ColorMode::Color,
        };
    }

    #[getter]
    fn get_color_mode(&self) -> PyResult<String> {
        match self.conf.color_mode {
            ColorMode::Binary => Ok("Binary".to_string()),
            ColorMode::Color => Ok("Color".to_string()),
        }
    }

    #[setter]
    fn set_hierarchical(&mut self, mode: &str) {
        self.conf.hierarchical = match Hierarchical::from_str(mode) {
            Ok(m) => m,
            _ => Hierarchical::Stacked,
        };
    }

    #[getter]
    fn get_hierarchical(&self) -> PyResult<String> {
        match self.conf.hierarchical {
            Hierarchical::Cutout => Ok("Cutout".to_string()),
            Hierarchical::Stacked => Ok("Stacked".to_string()),
        }
    }

    #[setter]
    fn set_path_simplify_mode(&mut self, mode: String) {
        let mode = mode.trim().to_lowercase();

        self.conf.mode = match mode.as_str() {
            "pixel" => PathSimplifyMode::None,
            "polygon" => PathSimplifyMode::Polygon,
            "spline" => PathSimplifyMode::Spline,
            _ => PathSimplifyMode::None,
        };
    }

    #[getter]
    fn get_path_simplify_mode(&self) -> PyResult<String> {
        match self.conf.mode {
            PathSimplifyMode::Polygon => Ok("Polygon".to_string()),
            PathSimplifyMode::Spline => Ok("Spline".to_string()),
            _ => Ok("Pixel".to_string()),
        }
    }

    #[setter]
    fn set_filter_speckle(&mut self, val: usize) {
        self.conf.filter_speckle = val;
    }

    #[getter]
    fn get_filter_speckle(&self) -> PyResult<usize> {
        Ok(self.conf.filter_speckle)
    }

    #[setter]
    fn set_color_precision(&mut self, val: i32) {
        self.conf.color_precision = val;
    }

    #[getter]
    fn get_color_precision(&self) -> PyResult<i32> {
        Ok(self.conf.color_precision)
    }

    #[setter]
    fn set_layer_difference(&mut self, val: i32) {
        self.conf.layer_difference = val;
    }

    #[getter]
    fn get_layer_difference(&self) -> PyResult<i32> {
        Ok(self.conf.layer_difference)
    }

    #[setter]
    fn set_corner_threshold(&mut self, val: i32) {
        self.conf.corner_threshold = val;
    }

    #[getter]
    fn get_corner_threshold(&self) -> PyResult<i32> {
        Ok(self.conf.corner_threshold)
    }

    #[setter]
    fn set_length_threshold(&mut self, val: f64) {
        self.conf.length_threshold = val;
    }

    #[getter]
    fn get_length_threshold(&self) -> PyResult<f64> {
        Ok(self.conf.length_threshold)
    }

    #[setter]
    fn set_splice_threshold(&mut self, val: i32) {
        self.conf.splice_threshold = val;
    }

    #[getter]
    fn get_splice_threshold(&self) -> PyResult<i32> {
        Ok(self.conf.splice_threshold)
    }

    #[setter]
    fn set_max_iterations(&mut self, val: usize) {
        self.conf.max_iterations = val;
    }

    #[getter]
    fn get_max_iterations(&self) -> PyResult<usize> {
        Ok(self.conf.max_iterations)
    }

    #[setter]
    fn set_path_precision(&mut self, val: u32) {
        let val = if val < 1 {
            1
        } else if val > 8 {
            8
        } else {
            val
        };
        self.conf.path_precision = Some(val);
    }

    #[getter]
    fn get_path_precision(&self) -> PyResult<u32> {
        match self.conf.path_precision {
            Some(val) => Ok(val),
            _ => Ok(8),
        }
    }

    fn to_svg(&self) {
        let config = Config {
            input_path: PathBuf::from(self.get_input_path().unwrap()),
            output_path: PathBuf::from(self.get_output_path().unwrap()),
            color_mode: match self.conf.color_mode {
                ColorMode::Binary => ColorMode::Binary,
                _ => ColorMode::Color,
            },
            hierarchical: match self.conf.hierarchical {
                Hierarchical::Cutout => Hierarchical::Cutout,
                _ => Hierarchical::Stacked,
            },
            filter_speckle: self.conf.filter_speckle,
            color_precision: self.conf.color_precision,
            layer_difference: self.conf.layer_difference,
            mode: self.conf.mode,
            corner_threshold: self.conf.corner_threshold,
            length_threshold: self.conf.length_threshold,
            splice_threshold: self.conf.splice_threshold,
            max_iterations: self.conf.max_iterations,
            path_precision: self.conf.path_precision,
        };

        convert_image_to_svg(config);
    }
}

#[pymodule]
fn pyvtracer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vtracer>()?;

    Ok(())
}
