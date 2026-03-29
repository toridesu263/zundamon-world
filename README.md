# ずんだもんわーるど

twitchの配信にvoicevoxの音声を乗せるためのツールです

## 目次

- [概要](#概要)
- [このリポジトリからダウンロードするもの](#このリポジトリからダウンロードするもの)
- [#別途準備が必要なもの](#別途準備が必要なもの)
- [外部ツールの設定](#外部ツールの設定)
- [ずんだもんわーるどの設定](#ずんだもんわーるどの設定)
- [使用手順](#使用手順)


## 概要

適切に設定ができた場合、以下の3つの方法でテキストを音声に変換して配信に乗せることができます

1. twitchのコメントを自動で読み上げる
2. コピーしたテキストをCtrl + Shift + F9でずんだもんに読み上げさせる
3. コピーしたテキストをCtrl + Shift + F10でめいめいひまりに読み上げさせる

## このリポジトリからダウンロードするもの 

このページのからto_useに移動して以下をダウンロードしてください
1. tts_copy.exe
2. tts_queue_player.exe
3. for_streamerbot.cs
4. config.toml

## 別途準備が必要なもの

- OBS Studio
- VOICEVOX
- VB-Audio Virtual Cable
- Streamerbot

## 外部ツールの設定

**よくわからない場合は、aiに聞きながらやっていくことを強く推奨します**

OBSに音声ソースを追加しましょう。デバイスは「CABLE Output (VB-Audio Virtual Cable)」を選んでください。

Streamerbotを起動して自分の配信チャンネルを登録しましょう

StreamerbotでActionパネルを右クリック -> add で新規actionを作りましょう

作成したactionを選択した状態でtrrigersパネルを右クリック-> add -> twitch -> chat -> chatmessageとしましょう

作成したactionを選択した状態でsubactionパネルを右クリック-> add -> Core -> C# ->Execute C# Codeとしましょう

Execute C# Codeで開かれた画面に、zundamon-world(このREADMEが置いてあるページ)からダウンロードしたfor_streamerbot.csの中身をコピペして完全に書き換えてください。*この際、pathとpathOfTextの「とりです」の部分は自分のユーザー名に書き換えてください

書き換え終わったら、ウィンドウの左下のfind refsをクリックしてから、compileをクリックして、ウィンドウの右側のSaveAndCompileをクリックしてください

これで外部ツールの設定は終わりです。お疲れさまでした

## ずんだもんわーるどの設定

デスクトップに「zundamon-world」という名前のフォルダを作ってください

その中にtts_copy.exeとtts_queue_player.exeとconfig.tomlを置いてください

config.tomlの中身の「とりです」の部分を自分のユーザー名に書き換えてください

これで事前準備は終わりです。お疲れ様でした。

## 使用手順

1.OBSを起動(必須)
2.streamerbot起動(必須)
3.VOICEVOXを起動(必須)
4.zundamon-worldの中のtts_queue_player.exeをダブルクリック
5.zundamon-worldの中のtts_copy.exeをダブルクリック
これで起動は完了です

以下、具体的な例です
テキストをコピーしてctrl + shift + F9でずんだもんがしゃべります結構ラグあります
テキストをコピーしてctrl + shift + F10でめいめいひまりがしゃべります結構ラグあります
コメントが来たら雨晴はうがしゃべります結構ラグあります