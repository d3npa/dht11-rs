# rp2040 Dht11ドライバ

最近買ったDht11という名の気温・湿度センサーを、ラズパイpicoで使ってみたくて、ドライバーを作ってみました。本プロジェクトでは、rp pico wがwifiネットワークに接続し、tcpポート番号1234でサーバーを立ち上げて接続を待ち受けます。

tcpサーバーに接続すると、picoはセンサーからデータを読み取って、そのまま送ります（パケットはいつも5バイトです）

## ビルドする前に

ビルドする前に、環境変数に `WIFI_SSID` 及び `WIFI_PASSWORD` を定義する必要があります。プログラムに埋め込まれるためです。

```sh
echo WIFI_SSID="abaaba" >> .env
echo WIFI_PASSWORD="abaaba" >> .env
source .env
cargo build --release
```
