#!/bin/bash

# 检查 Node.js 是否安装
if ! command -v node &> /dev/null; then
    echo "😣 错误：Node.js 未安装！请先安装 Node.js (https://nodejs.org/)"
    exit 1
fi

# 检查 npm 是否安装
if ! command -v npm &> /dev/null; then
    echo "😣 错误：npm 未安装！请确保 npm 已随 Node.js 正确安装。"
    exit 1
fi

echo "😄 检测到 Node.js ($(node -v)) 和 npm ($(npm -v))"

# 检查当前目录是否存在 package.json
if [ ! -f "package.json" ]; then
    echo "😗 create package.json ..."

    # 写入 JSON 内容到 package.json
    cat <<EOF > package.json
{
  "dependencies": {
    "@tailwindcss/cli": "^4.1.11",
    "tailwindcss": "^4.1.11"
  }
}
EOF

    echo "😄 package.json ok"
else
    echo "😄 package.json ok"
fi
npm i -s
npx @tailwindcss/cli -i app/style/tailwind.css -o app/public/css/normalize.css -m
echo "😗 clear temp file"
rm -rf package*.json node_modules
echo "😄 css has created!"