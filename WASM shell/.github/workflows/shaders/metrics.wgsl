struct TileStats {
    count: u32,
    sum_prev: f32, sum_curr: f32,
    sum_prev_sq: f32, sum_curr_sq: f32,
    sum_prev_curr: f32,
    entropy: f32,
}

@group(0) @binding(0) var prev: texture_2d<f32>;
@group(0) @binding(1) var curr: texture_2d<f32>;
@group(0) @binding(2) var<storage, read_write> tiles: array<TileStats>;

var<workgroup> wg_count: u32;
var<workgroup> wg_sum_prev: f32;
var<workgroup> wg_sum_curr: f32;
var<workgroup> wg_sum_prev_sq: f32;
var<workgroup> wg_sum_curr_sq: f32;
var<workgroup> wg_sum_prev_curr: f32;
var<workgroup> wg_entropy: f32;

// Luminance conversion
fn to_lum(c: vec3f) -> f32 {
    return dot(c, vec3f(0.299, 0.587, 0.114));
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid: vec3u, @builtin(local_invocation_id) lid: vec3u) {
    let dims = textureDimensions(prev);
    if gid.x >= dims.x || gid.y >= dims.y { return; }

    let px_prev = to_lum(textureLoad(prev, gid.xy, 0).rgb);
    let px_curr = to_lum(textureLoad(curr, gid.xy, 0).rgb);

    // Accumulate stats in workgroup memory
    atomicAdd(&wg_count, 1u);
    atomicAdd(&wg_sum_prev, px_prev);
    atomicAdd(&wg_sum_curr, px_curr);
    atomicAdd(&wg_sum_prev_sq, px_prev * px_prev);
    atomicAdd(&wg_sum_curr_sq, px_curr * px_curr);
    atomicAdd(&wg_sum_prev_curr, px_prev * px_curr);

    // Entropy binning (quantized to 16 levels for speed in shared mem)
    let bin = u32(px_curr * 15.0);
    if bin < 16 { atomicAdd(&wg_entropy, log(1.0 + 1.0) * 0.1); } // Placeholder: true hist needs 256 array, simplified for perf
}

@compute @workgroup_size(1, 1, 1)
fn reduce(@builtin(global_invocation_id) gid: vec3u) {
    let tile_idx = gid.x;
    if tile_idx >= arrayLength(&tiles) { return; }

    let n = wg_count;
    if n == 0 { return; }

    let inv_n = 1.0 / f32(n);
    let m_x = wg_sum_prev * inv_n;
    let m_y = wg_sum_curr * inv_n;
    let v_x = wg_sum_prev_sq * inv_n - m_x * m_x;
    let v_y = wg_sum_curr_sq * inv_n - m_y * m_y;
    let cov = wg_sum_prev_curr * inv_n - m_x * m_y;

    // SSIM constants
    let C1 = (0.01 * 255.0) * (0.01 * 255.0);
    let C2 = (0.03 * 255.0) * (0.03 * 255.0);

    let ssim = ((2.0 * m_x * m_y + C1) * (2.0 * cov + C2)) /
               ((m_x * m_x + m_y * m_y + C1) * (v_x + v_y + C2));

    tiles[tile_idx] = TileStats(n, wg_sum_prev, wg_sum_curr, wg_sum_prev_sq, wg_sum_curr_sq, wg_sum_prev_curr, ssim);
}
