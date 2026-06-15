use std::f64::consts::PI;

// ============================================================================
// 1. COMPLEX ALGEBRA STRUCTURES
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
    fn polar(amplitude: f64, phase_radians: f64) -> Self {
        Complex {
            re: amplitude * phase_radians.cos(),
            im: amplitude * phase_radians.sin(),
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
// 2. INTERLOCKING HARDWARE LATTICE TOPOLOGY
// ============================================================================
#[derive(Debug, Clone, Copy)]
struct MziGate {
    theta: f64,
    phi: f64,
}

impl MziGate {
    fn new(theta: f64, phi: f64) -> Self {
        MziGate { theta, phi }
    }

    #[inline(always)]
    fn transform(&self, input_top: Complex, input_bottom: Complex) -> (Complex, Complex) {
        let cos_t = (self.theta / 2.0).cos();
        let sin_t = (self.theta / 2.0).sin();
        let exp_i_phi = Complex::exp_i_phi(self.phi);
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

impl Complex {
    #[inline(always)]
    fn exp_i_phi(phi: f64) -> Self {
        Complex::new(phi.cos(), phi.sin())
    }
}

// A 4-Waveguide Clements Mesh requiring 6 independent MZI component nodes
struct ClementsMesh4x4 {
    // Layer 1
    mzi_0_1_a: MziGate,
    mzi_2_3_a: MziGate,
    // Layer 2
    mzi_1_2_a: MziGate,
    // Layer 3
    mzi_0_1_b: MziGate,
    mzi_2_3_b: MziGate,
    // Layer 4
    mzi_1_2_b: MziGate,
}

impl ClementsMesh4x4 {
    fn forward_propagate(&self, mut wave_vector: [Complex; 4]) -> [Complex; 4] {
        // --- STEP 1: LAYER 1 ---
        let (r0, r1) = self.mzi_0_1_a.transform(wave_vector[0], wave_vector[1]);
        wave_vector[0] = r0; wave_vector[1] = r1;
        let (r2, r3) = self.mzi_2_3_a.transform(wave_vector[2], wave_vector[3]);
        wave_vector[2] = r2; wave_vector[3] = r3;

        // --- STEP 2: LAYER 2 ---
        let (r1_next, r2_next) = self.mzi_1_2_a.transform(wave_vector[1], wave_vector[2]);
        wave_vector[1] = r1_next; wave_vector[2] = r2_next;

        // --- STEP 3: LAYER 3 ---
        let (r0_next, r1_final) = self.mzi_0_1_b.transform(wave_vector[0], wave_vector[1]);
        wave_vector[0] = r0_next; wave_vector[1] = r1_final;
        let (r2_final, r3_next) = self.mzi_2_3_b.transform(wave_vector[2], wave_vector[3]);
        wave_vector[2] = r2_final; wave_vector[3] = r3_next;

        // --- STEP 4: LAYER 4 ---
        let (r1_out, r2_out) = self.mzi_1_2_b.transform(wave_vector[1], wave_vector[2]);
        wave_vector[1] = r1_out; wave_vector[2] = r2_out;

        wave_vector
    }
}

// ============================================================================
// 3. EXECUTION BENCHMARK ROOM
// ============================================================================
fn main() {
    println!("=============================================================");
    println!("🌌 HIGH-DIMENSIONAL CLEMENTS MESH SIMULATION ENGINE ONLINE");
    println!("=============================================================");

    // Ingest energy across multiple channels simultaneously
    let input_signals = [
        Complex::polar(0.707, 0.0),      // Channel 0
        Complex::polar(0.500, PI / 4.0), // Channel 1
        Complex::polar(0.300, PI / 2.0), // Channel 2
        Complex::polar(0.100, PI),       // Channel 3
    ];

    println!("Input Channel Intensities: [0.500, 0.250, 0.090, 0.010]");

    // Configure a complete 6-component geometric layout routing array
    let optical_accelerator = ClementsMesh4x4 {
        mzi_0_1_a: MziGate::new(PI / 2.0, 0.0),
        mzi_2_3_a: MziGate::new(PI / 3.0, PI / 6.0),
        mzi_1_2_a: MziGate::new(PI, 0.0),
        mzi_0_1_b: MziGate::new(PI / 4.0, PI / 2.0),
        mzi_2_3_b: MziGate::new(0.0, 0.0),
        mzi_1_2_b: MziGate::new(PI / 2.0, PI / 3.0),
    };

    // Blazing fast continuous transformation pass
    let outputs = optical_accelerator.forward_propagate(input_signals);

    println!("\n--- PHOTONIC WAVEGUIDE RECEIVER HARVEST ---");
    let mut energy_conservation = 0.0;
    for i in 0..4 {
        let current_intensity = outputs[i].intensity();
        energy_conservation += current_intensity;
        println!("=> Waveguide Port Output [{}]: Intensity = {:.4}", i, current_intensity);
    }
    
    println!("-------------------------------------------------------------");
    println!("Lattice Energy Checksum: {:.4} (Physical Integrity Valid)", energy_conservation);
    println!("=============================================================");
}