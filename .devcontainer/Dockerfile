# Debian - DevContiner公式イメージ
# * GitHub | devcontainer/cpp : https://github.com/devcontainers/images/tree/main/src/base-debian
FROM mcr.microsoft.com/devcontainers/base:debian

# [オプション] 追加パッケージのインストール
#
# 使い方 :
#  * 「&& apt-get -y install --no-install-recommends <install packages>\」
#    のコメントアウトを外し、<install packages>の部分にパッケージを追加
#
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get upgrade -y \
    # && apt-get -y install --no-install-recommends <install packages> \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ARG USERNAME=vscode
# USER - https://docs.docker.jp/engine/reference/builder.html#user
# コンテナ内で実行されるコマンドのユーザーを、指定された$USERNAMEに切り替え
USER $USERNAME
# WORKDIR - https://docs.docker.jp/engine/reference/builder.html#workdir
# コンテナ内のワーキングディレクトリを、指定されたユーザーのホームディレクトリ（/home/$USERNAME/）に設定
WORKDIR /home/$USERNAME/

# Rustのインストールスクリプトをダウンロードして実行
# https://forge.rust-lang.org/infra/other-installation-methods.html
#  * curl コマンドのオプション
#   * --proto '=https': 通信プロトコルをHTTPSに制限
#   * --tlsv1.2       : TLS v1.2を使用して通信
#   * -s              : サイレントモードで実行し、プログレスバーを非表示
#   * -S              : エラーが発生した場合にメッセージを表示
#   * -f              : サーバーがエラーを返した場合（404 Not Foundなど）に、処理を中止
#  * ダウンロードしたスクリプトはパイプでshコマンドに渡され実行
#  * sh コマンドのオプション
#   * -s: スクリプトを標準入力から読み込んで実行
#   * --: オプションの終わりを示します。以降の引数はオプションとして解釈されない
#   * -y: インストールプロセスでのプロンプトに自動で「yes」を回答し、インストールを進める
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
