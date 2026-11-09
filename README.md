# Voxel Sandbox — этап 3, новые текстуры инструментов

[Скачать Windows EXE в ZIP](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/raw/download/2.0-stage3-tools-windows-x64/VoxelSandbox-2.0-Stage3-Tools-Windows-x64.zip)

Распакуйте весь ZIP и запустите `VoxelSandbox.exe`.

Деревянная кирка-образец использует оригинальный PNG из вашего `tools.zip`. Все 77 PNG сохранены без изменения; загрузчик поддерживает 25 изображений. В действующем каталоге пока только деревянная кирка — новые инструменты и игровая механика в это обновление не входят.

Текстуры: `assets/textures/tools/` в архиве и [в исходниках](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/a9bd00bde4901aa44af5f5c10eea6540e6f33311/assets/textures/tools).

Инвентарь, дроп и сундуки пока действуют до закрытия мира. Полное сохранение новых систем — этап 9; используйте отдельную копию мира для альфы.

Проверены 184 Linux-теста и 183 Windows-теста через Wine, Clippy, сборка и запуск Windows EXE без assets. Скриншоты проверенного значка — в `qa/`. Настоящая Windows-машина не проверена.

[Исходный код](https://github.com/tuulaleksandr5-byte/voxel-sandbox-rust/tree/a9bd00bde4901aa44af5f5c10eea6540e6f33311) · [Полный diff обновления](tool-textures.patch) · [Контрольные суммы](SHA256SUMS)
