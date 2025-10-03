import numpy as np
from PIL import Image

size = 64  # texture resolution
tex = np.zeros((size, size, 3), dtype=np.float32)

for y in range(size):
    for x in range(size):
        u = x / (size - 1)
        v = y / (size - 1)

        # Distances to edges (0 at edge, 1 at opposite edge)
        dist_left   = u
        dist_right  = 1.0 - u
        dist_top    = v
        dist_bottom = 1.0 - v

        wave_fac = 0.1
        # Strength of each border (closer = stronger)
        edge_thickness = 0.25  # controls width of border influence
        w_left   = max(0.0, (edge_thickness - dist_left)   / edge_thickness) * ((np.sin(y * wave_fac) + 1.0) * 0.1)
        w_right  = max(0.0, (edge_thickness - dist_right)  / edge_thickness) * ((np.cos(y * wave_fac) + 1.0) * 0.1)
        w_top    = max(0.0, (edge_thickness - dist_top)    / edge_thickness) * ((np.cos(x * wave_fac) + 1.0) * 0.1)
        w_bottom = max(0.0, (edge_thickness - dist_bottom) / edge_thickness) * ((np.sin(x * wave_fac) + 1.0) * 0.1)

        # Base weights (center always included)
        w_center = 1.0
        weights = np.array([w_center, w_left, w_right, w_top, w_bottom])

        # Normalize so everything sums to 1
        weights /= weights.sum()

        # Define colors
        c_center = np.array([0.5, 0.5, 1.0])
        c_left   = np.array([1.0, 0.5, 1.0])
        c_right  = np.array([1.0, 0.5, 1.0])
        c_top    = np.array([0.5, 1.0, 1.0])
        c_bottom = np.array([0.5, 1.0, 1.0])

        colors = np.stack([c_center, c_left, c_right, c_top, c_bottom], axis=0)

        # Weighted blend
        tex[y, x] = (weights[:, None] * colors).sum(axis=0)

# Save as RGB PNG
img = Image.fromarray((tex * 255).astype(np.uint8), mode="RGB")
img.save("offsets.png")
print("Saved")
