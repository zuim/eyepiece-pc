import { describe, expect, it } from "vitest";
import { poseScene, rotatePose } from "./pose-model";

describe("desktop-compatible pose projection", () => {
  it("uses the desktop yaw, pitch and roll conventions", () => {
    const right = rotatePose([0, 0, -1], { yaw: 90, pitch: 0, roll: 0 });
    expect(right[0]).toBeCloseTo(1);
    const up = rotatePose([0, 0, -1], { yaw: 0, pitch: 90, roll: 0 });
    expect(up[1]).toBeCloseTo(1);
    const rolled = rotatePose([0, 1, 0], { yaw: 0, pitch: 0, roll: 90 });
    expect(rolled[0]).toBeCloseTo(1);
  });

  it("projects the horizon, ordered goggles faces and direction arrow", () => {
    const scene = poseScene(300, 180, { yaw: 0, pitch: 0, roll: 0 });
    expect(scene.ring).toHaveLength(49);
    expect(scene.ring[0][0]).toBeCloseTo(scene.ring[48][0]);
    expect(scene.ring[0][1]).toBeCloseTo(scene.ring[48][1]);
    expect(scene.faces).toHaveLength(6);
    expect(scene.faces.filter((face) => face.front)).toHaveLength(1);
    expect(scene.faces.every((face, index) => index === 0 || scene.faces[index - 1].depth <= face.depth)).toBe(true);
    expect(scene.arrow[1][1]).toBeLessThan(scene.arrow[0][1]);
    expect(scene.shadow).toHaveLength(2);
  });
});
