use std::f64::consts::PI;

// ============================================================================
// 1. LIGHT WAVE PROPERTIES (COMPLEX NUMBERS LAYER)
// ============================================================================
#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    #[inline(always)]
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    #[inline(always)]
    fn polar(amplitude: f64, phase: f64) -> Self {
        Complex {
            re: amplitude * phase.cos(),
            im: amplitude * phase.sin(),
        }
    }

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    #[inline(always)]
    fn intensity(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

// ============================================================================
// 2. PHOTONIC LATTICE COMPONENTS (INTERFEROMETER SWITCHES)
// ============================================================================
#[derive(Debug, Clone, Copy)]
struct MziGate {
    theta: f64,
    phi: f64,
}

impl MziGate {
    #[inline(always)]
    fn new(theta: f64, phi: f64) -> Self {
        MziGate { theta, phi }
    }

    #[inline(always)]
    fn transform(&self, input_top: Complex, input_bottom: Complex) -> (Complex, Complex) {
        let cos_t = (self.theta / 2.0).cos();
        let sin_t = (self.theta / 2.0).sin();
        let exp_i_phi = Complex::polar(1.0, self.phi);
        let i_comp = Complex::new(0.0, 1.0);

        let m00 = Complex::new(cos_t, 0.0);
        let m01 = i_comp.mul(Complex::new(sin_t, 0.0));
        let m10 = exp_i_phi.mul(i_comp).mul(Complex::new(sin_t, 0.0));
        let m11 = exp_i_phi.mul(Complex::new(cos_t, 0.0));

        let out_top = m00.mul(input_top).add(m01.mul(input_bottom));
        let out_bottom = m10.mul(input_top).add(m11.mul(input_bottom));

        (out_top, out_bottom)
    }
}

// ============================================================================
// 3. PHOTONIC NETWORK MESH (4-CHANNEL GEOMETRIC ROUTING LATTICE)
// ============================================================================
#[derive(Clone)]
struct PhotonicNetworkMesh {
    gates: [MziGate; 6], // Array of 6 hardware gates mapping 4 waveguides
}

impl PhotonicNetworkMesh {
    fn forward(&self, mut wave_vector: [Complex; 4]) -> [Complex; 4] {
        // --- Layer 1 ---
        let (r0, r1) = self.gates[0].transform(wave_vector[0], wave_vector[1]);
        let (r2, r3) = self.gates[1].transform(wave_vector[2], wave_vector[3]);
        wave_vector = [r0, r1, r2, r3];

        // --- Layer 2 ---
        let (r1_n, r2_n) = self.gates[2].transform(wave_vector[1], wave_vector[2]);
        wave_vector[1] = r1_n; wave_vector[2] = r2_n;

        // --- Layer 3 ---
        let (r0_n, r1_f) = self.gates[3].transform(wave_vector[0], wave_vector[1]);
        let (r2_f, r3_n) = self.gates[4].transform(wave_vector[2], wave_vector[3]);
        wave_vector = [r0_n, r1_f, r2_f, r3_n];

        // --- Layer 4 ---
        let (r1_out, r2_out) = self.gates[5].transform(wave_vector[1], wave_vector[2]);
        wave_vector[1] = r1_out; wave_vector[2] = r2_out;

        wave_vector
    }
}

// ============================================================================
// 4. EVOLUTIONARY CO-PROCESSOR TRAINING ENGINE
// ============================================================================
struct PhotonicTrainer;

impl PhotonicTrainer {
    fn evaluate_loss(mesh: &PhotonicNetworkMesh, dataset: &Vec<([f64; 4], usize)>) -> f64 {
        let mut total_error = 0.0;

        for (features, target_channel) in dataset {
            // Encode numeric dataset profiles directly into coherent laser amplitudes
            let input_vector = [
                Complex::polar(features[0], 0.0),
                Complex::polar(features[1], 0.0),
                Complex::polar(features[2], 0.0),
                Complex::polar(features[3], 0.0),
            ];

            let output = mesh.forward(input_vector);
            let measured_brightness = output[*target_channel].intensity();
            
            // Error mapping boundary: distance squared to absolute 1.0 peak power routing
            total_error += (1.0 - measured_brightness).powi(2);
        }

        total_error / (dataset.len() as f64)
    }

    // High-velocity LCG pseudo-random number generator for zero heap/allocation mutations
    fn fast_rand(seed: &mut u32) -> f64 {
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        ((*seed >> 16) as f64) / 65535.0
    }

    fn train(mut mesh: PhotonicNetworkMesh, dataset: &Vec<([f64; 4], usize)>, epochs: usize) -> PhotonicNetworkMesh {
        let mut rand_seed = 424242;
        let mut current_loss = Self::evaluate_loss(&mesh, dataset);
        let mutation_scale = 0.05;

        println!("Initial Hardware Baseline Loss: {:.6}", current_loss);

        for epoch in 1..=epochs {
            let mut candidate_mesh = mesh.clone();
            
            // Randomly tweak structural parameters across our hardware mesh arrays
            for gate in &mut candidate_mesh.gates {
                gate.theta += (Self::fast_rand(&mut rand_seed) - 0.5) * mutation_scale;
                gate.phi += (Self::fast_rand(&mut rand_seed) - 0.5) * mutation_scale;
                
                gate.theta = gate.theta.clamp(0.0, 2.0 * PI);
                gate.phi = gate.phi.clamp(0.0, 2.0 * PI);
            }

            let candidate_loss = Self::evaluate_loss(&candidate_mesh, dataset);

            // Keep adjustments if they successfully focus the light waves better
            if candidate_loss < current_loss {
                mesh = candidate_mesh;
                current_loss = candidate_loss;
            }

            if epoch % 500 == 0 || epoch == 1 {
                println!("Epoch {:04} | Aggregated Hardware Training Loss: {:.6}", epoch, current_loss);
            }
        }

        mesh
    }
}

// ============================================================================
// 5. ML HARDWARE GATEWAY INTERFACE
// ============================================================================
// Keep all your previous structs (Complex, MziGate, PhotonicNetworkMesh, PhotonicTrainer) exactly the same.
// Just scroll to the very bottom and replace your `fn main()` block with this updated dashboard interface:

fn main() {
    println!("=============================================================");
    println!("🧬 TRAINING ENGINE INITIALIZED: OPTICAL NEURAL NETWORK (ONN)");
    println!("=============================================================");

    let machine_learning_dataset = vec![
        ([0.9, 0.1, 0.0, 0.0], 0), 
        ([0.8, 0.2, 0.1, 0.0], 0), 
        ([0.0, 0.0, 0.1, 0.9], 3), 
        ([0.1, 0.0, 0.2, 0.8], 3), 
    ];

    let initial_mesh = PhotonicNetworkMesh {
        gates: [MziGate::new(1.0, 0.5); 6],
    };

    println!("Commencing Optical Physical Training Over Silicon Mesh...");
    let trained_mesh = PhotonicTrainer::train(initial_mesh, &machine_learning_dataset, 3000);

    println!("\n=============================================================");
    println!("📊 HARDWARE DIAGNOSTICS: PHOTONIC LATTICE VISUALIZER");
    println!("=============================================================");

    // Test Pattern A (Should route cleanly to Channel 0)
    let test_pattern = [0.95, 0.05, 0.0, 0.0];
    let encoded_test = [
        Complex::polar(test_pattern[0], 0.0),
        Complex::polar(test_pattern[1], 0.0),
        Complex::polar(test_pattern[2], 0.0),
        Complex::polar(test_pattern[3], 0.0),
    ];
    
    let output_light = trained_mesh.forward(encoded_test);
    
    // Calculate intensities for energy routing visualization
    let i0 = output_light[0].intensity();
    let i1 = output_light[1].intensity();
    let i2 = output_light[2].intensity();
    let i3 = output_light[3].intensity();

    // Helper closure to draw energy density paths visually
    let draw_beam = |intensity: f64| -> &'static str {
        if intensity > 0.75 { "========>>" }
        else if intensity > 0.25 { "-------->" }
        else { ".........." }
    };

    println!("Injected Input Laser Matrix: [{:.2}, {:.2}, {:.2}, {:.2}]", test_pattern[0], test_pattern[1], test_pattern[2], test_pattern[3]);
    println!("\nLattice Propagation Flow Path map:");
    println!("Ch 0  {0} [MZI 0] {0}-----------{0} [MZI 3] {0}===> Recv 0 Intensity: {1:.4}", draw_beam(test_pattern[0]), i0);
    println!("                \\                 /");
    println!("Ch 1  {0}---------[MZI 2] {0}--------                        ===> Recv 1 Intensity: {1:.4}", draw_beam(test_pattern[1]), i1);
    println!("                          \\                              /");
    println!("Ch 2  {0}---------[MZI 1] {0}--------                        ===> Recv 2 Intensity: {1:.4}", draw_beam(test_pattern[2]), i2);
    println!("                /                 \\");
    println!("Ch 3  {0} [MZI 4] {0}-----------{0} [MZI 5] {0}===> Recv 3 Intensity: {1:.4}", draw_beam(test_pattern[3]), i3);
    println!("=============================================================");
}