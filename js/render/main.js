/**
 * Phase 1: static night box-castle scene (no sim sync yet).
 * Three.js is loaded from CDN as an ES module.
 */
import * as THREE from "https://unpkg.com/three@0.170.0/build/three.module.js";

const canvas = document.getElementById("game-canvas");
if (!canvas) {
  console.warn("[render] #game-canvas not found");
} else {
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: true,
    alpha: false,
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.setClearColor(0x0b1020, 1);

  const scene = new THREE.Scene();
  scene.fog = new THREE.Fog(0x0b1020, 40, 90);

  const camera = new THREE.PerspectiveCamera(45, 1, 0.1, 200);
  camera.position.set(18, 14, 22);
  camera.lookAt(0, 2, 0);

  const ambient = new THREE.AmbientLight(0x6a7aaa, 0.45);
  scene.add(ambient);
  const moon = new THREE.DirectionalLight(0xc8d4ff, 0.85);
  moon.position.set(-10, 20, 8);
  scene.add(moon);

  const ground = new THREE.Mesh(
    new THREE.BoxGeometry(36, 0.4, 36),
    new THREE.MeshStandardMaterial({ color: 0x1a2338, roughness: 0.95 }),
  );
  ground.position.y = -0.2;
  scene.add(ground);

  const wallMat = new THREE.MeshStandardMaterial({
    color: 0x4a5568,
    roughness: 0.85,
  });
  const wallH = 3.2;
  const wallT = 1.2;
  const wallL = 16;
  const walls = [
    new THREE.Mesh(new THREE.BoxGeometry(wallL, wallH, wallT), wallMat),
    new THREE.Mesh(new THREE.BoxGeometry(wallL, wallH, wallT), wallMat),
    new THREE.Mesh(new THREE.BoxGeometry(wallT, wallH, wallL), wallMat),
    new THREE.Mesh(new THREE.BoxGeometry(wallT, wallH, wallL), wallMat),
  ];
  walls[0].position.set(0, wallH / 2, -wallL / 2);
  walls[1].position.set(0, wallH / 2, wallL / 2);
  walls[2].position.set(-wallL / 2, wallH / 2, 0);
  walls[3].position.set(wallL / 2, wallH / 2, 0);
  for (const w of walls) scene.add(w);

  const keep = new THREE.Mesh(
    new THREE.BoxGeometry(5, 6, 5),
    new THREE.MeshStandardMaterial({ color: 0x3d4a5c, roughness: 0.8 }),
  );
  keep.position.set(0, 3, 0);
  scene.add(keep);

  const towerMat = new THREE.MeshStandardMaterial({ color: 0x5c6b7a });
  for (const [x, z] of [
    [-7, -7],
    [7, -7],
    [-7, 7],
    [7, 7],
  ]) {
    const tower = new THREE.Mesh(new THREE.BoxGeometry(2.2, 5, 2.2), towerMat);
    tower.position.set(x, 2.5, z);
    scene.add(tower);
  }

  function resize() {
    const rect = canvas.getBoundingClientRect();
    const w = Math.max(1, Math.floor(rect.width));
    const h = Math.max(1, Math.floor(rect.height));
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }

  window.addEventListener("resize", resize);
  resize();

  function frame() {
    renderer.render(scene, camera);
    requestAnimationFrame(frame);
  }
  requestAnimationFrame(frame);

  window.__tdRender = { scene, camera, renderer, THREE };
}
