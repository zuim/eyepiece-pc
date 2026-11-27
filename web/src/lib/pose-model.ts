export type Pose = { yaw: number; pitch: number; roll: number };
type Vec3 = [number, number, number];
type Point = [number, number];

const box: Vec3[] = [
  [-0.9, -0.36, -0.3],
  [0.9, -0.36, -0.3],
  [0.9, 0.36, -0.3],
  [-0.9, 0.36, -0.3],
  [-0.9, -0.36, 0.3],
  [0.9, -0.36, 0.3],
  [0.9, 0.36, 0.3],
  [-0.9, 0.36, 0.3],
];
const faces = [
  { indices: [0, 1, 2, 3], front: true },
  { indices: [5, 4, 7, 6], front: false },
  { indices: [4, 0, 3, 7], front: false },
  { indices: [1, 5, 6, 2], front: false },
  { indices: [3, 2, 6, 7], front: false },
  { indices: [4, 5, 1, 0], front: false },
];

export function rotatePose([x, y, z]: Vec3, pose: Pose): Vec3 {
  const radians = Math.PI / 180;
  let s = Math.sin(-pose.roll * radians);
  let c = Math.cos(-pose.roll * radians);
  [x, y] = [x * c - y * s, x * s + y * c];
  s = Math.sin(pose.pitch * radians);
  c = Math.cos(pose.pitch * radians);
  [y, z] = [y * c - z * s, y * s + z * c];
  s = Math.sin(-pose.yaw * radians);
  c = Math.cos(-pose.yaw * radians);
  [x, z] = [x * c + z * s, -x * s + z * c];
  return [x, y, z];
}

export function poseScene(width: number, height: number, pose: Pose) {
  const f = Math.min(height, width * 0.75) * 1.2;
  const s = 0.5;
  const c = Math.sqrt(3) / 2;
  const project = ([x, y, z]: Vec3): { point: Point; depth: number } => {
    [y, z] = [y * c - z * s, y * s + z * c];
    const scale = f / (4.6 - z);
    return { point: [width / 2 + x * scale, height / 2 + height * 0.08 - y * scale], depth: z };
  };
  const rotate = (point: Vec3) => rotatePose(point, pose);
  const corners = box.map((point) => project(rotate(point)));
  const projectedFaces = faces
    .map((face) => ({
      front: face.front,
      points: face.indices.map((index) => corners[index].point),
      depth: face.indices.reduce((sum, index) => sum + corners[index].depth, 0) / 4,
    }))
    .sort((a, b) => a.depth - b.depth);
  const ring = Array.from({ length: 49 }, (_, i) => {
    const angle = (i / 48) * 2 * Math.PI;
    return project([Math.sin(angle) * 1.5, -0.75, Math.cos(angle) * 1.5]).point;
  });
  const forward = rotate([0, 0, -2.2]);
  const length = Math.hypot(forward[0], forward[2]);
  return {
    faces: projectedFaces,
    ring,
    tick: [project([0, -0.75, -1.5]).point, project([0, -0.75, -1.9]).point],
    shadow:
      length > 0.05
        ? [
            project([0, -0.75, 0]).point,
            project([(forward[0] / length) * 1.5, -0.75, (forward[2] / length) * 1.5]).point,
          ]
        : [],
    arrow: [
      project(rotate([0, 0, -0.3])).point,
      project(forward).point,
      project(rotate([-0.22, 0, -1.85])).point,
      project(rotate([0.22, 0, -1.85])).point,
    ],
  };
}
