# アセットライセンス

## 方針

本リポジトリの 3D モデルは、将来 [Kenney](https://kenney.nl/) など **CC0** 公開アセットへ差し替える前提のプレースホルダです。

| パス | 状態 | ライセンス |
| --- | --- | --- |
| `assets/models/tower_cannon.gltf` | 手書きの最小 glTF（箱） | 本リポジトリと同じ（プレースホルダ） |

## Kenney CC0（意図）

- Kenney アセットを導入する場合は **CC0 1.0**（パブリックドメイン相当）として扱う。
- クレジットは任意だが、README / 本ファイルに出典を残すことを推奨する。
- 現時点の `tower_cannon.gltf` は Kenney 本体ではなく、ローダ配線確認用の代替メッシュである。

## 更新手順（予定）

1. CC0 モデルを `assets/models/` に配置
2. `js/render/main.js` の visual registry パスを更新
3. 本ファイルの表を実アセット名に書き換える
