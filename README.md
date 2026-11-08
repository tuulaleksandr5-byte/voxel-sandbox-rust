# Voxel Sandbox — Windows x64

[Скачать ZIP с EXE](VoxelSandbox-Windows-x64.zip?raw=true). Распакуйте весь архив и запустите `VoxelSandbox.exe`. При обновлении сохраните папку `saves`.

Обновление рендерера: texture array 18×128×128, Linear + мипмапы + анизотропия ×8, назначение граней в RON, угловой AO, Gaussian-тени, градиент неба и туман.

**Оригинальный пользовательский `atlas_128x128.png` не предоставлен и не включён.** Пока используются временные гладкие процедурные поверхности. Для оригинального рисунка положите PNG 768×384 в `assets/textures/atlas_128x128.png` рядом с EXE и перезапустите игру.

Управление: Ctrl — бег/ускорение полёта; двойное W — бег; двойной Space — полёт, Space/Shift — вверх/вниз. Колёсико — слот, F3 — координаты/FPS, F3+G — чанки. Интерфейс русский.

Проверены 73 теста Linux и 32 теста Windows-сборок через Wine. Упакованный EXE запускается и рисует мир через Wine/Vulkan. Настоящая Windows и аппаратный GPU ещё не проверены.

Исходники и инструкция: [feature/playable-foundation](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/feature/playable-foundation), [рендерер](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/blob/feature/playable-foundation/docs/ATLAS_RENDERER.md).

Source commit: `ebd26b288a1ea76b2b57836a5a64634ddc266622`

EXE SHA-256: `c6a28d60ec9178977f0b51c2c7562f218d44f877a0e753dd2705db488f327d9c`

ZIP SHA-256: `ef56b86dddc0eb43f29b48df3323ed17eee28efa16d1909e75c519fc6bb9a5c4`
