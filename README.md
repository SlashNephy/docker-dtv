# docker-dtv

快適な DTV 視聴環境を構築するための よくばり docker-compose 構成例です。

## 前提

- Linux 環境を想定しており, また Docker, docker-compose が必要です。
- チューナーは PLEX 社の PX-W3PE4 での構成例になっています。ドライバは [px4_drv](https://github.com/nns779/px4_drv) を使用させて頂いています。
- ホストの pcscd (カードリーダーデーモン) は停止する必要があります。
- 録画データは `/mnt` 以下のマウントポイントに保存されることを前提としています。そのような構成ではない場合は `/mnt` ディレクトリの中に録画ストレージへのシンボリックリンクを貼ると便利です。
- WAN 越しでの使用は想定していません。

なお, 以下の環境で動作確認済です。

- ホスト : `Arch Linux 5.9.11-arch2-1`
- Docker : `version 19.03.14-ce, build 5eb3275d40`
- docker-compose : `version 1.27.4, build unknown`
- チューナー : `PLEX PX-W3PE4`
- カードリーダー : `NTT SCR3310-NTTCom`

## 使い方

```shell
# このリポジトリを clone
git clone https://github.com/SlashNephy/docker-dtv
cd docker-dtv

# 各種設定を調整
vi docker-compose.yml

# 起動
sudo docker-compose up -d
# ログ表示
sudo docker-compose logs -f
# 停止
sudo docker-compose down
```

## services

- mirakurun
  - `http://<ip address>:40772`
  - チューナーを管理する [Mirakurun](https://github.com/Chinachu/Mirakurun) です。
  - `recpt1` を `--enable-b25` オブションを付けてビルドしています。これにより `--b25 --strip` オプションが使用可能です。`decoder: arib-b25-stream-test` 指定でうまく行かない場合に代用してください。

- chinachu
  - `http://<ip address>:20772`
  - 番組の録画・予約を管理する [Chinachu](https://github.com/Chinachu/Chinachu) です。

- chinachu-proxy
  - `http://<ip address>:20773`
  - Chinachu へのアクセス制御に OAuth2 を使用できるようにするための [oauth2-proxy](https://github.com/oauth2-proxy/oauth2-proxy) です。
  - 設定リファレンスは [こちら](https://oauth2-proxy.github.io/oauth2-proxy/docs/configuration/oauth_provider) にあります。

- comskip
  - `.m2ts` ファイルの CM 位置を特定して CM 位置をテキストとして出力するための [Comskip](https://github.com/erikkaashoek/Comskip) です。
  - Chinachu での新着の録画番組を監視して自動で Comskip を実行します。
  - `./Comskip/comskip.ini` で Comskip の挙動をカスタマイズできます。
  - Kodi など対応しているプレイヤーでは `.m2ts` と同じディレクトリにあるテキストファイルを読み込み, CM をスキップして再生することができます。

- epgstation-mysql
  - EPGStation 用の MySQL サーバです。
  - 別のデータベースを使用するなど, このサービスが不要の場合はコメントアウトしてください。

- epgstation
  - `http://<ip address>:8888`
  - 番組の録画・予約を管理する [EPGStation](https://github.com/l3tnun/EPGStation) です。

- epgstation-proxy
  - `http://<ip address>:8889`
  - chinachu-proxy と同じなので省略します。

- samba
  - `<ip address>:445/tcp`
  - 録画データの共有用 Samba です。docker-compose.yml 内の build args で指定したユーザ名とパスワードで Samba にアクセスできます。
  - `/mnt` を読み取り専用で共有するように構成されています。
  - 各種ログは `./samba/logs` に保存されます。

- storage-watcher
  - `/mnt` 以下のマウントポイントのストレージの空き容量を監視します。
  - `LOW_SPACE_THRESHOLD_PERCENT`, `LOW_SPACE_THRESHOLD_GB` で空き容量のしきい値を設定でき, 下回った際に `DISCORD_WEBHOOK_URL` に Discord メッセージを送信します。

## 活用例

- Mirakurun : `http://<ip address>:40772`
- Chinachu (認証なし) : `http://<ip address>:20772`
- Chinachu (OAuth 2) : `http://<ip address>:20773`
- EPGStation (認証なし) : `http://<ip address>:8888`
- EPGStation (OAuth 2) : `http://<ip address>:8889`
- Samba : `<ip address>:445/tcp`

### Samba での録画ストレージ共有

Windows であればエクスプローラーに `\\<ip address>` (バックスラッシュ 2 個のあとに IP アドレス) を打ち込むことでアクセスできます。

### Kodi アドオン: pvr.epgstation

[pvr.epgstation](https://mzyy94.com/blog/2020/08/18/libreelec-mirakurun-epgstation/) を導入することをおすすめします。  
この構成例では予約・録画管理は Chinachu のみで行い, Kodi 上で EPG 情報を表示するために EPGStation を同梱しています。

### Kodi + Comskip

Kodi で Samba にアクセス・再生すると CM スキップの恩恵を受けることができます。
