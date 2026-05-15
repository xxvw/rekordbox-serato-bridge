# rekordbox-serato-bridge

> DJ ライブラリのエクスポートを **rekordbox** と **Serato** の間で変換するブリッジ。

`rsb` はトラックメタデータ・Hot Cue・Beatgrid・Memory Cue を 2 大プロ DJ エコシステム間で行き来させるための単体 CLI。以下の入出力に対応予定:

| フォーマット | 読み込み | 書き出し | 補足 |
|--------------|----------|----------|------|
| `rekordbox-xml` | 計画中 | 計画中 | rekordbox.app からの `DJ_PLAYLISTS` XML エクスポート |
| `rekordbox-usb` | 計画中 | 計画中 | CDJ-2000 互換の USB レイアウト (`PIONEER/` + `Contents/`) |
| `serato` | 計画中 | 計画中 | 楽曲ファイル内の ID3 `GEOB` フレーム |

GUI ホストから意図的に分離している。[Conduction](https://github.com/xxvw/Conduction) の内部変換バックエンドにも使われるが、CLI 単体で完結する。

ライセンス: [MIT](./LICENSE)
English: [README.md](./README.md)

---

## ステータス

プレアルファ。CLI のサブコマンド (`convert` / `inspect`) は置いてあるが、実変換はまだ繋ぎ込んでいない。

---

## インストール

Rust stable (`rust-toolchain.toml` で固定) が必要。

```bash
# ソースから実行
cargo run -- --help

# `rsb` バイナリを入れる
cargo install --path .
```

---

## 使い方

```bash
# rekordbox XML を Serato タグ付きフォルダに変換
rsb convert \
  --from rekordbox-xml \
  --to   serato \
  --input  ./rekordbox.xml \
  --output ./music/

# Serato タグ付きフォルダを rekordbox XML に戻す
rsb convert \
  --from serato \
  --to   rekordbox-xml \
  --input  ./music/ \
  --output ./rekordbox.xml

# 何も書き出さずに中身だけ確認
rsb inspect --format rekordbox-xml --input ./rekordbox.xml
```

`convert` に `--dry-run` を付けると、書き込みを行わずに計画だけを出力する。

---

## 設計

- **シングルバイナリ、デーモンなし**。変換はステートレス。
- **可能な限りロスレス**。変換先が変換元の概念を持たないとき (例: typed Cue の Drop / Intro)、サイレントに捨てるのではなくレポートに損失として記録する。
- **音声解析はしない**。Beatgrid / 波形はパススルー。生成・再解析が必要な場合は [Conduction](https://github.com/xxvw/Conduction) 等を併用する。

---

## ロードマップ

1. rekordbox XML reader + writer + ラウンドトリップ fixture
2. Serato GEOB タグ reader + writer
3. rekordbox-xml ↔ serato ブリッジ (主目的)
4. [`rekordcrate`](https://github.com/Holzhaus/rekordcrate) を使った rekordbox USB (PIONEER/ANLZ + PDB) reader / writer
5. 実 fixture によるロスレス・ラウンドトリップの結合テスト

---

## ライセンス

MIT — [`LICENSE`](./LICENSE) を参照。
