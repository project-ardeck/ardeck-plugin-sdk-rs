# Ardeck plugin SDK for Rust
Ardeck-studioを便利に利用するためのプラグインを開発するための、ツールとサンプルです。

# 利用方法
pluginを開発するあなたが編集するファイルはすべてpluginディレクトリ下に配置されており、その構造は以下のようになっています。
```
plugin/
├── src/
│   └── main.rs
├── actions.json
├── manifest.json
├── build.bat
└── Cargo.toml
```

## manifest.json
`manifest.json` は、プラグインの情報をardeck-studioに提供するものであり、以下のような内容です
```json
{
    "name": "PLUGIN_NAME", // プラグインの表示名
    "version": "0.1.0", // プラグインのバージョン
    "id": "6ddf86cb-013b-4545-9ff0-854ca396ee6e", // プラグインのID
    "description": "Examples of ardeck-studio plugins.", // プラグインの説明
    "author": "Project Ardeck", // プラグインの著者
    "main": "main.exe" // 実行ファイルの名称
}
```
### 記述時の注意点
- `version`: 現在は特に指定はありませんが、セマンティックバージョニングに従うことを推奨します。
- `id`: 現在は特に指定はありませんが、`UUIDv4` を用いることを推奨します。
- `description`, `author`: 記述しなくても良いですが、ユーザビリティの向上のために記述することを推奨します。
- `main`: 基本的に、`main.exe` としておくほうが良いです。（現在のバージョンでは、`build.bat` 内で`main.exe` という名前で`dist` ディレクトリにコピーするように実装されています。）

## action.json
`action.json` は、プラグインで実装するactionの名称とIDを指定するもので、以下のような内容です。
```json
[
    {
        "id": "hello",
        "name": "Hello",
        "description": "Print 'Hello Ardeck!'"
    },
    ...
]
```
### 記述時の注意点
- `description`: 記述しなくても良いですが、ユーザビリティの向上のために記述することを推奨します。

## main.rs
プラグインの開発者は`main.rs` の2行目のCopyrightの部分に必要な情報を記入する必要があります。
```rust main.rs
/*
Copyright (C) <year> <name of author>
...
```
例:
```rust
/*
Copyright (C) 1999 Jon Doe
...
```

### 記述
コードは以下のように記述します。
```rust 
use lib_plugin::ardeck_plugin::ArdeckPlugin;

#[tokio::main]
async fn main() {
    // 初期化
    let mut plugin = ArdeckPlugin::new().await;

    // ここにアクションが起こった時に実行する処理を記述
    // ...

    // アクションの受信を開始
    plugin.start_listening().await;
}
```

`add_action_handler`を用いてアクションが起こった時に実行する処理を記述します。
先ほどの例では、`action.json`に`hello`というアクションを記述しました。この場合、以下のように記述します。
```rust
plugin.add_action_handler("hello", |switch_info| {
    println!("Hello Ardeck!");
});
```

すべてのハンドラーを登録し、プラグインの動作に必要な処理のあと、一番最後に`start_listening()`を呼び出して、Ardeck Studioからのデータの受信を待機します。

# ビルド
plugin ディレクトリ内にある`build.bat` を実行します。
```shell
.\build.bat
```

# ライセンス
当プロジェクトは GPL-3.0 ライセンスに基づいて付与されています。
